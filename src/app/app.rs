use crate::api_type::api_type::{AccountDetail, Playlist, PlaylistDetail, Track};
use crate::util::network;
use crate::util::StatefulList;
use crossterm::event::KeyCode;
use reqwest::blocking::Client;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

#[derive(PartialEq, Eq)]
pub enum Focus {
    Playlist,
    Track,
}

#[derive(PartialEq, Eq)]
pub enum Route {
    Login,         // 登陆页面
    Home,          // 主页面
    Search,        // 搜索页面
    MusicAnalysis, // 音乐播放详情页面
}

pub struct PlayerController {
    pub is_pause: bool,
    pub seek: i32,
    pub volume: f32,
}

pub struct App<'a> {
    // 路由
    pub route: Route,
    callbacks: Vec<Box<dyn FnMut()>>,
    // 系统运行总时间
    pub system_tick: u64,

    pub client: Client,

    pub title: &'a str,
    pub should_quit: bool,
    pub progress: f64,

    // user
    pub userinfo: Option<AccountDetail>,
    pub playlists: Vec<Playlist>,
    pub playlists_state: StatefulList<Playlist>,
    pub playing_playlist_idx: usize,

    pub selected_playlist_index: usize,
    pub current_playlist: Option<PlaylistDetail>,
    pub current_playlist_track_state: StatefulList<Track>,
    pub cookie: String,
    pub likelist: Vec<i64>,

    pub focus: Focus,

    current_track_idx: usize,
    current_tracks: Vec<Track>,

    pub handle: &'a OutputStreamHandle,
    pub sink: Sink,

    pub player_controller: PlayerController,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, handle: &'a OutputStreamHandle) -> Self {
        Self {
            route: Route::Login,
            callbacks: vec![],
            system_tick: 0,
            client: Client::builder().cookie_store(true).build().unwrap(),
            title,
            should_quit: false,
            progress: 0.0,
            userinfo: Option::None,
            playlists: Vec::new(),
            playlists_state: StatefulList::with_items(vec![]),
            playing_playlist_idx: 0,
            current_playlist: Option::None,
            current_playlist_track_state: StatefulList::with_items(vec![]),
            selected_playlist_index: 0,
            cookie: String::new(),
            likelist: vec![],

            focus: Focus::Playlist,

            current_tracks: vec![],
            current_track_idx: 0,

            handle,
            sink: Sink::try_new(handle).unwrap(),
            player_controller: PlayerController {
                is_pause: true,
                seek: 0,
                volume: 1.0,
            },
        }
    }

    pub fn set_playlists(&mut self, list: Vec<Playlist>) {
        self.playlists_state = StatefulList::with_items(list);
        self.playlists_state.next();
    }

    pub fn set_current_playlist(&mut self, list: Vec<Track>) {
        self.current_playlist_track_state = StatefulList::with_items(list);
        self.current_playlist_track_state.next();
    }

    pub fn on_up(&mut self) {
        match self.focus {
            Focus::Playlist => self.playlists_state.previous(),
            Focus::Track => self.current_playlist_track_state.previous(),
        }
    }

    pub fn on_down(&mut self) {
        match self.focus {
            Focus::Playlist => self.playlists_state.next(),
            Focus::Track => self.current_playlist_track_state.next(),
        }
    }

    pub fn on_right(&mut self) {
        match self.focus {
            Focus::Playlist => self.focus = Focus::Track,
            Focus::Track => self.focus = Focus::Playlist,
        }
    }

    pub fn on_left(&mut self) {
        // self.tabs.previous();
        match self.focus {
            Focus::Playlist => self.focus = Focus::Track,
            Focus::Track => self.focus = Focus::Playlist,
        }
    }

    pub fn on_enter(&mut self) {
        match self.focus {
            Focus::Playlist => {
                let _ = network::get_playlist_detail(self);
                self.on_left();
            }
            Focus::Track => {
                let track = self.current_playlist_track_state.items[self
                    .current_playlist_track_state
                    .state
                    .selected()
                    .unwrap_or(0)]
                .clone();
                self.current_track_idx = self
                    .current_playlist_track_state
                    .state
                    .selected()
                    .unwrap_or(0);
                self.playing_playlist_idx = self.playlists_state.state.selected().unwrap_or(0);
                self.download_and_play_track(track);
                self.current_tracks = self.current_playlist_track_state.items.clone();
            }
        }
    }

    fn download_and_play_track(&mut self, track: Track) {
        let id = track.id;
        let track_name = track.name.clone();
        if let Ok(v) = network::get_music_detail(vec![id], self) {
            if v.len() > 0 {
                let music_url = v[0].url.clone();
                let vec: Vec<&str> = music_url.rsplitn(2, ".").collect();
                let file_type = vec.first().unwrap_or(&&"mp3");
                let current_music_path = network::download_music(
                    music_url.as_ref(),
                    format!("{}.{}", track_name, file_type).as_ref(),
                );
                if let Some(path) = current_music_path {
                    self.play_music(&path);
                }
            }
        }
    }

    fn play_music(&mut self, path: &String) {
        let file = std::fs::File::open(path).unwrap();
        let buf = std::io::BufReader::new(file);
        // 停止当前播放
        if !self.sink.empty() {
            self.sink.stop();
            self.sink = Sink::try_new(self.handle).unwrap();
        }
        self.player_controller.is_pause = false;
        self.player_controller.seek = 0;
        self.sink.set_volume(self.player_controller.volume);
        self.sink.append(Decoder::new(buf).unwrap());
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            ' ' => {
                self.on_space();
            }
            '-' => {
                let volumn = self.sink.volume();
                if volumn > 0.0 {
                    self.sink.set_volume(volumn - 0.01);
                }
            }
            '=' => {
                let volumn = self.sink.volume();
                if volumn < 1.0 {
                    self.sink.set_volume(volumn + 0.01);
                }
            }
            _ => {}
        }
    }

    pub fn on_space(&mut self) {
        self.player_controller.is_pause = !self.player_controller.is_pause;
        if self.player_controller.is_pause {
            self.sink.pause();
        } else {
            self.sink.play();
        }
    }

    pub fn on_tick(&mut self) {
        self.system_tick += 1;
        if self.player_controller.is_pause {
            return;
        }
        if self.sink.empty() {
            self.player_controller.is_pause = true;
            self.on_ctrl_key(KeyCode::Right);
        }
        if !self.player_controller.is_pause {
            self.player_controller.seek += 1;
        }
    }

    pub fn is_liked(&self, id: &i64) -> bool {
        self.likelist.contains(id)
    }
}

/// 当前状态访问
impl<'a> App<'a> {
    pub fn current_track(&self) -> &Track {
        &(self.current_playlist_track_state.items[self
            .current_playlist_track_state
            .state
            .selected()
            .unwrap_or(0)])
    }

    pub fn current_playing_track(&self) -> Option<&Track> {
        if self.current_tracks.is_empty() {
            Option::None
        } else {
            Option::Some(&self.current_tracks[self.current_track_idx])
        }
    }

    pub fn current_playing_playlist(&self) -> &Playlist {
        &(self.playlists_state.items[self.playing_playlist_idx])
    }

    pub fn current_playlist(&self) -> &Playlist {
        &(self.playlists_state.items[self.playlists_state.state.selected().unwrap_or(0)])
    }
}

/// 播放控制
impl<'a> App<'a> {
    pub fn on_ctrl_key(&mut self, code: KeyCode) {
        match code {
            // 上一首
            KeyCode::Left => self.previous_track(),
            // 下一首
            KeyCode::Right => self.next_track(),
            // 音量加
            KeyCode::Up => self.volume_up(),
            // 音量减
            KeyCode::Down => self.volume_down(),
            // 喜欢音乐
            KeyCode::Char('l') => self.like(),
            // 打开/关闭歌词
            KeyCode::Char('d') => self.show_lrc(),
            // 帮助
            KeyCode::Char('h') => self.show_help(),
            _ => {}
        }
    }

    // 播放上一首
    pub fn previous_track(&mut self) {
        let len = self.current_tracks.len();
        if len > 0 {
            self.current_track_idx = (self.current_track_idx + len - 1) % len;
            self.download_and_play_track(self.current_tracks[self.current_track_idx].clone());
        }
    }

    // 播放下一首
    pub fn next_track(&mut self) {
        let len = self.current_tracks.len();
        if len > 0 {
            self.current_track_idx = (self.current_track_idx + 1) % len;
            self.download_and_play_track(self.current_tracks[self.current_track_idx].clone());
        }
    }

    // 音量加
    pub fn volume_up(&mut self) {
        let volume = self.sink.volume();
        if volume < 1.0 {
            self.sink.set_volume(volume + 0.01);
            self.player_controller.volume += 0.01;
        }
    }

    // 音量减
    pub fn volume_down(&mut self) {
        let volume = self.sink.volume();
        if volume > 0.0 {
            self.sink.set_volume(volume - 0.01);
            self.player_controller.volume -= 0.01;
        }
    }

    // 喜欢音乐
    pub fn like(&mut self) {}

    // 打开/关闭歌词
    pub fn show_lrc(&mut self) {}

    //  显示帮助
    pub fn show_help(&mut self) {}
}

// 路由
impl<'a> App<'a> {
    pub fn goto_page(&mut self, route: Route) {
        self.route = route;
    }
}

// 回调函数

impl<'a> App<'a> {
    pub fn register<F>(&mut self, mut func: F)
    where
        F: FnMut(),
    {
        // self.callbacks.push(Box::new(func));
    }
}

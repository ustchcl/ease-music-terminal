use crate::api_type::api_type::{AccountDetail, Playlist, PlaylistDetail, Track};
use crate::util::network;
use crate::util::StatefulList;
use reqwest::blocking::Client;
use rodio::{Decoder, OutputStream, Sink};

#[derive(PartialEq, Eq)]
pub enum Focus {
    Playlist,
    Track,
}

pub struct PlayerController {
    pub is_pause: bool,
    pub seek: i32,
}

pub struct App<'a> {
    pub client: Client,

    pub title: &'a str,
    pub should_quit: bool,
    pub progress: f64,

    // user
    pub userinfo: Option<AccountDetail>,
    pub playlists: Vec<Playlist>,
    pub playlists_state: StatefulList<Playlist>,
    pub selected_playlist_index: usize,
    pub current_playlist: Option<PlaylistDetail>,
    pub current_playlist_track_state: StatefulList<Track>,
    pub cookie: String,

    pub focus: Focus,

    current_music_path: Option<String>,

    pub sink: &'a Sink,

    pub player_controller: PlayerController,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, sink: &'a Sink) -> Self {
        Self {
            client: Client::builder().cookie_store(true).build().unwrap(),
            title,
            should_quit: false,
            progress: 0.0,
            userinfo: Option::None,
            playlists: Vec::new(),
            playlists_state: StatefulList::with_items(vec![]),
            current_playlist: Option::None,
            current_playlist_track_state: StatefulList::with_items(vec![]),
            selected_playlist_index: 0,
            cookie: String::new(),

            focus: Focus::Playlist,

            current_music_path: Option::None,

            sink,

            player_controller: PlayerController { is_pause: true, seek: 0 },
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

    pub fn on_space(&mut self) {
        self.player_controller.is_pause = !self.player_controller.is_pause;
        if self.player_controller.is_pause {
            self.sink.pause();
        } else {
            self.sink.play();
        }
    }

    pub fn on_enter(&mut self) {
        match self.focus {
            Focus::Playlist => {
                let _ = network::get_playlist_detail(self);
                self.on_left();
            }
            Focus::Track => {
                let track = self.current_track();
                let id = track.id;
                if let Ok(v) = network::get_music_detail(vec![id], self) {
                    if v.len() > 0 {
                        let music_url = &v[0].url;
                        let vec: Vec<&str> = music_url.rsplitn(2, ".").collect();
                        let file_type = vec.first().unwrap_or(&&"mp3");
                        self.current_music_path = network::download_music(
                            music_url.as_ref(),
                            format!("{}.{}", track.name, file_type).as_ref(),
                        );
                        if let Some(path) = &self.current_music_path {
                            self.player_controller.is_pause = false;
                            self.play_music(path);
                        }
                    }
                }
            }
        }
    }

    pub fn current_track(&self) -> &Track {
        &(self.current_playlist_track_state.items[self
            .current_playlist_track_state
            .state
            .selected()
            .unwrap_or(0)])
    }

    pub fn current_playlist(&self) -> &Playlist {
        &(
            self.playlists_state.items[self.playlists_state.state.selected().unwrap_or(0)]
        )
    }

    fn play_music(&self, path: &String) {
        let file = std::fs::File::open(path).unwrap();
        let buf = std::io::BufReader::new(file);
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

    pub fn on_tick(&mut self) {
        // Update progress
        if !self.player_controller.is_pause {
            self.player_controller.seek += 1;
        }

        // self.sparkline.on_tick();
        // self.signals.on_tick();

        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);

        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }
}

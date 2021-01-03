use crate::api_type::api_type::{AccountDetail, Playlist, PlaylistDetail, Track};
use crate::util::StatefulList;
use crate::util::network;
use reqwest::blocking::Client;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Playlist,
    Track
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
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
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
        let _ = network::get_playlist_detail(self);
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        // self.sparkline.on_tick();
        // self.signals.on_tick();

        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);

        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }
}
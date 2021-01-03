// use std::io::BufReader;
use rodio::{Sink, OutputStream, Decoder};
// use std::fs::File;
mod downloader;
use dirs;
mod app;
#[allow(dead_code)]
mod util;
mod api_type;
use anyhow::Result;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};
use crate::app::{ui, App};
use crate::util::network;
use std::{
    error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use argh::FromArgs;

enum Event<I> {
    Input(I),
    Tick,
}

/// Crossterm demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "1000")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}
fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(cli.tick_rate);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    

    let mut app = App::new("Ease Music Termianl");
    network::login(&mut app)?;
    network::playlists(&mut app)?;
    network::get_playlist_detail(&mut app)?;
    terminal.clear()?;
    
    loop {
        terminal.draw(|f| ui::draw_main_page(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                KeyCode::Enter => app.on_enter(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
    }

    Ok(())
}   

// // #[tokio::main]
// fn nss() -> io::Result<()> {
//     let filepath = dirs::audio_dir().map(|p| p.join("music.flac")).unwrap();
//     println!("filepath = {}", filepath.to_str().unwrap());
//     let music_flac = "http://m801.music.126.net/20210101180909/627de66f030afed918543495211e574e/jdymusic/obj/wo3DlMOGwrbDjj7DisKw/4779027660/d9e2/322a/bcbe/bad0ba4cafe18cbc02ccae6ae2d6e98f.flac";
//     let _ = downloader::Downloader::download(music_flac, filepath);
//     let (_stream, handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&handle).unwrap();
//     // let file = File::open("./music.mp3").await?;
//     // let buf = BufReader::new(file);
//     // let mut stream = TcpStream::connect(music_flac).unwrap();
//     let file2 = std::fs::File::open("./music.mp3").unwrap();
//     let buf2 = std::io::BufReader::new(file2);
//     sink.append(Decoder::new(buf2).unwrap());
//     sink.sleep_until_end();
//     Ok(())
// }



// use std::io::BufReader;
use rodio::{Sink, OutputStream, Decoder};
// use std::fs::File;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, BufReader};
use std::net::TcpStream;
mod downloader;
use dirs;
mod app;
use crate::app::{ui, App};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use std::{
    error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new("Crossterm Demo");
    terminal.clear()?;
    terminal.draw(|f| ui::draw(f, &mut app))?;

    Ok(())
}   

// #[tokio::main]
fn nss() -> io::Result<()> {
    let filepath = dirs::audio_dir().map(|p| p.join("music.flac")).unwrap();
    println!("filepath = {}", filepath.to_str().unwrap());
    let music_flac = "http://m801.music.126.net/20210101180909/627de66f030afed918543495211e574e/jdymusic/obj/wo3DlMOGwrbDjj7DisKw/4779027660/d9e2/322a/bcbe/bad0ba4cafe18cbc02ccae6ae2d6e98f.flac";
    let _ = downloader::Downloader::download(music_flac, filepath);
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    // let file = File::open("./music.mp3").await?;
    // let buf = BufReader::new(file);
    // let mut stream = TcpStream::connect(music_flac).unwrap();
    let file2 = std::fs::File::open("./music.mp3").unwrap();
    let buf2 = std::io::BufReader::new(file2);
    sink.append(Decoder::new(buf2).unwrap());
    sink.sleep_until_end();
    Ok(())
}



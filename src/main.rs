// use std::io::BufReader;
use rodio::{Sink, OutputStream, Decoder};
// use std::fs::File;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, BufReader};

#[tokio::main]
async fn main() -> io::Result<()> {
    let music_flac = "http://m801.music.126.net/20210101180909/627de66f030afed918543495211e574e/jdymusic/obj/wo3DlMOGwrbDjj7DisKw/4779027660/d9e2/322a/bcbe/bad0ba4cafe18cbc02ccae6ae2d6e98f.flac";
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let file = File::open("./music.flac").await?;
    sink.append(Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
    Ok(())
}



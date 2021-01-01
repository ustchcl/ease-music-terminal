use std::io::BufReader;
use rodio::{Sink, OutputStream, Decoder};
use std::fs::File;

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let file = File::open("./music.flac").unwrap();
    sink.append(Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}



use error_chain::*;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::fs::File;
use std::str::FromStr;
use std::io::copy;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Header(reqwest::header::ToStrError);
    }
}

struct PartialRangeIter {
  start: u64,
  end: u64,
  buffer_size: u32,
}

impl PartialRangeIter {
  pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
    if buffer_size == 0 {
      Err("invalid buffer_size, give a value greater than zero.")?;
    }
    Ok(PartialRangeIter {
      start,
      end,
      buffer_size,
    })
  }
}

impl Iterator for PartialRangeIter {
  type Item = HeaderValue;
  fn next(&mut self) -> Option<Self::Item> {
    if self.start > self.end {
      None
    } else {
      let prev_start = self.start;
      self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
      Some(
        HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
          .expect("string provided by format!"),
      )
    }
  }
}

pub struct Downloader {}

impl Downloader {
  pub fn download(url: &str, filepath: std::path::PathBuf) -> Result<()> {
    let mut output_file = match File::create(filepath) {
      Err(why) => panic!("couldn't create {}", why),
      Ok(file) => file,
    };
    let response = reqwest::blocking::get(url)?.text()?;
    copy(&mut response.as_bytes(), &mut output_file);
    Ok(())
  }
}

use std::fmt::Display;
use std::iter::{self, FromIterator};

pub fn pre_format<T>(i: T, len: usize, prefix: char) -> String
where
    T: Display,
{
    let s = i.to_string();
    if s.len() >= len {
        s
    } else {
        let pre = iter::repeat(prefix).take(len - s.len());
        format!("{}{}", String::from_iter(pre), i)
    }
}

pub fn show_duration(duration: i32) -> String {
    let hour_duration = 3600000;
    let minute_duration = 60000;
    let second_duration = 1000;
    let hour = duration / hour_duration;
    let minute = (duration % hour_duration) / minute_duration;
    let second = (duration % minute_duration) / second_duration;

    if hour > 0 {
        format!(
            "{}:{}:{}",
            hour,
            pre_format(minute, 2, '0'),
            pre_format(second, 2, '0')
        )
    } else {
        format!(
            "{}:{}",
            pre_format(minute, 2, '0'),
            pre_format(second, 2, '0')
        )
    }
}

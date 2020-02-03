extern crate chrono;
use chrono::Local;

pub fn now<T: std::fmt::Display>(t: T) -> String {
    let date = Local::now();
    format!("{} {}", date.format("[%H:%M:%S]"), t)
}

extern crate livetools;

use chrono::{Local};
use std::time::{Duration};
use std::io::prelude::*;
use std::io;
use std::thread;

use livetools::{CLEAR_LINE};

fn live_date() {
    loop {
        let now = Local::now();
        let time_fmt = "%Y-%m-%d %H:%M:%S%.3f %Z";
        //let time_fmt = "%a %b %e %T %Z %Y";
        //let time_fmt = "%+";
        let time_str = now.format(time_fmt).to_string();
        io::stdout().write(
            format!("{}{}", CLEAR_LINE, time_str).as_bytes()
            ).expect("Write failed");
        io::stdout().flush().expect("Flush failed");

        let nanos_elapsed = Local::now().timestamp_subsec_nanos() % 1_000_000_000;
        let nanos_until_next_full_second = 1_000_000_000 - nanos_elapsed;
        thread::sleep(Duration::new(0, nanos_until_next_full_second));
    }
}

fn main() {
    live_date()
}

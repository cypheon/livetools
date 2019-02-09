use chrono::{Local};
use std::time::{Duration, Instant};
use std::io::prelude::*;
use std::io;
use std::thread;

const CLEAR_LINE: &'static str = "\r\x1b[K";

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

#[derive(Debug, Copy, Clone)]
struct WordCount {
    bytes: u64,
    lines: u64,
    words: u64,
    in_word: bool,
}

fn is_posix_whitespace(c: u8) -> bool {
    match c {
        | b' '
        | 0x0c // form feed
        | 0x0b // vertical tab
        | b'\t'
        | b'\n'
        | b'\r'
        => {true}
        _ => {false}
    }
}

fn is_word_char(c: u8) -> bool {
    !is_posix_whitespace(c)
    // !c.is_ascii_whitespace();
}

fn update_wordcount(s: &WordCount, buf: &[u8]) -> WordCount {
    let mut upd = *s;
    upd.bytes += buf.len() as u64;
    buf.iter().for_each(|b|
                        {
                            if *b == b'\n' {
                                upd.lines += 1;
                            }
                            if is_word_char(*b) {
                                if !upd.in_word {
                                    upd.in_word = true;
                                    upd.words += 1;
                                }
                            } else {
                                upd.in_word = false;
                            }
                        }
                       );
    return upd;
}

fn format_wordcount(wc: &WordCount) -> String {
    return format!("{:8} {:7} {:8}", wc.lines, wc.words, wc.bytes);
}

fn output_wordcount(wc: &WordCount) {
    let wc_str = format_wordcount(wc);
    io::stdout().write(
        format!("{}{}", CLEAR_LINE, wc_str).as_bytes()
        ).expect("Write failed");
    io::stdout().flush().expect("Flush failed");
}

fn live_wordcount() {
    let mut wc = WordCount {
        bytes: 0,
        lines: 0,
        words: 0,
        in_word: false,
    };

    let mut buffer = vec![0; 1024 * 1024].into_boxed_slice();
    let mut f = io::stdin();
    let mut last_update: Option<Instant> = None;
    loop {
        let read = f.read(&mut buffer).expect("Read error");
        if read == 0 {
            break
        }
        wc = update_wordcount(&wc, &buffer[0..read]);

        if last_update.map(|i| i.elapsed().as_secs()).unwrap_or(1) >= 1 {
            output_wordcount(&wc);
            last_update = Some(Instant::now());
        }
    }
    output_wordcount(&wc);
    io::stdout().write(b"\n");
    io::stdout().flush();
}

fn main() {
    //live_date()
    live_wordcount()
}

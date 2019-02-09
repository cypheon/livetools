extern crate livetools;
extern crate getopts;

use getopts::Options;
use std::env;
use std::time::{Instant};
use std::io::prelude::*;
use std::io;

use livetools::*;

#[derive(Debug, Copy, Clone)]
struct WordCount {
    bytes: u64,
    lines: u64,
    words: u64,
    in_word: bool,
}

#[derive(Debug, Copy, Clone)]
struct WordCountConfig {
    count_bytes: bool,
    count_lines: bool,
    count_words: bool,
}

impl WordCountConfig {
    pub fn new() -> Self {
        WordCountConfig {
            count_bytes: false,
            count_lines: false,
            count_words: false,
        }
    }
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

fn update_wordcount_chunk(cfg: &WordCountConfig, s: &WordCount, buf: &[u8]) -> WordCount {
    let mut upd = *s;
    upd.bytes += buf.len() as u64;
    if cfg.count_words || cfg.count_lines {
        buf.iter().for_each(|b|
                            {
                                if cfg.count_lines {
                                    if *b == b'\n' {
                                        upd.lines += 1;
                                    }
                                }
                                if cfg.count_words {
                                    if is_word_char(*b) {
                                        if !upd.in_word {
                                            upd.in_word = true;
                                            upd.words += 1;
                                        }
                                    } else {
                                        upd.in_word = false;
                                    }
                                }
                            }
                           );
    }
    return upd;
}

fn format_wordcount(cfg: &WordCountConfig, wc: &WordCount) -> String {
    let mut output = String::with_capacity(64);

    if cfg.count_lines {
        output.push_str(&format!(" {:7}", wc.lines));
    }
    if cfg.count_words {
        output.push_str(&format!(" {:7}", wc.words));
    }
    if cfg.count_bytes {
        output.push_str(&format!(" {:7}", wc.bytes));
    }

    return output;
}

fn output_wordcount(cfg: &WordCountConfig, wc: &WordCount) {
    let wc_str = format_wordcount(cfg, wc);
    io::stdout().write(
        format!("{}{}", CLEAR_LINE, wc_str).as_bytes()
        ).expect("Write failed");
    io::stdout().flush().expect("Flush failed");
}

fn live_wordcount(cfg: &WordCountConfig) {
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
        wc = update_wordcount_chunk(cfg, &wc, &buffer[0..read]);

        if last_update.map(|i| i.elapsed().as_secs()).unwrap_or(1) >= 1 {
            output_wordcount(cfg, &wc);
            last_update = Some(Instant::now());
        }
    }
    output_wordcount(cfg, &wc);
    io::stdout().write(b"\n").expect("Output Error");
    io::stdout().flush().expect("Output Error");
}

fn print_usage(program: &String, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];//.clone();

    let mut cfg = WordCountConfig::new();
    let mut noflags = true;
    let mut opts = Options::new();
    opts.optflag("c", "bytes", "Print count of bytes");
    opts.optflag("l", "lines", "Print count of lines");
    opts.optflag("w", "words", "Print count of words");
    opts.optflag("h", "help", "Show usage");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(program, &opts);
            println!("\nError parsing arguments:\n{}", f.to_string());
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(program, &opts);
        return;
    }

    if matches.opt_present("c") {
        cfg.count_bytes = true;
        noflags = false;
    }
    if matches.opt_present("l") {
        cfg.count_lines = true;
        noflags = false;
    }
    if matches.opt_present("w") {
        cfg.count_words = true;
        noflags = false;
    }
    if noflags {
        cfg.count_bytes = true;
        cfg.count_lines = true;
        cfg.count_words = true;
    }

    live_wordcount(&cfg)
}

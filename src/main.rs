use std::fs::File;
use std::io::Read;

extern crate ansi_term;
extern crate clap;

use ansi_term::Colour::{Blue, Green};
use clap::{Arg, App};

// FileStats holds information about an individual file
struct FileStats {
    name: String,
    lines: usize,
    words: usize,
    characters: usize,
}

// WcStats holds information for all the files
struct WcStats {
    stats: Vec<FileStats>,
    number_of_files: usize,
    total: FileStats,
    line_flag: bool,
    word_flag: bool,
    char_flag: bool,
}

impl WcStats {
    // new creates an emtpy WcStats instance
    fn new() -> WcStats {
        WcStats {
            stats: Vec::new(),
            number_of_files: 0,
            total: FileStats{name: String::from(""), lines: 0, words: 0, characters: 0},
            line_flag: false,
            word_flag: false,
            char_flag: false,
        }
    }

    // get_stats collects and stores the file stats
    fn get_stats(&mut self, file: &str) -> Result<(), String> {
        let filename = file.to_string();

        match File::open(file) {
            Ok(mut fd) => {
                let mut contents = String::new();
                match fd.read_to_string(&mut contents) {
                    Ok(_) => {
                        let lines: Vec<&str> = contents.lines().collect();
                        let words: Vec<&str> = contents.split_ascii_whitespace().collect();

                        self.total.lines += lines.len();
                        self.total.words += words.len();
                        self.total.characters += contents.len();
                        self.stats.push(FileStats{
                            name: filename, lines: lines.len(), words: words.len(), characters: contents.len()
                        });
                        Ok(())
                    }
                    Err(e) => {
                        Err(format!("wc: {}: {}", filename, e))
                    }
                }
            }
            Err(_) => {
                Err(format!("wc: {}: No such file or directory", filename))
            }
        }
    }

    fn print_to_console(self) {
        for stat in self.stats {
            if self.line_flag {
                print!("{}\t", stat.lines);
            }
            if self.word_flag {
                print!("{}\t", stat.words);
            }
            if self.char_flag {
                print!("{}\t", stat.characters);
            }
            println!("{}", Green.dimmed().paint(stat.name));
        }

        if self.number_of_files > 1 {
            if self.line_flag {
                print!("{}\t", self.total.lines);
            }
            if self.word_flag {
                print!("{}\t", self.total.words);
            }
            if self.char_flag {
                print!("{}\t", self.total.characters);
            }
            println!("{}", Blue.bold().paint("total"));
        }
    }
}

fn main() {
    let matches = App::new("wc.rs")
        .version("0.1.0")
        .author("Palash Nigam <npalash25@gmail.com>")
        .about("The good old unix wc rewritten in rust")
        .arg(Arg::with_name("lines")
                 .short("l")
                 .long("lines")
                 .help("prints the newline counts"))
        .arg(Arg::with_name("words")
                 .short("w")
                 .long("words")
                 .help("print the words counts"))
        .arg(Arg::with_name("chars")
                 .short("m")
                 .long("chars")
                 .help("prints the character counts"))
        .arg(Arg::with_name("FILE")
                 .help("file path(s) to run wc on")
                 .multiple(true)
                 .empty_values(false))
        .get_matches();

    let mut wc = WcStats::new();
    wc.line_flag = matches.is_present("lines");
    wc.word_flag = matches.is_present("words");
    wc.char_flag = matches.is_present("chars");

    // if all are false then flip all the flags to `true`
    if !(wc.line_flag || wc.word_flag || wc.char_flag) {
        wc.line_flag = true;
        wc.word_flag = true;
        wc.char_flag = true;
    }

    if matches.is_present("FILE") {
        let file_path_vec: Vec<&str> = matches.values_of("FILE").unwrap().collect();
        for path in file_path_vec {
            match wc.get_stats(&String::from(path)) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        wc.number_of_files = wc.stats.len();
        wc.print_to_console();
    } else {
        println!("Switch on stdin");
    }
}

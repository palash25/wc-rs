use std::fs;
use std::io::Read;

extern crate clap;

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
            line_flag: true,
            word_flag: true,
            char_flag: true,
        }
    }

    // get_stats collects and stores the file stats
    fn get_stats(&mut self, file: &String) {
        let filename = file.clone();

        let mut fobj = fs::File::open(file).expect("couldn't read the file");
        let mut contents = String::new();
        fobj.read_to_string(&mut contents);

        let lines: Vec<&str> = contents.lines().collect();
        let words: Vec<&str> = contents.split_ascii_whitespace().collect();

        self.total.lines += lines.len();
        self.total.words += words.len();
        self.total.characters += contents.len();
        self.stats.push(FileStats{name: filename, lines: lines.len(), words: words.len(), characters: contents.len()});
    }

    // prints resutls to the console
    fn display(self) {
        if (self.line_flag && self.word_flag && self.char_flag) || !(self.line_flag || self.word_flag || self.char_flag) {
            self.print_all();
        } else if self.line_flag == true && !(self.word_flag || self.char_flag) {
            self.print_lines();
        } else if self.word_flag == true && !(self.line_flag || self.char_flag) {
            self.print_words();
        } else if self.char_flag == true && !(self.word_flag || self.line_flag) {
            self.print_chars();
        } else if self.line_flag && self.word_flag && self.char_flag == false {
            self.print_lines_and_words();
        } else if self.line_flag && self.char_flag && self.word_flag == false {
            self.print_lines_and_chars();
        } else {
            self.print_words_and_chars();
        }
    }

    fn print_all(self) {
        for stat in self.stats {
            println!("{}\t{}\t{}\t{}", stat.lines, stat.words, stat.characters, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\t{}\t{}\ttotal", self.total.lines, self.total.words, self.total.characters);
        }
    }

    fn print_lines_and_words(self) {
        for stat in self.stats {
            println!("{}\t{}\t{}", stat.lines, stat.words, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\t{}\ttotal", self.total.lines, self.total.words);
        }
    }

    fn print_lines_and_chars(self) {
        for stat in self.stats {
            println!("{}\t{}\t{}", stat.lines, stat.characters, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\t{}\ttotal", self.total.lines, self.total.characters);
        }
    }

    fn print_words_and_chars(self) {
        for stat in self.stats {
            println!("{}\t{}\t{}", stat.words, stat.characters, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\t{}\ttotal", self.total.words, self.total.characters);
        }
    }

    fn print_lines(self) {
        for stat in self.stats {
            println!("{}\t{}", stat.lines, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\ttotal", self.total.lines);
        }
    }

    fn print_words(self) {
        for stat in self.stats {
            println!("{}\t{}", stat.words, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\ttotal", self.total.words);
        }
    }

    fn print_chars(self) {
        for stat in self.stats {
            println!("{}\t{}", stat.characters, stat.name);
        }

        if self.number_of_files > 1 {
            println!("{}\ttotal", self.total.characters);
        }
    }
}

/*
TODOs:
- Read stdin with no file & listen for ctrl+d in case of stdin
- Emit a warning in case of dirs but don't fail "wc: benches: Is a directory"
- Add long help if needed
- docs/comments
- add line lenght flag L
- add bytes flag c

Ehancements:
- Add color to the output, themes maybe?
- Add a recursive call to process files under directories
*/
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

    if matches.is_present("FILE") {
        let file_path_vec: Vec<&str> = matches.values_of("FILE").unwrap().collect();
        for path in file_path_vec {
            wc.get_stats(&String::from(path));
        }
        wc.number_of_files = wc.stats.len();
        wc.display();
    } else {
        println!("Switch on stdin");
    }
}

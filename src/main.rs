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
    total: FileStats,
}

impl WcStats {
    // new creates an emtpy WcStats instance
    fn new() -> WcStats {
        WcStats {
            stats: Vec::new(),
            total: FileStats{name: String::from(""), lines: 0, words: 0, characters: 0},
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
        for stat in self.stats {
            println!("{} {} {} {}", stat.lines, stat.words, stat.characters, stat.name);
        }
    }
}

/*
TODOs:
- Evenly spaced columns in the output
- Read stdin with no file
- Listen for ctrl+d in case of stdin
- Emit a warning in case of dirs but don't fail "wc: benches: Is a directory"
- Add long help if needed
- docs/comments

Ehancements:
- Add color to the output
- Themes?
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
                 .short("W")
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

    let mut stats = WcStats::new();
    if matches.is_present("FILE") {
        let file_path_vec: Vec<&str> = matches.values_of("FILE").unwrap().collect();
        for path in file_path_vec {
        stats.get_stats(&String::from(path));
        }
        stats.display();
    } else {
        println!("Switch on stdin");
    }
}

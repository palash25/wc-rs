use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let flags = &args[1..];

    println!("{:?}", flags);

    let contents = fs::read_to_string(&args[args.len()-1]).expect("couldn't read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let words: Vec<&str> = contents.split_ascii_whitespace().collect();

    println!("chars {} \nlines: {} \nwords: {}", contents.len(), lines.len(), words.len());
}

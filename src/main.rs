use std::env;
use std::fs;
use std::io::Read;

struct Args {
    filename: Vec<String>,
    flags: Vec<String>,
}

fn get_args() -> Args {
    let cmd_args: Vec<String> = env::args().collect();
    let mut args = Args{
        filename: Vec::new(),
        flags: Vec::new(),
    };

    // Separate flags from filename
    for a in &cmd_args {
        if a.starts_with("-") {
            args.flags.push(String::from(a));
        } else {
            args.filename.push(String::from(a));
        }
    }
    args
}


fn display(file: &String, flags: Vec<String>) -> std::io::Result<()> {
    let filename = file.clone();

    let mut fobj = fs::File::open(file).expect("couldn't read the file");
    let mut contents = String::new();
    fobj.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();
    let words: Vec<&str> = contents.split_ascii_whitespace().collect();

    println!("{} {} {} {}", lines.len(), words.len(), contents.len(), filename);
    Ok(())
}

fn main() {
    let args = get_args();

    for a in &args.filename {
        let res = display(a, args.flags.clone());
        match res {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}

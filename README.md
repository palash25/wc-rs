# wc-rs

The good old unix tool rewritten in Rust.

## Why write it in Rust?

I wanted to learn Rust and this seemed like a small and simple enough project to get started with. This is my first rust project so it might contain some bad code, please raise an issue if you find any room for improvment within the codebase.

## Usage

Same as the standard `wc` command that is shipped with every linux distro.
```
$ wc --help

USAGE:
    wc [FLAGS] [FILE]...

FLAGS:
    -m, --chars      prints the character counts
    -h, --help       Prints help information
    -l, --lines      prints the newline counts
    -V, --version    Prints version information
    -w, --words      print the words counts

ARGS:
    <FILE>...    file path(s) to run wc on
```

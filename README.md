# wc-rs
![](https://img.shields.io/crates/d/wc-rs?style=flat)

The good old unix tool rewritten in Rust.

## Why write it in Rust?

I wanted to learn Rust and this seemed like a small and simple enough project to get started with.
To my surprise this naive implementation turned out to be faster than the original C implementation of `wc`, take a look at some [numbers](#Stats). This is my first rust project so it might contain some bad code, please raise an issue if you find any room for improvment within the codebase.

## Install

- To download and install using cargo run `cargo install wc-rs`
- If you don't have cargo just grab the binary from the [releases](https://github.com/palash25/wc-rs/releases) page and put it in your bin
- To build from source
  ```
    git clone https://github.com/palash25/wc-rs.git
    cd wc-rs/
    cargo build --release
  ```

## Usage

Same as the standard `wc` command that is shipped with every linux distro.
```
$ wc-rs --help

USAGE:
    wc-rs [FLAGS] [FILE]...

FLAGS:
    -m, --chars      prints the character counts
    -h, --help       Prints help information
    -l, --lines      prints the newline counts
    -V, --version    Prints version information
    -w, --words      print the words counts

ARGS:
    <FILE>...    file path(s) to run wc-rs on
```

## Stats

### Testing with a denser 10 million line file

This is where things start to slow down a little. This file has double the number of characters than the previous one and almost double the size (~250MB)

#### Time test

```
$ \time -p wc-rs big-and-chonky.txt
10048951    31711680    274285495  big-and-chonky.txt

real 4.72   user 4.67   sys 0.04

$ \time -p ./target/release/wc-rs big-and-chonky.txt
10048951	31711680	274285495  big-and-chonky.txt

real 1.89   user 1.36   sys 0.53
```

#### PV test

```
$ wc-rs big-and-chonky.txt | pv
10048951    31711680    274285495   big-and-chonky.txt

46 B 0:00:04 [9.81 B/s] [  <=>                  ]

$ ./target/release/wc-rs big-and-chonky.txt | pv
10048951    31711680    274285495   big-and-chonky.txt

55 B 0:00:01 [29.2 B/s] [  <=>                  ]
```

#### Hyperfine benchmarks

**GNU wc**

```
  Benchmark #1: wc big-file-1 big-file-2

  Time (mean ± σ):     24.849 s ±  1.183 s    [User: 24.538 s, System: 0.309 s]
  Range (min … max):   23.628 s … 26.437 s    10 runs
```

**wc-rs**

```
  Benchmark #1: ./Projects/wc/target/release/wc-rs big-file-1 big-file-2

  Time (mean ± σ):      9.298 s ±  0.224 s    [User: 6.519 s, System: 2.777 s]
  Range (min … max):    9.086 s …  9.674 s    10 runs
```

## TODO
- Read stdin in case no file paths are provided
- Add line length flag L
- Add bytes flag c

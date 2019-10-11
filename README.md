# wc-rs

The good old unix tool rewritten in Rust.

## Why write it in Rust?

I wanted to learn Rust and this seemed like a small and simple enough project to get started with.
To my surprise this naive implementation turned out to be faster than the original C implementation of `wc`, take a look at some [numbers](#Stats). This is my first rust project so it might contain some bad code, please raise an issue if you find any room for improvment within the codebase.

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

## Stats

### Testing with 3 files each having 48K lines

#### Time test
```
$ \time  wc small-file-*
 48414   81402  556859 small-file-1
 48414   81402  556859 small-file-2
 48414   81402  556859 small-file-3
 145242  244206 1670577 total

 real 0:00.05	 user 0.05	 sys 0.00


$ \time  ./target/release/wc-rs small-file-*
 48414	81402	556859	small-file-1
 48414	81402	556859	small-file-2
 48414	81402	556859	small-file-3
 145242	244206	1670577	total

 real 0:00.02	 user 0.00	 sys 0.01
```

#### PV test

```
$ wc small-file-* | pv
  48414   81402  556859 small-file-1
  48414   81402  556859 small-file-2
  48414   81402  556859 small-file-3
 145242  244206 1670577 total

 159 B 0:00:00 [ 3.1KiB/s] [  <=>                ]

$ ./target/release/wc-rs small-file-* | pv
 48414	81402	556859	small-file-1
 48414	81402	556859	small-file-2
 48414	81402	556859	small-file-3
 145242	244206	1670577	total

 186 B 0:00:00 [9.57KiB/s] [  <=>                ]
```

### Testing with 2 files each having 2 million lines

#### Time test

```
$ \time  wc big-file-*
 2091965  3549748 24604624 big-file-1
 2091965  3549748 24604624 big-file-2
 4183930  7099496 49209248 total

 real 0:01.14	 user 1.11	 sys 0.02

$ \time  ./target/release/wc-rs big-file-*
2091965	3549748	24604624	big-file-1
2091965	3549748	24604624	big-file-2
4183930	7099496	49209248	total

 real 0:00.49	 user 0.36	 sys 0.12
```

#### PV test

```
$ wc big-file-* | pv
 2091965  3549748 24604624 big-file-1
 2091965  3549748 24604624 big-file-2
 4183930  7099496 49209248 total

 121 B 0:00:01 [ 101 B/s] [    <=>               ]

$ ./target/release/wc-rs big-file-* | pv
2091965	3549748	24604624	big-file-1
2091965	3549748	24604624	big-file-2
4183930	7099496	49209248	total

 148 B 0:00:00 [ 315 B/s] [  <=>                  ]
```

### Testing with 2 files each having 10 million lines

#### Time test

```
$ \time  wc bigger-file-*
 10048406  17328448 124739305 bigger-file-1
 10446226  18017383 129746037 bigger-file-2
 20494632  35345831 254485342 total

 real 0:05.73	 user 5.66	 sys 0.05

$ \time  ./target/release/wc-rs bigger-file-*
 10048406	17328448	124739305	bigger-file-1
 10446226	18017383	129746037	bigger-file-2
 20494632	35345831	254485342	total

 real 0:02.20	 user 1.64	 sys 0.55
```

#### PV test
```
 wc big-file-* | pv
 10048406  17328448 124739305 bigger-file-1
 10446226  18017383 129746037 bigger-file-2
 20494632  35345831 254485342 total

 130 B 0:00:05 [22.5 B/s] [    <=>            ]

 ./target/release/wc-rs bigger-file-* | pv
 10048406  17328448 124739305 bigger-file-1
 10446226  18017383 129746037 bigger-file-2
 20494632  35345831 254485342 total

 157 B 0:00:02 [69.5 B/s] [  <=>              ]
```

### Testing with a denser 10 million line file

This is where things start to slow down a little. This file has double the number of characters than the previous one and almost double the size (~250MB)

#### Time test

```
$ \time -p wc big-and-chonky.txt
10048951    31711680    274285495  big-and-chonky.txt

real 4.72   user 4.67   sys 0.04

$ \time -p ./target/release/wc-rs big-and-chonky.txt
10048951	31711680	274285495  big-and-chonky.txt

real 1.89   user 1.36   sys 0.53
```

#### PV test

```
$ wc big-and-chonky.txt | pv
10048951    31711680    274285495   big-and-chonky.txt

46 B 0:00:04 [9.81 B/s] [  <=>                  ]

$ ./target/release/wc-rs big-and-chonky.txt | pv
10048951    31711680    274285495   big-and-chonky.txt

55 B 0:00:01 [29.2 B/s] [  <=>                  ]
```

## TODO
- Read stdin in case no file paths are provided
- Add line lenght flag L
- Add bytes flag c

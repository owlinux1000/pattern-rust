# pattern

`pattern` is a command line tool to create cyclic pattern like Metasploit.

## Installation

```sh
$ cargo install pattern
```

## How to use

```sh
$ pattern -l 10
Aa0Aa1Aa2Aa3Aa4Aa5Aa6Aa7Aa8Aa9Ab0Ab1Ab2Ab3Ab4Ab5Ab
$ pattern -o Aa6A
18
```

```sh
pattern 1.0.0
encry1024 <encry1024@gmail.com>


USAGE:
    pattern [FLAGS] [OPTIONS]

FLAGS:
    -b               Using big endian
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>    The length of payload
    -o, --offset <offset>    Calculate the offset with given value
```


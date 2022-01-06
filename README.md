# expander-rs [![Build/Test](https://github.com/gokberkkocak/expander-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/gokberkkocak/expander-rs/actions/workflows/ci.yml)

Closed/Maximal Itemset Expander

## Build

Release build;

```
cargo build --release
```

There is an optional compilation feature which uses mimalloc __mim-alloc__;

```
cargo build --release --features mim-alloc
```

For fat lto and codegen-units=1 use __production__ profile;

```
cargo build --profile production
```


## Usage
There are 3 different expanding systems (default is hash-only) and 4 different hashing algorithms (default is FNV). Use the flags to change the expander system and the hasher.  

```
expander-rust 0.5.0
Closed/Maximal Itemset Expander

USAGE:
    expander-rs [FLAGS] <input>

FLAGS:
    -a, --aes-hasher            Use AHash for Hasher (uses AES)
    -b, --bit-man-expander      Use Bit Manipulator expander
    -v, --bit-vec-expander      Use Bit Vec expander
    -f, --fnv-hasher            Use FNVHash for Hasher (default)
    -x, --fx-hasher             Use FXHash for Hasher
    -o, --hash-only-expander    Use Hash-only expander (default)
    -h, --help                  Prints help information
    -s, --std-hasher            Use Rust's std Hasher (HashBrown)
    -V, --version               Prints version information

ARGS:
    <input>    Input file in JSON format
```

## Examples

There are 3 different examples in examples folder.

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

For release optimisations with fat lto and codegen-units=1 use __production__ profile;

```
cargo build --profile production
```


## Usage
There are 4 different expanding systems (default is bit-vec) and 4 different hashing algorithms (default is FNV). Use the flags to change the expander system and the hasher.  

```
expander-rust 0.5.1
Closed/Maximal Itemset Expander

USAGE:
    expander-rs [FLAGS] <input>

FLAGS:
    -a, --aes-hasher            Use AHash for Hasher (uses AES)
    -m, --bit-man-expander      Use Bit Manipulator expander (u128 for itemset - up to 128 items)
    -b, --bit-vec-expander      Use Bit Vec expander (dynamic BitVec for itemset - no limits) (default)
    -f, --fnv-hasher            Use FNVHash for Hasher (default)
    -x, --fx-hasher             Use FXHash for Hasher
    -o, --hash-only-expander    Use Vec expander (u8 for each item - up to 256 items) with storing only hashes
                                (experimental feature which can cause collisions - use with care)
    -h, --help                  Prints help information
    -s, --std-hasher            Use Rust's std Hasher (uses Google's SwissTable / HashBrown)
    -V, --version               Prints version information
    -v, --vec-expander          Use Vec expander (u8 for each item - up to 256 items)

ARGS:
    <input>    Input file in JSON format
```

## Examples

There are 3 different examples in examples folder.

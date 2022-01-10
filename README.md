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
expander-rust 0.6.0
Closed/Maximal Itemset Expander

USAGE:
    expander-rs [FLAGS] [OPTIONS] <input>

FLAGS:
    -a, --aes-hasher            Use AHash for Hasher (uses AES)
    -m, --bit-man-expander      Use Bit Manipulator Expander (u128 for itemset - up to 128 items)
    -b, --bit-vec-expander      Use Bit Vec Expander (dynamic BitVec for itemset - no limits) (default)
    -f, --fnv-hasher            Use FNVHash for Hasher (default)
    -x, --fx-hasher             Use FXHash for Hasher
    -o, --hash-only-expander    Use Hash-only Vec Expander (u8 for each item - up to 256 items) which operates only on
                                hashes. Experimental feature which can be additionally fast but also can cause
                                collisions. Use with care
    -h, --help                  Prints help information
    -s, --std-hasher            Use Rust's std Hasher (uses Google's SwissTable / HashBrown)
    -V, --version               Prints version information
    -v, --vec-expander          Use Vec Expander (u8 for each item - up to 256 items)

OPTIONS:
    -o, --output <output>    Optional output file in JSON format. Each Expander serializes itemsets differently; - Bit
                             Vec Expander: Vec<usize> per itemset (Human-Readable), - Vec Expander: Vec<u8> per itemset
                             (Human-Readable), - BitMan Expander: u128 per itemset, - Hash-only Vec Expander: u64 Hash
                             per itemset (pretty much useless)

ARGS:
    <input>    Input file in JSON format
```

## Examples

There are 3 different examples in examples folder.

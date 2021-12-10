# Expander-rust [![Build/Test](https://github.com/gokberkkocak/expander-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/gokberkkocak/expander-rs/actions/workflows/ci.yml)

Experiment database maintenance tool.

## Build

```rust
cargo build --release
```

There is an optional compilation feature which uses mimalloc: __cheap-alloc__

```
cargo build --release --features cheap-alloc
```

## Usage

```
expander-rust 0.4.0
Closed/Maximal Itemset Expander

USAGE:
    expander-rs [FLAGS] <input>

FLAGS:
    -b, --bitman      Use Bit Manipulator expander
    -v, --bitvec      Use Bitvec expander
    -h, --hashonly    Use Hash-only expander (default)
        --help        Prints help information
    -V, --version     Prints version information

ARGS:
    <input>    Input file in JSON format
```

## Examples

There are 3 different examples in examples folder.

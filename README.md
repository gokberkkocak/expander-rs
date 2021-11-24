# Expander-rust [![Build/Test](https://github.com/gokberkkocak/expander-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/gokberkkocak/expander-rs/actions/workflows/ci.yml)

Experiment database maintenance tool.

## Build

```rust
cargo build --release
```

Optional compilation features are __bitvec__ and __cheap-alloc__

```
cargo build --release --features bitvec cheap-alloc
```

## Usage

```
expander-rust 0.3.0
Closed/Maximal Itemset Expander with only storing hashes.

USAGE:
    expander-rs <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    Input file in JSON format
```

## Examples

There are 3 different examples in examples folder.

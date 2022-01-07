mod expander;

use crate::expander::bitman::BitManipulatorExpander;
use crate::expander::bitvec::BitVecExpander;
use crate::expander::vec::VecExpander;
use crate::expander::vechashonly::VecHashOnlyExpander;
use crate::expander::Expander;

use ahash::{AHashSet, AHasher};
use anyhow::Result;
use bitvec::prelude::BitVec;
use fnv::{FnvHashSet, FnvHasher};
use fxhash::{FxHashSet, FxHasher};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[cfg(feature = "mim-alloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Deserialize)]
struct JsonSet {
    set: Vec<u8>,
}

const ABOUT: &str = "Closed/Maximal Itemset Expander";

#[derive(Debug, StructOpt)]
#[structopt(name = "expander-rust", about = ABOUT)]
struct Opt {
    /// Input file in JSON format
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Use Vec expander with storing only hashes
    /// (experimental feature which can cause collisions - use with care)
    #[structopt(
        short = "o",
        long,
        conflicts_with = "bit_vec_expander",
        conflicts_with = "bit_man_expander",
        conflicts_with = "vec_expander"
    )]
    hash_only_expander: bool,
    /// Use Vec expander (default)
    #[structopt(
        short = "o",
        long,
        conflicts_with = "bit_vec_expander",
        conflicts_with = "hash_only_expander",
        conflicts_with = "bit_man_expander"
    )]
    vec_expander: bool,
    /// Use Bit Manipulator expander
    #[structopt(
        short = "b",
        long,
        conflicts_with = "hash_only_expander",
        conflicts_with = "vec_expander",
        conflicts_with = "bit_vec_expander"
    )]
    bit_man_expander: bool,
    /// Use Bit Vec expander
    #[structopt(
        short = "v",
        long,
        conflicts_with = "bit_man_expander",
        conflicts_with = "vec_expander",
        conflicts_with = "hash_only_expander"
    )]
    bit_vec_expander: bool,
    /// Use FNVHash for Hasher (default)
    #[structopt(
        short = "f",
        long,
        conflicts_with = "fx_hasher",
        conflicts_with = "aes_hasher",
        conflicts_with = "std_hasher"
    )]
    fnv_hasher: bool,
    /// Use FXHash for Hasher
    #[structopt(
        short = "x",
        long,
        conflicts_with = "fnv_hasher",
        conflicts_with = "aes_hasher",
        conflicts_with = "std_hasher"
    )]
    fx_hasher: bool,
    /// Use Rust's std Hasher (HashBrown)
    #[structopt(
        short = "s",
        long,
        conflicts_with = "fnv_hasher",
        conflicts_with = "fx_hasher",
        conflicts_with = "aes_hasher"
    )]
    std_hasher: bool,
    /// Use AHash for Hasher (uses AES)
    #[structopt(
        short = "a",
        long,
        conflicts_with = "fnv_hasher",
        conflicts_with = "fx_hasher",
        conflicts_with = "std_hasher"
    )]
    aes_hasher: bool,
}

pub fn read_file(filepath: &Path) -> Result<String> {
    let file = File::open(filepath)?;
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let contents = read_file(&opt.input);
    let parsed_set: Vec<JsonSet> = serde_json::from_str(&contents?)?;
    work(&opt, parsed_set);
    Ok(())
}

fn work(opt: &Opt, parsed_set: Vec<JsonSet>) {
    let len = match (
        opt.vec_expander,
        opt.hash_only_expander,
        opt.bit_vec_expander,
        opt.bit_man_expander,
    ) {
        (_, false, false, false) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => {
                VecExpander::<FnvHashSet<Vec<u8>>>::expand(parsed_set).len()
            }
            (false, true, false, false) => {
                VecExpander::<FxHashSet<Vec<u8>>>::expand(parsed_set).len()
            }
            (false, false, true, false) => {
                VecExpander::<HashSet<Vec<u8>>>::expand(parsed_set).len()
            }
            (false, false, false, true) => {
                VecExpander::<AHashSet<Vec<u8>>>::expand(parsed_set).len()
            }
            _ => unreachable!(),
        },
        (false, true, false, false) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => {
                VecHashOnlyExpander::<FnvHashSet<u64>, FnvHasher>::expand(parsed_set).len()
            }
            (false, true, false, false) => {
                VecHashOnlyExpander::<FxHashSet<u64>, FxHasher>::expand(parsed_set).len()
            }
            (false, false, true, false) => {
                VecHashOnlyExpander::<HashSet<u64>, DefaultHasher>::expand(parsed_set).len()
            }
            (false, false, false, true) => {
                VecHashOnlyExpander::<AHashSet<u64>, AHasher>::expand(parsed_set).len()
            }
            _ => unreachable!(),
        },
        (false, false, true, false) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => {
                BitVecExpander::<FnvHashSet<BitVec>>::expand(parsed_set).len()
            }
            (false, true, false, false) => {
                BitVecExpander::<FxHashSet<BitVec>>::expand(parsed_set).len()
            }
            (false, false, true, false) => {
                BitVecExpander::<HashSet<BitVec>>::expand(parsed_set).len()
            }
            (false, false, false, true) => {
                BitVecExpander::<AHashSet<BitVec>>::expand(parsed_set).len()
            }
            _ => unreachable!(),
        },
        (false, false, false, true) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => {
                BitManipulatorExpander::<FnvHashSet<u128>>::expand(parsed_set).len()
            }
            (false, true, false, false) => {
                BitManipulatorExpander::<FxHashSet<u128>>::expand(parsed_set).len()
            }
            (false, false, true, false) => {
                BitManipulatorExpander::<HashSet<u128>>::expand(parsed_set).len()
            }
            (false, false, false, true) => {
                BitManipulatorExpander::<AHashSet<u128>>::expand(parsed_set).len()
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    println!("Total nb of item-sets: {}", len);
}

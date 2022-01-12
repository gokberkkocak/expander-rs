mod expander;

use ahash::AHasher;
use anyhow::Result;
use expander::bitman::BitManipulatorExpander;
use expander::bitvec::BitVecExpander;
use expander::set::SerializedSetLen;
use expander::set::WrappedAHashSet;
use expander::set::WrappedBitVec;
use expander::set::Wrappedu128;
use expander::vec::VecExpander;
use expander::vechashonly::VecHashOnlyExpander;
use expander::Expander;
use fnv::{FnvHashSet, FnvHasher};
use fxhash::{FxHashSet, FxHasher};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
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
    /// Use Hash-only Vec Expander (u8 for each item - up to 256 items) which operates only on hashes.
    /// Experimental feature which can be additionally fast but also can cause collisions. Use with care.
    #[structopt(
        short = "o",
        long,
        conflicts_with = "bit_vec_expander",
        conflicts_with = "bit_man_expander",
        conflicts_with = "vec_expander"
    )]
    hash_only_expander: bool,
    /// Use Vec Expander (u8 for each item - up to 256 items).
    #[structopt(
        short = "v",
        long,
        conflicts_with = "bit_vec_expander",
        conflicts_with = "hash_only_expander",
        conflicts_with = "bit_man_expander"
    )]
    vec_expander: bool,
    /// Use Bit Manipulator Expander (u128 for itemset - up to 128 items).
    #[structopt(
        short = "m",
        long,
        conflicts_with = "hash_only_expander",
        conflicts_with = "vec_expander",
        conflicts_with = "bit_vec_expander"
    )]
    bit_man_expander: bool,
    /// Use Bit Vec Expander (dynamic BitVec for itemset - no limits) (default).
    #[structopt(
        short = "b",
        long,
        conflicts_with = "bit_man_expander",
        conflicts_with = "vec_expander",
        conflicts_with = "hash_only_expander"
    )]
    bit_vec_expander: bool,
    /// Use FNVHash for Hasher (default).
    #[structopt(
        short = "f",
        long,
        conflicts_with = "fx_hasher",
        conflicts_with = "aes_hasher",
        conflicts_with = "std_hasher"
    )]
    fnv_hasher: bool,
    /// Use FXHash for Hasher.
    #[structopt(
        short = "x",
        long,
        conflicts_with = "fnv_hasher",
        conflicts_with = "aes_hasher",
        conflicts_with = "std_hasher"
    )]
    fx_hasher: bool,
    /// Use Rust's std Hasher (uses Google's SwissTable / HashBrown).
    #[structopt(
        short = "s",
        long,
        conflicts_with = "fnv_hasher",
        conflicts_with = "fx_hasher",
        conflicts_with = "aes_hasher"
    )]
    std_hasher: bool,
    /// Use AHash for Hasher (uses AES).
    #[structopt(
        short = "a",
        long,
        conflicts_with = "fnv_hasher",
        conflicts_with = "fx_hasher",
        conflicts_with = "std_hasher"
    )]
    aes_hasher: bool,
    /// Optional output file in JSON format.
    /// Each Expander serializes itemsets differently;
    /// - Bit Vec Expander: Vec<usize> per itemset (Human-Readable),
    /// - Vec Expander: Vec<u8> per itemset (Human-Readable),
    /// - BitMan Expander: Vec<0..128> per itemset (Human-Readable),
    /// - Hash-only Vec Expander: u64 Hash per itemset (pretty much useless).
    #[structopt(short = "o", long, parse(from_os_str))]
    output: Option<PathBuf>,
}

pub fn read_file(filepath: &Path) -> Result<String> {
    let file = File::open(filepath)?;
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_to_file(contents: &[u8], filepath: &Path) -> Result<()> {
    let file = File::create(filepath)?;
    let mut buffered_writer = BufWriter::new(file);
    buffered_writer.write_all(contents)?;
    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let contents = read_file(&opt.input);
    let parsed_set: Vec<JsonSet> = serde_json::from_str(&contents?)?;
    let boxed_set = work(&opt, parsed_set);
    println!("Total nb of item-sets: {}", boxed_set.set_len());
    if let Some(output_path) = opt.output {
        let boxed_set_str = serde_json::to_string(&boxed_set)?;
        write_to_file(boxed_set_str.as_bytes(), &output_path)?;
    }
    Ok(())
}

fn work(opt: &Opt, parsed_set: Vec<JsonSet>) -> Box<dyn SerializedSetLen> {
    match (
        opt.vec_expander,
        opt.hash_only_expander,
        opt.bit_vec_expander,
        opt.bit_man_expander,
    ) {
        (true, false, false, false) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => {
                Box::new(VecExpander::<FnvHashSet<Vec<u8>>>::expand(parsed_set))
            }
            (false, true, false, false) => {
                Box::new(VecExpander::<FxHashSet<Vec<u8>>>::expand(parsed_set))
            }
            (false, false, true, false) => {
                Box::new(VecExpander::<HashSet<Vec<u8>>>::expand(parsed_set))
            }
            (false, false, false, true) => {
                Box::new(VecExpander::<WrappedAHashSet<Vec<u8>>>::expand(parsed_set))
            }
            _ => unreachable!(),
        },
        (false, true, false, false) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => Box::new(
                VecHashOnlyExpander::<FnvHashSet<u64>, FnvHasher>::expand(parsed_set),
            ),
            (false, true, false, false) => Box::new(
                VecHashOnlyExpander::<FxHashSet<u64>, FxHasher>::expand(parsed_set),
            ),
            (false, false, true, false) => Box::new(VecHashOnlyExpander::<
                HashSet<u64>,
                DefaultHasher,
            >::expand(parsed_set)),
            (false, false, false, true) => {
                Box::new(VecHashOnlyExpander::<WrappedAHashSet<u64>, AHasher>::expand(parsed_set))
            }
            _ => unreachable!(),
        },
        (false, false, _, false) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => Box::new(
                BitVecExpander::<FnvHashSet<WrappedBitVec>>::expand(parsed_set),
            ),
            (false, true, false, false) => Box::new(
                BitVecExpander::<FxHashSet<WrappedBitVec>>::expand(parsed_set),
            ),
            (false, false, true, false) => {
                Box::new(BitVecExpander::<HashSet<WrappedBitVec>>::expand(parsed_set))
            }
            (false, false, false, true) => Box::new(
                BitVecExpander::<WrappedAHashSet<WrappedBitVec>>::expand(parsed_set),
            ),
            _ => unreachable!(),
        },
        (false, false, false, true) => match (
            opt.fnv_hasher,
            opt.fx_hasher,
            opt.std_hasher,
            opt.aes_hasher,
        ) {
            (_, false, false, false) => Box::new(
                BitManipulatorExpander::<FnvHashSet<Wrappedu128>>::expand(parsed_set),
            ),
            (false, true, false, false) => Box::new(
                BitManipulatorExpander::<FxHashSet<Wrappedu128>>::expand(parsed_set),
            ),
            (false, false, true, false) => {
                Box::new(BitManipulatorExpander::<HashSet<Wrappedu128>>::expand(parsed_set))
            }
            (false, false, false, true) => Box::new(
                BitManipulatorExpander::<WrappedAHashSet<Wrappedu128>>::expand(parsed_set),
            ),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

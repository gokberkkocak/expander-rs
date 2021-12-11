mod expander;

use crate::expander::Expander;
use crate::expander::hashonly::HashOnlyExpander;
use crate::expander::bitvec::BitVecExpander;
use crate::expander::bitman::BitManipulatorExpander;

use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[cfg(feature = "cheap-alloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Deserialize)]
struct JsonSet {
    set: Vec<u8>,
}

const ABOUT: &'static str = "Closed/Maximal Itemset Expander";

#[derive(Debug, StructOpt)]
#[structopt(name = "expander-rust", about = ABOUT)]
struct Opt {
    /// Input file in JSON format
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Use Hash-only expander (default)
    #[structopt(
        short = "h",
        long = "hashonly",
        conflicts_with = "bitvec",
        conflicts_with = "bitman"
    )]
    hashonly: bool,
    /// Use Bit Manipulator expander
    #[structopt(
        short = "b",
        long = "bitman",
        conflicts_with = "hashonly",
        conflicts_with = "bitvec"
    )]
    bitman: bool,
    /// Use Bitvec expander
    #[structopt(
        short = "v",
        long = "bitvec",
        conflicts_with = "bitman",
        conflicts_with = "hashonly"
    )]
    bitvec: bool,
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
    let len = match (opt.hashonly, opt.bitvec, opt.bitman) {
        (_, false, false) => HashOnlyExpander::expand(parsed_set).len(),
        (false, true, false) => BitVecExpander::expand(parsed_set).len(),
        (false, false, true) => BitManipulatorExpander::expand(parsed_set).len(),
        _ => unreachable!(),
    };
    println!("Total nb of item-sets: {}", len);
}

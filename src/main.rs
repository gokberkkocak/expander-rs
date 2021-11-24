use anyhow::Result;
use fnv::FnvHashSet;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[cfg(not(feature = "bitvec"))]
use std::hash::{Hash, Hasher};

#[cfg(not(feature = "bitvec"))]
use fnv::FnvHasher;

#[cfg(feature = "bitvec")]
use bit_vec::BitVec;

#[cfg(feature = "cheap-alloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Deserialize)]
struct JsonSet {
    set: Vec<u8>,
}

#[cfg(not(feature = "bitvec"))]
fn explode_one_solution_to_lower_level(sol: &mut Vec<u8>, final_hash_set: &mut FnvHashSet<u64>) {
    let length = sol.len();
    if length > 1 {
        for i in 0..length {
            let el = sol.remove(i);
            let mut hasher = FnvHasher::default();
            Hash::hash_slice(&sol, &mut hasher);
            if !final_hash_set.contains(&hasher.finish()) {
                explode_one_solution_to_lower_level(sol, final_hash_set);
            }
            sol.insert(i, el);
        }
    }
    let mut hasher = FnvHasher::default();
    Hash::hash_slice(&sol, &mut hasher);
    final_hash_set.insert(hasher.finish());
}
#[cfg(feature = "bitvec")]
const NB_BITS: u8 = 100;

#[cfg(feature = "bitvec")]
fn explode_one_solution_to_lower_level(sol: &mut Vec<u8>, final_hash_set: &mut FnvHashSet<BitVec>) {
    let length = sol.len();
    if length > 1 {
        for i in 0..length {
            let el = sol.remove(i);
            if !final_hash_set.contains(&convert_itemset(sol)) {
                explode_one_solution_to_lower_level(sol, final_hash_set);
            }
            sol.insert(i, el);
        }
    }
    final_hash_set.insert(convert_itemset(sol));
}

#[cfg(feature = "bitvec")]
fn convert_itemset(itemset: &[u8]) -> BitVec {
    let mut bv = BitVec::from_elem(NB_BITS.into(), false);
    for i in itemset {
        bv.set((*i).into(), true);
    }
    bv
}

#[cfg(feature = "bitvec")]
const ABOUT: &'static str = "Closed/Maximal Itemset Expander with BitVecs.";

#[cfg(not(feature = "bitvec"))]
const ABOUT: &'static str = "Closed/Maximal Itemset Expander with only storing hashes.";

#[derive(Debug, StructOpt)]

#[structopt(name = "expander-rust", about = ABOUT)]
struct Opt {
    /// Input file in JSON format
    #[structopt(parse(from_os_str))]
    input: PathBuf,
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
    let mut final_set = FnvHashSet::default();
    for mut set in parsed_set {
        explode_one_solution_to_lower_level(&mut set.set, &mut final_set);
    }
    println!("Expander result: {:?}", final_set.len());
    Ok(())
}

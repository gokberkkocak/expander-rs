use crate::exploder::Exploder;

use bit_vec::BitVec;
use fnv::FnvHashSet;

const NB_BITS: u8 = 100;

fn convert_itemset(itemset: &[u8]) -> BitVec {
    let mut bv = BitVec::from_elem(NB_BITS.into(), false);
    for i in itemset {
        bv.set((*i).into(), true);
    }
    bv
}
#[derive(Default)]
pub struct BitVecExploder;

impl Exploder for BitVecExploder {
    type SolutionType = Vec<u8>;

    type HashType = BitVec;

    fn explode_one_solution_to_lower_level(
        solution: &mut Self::SolutionType,
        final_set: &mut FnvHashSet<Self::HashType>,
    ) {
        let length = solution.len();
        if length > 1 {
            for i in 0..length {
                let el = solution.remove(i);
                if !final_set.contains(&convert_itemset(solution)) {
                    Self::explode_one_solution_to_lower_level(solution, final_set);
                }
                solution.insert(i, el);
            }
        }
        final_set.insert(convert_itemset(solution));
    }

    fn explode(parsed_set: Vec<crate::JsonSet>) -> FnvHashSet<Self::HashType> {
        let mut final_set = FnvHashSet::default();
        for mut i in parsed_set {
            Self::explode_one_solution_to_lower_level(&mut i.set, &mut final_set);
        }
        final_set
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let parsed_set = vec![
            crate::JsonSet {
                set: vec![1, 2, 3],
            },
            crate::JsonSet {
                set: vec![4, 5, 6],
            },
        ];
        assert_eq!(BitVecExploder::explode(parsed_set).len(), 14);
    }

    #[test]
    fn test_2() {
        let parsed_set = vec![
            crate::JsonSet {
                set: vec![57, 58, 59, 60],
            },
            crate::JsonSet {
                set: vec![60, 99],
            },
        ];
        assert_eq!(BitVecExploder::explode(parsed_set).len(), 17);
    }
}

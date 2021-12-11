use crate::{expander::Expander, JsonSet};

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
pub(crate) struct BitVecExpander;

impl Expander for BitVecExpander {
    type SolutionType = BitVec;

    type HashType = BitVec;

    fn expand(parsed_set: Vec<JsonSet>) -> FnvHashSet<Self::HashType> {
        let mut final_set = FnvHashSet::default();
        let parsed_set = parsed_set
            .iter()
            .map(|x| convert_itemset(&x.set))
            .collect::<Vec<_>>();
        for mut i in parsed_set {
            Self::expand_one_solution_to_lower_level(&mut i, &mut final_set);
        }
        final_set
    }

    fn expand_one_solution_to_lower_level(
        solution: &mut Self::SolutionType,
        final_set: &mut FnvHashSet<Self::HashType>,
    ) {
        let ones_length = solution.iter().filter(|&x| x).count();
        if ones_length > 1 {
            for i in 0..solution.len() {
                if solution[i] {
                    solution.set(i, false);
                    if !final_set.contains(solution) {
                        Self::expand_one_solution_to_lower_level(solution, final_set);
                    }
                    solution.set(i, true);
                }
            }
        }
        final_set.insert(solution.clone());
    }


}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(BitVecExpander::expand(parsed_set).len(), 14);
    }

    #[test]
    fn test_2() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        assert_eq!(BitVecExpander::expand(parsed_set).len(), 17);
    }
}

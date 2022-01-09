use crate::{expander::Expander, JsonSet};

use bitvec::vec::BitVec;

use super::WrappedBitVec;

fn convert_itemset(itemset: &[u8], nb_bits: usize) -> WrappedBitVec {
    let mut bv = BitVec::with_capacity(nb_bits);
    (0..nb_bits).for_each(|i| match itemset.iter().find(|&&x| x == i as u8) {
        Some(_) => bv.push(true),
        None => bv.push(false),
    });
    WrappedBitVec(bv)
}

fn get_number_of_required_bits(parsed_set: &[JsonSet]) -> usize {
    *parsed_set
        .iter()
        .map(|x| x.set.iter().max().unwrap())
        .max()
        .unwrap() as usize
        + 1
}

pub(crate) struct BitVecExpander<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Expander for BitVecExpander<T>
where
    T: Default,
    T: crate::expander::SetLike<WrappedBitVec>,
{
    type SolutionType = WrappedBitVec;
    type SetType = T;
    type HashType = WrappedBitVec;

    fn expand(parsed_set: Vec<JsonSet>) -> T {
        let mut final_set = T::default();
        let nb_bits = get_number_of_required_bits(&parsed_set);
        let parsed_set = parsed_set
            .iter()
            .map(|x| convert_itemset(&x.set, nb_bits))
            .collect::<Vec<_>>();
        for mut i in parsed_set {
            Self::expand_one_solution_to_lower_level(&mut i, &mut final_set);
        }
        final_set
    }

    fn expand_one_solution_to_lower_level(solution: &mut Self::SolutionType, final_set: &mut T) {
        let ones_length = solution.0.iter().filter(|x| **x).count();
        if ones_length > 1 {
            for i in 0..solution.0.len() {
                if solution.0[i] {
                    solution.0.set(i, false);
                    if !final_set.set_contains(solution) {
                        Self::expand_one_solution_to_lower_level(solution, final_set);
                    }
                    solution.0.set(i, true);
                }
            }
        }
        final_set.set_insert(solution.clone());
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use ahash::AHashSet;
    use fnv::FnvHashSet;
    use fxhash::FxHashSet;

    use crate::expander::WrappedAHashSet;

    use super::*;
    #[test]
    fn test_1_fnv() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            BitVecExpander::<FnvHashSet<WrappedBitVec>>::expand(parsed_set).len(),
            14
        );
    }

    #[test]
    fn test_2_fnv() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        assert_eq!(
            BitVecExpander::<FnvHashSet<WrappedBitVec>>::expand(parsed_set).len(),
            17
        );
    }

    #[test]
    fn test_1_fx() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            BitVecExpander::<FxHashSet<WrappedBitVec>>::expand(parsed_set).len(),
            14
        );
    }
    #[test]
    fn test_2_fx() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        assert_eq!(
            BitVecExpander::<FxHashSet<WrappedBitVec>>::expand(parsed_set).len(),
            17
        );
    }

    #[test]
    fn test_1_std() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            BitVecExpander::<HashSet<WrappedBitVec>>::expand(parsed_set).len(),
            14
        );
    }
    #[test]
    fn test_2_std() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        assert_eq!(
            BitVecExpander::<HashSet<WrappedBitVec>>::expand(parsed_set).len(),
            17
        );
    }

    #[test]
    fn test_1_ahash() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            BitVecExpander::<WrappedAHashSet<WrappedBitVec>>::expand(parsed_set).len(),
            14
        );
    }
    #[test]
    fn test_2_ahash() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        assert_eq!(
            BitVecExpander::<WrappedAHashSet<WrappedBitVec>>::expand(parsed_set).len(),
            17
        );
    }
}

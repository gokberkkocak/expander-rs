use crate::{expander::Expander, JsonSet};

use bitvec::vec::BitVec;

const NB_BITS: u8 = 100;

fn convert_itemset(itemset: &[u8]) -> BitVec {
    let mut bv = BitVec::with_capacity(NB_BITS.into());
    (0..NB_BITS).for_each(|i| match itemset.iter().find(|&&x| x == i) {
        Some(_) => bv.push(true),
        None => bv.push(false),
    });
    bv
}
pub(crate) struct BitVecExpander<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Expander<T> for BitVecExpander<T>
where
    T: Default,
    T: IntoIterator,
    T::Item: Into<BitVec>,
    T: crate::expander::Insert<BitVec>,
    T: crate::expander::Contains<BitVec>,
{
    type SolutionType = BitVec;

    type HashType = BitVec;

    fn expand(parsed_set: Vec<JsonSet>) -> T {
        let mut final_set = T::default();
        let parsed_set = parsed_set
            .iter()
            .map(|x| convert_itemset(&x.set))
            .collect::<Vec<_>>();
        for mut i in parsed_set {
            Self::expand_one_solution_to_lower_level(&mut i, &mut final_set);
        }
        final_set
    }

    fn expand_one_solution_to_lower_level(solution: &mut Self::SolutionType, final_set: &mut T) {
        let ones_length = solution.iter().filter(|x| **x).count();
        if ones_length > 1 {
            for i in 0..solution.len() {
                if solution[i] {
                    solution.set(i, false);
                    if !final_set.contains_(solution) {
                        Self::expand_one_solution_to_lower_level(solution, final_set);
                    }
                    solution.set(i, true);
                }
            }
        }
        final_set.insert_(solution.clone());
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use ahash::AHashSet;
    use fnv::FnvHashSet;
    use fxhash::FxHashSet;

    use super::*;
    #[test]
    fn test_1_fnv() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            BitVecExpander::<FnvHashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<FnvHashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<FxHashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<FxHashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<HashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<HashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<AHashSet<BitVec>>::expand(parsed_set).len(),
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
            BitVecExpander::<AHashSet<BitVec>>::expand(parsed_set).len(),
            17
        );
    }
}

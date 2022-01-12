use crate::{expander::Expander, JsonSet};

use super::set::Wrappedu128;

fn convert_itemset(sol: &[u8]) -> Wrappedu128 {
    let mut sol_u128 = 0;
    for i in sol {
        sol_u128 |= 1 << i;
    }
    Wrappedu128(sol_u128)
}

pub(crate) struct BitManipulatorExpander<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Expander for BitManipulatorExpander<T>
where
    T: Default,
    T: crate::expander::SetLike<Wrappedu128>,
{
    type SolutionType = Wrappedu128;
    type SetType = T;
    type HashType = Wrappedu128;

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
        let length = solution.0.count_ones();
        if length > 1 {
            for i in 0..u128::BITS {
                if (solution.0 & (1 << i)) >> i == 1 {
                    let mut new_sol = Wrappedu128(solution.0 ^ (1 << i));
                    if !final_set.set_contains(&new_sol) {
                        Self::expand_one_solution_to_lower_level(&mut new_sol, final_set);
                    }
                }
            }
        }
        final_set.set_insert(*solution);
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use fnv::FnvHashSet;
    use fxhash::FxHashSet;

    use crate::expander::set::WrappedAHashSet;

    use super::*;
    #[test]
    fn test_1_fnv() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            BitManipulatorExpander::<FnvHashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<FnvHashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<FxHashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<FxHashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<HashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<HashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<WrappedAHashSet<Wrappedu128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<WrappedAHashSet<Wrappedu128>>::expand(parsed_set).len(),
            17
        );
    }

    #[test]
    fn test_1_serialize() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        let expanded_set = BitManipulatorExpander::<FnvHashSet<Wrappedu128>>::expand(parsed_set);
        let serialized_set = serde_json::to_string(&expanded_set).unwrap();
        assert_eq!(serialized_set.len(), 77);
    }

    #[test]
    fn test_2_serialize() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        let expanded_set = BitManipulatorExpander::<FnvHashSet<Wrappedu128>>::expand(parsed_set);
        let serialized_set = serde_json::to_string(&expanded_set).unwrap();
        assert_eq!(serialized_set.len(), 140);
    }
}

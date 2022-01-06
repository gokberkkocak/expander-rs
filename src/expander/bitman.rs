use crate::{expander::Expander, JsonSet};

fn convert_itemset(sol: &[u8]) -> u128 {
    let mut sol_u128 = 0;
    for i in sol {
        sol_u128 |= 1 << (i - 1);
    }
    sol_u128
}

pub(crate) struct BitManipulatorExpander<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Expander<T> for BitManipulatorExpander<T>
where
    T: Default,
    T: IntoIterator,
    T::Item: Into<u128>,
    T: crate::expander::Insert<u128>,
    T: crate::expander::Contains<u128>,
{
    type SolutionType = u128;

    type HashType = u128;

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
        let length = solution.count_ones();
        if length > 1 {
            for i in 0..u128::BITS {
                if (*solution & (1 << i)) >> i == 1 {
                    let mut new_sol = *solution ^ (1 << i);
                    if !final_set.contains_(&new_sol) {
                        Self::expand_one_solution_to_lower_level(&mut new_sol, final_set);
                    }
                }
            }
        }
        final_set.insert_(*solution);
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
            BitManipulatorExpander::<FnvHashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<FnvHashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<FxHashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<FxHashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<HashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<HashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<AHashSet<u128>>::expand(parsed_set).len(),
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
            BitManipulatorExpander::<AHashSet<u128>>::expand(parsed_set).len(),
            17
        );
    }
}

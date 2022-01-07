use std::hash::{Hash, Hasher};

use crate::expander::Expander;
use crate::JsonSet;

pub(crate) struct VecHashOnlyExpander<T, S> {
    _phantom_hash_set: std::marker::PhantomData<T>,
    _phantom_hasher: std::marker::PhantomData<S>,
}

impl<T, S> Expander<T> for VecHashOnlyExpander<T, S>
where
    T: Default,
    T: IntoIterator,
    T::Item: Into<u64>,
    T: crate::expander::SetLike<u64>,
    S: Hasher,
    S: Default,
{
    type SolutionType = Vec<u8>;

    type HashType = u64;

    fn expand(parsed_set: Vec<JsonSet>) -> T {
        let mut final_set = T::default();
        for mut i in parsed_set {
            Self::expand_one_solution_to_lower_level(&mut i.set, &mut final_set);
        }
        final_set
    }

    fn expand_one_solution_to_lower_level(solution: &mut Self::SolutionType, final_set: &mut T) {
        let length = solution.len();
        if length > 1 {
            for i in 0..length {
                let el = solution.remove(i);
                let mut hasher = S::default();
                Hash::hash_slice(solution, &mut hasher);
                if !final_set.set_contains(&hasher.finish()) {
                    Self::expand_one_solution_to_lower_level(solution, final_set);
                }
                solution.insert(i, el);
            }
        }
        let mut hasher = S::default();
        Hash::hash_slice(solution, &mut hasher);
        final_set.set_insert(hasher.finish());
    }
}

#[cfg(test)]
mod tests {

    use std::collections::{hash_map::DefaultHasher, HashSet};

    use ahash::{AHashSet, AHasher};
    use fnv::{FnvHashSet, FnvHasher};
    use fxhash::{FxHashSet, FxHasher};

    use super::*;
    #[test]
    fn test_1_fnv() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(
            VecHashOnlyExpander::<FnvHashSet<u64>, FnvHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<FnvHashSet<u64>, FnvHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<FxHashSet<u64>, FxHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<FxHashSet<u64>, FxHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<HashSet<u64>, DefaultHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<HashSet<u64>, DefaultHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<AHashSet<u64>, AHasher>::expand(parsed_set).len(),
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
            VecHashOnlyExpander::<AHashSet<u64>, AHasher>::expand(parsed_set).len(),
            17
        );
    }
}

use crate::expander::Expander;
use crate::JsonSet;

pub(crate) struct VecExpander<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Expander for VecExpander<T>
where
    T: Default,
    T: crate::expander::SetLike<Vec<u8>>,
{
    type SolutionType = Vec<u8>;
    type SetType = T;
    type HashType = Vec<u8>;

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
                if !final_set.set_contains(solution) {
                    Self::expand_one_solution_to_lower_level(solution, final_set);
                }
                solution.insert(i, el);
            }
        }
        final_set.set_insert(solution.clone());
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
            VecExpander::<FnvHashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<FnvHashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<FxHashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<FxHashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<HashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<HashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<WrappedAHashSet<Vec<u8>>>::expand(parsed_set).len(),
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
            VecExpander::<WrappedAHashSet<Vec<u8>>>::expand(parsed_set).len(),
            17
        );
    }
    #[test]
    fn test_1_serialize() {
        let parsed_set = vec![
            JsonSet { set: vec![1, 2, 3] },
            JsonSet { set: vec![4, 5, 6] },
        ];
        let expanded_set = VecExpander::<FnvHashSet<Vec<u8>>>::expand(parsed_set);
        let serialized_set = serde_json::to_string(&expanded_set).unwrap();
        assert_eq!(
            serialized_set.len(),
            77
        );
    }

    #[test]
    fn test_2_serialize() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        let expanded_set = VecExpander::<FnvHashSet<Vec<u8>>>::expand(parsed_set);
        let serialized_set = serde_json::to_string(&expanded_set).unwrap();
        assert_eq!(
            serialized_set.len(),
            140
        );
    }
}

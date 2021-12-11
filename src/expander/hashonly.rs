use std::hash::{Hash, Hasher};

use fnv::{FnvHashSet, FnvHasher};

use crate::expander::Expander;
use crate::JsonSet;

pub(crate) struct HashOnlyExpander;

impl Expander for HashOnlyExpander {
    type SolutionType = Vec<u8>;

    type HashType = u64;

    fn expand(parsed_set: Vec<JsonSet>) -> FnvHashSet<Self::HashType> {
        let mut final_set = FnvHashSet::default();
        for mut i in parsed_set {
            Self::expand_one_solution_to_lower_level(&mut i.set, &mut final_set);
        }
        final_set
    }

    fn expand_one_solution_to_lower_level(
        solution: &mut Self::SolutionType,
        final_set: &mut FnvHashSet<Self::HashType>,
    ) {
        let length = solution.len();
        if length > 1 {
            for i in 0..length {
                let el = solution.remove(i);
                let mut hasher = FnvHasher::default();
                Hash::hash_slice(&solution, &mut hasher);
                if !final_set.contains(&hasher.finish()) {
                    Self::expand_one_solution_to_lower_level(solution, final_set);
                }
                solution.insert(i, el);
            }
        }
        let mut hasher = FnvHasher::default();
        Hash::hash_slice(&solution, &mut hasher);
        final_set.insert(hasher.finish());
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
        assert_eq!(HashOnlyExpander::expand(parsed_set).len(), 14);
    }

    #[test]
    fn test_2() {
        let parsed_set = vec![
            JsonSet {
                set: vec![57, 58, 59, 60],
            },
            JsonSet { set: vec![60, 99] },
        ];
        assert_eq!(HashOnlyExpander::expand(parsed_set).len(), 17);
    }
}

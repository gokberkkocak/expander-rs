use std::hash::{Hash, Hasher};

use fnv::{FnvHasher, FnvHashSet};

use crate::exploder::Exploder;
#[derive(Default)]
pub struct HashOnlyExploder;


impl Exploder for HashOnlyExploder {
    type SolutionType = Vec<u8>;

    type HashType = u64;

    fn explode_one_solution_to_lower_level(
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
                    Self::explode_one_solution_to_lower_level(solution, final_set);
                }
                solution.insert(i, el);
            }
        }
        let mut hasher = FnvHasher::default();
        Hash::hash_slice(&solution, &mut hasher);
        final_set.insert(hasher.finish());
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
        assert_eq!(HashOnlyExploder::explode(parsed_set).len(), 14);
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
        assert_eq!(HashOnlyExploder::explode(parsed_set).len(), 17);
    }
}

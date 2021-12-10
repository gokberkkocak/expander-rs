use fnv::FnvHashSet;

use crate::expander::Expander;

pub(crate) struct BitManipulatorExpander;

impl Expander for BitManipulatorExpander {
    type SolutionType = u128;

    type HashType = u128;

    fn explode_one_solution_to_lower_level(
        solution: &mut Self::SolutionType,
        final_set: &mut FnvHashSet<Self::HashType>,
    ) {
        let length = solution.count_ones();
        if length > 1 {
            for i in 0..u128::BITS {
                if (*solution & (1 << i)) >> i == 1 {
                    let mut new_sol = *solution ^ (1 << i);
                    if !final_set.contains(&new_sol) {
                        Self::explode_one_solution_to_lower_level(&mut new_sol, final_set);
                    }
                }
            }
        }
        final_set.insert(*solution);
    }

    fn explode(parsed_set: Vec<crate::JsonSet>) -> FnvHashSet<Self::HashType> {
        let mut final_set = FnvHashSet::default();
        let parsed_set = parsed_set
            .iter()
            .map(|x| convert_set_to_u128(&x.set))
            .collect::<Vec<_>>();
        for mut i in parsed_set {
            Self::explode_one_solution_to_lower_level(&mut i, &mut final_set);
        }
        final_set
    }
}

fn convert_set_to_u128(sol: &[u8]) -> u128 {
    let mut sol_u64 = 0;
    for i in sol {
        sol_u64 = sol_u64 | (1 << ((i - 1) as u64));
    }
    sol_u64
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let parsed_set = vec![
            crate::JsonSet { set: vec![1, 2, 3] },
            crate::JsonSet { set: vec![4, 5, 6] },
        ];
        assert_eq!(BitManipulatorExpander::explode(parsed_set).len(), 14);
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
        assert_eq!(BitManipulatorExpander::explode(parsed_set).len(), 17);
    }
}

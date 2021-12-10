use fnv::FnvHashSet;

use crate::JsonSet;

pub(crate) trait Exploder {
    type SolutionType;
    type HashType;
    fn explode(parsed_set: Vec<JsonSet>) -> FnvHashSet<Self::HashType>;
    fn explode_one_solution_to_lower_level(solution: &mut Self::SolutionType, final_set: &mut FnvHashSet<Self::HashType>);
}
pub(crate) mod bitman;
pub(crate) mod bitvec;
pub(crate) mod hashonly;

use fnv::FnvHashSet;
use crate::JsonSet;

pub(crate) trait Expander {
    type SolutionType;
    type HashType;
    fn expand(parsed_set: Vec<JsonSet>) -> FnvHashSet<Self::HashType>;
    fn expand_one_solution_to_lower_level(
        solution: &mut Self::SolutionType,
        final_set: &mut FnvHashSet<Self::HashType>,
    );
}

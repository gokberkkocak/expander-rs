pub(crate) mod bitman;
pub(crate) mod bitvec;
pub(crate) mod set;
pub(crate) mod vec;
pub(crate) mod vechashonly;

use crate::JsonSet;
use set::SetLike;
use std::hash::Hash;

pub(crate) trait Expander
where
    Self::HashType: Eq + Hash,
    Self::SetType: Default,
    Self::SetType: SetLike<Self::HashType>,
{
    type SolutionType;
    type SetType;
    type HashType;
    fn expand(parsed_set: Vec<JsonSet>) -> Self::SetType;
    fn expand_one_solution_to_lower_level(
        solution: &mut Self::SolutionType,
        final_set: &mut Self::SetType,
    );
}

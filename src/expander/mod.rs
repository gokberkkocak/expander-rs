pub(crate) mod bitman;
pub(crate) mod bitvec;
pub(crate) mod set;
pub(crate) mod vec;
pub(crate) mod vechashonly;

use std::hash::Hash;

use crate::JsonSet;

use self::set::SetLike;
pub(crate) trait Expander
where
    Self::HashType: Eq,
    Self::HashType: Hash,
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

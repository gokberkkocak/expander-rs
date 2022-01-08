pub(crate) mod bitman;
pub(crate) mod bitvec;
pub(crate) mod vechashonly;
pub(crate) mod vec;

use std::{collections::HashSet, hash::Hash};

use ahash::AHashSet;
use fnv::FnvHashSet;
use fxhash::FxHashSet;
use serde::Serialize;

use crate::JsonSet;
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
    fn expand_one_solution_to_lower_level(solution: &mut Self::SolutionType, final_set: &mut Self::SetType);
}

pub(crate) trait SetLike<T>
where
    T: Eq,
    T: Hash,
{
    fn set_insert(&mut self, item: T);
    fn set_contains(&self, item: &T) -> bool;
}

macro_rules! impl_setlike {
    ($t:ident) => {
        impl<T> SetLike<T> for $t<T>
        where
            T: Eq,
            T: Hash,
        {
            #[inline]
            fn set_insert(&mut self, item: T) {
                self.insert(item);
            }
            #[inline]
            fn set_contains(&self, item: &T) -> bool {
                self.contains(item)
            }
        }
    };
}

impl_setlike!(HashSet);
impl_setlike!(FxHashSet);
impl_setlike!(FnvHashSet);
impl_setlike!(AHashSet);

trait My : Serialize {}

impl<T : Serialize + Eq + Hash> My for AHashSet<T> {}
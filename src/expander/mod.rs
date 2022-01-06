pub(crate) mod bitman;
pub(crate) mod bitvec;
pub(crate) mod hashonly;

use std::{collections::HashSet, hash::Hash};

use ahash::AHashSet;
use fnv::FnvHashSet;
use fxhash::FxHashSet;

use crate::JsonSet;
pub(crate) trait Expander<T>
where
    Self::HashType: Eq,
    Self::HashType: Hash,
    T: Default,
    T: IntoIterator,
    T::Item: Into<Self::HashType>,
    T: Insert<Self::HashType>,
    T: Contains<Self::HashType>,
{
    type SolutionType;
    type HashType;
    fn expand(parsed_set: Vec<JsonSet>) -> T;
    fn expand_one_solution_to_lower_level(solution: &mut Self::SolutionType, final_set: &mut T);
}

pub(crate) trait Insert<T>
where
    T: Eq,
    T: Hash,
{
    fn insert_(&mut self, item: T);
}

pub(crate) trait Contains<T>
where
    T: Eq,
    T: Hash,
{
    fn contains_(&self, item: &T) -> bool;
}

macro_rules! impl_insert_and_contains {
    ($t:ident) => {
        impl<T> Insert<T> for $t<T>
        where
            T: Eq,
            T: Hash,
        {
            #[inline]
            fn insert_(&mut self, item: T) {
                self.insert(item);
            }
        }

        impl<T> Contains<T> for $t<T>
        where
            T: Eq,
            T: Hash,
        {
            #[inline]
            fn contains_(&self, item: &T) -> bool {
                self.contains(item)
            }
        }
    };
}

impl_insert_and_contains!(HashSet);
impl_insert_and_contains!(FxHashSet);
impl_insert_and_contains!(FnvHashSet);
impl_insert_and_contains!(AHashSet);
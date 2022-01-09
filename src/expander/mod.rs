pub(crate) mod bitman;
pub(crate) mod bitvec;
pub(crate) mod vechashonly;
pub(crate) mod vec;

use std::{collections::HashSet, hash::Hash};

use ahash::AHashSet;
use ::bitvec::prelude::BitVec;
use fnv::FnvHashSet;
use fxhash::FxHashSet;

use serde::{Serialize, ser::SerializeSeq};
use erased_serde::Serialize as ErasedSerialize;

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

#[derive(Default)]
pub(crate) struct WrappedAHashSet<T : Eq + Hash>(AHashSet<T>);

impl_setlike!(WrappedAHashSet);


impl<T: Eq + Hash> std::ops::Deref for WrappedAHashSet<T> {
    type Target = AHashSet<T>;
    fn deref(&self) -> &AHashSet<T> { &self.0 }
}

impl<T: Eq + Hash> std::ops::DerefMut for WrappedAHashSet<T> {
    fn deref_mut(&mut self) -> &mut AHashSet<T> { &mut self.0 }
}

impl<T: Eq + Hash> IntoIterator for WrappedAHashSet<T> {
    type Item = T;
    type IntoIter = <AHashSet<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Eq + Hash + Serialize> Serialize for WrappedAHashSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub(crate) struct WrappedBitVec(BitVec);

impl Serialize for WrappedBitVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for e in 0..self.0.len() {
                seq.serialize_element( &self.0[e] )?;
            }
            seq.end()
    }
}

pub(crate) trait SetLen {
    fn set_len(&self) -> usize;
}

macro_rules! impl_setlen {
    ($t:ident) => {
        impl<T : Eq + Hash> SetLen for $t<T>
        {
            #[inline]
            fn set_len(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_setlen!(HashSet);
impl_setlen!(FxHashSet);
impl_setlen!(FnvHashSet);
impl_setlen!(AHashSet);
impl_setlen!(WrappedAHashSet);

impl Serialize for dyn SetLen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            erased_serde::serialize(self, serializer)
    }
}

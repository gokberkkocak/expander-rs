use ahash::AHashSet;
use bitvec::prelude::BitVec;
use fnv::FnvHashSet;
use fxhash::FxHashSet;
use serde::{ser::SerializeSeq, Serialize};
use std::{collections::HashSet, hash::Hash};

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
pub(crate) struct WrappedAHashSet<T: Eq + Hash>(AHashSet<T>);

impl_setlike!(WrappedAHashSet);

impl<T: Eq + Hash> std::ops::Deref for WrappedAHashSet<T> {
    type Target = AHashSet<T>;
    fn deref(&self) -> &AHashSet<T> {
        &self.0
    }
}

impl<T: Eq + Hash> std::ops::DerefMut for WrappedAHashSet<T> {
    fn deref_mut(&mut self) -> &mut AHashSet<T> {
        &mut self.0
    }
}

impl<T: Eq + Hash + Serialize> Serialize for WrappedAHashSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub(crate) struct WrappedBitVec(pub BitVec);

impl Serialize for WrappedBitVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in 0..self.0.len() {
            if self.0[e] {
                seq.serialize_element(&e)?;
            }
        }
        seq.end()
    }
}

pub(crate) trait SerializedSetLen: erased_serde::Serialize {
    fn set_len(&self) -> usize;
}

macro_rules! impl_setlen {
    ($t:ident) => {
        impl<T: Eq + Hash + Serialize> SerializedSetLen for $t<T> {
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
impl_setlen!(WrappedAHashSet);

impl Serialize for dyn SerializedSetLen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}

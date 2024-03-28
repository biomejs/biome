use crate::{self as biome_deserialize, Merge};
use biome_deserialize_macros::Deserializable;
use indexmap::set::IntoIter;
use indexmap::IndexSet;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

// To implement serde's traits, we encapsulate `IndexSet<String>` in a new type `StringSet`.

#[derive(Clone, Default, Debug, Deserializable, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StringSet(IndexSet<String>);

impl StringSet {
    pub fn new(index_set: IndexSet<String>) -> Self {
        Self(index_set)
    }

    pub fn into_index_set(self) -> IndexSet<String> {
        self.0
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.0.extend(iter)
    }
}

impl Deref for StringSet {
    type Target = IndexSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for StringSet {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(StringSet::default())
    }
}

impl<'de> Deserialize<'de> for StringSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IndexVisitor;
        impl<'de> Visitor<'de> for IndexVisitor {
            type Value = IndexSet<String>;

            // Format a message stating what data this Visitor expects to receive.
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut index_set = IndexSet::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(value) = seq.next_element()? {
                    index_set.insert(value);
                }
                Ok(index_set)
            }
        }
        deserializer.deserialize_seq(IndexVisitor).map(StringSet)
    }
}

impl FromIterator<String> for StringSet {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        StringSet::new(IndexSet::from_iter(iter))
    }
}

impl IntoIterator for StringSet {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Merge for StringSet {
    fn merge_with(&mut self, other: Self) {
        self.extend(other)
    }
}

impl Serialize for StringSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for item in self.0.iter() {
            sequence.serialize_element(&item)?;
        }
        sequence.end()
    }
}

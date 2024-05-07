use crate::{self as biome_deserialize, Merge};
use biome_deserialize_macros::Deserializable;
use indexmap::set::IntoIter;
use indexmap::IndexSet;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

// To implement schemars trait, we encapsulate `IndexSet<String>` in a new type `StringSet`.

#[derive(
    Clone, Default, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for StringSet {
    fn schema_name() -> String {
        String::from("StringSet")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <std::collections::HashSet<String>>::json_schema(gen)
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

use indexmap::IndexSet;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Default, Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StringSet(
    #[serde(
        deserialize_with = "crate::deserialize_string_set",
        serialize_with = "crate::serialize_string_set"
    )]
    pub IndexSet<String>,
);

impl StringSet {
    pub fn new(index_set: IndexSet<String>) -> Self {
        Self(index_set)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn index_set(&self) -> &IndexSet<String> {
        &self.0
    }

    pub fn into_index_set(self) -> IndexSet<String> {
        self.0
    }

    pub fn extend(&mut self, entries: impl IntoIterator<Item = String>) {
        self.0.extend(entries);
    }
}

/// Some documentation
pub fn deserialize_string_set<'de, D>(deserializer: D) -> Result<IndexSet<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct IndexVisitor {
        marker: PhantomData<fn() -> IndexSet<String>>,
    }

    impl IndexVisitor {
        fn new() -> Self {
            IndexVisitor {
                marker: PhantomData,
            }
        }
    }

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

    deserializer.deserialize_seq(IndexVisitor::new())
}

pub fn serialize_string_set<S>(string_set: &IndexSet<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let mut sequence = s.serialize_seq(Some(string_set.len()))?;
    let iter = string_set.into_iter();
    for global in iter {
        sequence.serialize_element(&global)?;
    }

    sequence.end()
}

impl FromStr for StringSet {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(StringSet::default())
    }
}

impl From<IndexSet<String>> for StringSet {
    fn from(value: IndexSet<String>) -> Self {
        Self::new(value)
    }
}

use crate::{Deserializable, DeserializableValue};
use indexmap::IndexSet;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// To implement serde's traits, we encapsulate `IndexSet<String>` in a new type `StringSet`.

#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StringSet(IndexSet<String>);

impl StringSet {
    pub fn new(index_set: IndexSet<String>) -> Self {
        Self(index_set)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

impl FromStr for StringSet {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(StringSet::default())
    }
}

impl From<IndexSet<String>> for StringSet {
    fn from(value: IndexSet<String>) -> Self {
        Self::new(value)
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

impl Deserializable for StringSet {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<crate::DeserializationDiagnostic>,
    ) -> Option<Self> {
        Deserializable::deserialize(value, diagnostics).map(StringSet)
    }
}

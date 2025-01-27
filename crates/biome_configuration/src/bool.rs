use biome_console::fmt::Formatter;
use biome_deserialize::{Deserializable, DeserializableValue, DeserializationContext, Merge};
use std::{
    fmt,
    str::{FromStr, ParseBoolError},
};

/// A `bool` type wrapper with a configurable default value (`true` or `false`)
///
/// The const generic indicates the default bool value of this wrapper type
#[derive(Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bool<const D: bool>(pub bool);

impl<const D: bool> Merge for Bool<D> {
    fn merge_with(&mut self, other: Self) {
        self.value().merge_with(other.value())
    }
}

impl<const D: bool> Bool<D> {
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Default for Bool<true> {
    fn default() -> Self {
        Self(true)
    }
}

impl Default for Bool<false> {
    fn default() -> Self {
        Self(false)
    }
}

impl<const D: bool> FromStr for Bool<D> {
    type Err = ParseBoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(bool::from_str(s)?))
    }
}

impl<const D: bool> Deserializable for Bool<D> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        bool::deserialize(ctx, value, name).map(Self)
    }
}

impl<const D: bool> fmt::Debug for Bool<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(f)
    }
}

impl<const D: bool> fmt::Display for Bool<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(f)
    }
}

impl biome_console::fmt::Display for Bool<true> {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_str("true")
    }
}

impl biome_console::fmt::Display for Bool<false> {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_str("false")
    }
}

impl<const D: bool> From<bool> for Bool<D> {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl<const D: bool> From<Bool<D>> for bool {
    fn from(value: Bool<D>) -> Self {
        value.value()
    }
}

impl From<Bool<false>> for Bool<true> {
    fn from(value: Bool<false>) -> Self {
        Self(value.value())
    }
}

impl From<Bool<true>> for Bool<false> {
    fn from(value: Bool<true>) -> Self {
        Self(value.value())
    }
}

#[cfg(feature = "schema")]
impl<const D: bool> schemars::JsonSchema for Bool<D> {
    fn schema_name() -> String {
        "Bool".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        bool::json_schema(gen)
    }
}

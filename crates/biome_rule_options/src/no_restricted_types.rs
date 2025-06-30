use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedTypesOptions {
    pub types: FxHashMap<Box<str>, CustomRestrictedType>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum CustomRestrictedType {
    Plain(Box<str>),
    WithOptions(CustomRestrictedTypeOptions),
}

impl From<CustomRestrictedType> for CustomRestrictedTypeOptions {
    fn from(options: CustomRestrictedType) -> Self {
        match options {
            CustomRestrictedType::Plain(message) => Self {
                message,
                use_instead: None,
            },
            CustomRestrictedType::WithOptions(options) => options,
        }
    }
}

impl Deserializable for CustomRestrictedType {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    biome_deserialize_macros::Deserializable,
    Deserialize,
    Serialize,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct CustomRestrictedTypeOptions {
    pub message: Box<str>,
    #[serde(rename = "use")]
    pub use_instead: Option<String>,
}

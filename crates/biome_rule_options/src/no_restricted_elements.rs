use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedElementsOptions {
    /// Elements to restrict.
    /// Each key is the element name, and the value is the message to show when the element is used.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub elements: Option<CustomRestrictedElements>,
}

impl biome_deserialize::Merge for NoRestrictedElementsOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(elements) = other.elements {
            self.elements = Some(elements);
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    biome_deserialize_macros::Deserializable,
    Eq,
    PartialEq,
    Deserialize,
    Serialize,
)]
pub struct CustomRestrictedElements(CustomRestrictedElementsBaseType);

impl Deref for CustomRestrictedElements {
    type Target = CustomRestrictedElementsBaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for CustomRestrictedElements {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("CustomRestrictedElements")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Generate schema for FxHashMap<Box<str>, Box<str>> inline
        schemars::json_schema!({
            "type": "object",
            "additionalProperties": {
                "type": "string"
            },
            "minProperties": 1,
            "description": "Elements to restrict. Each key is the element name, and the value is the message to show when the element is used."
        })
    }
}

type CustomRestrictedElementsBaseType = FxHashMap<Box<str>, Box<str>>;

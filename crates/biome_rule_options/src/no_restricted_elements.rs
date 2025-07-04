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
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub elements: CustomRestrictedElements,
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
    fn schema_name() -> String {
        "CustomRestrictedElements".to_owned()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema = generator
            .subschema_for::<CustomRestrictedElementsBaseType>()
            .into_object();
        schema.object().min_properties = Some(1);
        schemars::schema::Schema::Object(schema)
    }
}

type CustomRestrictedElementsBaseType = FxHashMap<Box<str>, Box<str>>;

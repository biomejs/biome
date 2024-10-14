use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationDiagnostic,
};
use biome_deserialize_macros::{Deserializable, Merge};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Plugins(pub Vec<PluginConfiguration>);

impl FromStr for Plugins {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PluginConfiguration {
    Path(String),
    // TODO: PathWithOptions(PluginPathWithOptions),
}

impl Deserializable for PluginConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(value, rule_name, diagnostics).map(Self::Path)
        } else {
            // TODO: Fix this to allow plugins to receive options.
            //       Difficulty is that we need a `Deserializable` implementation
            //       for `serde_json::Value`, since plugin options are untyped.
            //       Also, we don't have a way to configure Grit plugins yet.
            /*Deserializable::deserialize(value, rule_name, diagnostics)
            .map(|plugin| Self::PathWithOptions(plugin))*/
            None
        }
    }
}

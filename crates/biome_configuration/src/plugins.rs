use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Plugins(pub Vec<PluginConfiguration>);

impl Plugins {
    pub fn iter(&self) -> impl Iterator<Item = &PluginConfiguration> {
        self.0.iter()
    }
}

impl FromStr for Plugins {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PluginConfiguration {
    Path(String),
    // TODO: PathWithOptions(PluginPathWithOptions),
}

impl Deserializable for PluginConfiguration {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        rule_name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, rule_name).map(Self::Path)
        } else {
            // TODO: Fix this to allow plugins to receive options.
            //       We probably need to pass them as `AnyJsonValue` or
            //       `biome_json_value::JsonValue`, since plugin options are
            //       untyped.
            //       Also, we don't have a way to configure Grit plugins yet.
            /*Deserializable::deserialize(value, rule_name, diagnostics)
            .map(|plugin| Self::PathWithOptions(plugin))*/
            None
        }
    }
}

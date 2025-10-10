use biome_console::fmt::Display;
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentGraphqlDescriptionsOptions {
    /// The description style to enforce. Defaults to "block"
    pub style: Style,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Style {
    /// Requires triple-quoted block descriptions (`"""..."""`)
    #[default]
    Block,
    /// Requires single-quoted inline descriptions (`"..."`)
    Inline,
}

impl FromStr for Style {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "block" => Ok(Self::Block),
            "inline" => Ok(Self::Inline),
            _ => Err("Value not supported for description style, expected 'block' or 'inline'"),
        }
    }
}

impl Display for Style {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            Self::Block => fmt.write_str("block"),
            Self::Inline => fmt.write_str("inline"),
        }
    }
}

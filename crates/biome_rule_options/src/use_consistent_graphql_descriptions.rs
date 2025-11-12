use biome_console::fmt::Display;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentGraphqlDescriptionsOptions {
    /// The description style to enforce. Defaults to "block"
    pub style: Option<Style>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    /// Requires triple-quoted block descriptions (`"""..."""`)
    #[default]
    Block,
    /// Requires single-quoted inline descriptions (`"..."`)
    Inline,
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Style {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("UseConsistentGraphqlDescriptionsStyle")
    }

    fn json_schema(_generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "enum": ["block", "inline"],
            "description": "The GraphQL description style to enforce."
        })
    }
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

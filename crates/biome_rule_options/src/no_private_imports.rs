use biome_console::fmt::Display;
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoPrivateImportsOptions {
    /// The default visibility to assume for symbols without visibility tag.
    ///
    /// Default: **public**.
    pub default_visibility: Visibility,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    #[default]
    Public,
    Package,
    Private,
}

impl Display for Visibility {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            Self::Public => fmt.write_str("public"),
            Self::Package => fmt.write_str("package"),
            Self::Private => fmt.write_str("private"),
        }
    }
}

impl FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Self::Public),
            "package" => Ok(Self::Package),
            "private" => Ok(Self::Private),
            _ => Err(()),
        }
    }
}

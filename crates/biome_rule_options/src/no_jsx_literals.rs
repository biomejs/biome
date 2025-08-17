use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoJsxLiteralsOptions {
    /// When enabled, no strings are allowed inside JSX children
    #[serde(default)]
    pub no_strings: bool,

    /// An array of strings that won't trigger the rule. Whitespaces are taken into consideration
    #[serde(default)]
    pub allowed_strings: Box<[Box<str>]>,
}

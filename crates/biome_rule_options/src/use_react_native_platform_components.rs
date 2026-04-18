use crate::restricted_regex::RestrictedRegex;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Merge, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseReactNativePlatformComponentsOptions {
    /// A regular expression pattern to identify Android-specific files.
    /// Defaults to `.*[.]android[.][jt]sx?`.
    pub android_path_regex: RestrictedRegex,

    /// A regular expression pattern to identify iOS-specific files.
    /// Defaults to `.*[.]ios[.][jt]sx?`.
    pub ios_path_regex: RestrictedRegex,
}

impl Default for UseReactNativePlatformComponentsOptions {
    fn default() -> Self {
        Self {
            android_path_regex: RestrictedRegex::from_str(".*[.]android[.][jt]sx?")
                .expect("valid regex"),
            ios_path_regex: RestrictedRegex::from_str(".*[.]ios[.][jt]sx?").expect("valid regex"),
        }
    }
}

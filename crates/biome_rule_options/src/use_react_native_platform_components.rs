use biome_deserialize_macros::{Deserializable, Merge};
use biome_glob::NormalizedGlob;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

fn default_android_path_patterns() -> Vec<NormalizedGlob> {
    vec![NormalizedGlob::from_str("**/*.android.{js,jsx,ts,tsx}").expect("valid glob")]
}

fn default_ios_path_patterns() -> Vec<NormalizedGlob> {
    vec![NormalizedGlob::from_str("**/*.ios.{js,jsx,ts,tsx}").expect("valid glob")]
}

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseReactNativePlatformComponentsOptions {
    /// A list of glob patterns to identify Android-specific files.
    /// Defaults to `["**/*.android.{js,jsx,ts,tsx}"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_path_patterns: Option<Vec<NormalizedGlob>>,

    /// A list of glob patterns to identify iOS-specific files.
    /// Defaults to `["**/*.ios.{js,jsx,ts,tsx}"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_path_patterns: Option<Vec<NormalizedGlob>>,
}

impl Default for UseReactNativePlatformComponentsOptions {
    fn default() -> Self {
        Self {
            android_path_patterns: Some(default_android_path_patterns()),
            ios_path_patterns: Some(default_ios_path_patterns()),
        }
    }
}

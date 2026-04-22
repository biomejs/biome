use biome_deserialize_macros::{Deserializable, Merge};
use biome_glob::NormalizedGlob;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::LazyLock;

static DEFAULT_ANDROID_PATH_PATTERNS: LazyLock<Vec<NormalizedGlob>> = LazyLock::new(|| {
    vec![NormalizedGlob::from_str("**/*.android.{js,jsx,ts,tsx}").expect("valid glob")]
});

static DEFAULT_IOS_PATH_PATTERNS: LazyLock<Vec<NormalizedGlob>> = LazyLock::new(|| {
    vec![NormalizedGlob::from_str("**/*.ios.{js,jsx,ts,tsx}").expect("valid glob")]
});

#[derive(Clone, Default, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Merge)]
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

impl UseReactNativePlatformComponentsOptions {
    pub fn android_path_patterns(&self) -> &[NormalizedGlob] {
        self.android_path_patterns
            .as_deref()
            .unwrap_or(DEFAULT_ANDROID_PATH_PATTERNS.as_ref())
    }

    pub fn ios_path_patterns(&self) -> &[NormalizedGlob] {
        self.ios_path_patterns
            .as_deref()
            .unwrap_or(DEFAULT_IOS_PATH_PATTERNS.as_ref())
    }
}

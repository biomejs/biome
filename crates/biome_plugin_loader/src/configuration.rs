use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_fs::normalize_path;
use biome_glob::NormalizedGlob;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Plugins(pub Vec<PluginConfiguration>);

impl Plugins {
    pub fn iter(&self) -> impl Iterator<Item = &PluginConfiguration> {
        self.deref().iter()
    }

    /// Normalizes plugin paths in-place.
    ///
    /// For each relative path, this joins it with `base_dir` and normalizes
    /// `.` / `..` segments (without resolving symlinks).
    pub fn normalize_relative_paths(&mut self, base_dir: &Utf8Path) {
        for plugin_config in self.0.iter_mut() {
            let plugin_path = match plugin_config {
                PluginConfiguration::Path(path) => path,
                PluginConfiguration::PathWithOptions(opts) => &mut opts.path,
            };
            let path_buf = Utf8Path::new(plugin_path.as_str());
            if path_buf.is_absolute() {
                continue;
            }
            let normalized = normalize_path(&base_dir.join(path_buf));
            *plugin_path = normalized.to_string();
        }
    }
}

impl FromStr for Plugins {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

impl Deref for Plugins {
    type Target = Vec<PluginConfiguration>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Plugins {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Configuration for a single plugin entry.
///
/// Can be either a plain path string or an object with path and options:
///
/// ```json
/// {
///   "plugins": [
///     "simple-plugin.grit",
///     { "path": "scoped-plugin.grit", "includes": ["src/**/*.ts"] }
///   ]
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PluginConfiguration {
    /// A plain path to the plugin.
    Path(String),

    /// A path with additional options.
    PathWithOptions(PluginWithOptions),
}

impl PluginConfiguration {
    /// Returns the plugin path.
    pub fn path(&self) -> &str {
        match self {
            Self::Path(path) => path,
            Self::PathWithOptions(opts) => &opts.path,
        }
    }

    /// Returns the includes patterns, if any.
    pub fn includes(&self) -> Option<&[NormalizedGlob]> {
        match self {
            Self::Path(_) => None,
            Self::PathWithOptions(opts) => opts.includes.as_deref(),
        }
    }
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
            Deserializable::deserialize(ctx, value, rule_name).map(Self::PathWithOptions)
        }
    }
}

/// Plugin path with additional options.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginWithOptions {
    /// The path to the plugin.
    pub path: String,

    /// A list of glob patterns. The plugin will only run on files matching
    /// these patterns. Use negated globs (e.g., `!**/*.test.ts`) for exclusions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<NormalizedGlob>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_relative_paths_makes_paths_base_dir_relative_and_normalized() {
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![
            PluginConfiguration::Path("./biome/../biome/my-plugin.grit".into()),
            PluginConfiguration::Path("other.grit".into()),
        ]);

        plugins.normalize_relative_paths(base_dir);

        let first = plugins.0[0].path();
        assert!(Utf8Path::new(first).starts_with(base_dir));
        let expected_suffix = Utf8Path::new("biome").join("my-plugin.grit");
        assert!(Utf8Path::new(first).ends_with(expected_suffix.as_path()));

        let second = plugins.0[1].path();
        assert!(Utf8Path::new(second).starts_with(base_dir));
        assert!(Utf8Path::new(second).ends_with("other.grit"));
    }

    #[test]
    fn normalize_relative_paths_leaves_absolute_paths_unchanged() {
        let base_dir = Utf8Path::new("base");
        let sep = std::path::MAIN_SEPARATOR;
        let absolute = format!("{sep}abs{sep}my-plugin.grit");
        let mut plugins = Plugins(vec![PluginConfiguration::Path(absolute.clone())]);

        plugins.normalize_relative_paths(base_dir);

        assert_eq!(plugins.0[0].path(), &absolute);
    }

    #[test]
    fn normalize_relative_paths_with_options() {
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![PluginConfiguration::PathWithOptions(
            PluginWithOptions {
                path: "./my-plugin.grit".into(),
                includes: Some(vec!["src/**/*.ts".parse().unwrap()]),
            },
        )]);

        plugins.normalize_relative_paths(base_dir);

        let path = plugins.0[0].path();
        assert!(Utf8Path::new(path).starts_with(base_dir));
        assert!(Utf8Path::new(path).ends_with("my-plugin.grit"));

        // includes should be unchanged
        assert!(plugins.0[0].includes().is_some());
    }
}

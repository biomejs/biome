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
            normalize_plugin_path(plugin_path, base_dir);
        }
    }

    /// Normalizes object-syntax plugin paths with `resolvePath: "config"` in-place.
    pub fn normalize_object_relative_paths(&mut self, base_dir: &Utf8Path) {
        for plugin_config in self.0.iter_mut() {
            let PluginConfiguration::PathWithOptions(opts) = plugin_config else {
                continue;
            };
            // Only normalize paths for plugins that explicitly opt in to config-relative resolution.
            if opts.resolve_path != Some(PluginResolvePath::Config) {
                continue;
            }

            normalize_plugin_path(&mut opts.path, base_dir);
        }
    }
}

fn normalize_plugin_path(plugin_path: &mut String, base_dir: &Utf8Path) {
    let path_buf = Utf8Path::new(plugin_path.as_str());
    if path_buf.is_absolute() {
        return;
    }
    let normalized = normalize_path(&base_dir.join(path_buf));
    *plugin_path = normalized.to_string();
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
///     { "path": "scoped-plugin.grit", "includes": ["src/**/*.ts"] },
///     { "path": "./local-plugin.grit", "resolvePath": "config" }
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
    #[deserializable(required)]
    pub path: String,

    /// A list of glob patterns. The plugin will only run on files matching
    /// these patterns. Use negated globs (e.g., `!**/*.test.ts`) for exclusions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<NormalizedGlob>>,

    /// Controls how relative plugin paths are resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_path: Option<PluginResolvePath>,
}

impl PluginWithOptions {
    pub fn resolve_path(&self) -> PluginResolvePath {
        self.resolve_path.clone().unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PluginResolvePath {
    /// Resolve relative paths from the consuming workspace.
    #[default]
    Workspace,
    /// Resolve relative paths from the configuration file that declared the plugin.
    Config,
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_json_parser::JsonParserOptions;

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
                resolve_path: None,
            },
        )]);

        plugins.normalize_relative_paths(base_dir);

        let path = plugins.0[0].path();
        assert!(Utf8Path::new(path).starts_with(base_dir));
        assert!(Utf8Path::new(path).ends_with("my-plugin.grit"));

        // includes should be unchanged
        assert!(plugins.0[0].includes().is_some());
    }

    #[test]
    fn normalize_object_relative_paths_leaves_string_entries_unchanged() {
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![
            PluginConfiguration::Path("./biome/../biome/my-plugin.grit".into()),
            PluginConfiguration::PathWithOptions(PluginWithOptions {
                path: "./default-plugin.grit".into(),
                includes: None,
                resolve_path: None,
            }),
            PluginConfiguration::PathWithOptions(PluginWithOptions {
                path: "./other/../other/object-plugin.grit".into(),
                includes: None,
                resolve_path: Some(PluginResolvePath::Config),
            }),
            PluginConfiguration::PathWithOptions(PluginWithOptions {
                path: "./workspace-plugin.grit".into(),
                includes: None,
                resolve_path: Some(PluginResolvePath::Workspace),
            }),
        ]);

        plugins.normalize_object_relative_paths(base_dir);

        assert_eq!(plugins.0[0].path(), "./biome/../biome/my-plugin.grit");

        assert_eq!(plugins.0[1].path(), "./default-plugin.grit");

        let config_path = plugins.0[2].path();
        assert!(Utf8Path::new(config_path).starts_with(base_dir));
        let expected_suffix = Utf8Path::new("other").join("object-plugin.grit");
        assert!(Utf8Path::new(config_path).ends_with(expected_suffix.as_path()));

        assert_eq!(plugins.0[3].path(), "./workspace-plugin.grit");
    }

    #[test]
    fn deserialize_plain_string() {
        let config: PluginConfiguration = serde_json::from_str(r#""my-plugin.grit""#).unwrap();
        assert_eq!(config.path(), "my-plugin.grit");
        assert!(config.includes().is_none());
    }

    #[test]
    fn deserialize_object_with_includes() {
        let config: PluginConfiguration =
            serde_json::from_str(r#"{ "path": "my-plugin.grit", "includes": ["src/**/*.ts"] }"#)
                .unwrap();
        assert_eq!(config.path(), "my-plugin.grit");
        assert_eq!(config.includes().unwrap().len(), 1);
    }

    #[test]
    fn deserialize_object_without_includes() {
        let config: PluginConfiguration =
            serde_json::from_str(r#"{ "path": "my-plugin.grit" }"#).unwrap();
        assert_eq!(config.path(), "my-plugin.grit");
        assert!(config.includes().is_none());
    }

    #[test]
    fn deserialize_object_with_config_resolve_path() {
        let config: PluginConfiguration =
            serde_json::from_str(r#"{ "path": "my-plugin.grit", "resolvePath": "config" }"#)
                .unwrap();
        let PluginConfiguration::PathWithOptions(options) = config else {
            panic!("expected object syntax plugin");
        };
        assert_eq!(options.resolve_path(), PluginResolvePath::Config);
    }

    #[test]
    fn deserialize_object_missing_path_emits_error() {
        let source = r#"{ "includes": ["src/**"] }"#;
        let result = deserialize_from_json_str::<PluginWithOptions>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(result.has_errors());
    }

    #[test]
    fn deserialize_plugins_list_mixed() {
        let plugins: Plugins = serde_json::from_str(
            r#"["simple.grit", { "path": "scoped.grit", "includes": ["src/**"] }]"#,
        )
        .unwrap();
        assert_eq!(plugins.0.len(), 2);
        assert!(matches!(plugins.0[0], PluginConfiguration::Path(_)));
        assert!(matches!(
            plugins.0[1],
            PluginConfiguration::PathWithOptions(_)
        ));
    }
}

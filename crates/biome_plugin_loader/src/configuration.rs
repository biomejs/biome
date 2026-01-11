use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_fs::normalize_path;
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
                PluginConfiguration::WithOptions(opts) => &mut opts.path,
            };

            let plugin_path_buf = Utf8Path::new(plugin_path.as_str());
            if plugin_path_buf.is_absolute() {
                continue;
            }

            let normalized = normalize_path(&base_dir.join(plugin_path_buf));
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PluginConfiguration {
    /// A simple path to a plugin file
    Path(String),
    /// A plugin with additional options
    WithOptions(PluginWithOptions),
}

impl PluginConfiguration {
    /// Returns the path to the plugin file
    pub fn path(&self) -> &str {
        match self {
            Self::Path(path) => path,
            Self::WithOptions(opts) => &opts.path,
        }
    }

    /// Returns whether the plugin is enabled (default: true)
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Path(_) => true,
            Self::WithOptions(opts) => opts.enabled.unwrap_or(true),
        }
    }
}

/// Plugin configuration with additional options
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginWithOptions {
    /// Path to the plugin file
    #[serde(default)]
    pub path: String,
    /// Whether the plugin is enabled (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Deserializable for PluginConfiguration {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        rule_name: &str,
    ) -> Option<Self> {
        match value.visitable_type()? {
            DeserializableType::Str => {
                Deserializable::deserialize(ctx, value, rule_name).map(Self::Path)
            }
            DeserializableType::Map => {
                Deserializable::deserialize(ctx, value, rule_name).map(Self::WithOptions)
            }
            _ => None,
        }
    }
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
    fn normalize_relative_paths_works_with_options() {
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![PluginConfiguration::WithOptions(PluginWithOptions {
            path: "./my-plugin.grit".into(),
            enabled: Some(false),
        })]);

        plugins.normalize_relative_paths(base_dir);

        let first = plugins.0[0].path();
        assert!(Utf8Path::new(first).starts_with(base_dir));
        assert!(Utf8Path::new(first).ends_with("my-plugin.grit"));
        assert!(!plugins.0[0].is_enabled());
    }

    #[test]
    fn plugin_is_enabled_by_default() {
        let path_only = PluginConfiguration::Path("./plugin.grit".into());
        assert!(path_only.is_enabled());

        let with_none = PluginConfiguration::WithOptions(PluginWithOptions {
            path: "./plugin.grit".into(),
            enabled: None,
        });
        assert!(with_none.is_enabled());

        let explicitly_enabled = PluginConfiguration::WithOptions(PluginWithOptions {
            path: "./plugin.grit".into(),
            enabled: Some(true),
        });
        assert!(explicitly_enabled.is_enabled());

        let explicitly_disabled = PluginConfiguration::WithOptions(PluginWithOptions {
            path: "./plugin.grit".into(),
            enabled: Some(false),
        });
        assert!(!explicitly_disabled.is_enabled());
    }
}

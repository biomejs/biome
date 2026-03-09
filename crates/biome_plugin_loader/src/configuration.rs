//! Plugin configuration types for the `plugins` array in `biome.json`.
//!
//! Each entry in the `plugins` array is a [`PluginConfiguration`], which can be
//! either a simple path string or a [`PluginPathWithOptions`] object:
//!
//! ```json
//! {
//!   "plugins": [
//!     "./simple-rule.wasm",
//!     {
//!       "path": "./configurable.wasm",
//!       "options": { "convention": "camelCase" },
//!       "rules": { "myRule": "warn" }
//!     }
//!   ]
//! }
//! ```
//!
//! The `options` object is serialized to JSON and passed to the plugin's
//! `configure()` export. The `rules` map allows per-rule configuration using
//! severity levels (`off`, `on`, `info`, `warn`, `error`) or an object with
//! `level` and `fix`.

use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableTypes, DeserializableValue,
    DeserializationContext, DeserializationVisitor,
};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_fs::normalize_path;
use biome_rowan::Text;
use biome_text_size::TextRange;
use camino::Utf8Path;
use rustc_hash::FxHashMap;
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
                PluginConfiguration::Path(p) => p,
                PluginConfiguration::PathWithOptions(opts) => &mut opts.path,
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
    Path(String),
    PathWithOptions(PluginPathWithOptions),
}

impl PluginConfiguration {
    /// Returns the plugin path regardless of variant.
    pub fn path(&self) -> &str {
        match self {
            Self::Path(p) => p,
            Self::PathWithOptions(opts) => &opts.path,
        }
    }

    /// Returns the options JSON string, if any.
    pub fn options_json(&self) -> Option<&str> {
        match self {
            Self::Path(_) => None,
            Self::PathWithOptions(opts) => Some(&opts.options),
        }
    }

    /// Returns per-rule configuration overrides, if any.
    pub fn rules(&self) -> Option<&FxHashMap<String, PluginRuleConfiguration>> {
        match self {
            Self::Path(_) => None,
            Self::PathWithOptions(opts) => opts.rules.as_ref(),
        }
    }
}

/// Per-rule configuration for a plugin rule.
///
/// Can be a short string (`"off"`, `"warn"`, `"error"`) or an object
/// with `level` and optional `fix` fields.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum PluginRuleConfiguration {
    Plain(PluginRulePlainConfiguration),
    WithOptions(PluginRuleWithOptions),
}

impl PluginRuleConfiguration {
    /// Returns `true` if this rule is disabled.
    pub fn is_disabled(&self) -> bool {
        self.level() == PluginRulePlainConfiguration::Off
    }

    /// Returns the configured severity level.
    pub fn level(&self) -> PluginRulePlainConfiguration {
        match self {
            Self::Plain(level) => *level,
            Self::WithOptions(opts) => opts.level,
        }
    }

    /// Returns the configured fix kind string, if any.
    pub fn fix_kind_str(&self) -> Option<&str> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(opts) => opts.fix.as_deref(),
        }
    }
}

/// Short-form severity level for a plugin rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PluginRulePlainConfiguration {
    Off,
    On,
    Info,
    Warn,
    Error,
}

/// Object form with a severity level and an optional fix kind.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PluginRuleWithOptions {
    pub level: PluginRulePlainConfiguration,
    /// Fix kind: `"safe"`, `"unsafe"`, or `"none"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<String>,
}

impl Deserializable for PluginRuleConfiguration {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}

impl Deserializable for PluginRulePlainConfiguration {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let text: Text = Deserializable::deserialize(ctx, value, name)?;
        match text.text() {
            "off" => Some(Self::Off),
            "on" => Some(Self::On),
            "info" => Some(Self::Info),
            "warn" => Some(Self::Warn),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}

impl Deserializable for PluginRuleWithOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, PluginRuleWithOptionsVisitor, name)
    }
}

struct PluginRuleWithOptionsVisitor;

impl DeserializationVisitor for PluginRuleWithOptionsVisitor {
    type Output = PluginRuleWithOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl ExactSizeIterator<
            Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
        >,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut level = None;
        let mut fix = None;

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "level" => {
                    level = Deserializable::deserialize(ctx, &value, &key_text);
                }
                "fix" => {
                    fix = Deserializable::deserialize(ctx, &value, &key_text);
                }
                _ => {}
            }
        }

        Some(PluginRuleWithOptions { level: level?, fix })
    }
}

/// A plugin path paired with a JSON string of configuration options.
///
/// Example biome.json:
/// ```json
/// { "path": "./my-plugin.wasm", "options": { "maxLength": 100 }, "rules": { "noConsoleLog": "off" } }
/// ```
///
/// The `options` field is stored as a raw JSON string because plugin options
/// are untyped — the guest receives them via the `configure(options-json)` export.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginPathWithOptions {
    pub path: String,
    /// Raw JSON string representing the plugin options.
    pub options: String,
    /// Per-rule configuration overrides (severity, fix kind, enable/disable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<FxHashMap<String, PluginRuleConfiguration>>,
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

impl Deserializable for PluginPathWithOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, PluginPathWithOptionsVisitor, name)
    }
}

struct PluginPathWithOptionsVisitor;

impl DeserializationVisitor for PluginPathWithOptionsVisitor {
    type Output = PluginPathWithOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl ExactSizeIterator<
            Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
        >,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut path: Option<String> = None;
        let mut options: Option<String> = None;
        let mut rules: Option<FxHashMap<String, PluginRuleConfiguration>> = None;

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "path" => {
                    path = Deserializable::deserialize(ctx, &value, &key_text);
                }
                "options" => {
                    // Deserialize as serde_json::Value to handle arbitrary JSON,
                    // then serialize back to a JSON string for the WASM guest.
                    let json_value: Option<serde_json::Value> =
                        Deserializable::deserialize(ctx, &value, &key_text);
                    options = json_value.and_then(|v| serde_json::to_string(&v).ok());
                }
                "rules" => {
                    rules = Deserializable::deserialize(ctx, &value, &key_text);
                }
                _ => {}
            }
        }

        let path = path?;
        Some(PluginPathWithOptions {
            path,
            options: options.unwrap_or_else(|| "{}".to_string()),
            rules,
        })
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

        let PluginConfiguration::Path(first) = &plugins.0[0] else {
            panic!("expected Path variant");
        };
        assert!(Utf8Path::new(first).starts_with(base_dir));
        let expected_suffix = Utf8Path::new("biome").join("my-plugin.grit");
        assert!(Utf8Path::new(first).ends_with(expected_suffix.as_path()));

        let PluginConfiguration::Path(second) = &plugins.0[1] else {
            panic!("expected Path variant");
        };
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

        let PluginConfiguration::Path(result) = &plugins.0[0] else {
            panic!("expected Path variant");
        };
        assert_eq!(result, &absolute);
    }

    #[test]
    fn normalize_relative_paths_handles_path_with_options() {
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![PluginConfiguration::PathWithOptions(
            PluginPathWithOptions {
                path: "./my-plugin.wasm".into(),
                options: r#"{"maxLength": 100}"#.into(),
                rules: None,
            },
        )]);

        plugins.normalize_relative_paths(base_dir);

        let PluginConfiguration::PathWithOptions(opts) = &plugins.0[0] else {
            panic!("expected PathWithOptions variant");
        };
        assert!(Utf8Path::new(&opts.path).starts_with(base_dir));
        assert!(Utf8Path::new(&opts.path).ends_with("my-plugin.wasm"));
        // Options should be unchanged
        assert_eq!(opts.options, r#"{"maxLength": 100}"#);
    }

    #[test]
    fn plugin_configuration_path_accessors() {
        let path_config = PluginConfiguration::Path("./foo.grit".into());
        assert_eq!(path_config.path(), "./foo.grit");
        assert_eq!(path_config.options_json(), None);

        let opts_config = PluginConfiguration::PathWithOptions(PluginPathWithOptions {
            path: "./bar.wasm".into(),
            options: r#"{"key": "value"}"#.into(),
            rules: None,
        });
        assert_eq!(opts_config.path(), "./bar.wasm");
        assert_eq!(opts_config.options_json(), Some(r#"{"key": "value"}"#));
    }

    #[test]
    fn deserialize_path_with_options_from_json() {
        use biome_deserialize::json::deserialize_from_json_str;
        use biome_json_parser::JsonParserOptions;

        let json = r#"[{"path": "./plugin.wasm", "options": {"maxLength": 100}}]"#;
        let (plugins, errors) =
            deserialize_from_json_str::<Plugins>(json, JsonParserOptions::default(), "").consume();

        assert!(errors.is_empty(), "Unexpected errors: {errors:?}");
        let plugins = plugins.unwrap();
        assert_eq!(plugins.0.len(), 1);

        let PluginConfiguration::PathWithOptions(opts) = &plugins.0[0] else {
            panic!("expected PathWithOptions variant, got: {:?}", plugins.0[0]);
        };
        assert_eq!(opts.path, "./plugin.wasm");
        // The options should be a JSON string containing the object
        assert!(opts.options.contains("maxLength"));
    }

    #[test]
    fn deserialize_path_only_from_json() {
        use biome_deserialize::json::deserialize_from_json_str;
        use biome_json_parser::JsonParserOptions;

        let json = r#"["./plugin.grit"]"#;
        let (plugins, errors) =
            deserialize_from_json_str::<Plugins>(json, JsonParserOptions::default(), "").consume();

        assert!(errors.is_empty(), "Unexpected errors: {errors:?}");
        let plugins = plugins.unwrap();
        assert_eq!(plugins.0.len(), 1);

        let PluginConfiguration::Path(path) = &plugins.0[0] else {
            panic!("expected Path variant, got: {:?}", plugins.0[0]);
        };
        assert_eq!(path, "./plugin.grit");
    }

    #[test]
    fn deserialize_path_with_rules_from_json() {
        use biome_deserialize::json::deserialize_from_json_str;
        use biome_json_parser::JsonParserOptions;

        let json = r#"[{
            "path": "./plugin.wasm",
            "options": {},
            "rules": {
                "noConsoleLog": "off",
                "useStrictEquality": "error",
                "booleanNaming": { "level": "warn", "fix": "safe" }
            }
        }]"#;
        let (plugins, errors) =
            deserialize_from_json_str::<Plugins>(json, JsonParserOptions::default(), "").consume();

        assert!(errors.is_empty(), "Unexpected errors: {errors:?}");
        let plugins = plugins.unwrap();
        assert_eq!(plugins.0.len(), 1);

        let PluginConfiguration::PathWithOptions(opts) = &plugins.0[0] else {
            panic!("expected PathWithOptions variant, got: {:?}", plugins.0[0]);
        };

        let rules = opts.rules.as_ref().expect("rules should be present");
        assert_eq!(rules.len(), 3);

        let no_console = &rules["noConsoleLog"];
        assert!(no_console.is_disabled());
        assert_eq!(no_console.level(), PluginRulePlainConfiguration::Off);

        let strict_eq = &rules["useStrictEquality"];
        assert!(!strict_eq.is_disabled());
        assert_eq!(strict_eq.level(), PluginRulePlainConfiguration::Error);

        let bool_naming = &rules["booleanNaming"];
        assert_eq!(bool_naming.level(), PluginRulePlainConfiguration::Warn);
        assert_eq!(bool_naming.fix_kind_str(), Some("safe"));
    }

    #[test]
    fn plugin_configuration_rules_accessor() {
        let path_config = PluginConfiguration::Path("./foo.grit".into());
        assert!(path_config.rules().is_none());

        let mut rules_map = FxHashMap::default();
        rules_map.insert(
            "someRule".into(),
            PluginRuleConfiguration::Plain(PluginRulePlainConfiguration::Off),
        );
        let opts_config = PluginConfiguration::PathWithOptions(PluginPathWithOptions {
            path: "./bar.wasm".into(),
            options: "{}".into(),
            rules: Some(rules_map),
        });
        let rules = opts_config.rules().unwrap();
        assert!(rules["someRule"].is_disabled());
    }
}

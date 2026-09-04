use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_fs::normalize_path;
use biome_glob::NormalizedGlob;
use biome_resolver::{
    FsWithResolverProxy, ResolveError, is_relative_specifier, package_specifier_parts,
    resolve_package_root,
};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
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

    /// Rewrites relative filesystem plugin paths in-place by joining them to
    /// `base_dir` and lexically collapsing `.` and `..` components.
    ///
    /// Package references remain unresolved unless they use object syntax with
    /// `resolutionKind: "config"`. Config-relative packages are resolved from
    /// `base_dir`.
    ///
    /// # Errors
    ///
    /// Returns an error when a config-relative package cannot be resolved.
    pub fn normalize_relative_paths(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        base_dir: &Utf8Path,
    ) -> Result<(), crate::PluginDiagnostic> {
        for plugin_config in self.0.iter_mut() {
            match plugin_config {
                PluginConfiguration::Path(path) => {
                    normalize_plugin_reference(fs, path, base_dir, false)?;
                }
                PluginConfiguration::PathWithOptions(opts) => {
                    let resolve_package = opts.resolution_kind == Some(PluginResolvePath::Config);
                    if let Some(package_name) =
                        normalize_plugin_reference(fs, &mut opts.path, base_dir, resolve_package)?
                    {
                        opts.resolved_package_specifier = Some(package_name);
                    }
                }
            }
        }
        Ok(())
    }

    /// Resolves object-syntax plugins with `resolutionKind: "config"` in-place.
    ///
    /// Relative filesystem paths are joined to `base_dir` and have `.` and `..`
    /// components lexically collapsed. Package names are replaced with their
    /// installed package roots.
    ///
    /// # Errors
    ///
    /// Returns an error when a package cannot be resolved.
    pub fn normalize_object_relative_paths(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        base_dir: &Utf8Path,
    ) -> Result<(), crate::PluginDiagnostic> {
        for plugin_config in self.0.iter_mut() {
            let PluginConfiguration::PathWithOptions(opts) = plugin_config else {
                continue;
            };
            // Only normalize paths for plugins that explicitly opt in to config-relative resolution.
            if opts.resolution_kind != Some(PluginResolvePath::Config) {
                continue;
            }

            if let Some(package_name) =
                normalize_plugin_reference(fs, &mut opts.path, base_dir, true)?
            {
                opts.resolved_package_specifier = Some(package_name);
            }
        }
        Ok(())
    }

    /// Resolves every plugin reference from `base_dir`.
    ///
    /// Package names remain unresolved for the plugin loader. Relative filesystem
    /// paths are made absolute using `base_dir`.
    ///
    /// # Errors
    ///
    /// Returns an error when a package cannot be resolved.
    pub fn resolve_paths(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        base_dir: &Utf8Path,
    ) -> Result<(), crate::PluginDiagnostic> {
        for plugin_config in self.0.iter_mut() {
            let plugin_path = match plugin_config {
                PluginConfiguration::Path(path) => path,
                PluginConfiguration::PathWithOptions(opts) => &mut opts.path,
            };
            normalize_plugin_reference(fs, plugin_path, base_dir, false)?;
        }
        Ok(())
    }
}

/// Rewrites a plugin reference relative to `base_dir`.
///
/// Relative filesystem references are joined to `base_dir`, then `.` and `..`
/// components are lexically collapsed without resolving symbolic links. Package
/// specifiers remain unchanged unless `resolve_package` is set, in which case
/// `plugin_path` is replaced with the package's Biome manifest and the original
/// package specifier is returned.
///
/// # Errors
///
/// Returns an error when package resolution is requested and fails.
fn normalize_plugin_reference(
    fs: &dyn FsWithResolverProxy,
    plugin_path: &mut String,
    base_dir: &Utf8Path,
    resolve_package: bool,
) -> Result<Option<String>, crate::PluginDiagnostic> {
    if is_package_plugin_specifier(fs, plugin_path, base_dir) {
        if resolve_package {
            let package_specifier = plugin_path.clone();
            let (package_name, _) =
                package_specifier_parts(&package_specifier).map_err(|error| {
                    crate::PluginDiagnostic::cant_resolve_package(plugin_path, base_dir, error)
                })?;
            *plugin_path =
                crate::resolve_package_manifest(fs, base_dir, package_name)?.into_string();
            return Ok(Some(package_specifier));
        }
    } else if let Some(normalized_path) = normalize_plugin_path(plugin_path, base_dir) {
        *plugin_path = normalized_path;
    }
    Ok(None)
}

/// Returns whether `plugin_path` should be resolved as an installed package.
///
/// Absolute paths, explicit relative paths, `node_modules` paths, and paths
/// that exist below `base_dir` are filesystem references. Bare references ending
/// in a plugin file extension are package specifiers only when a package with
/// that exact name can be resolved; otherwise they retain legacy file semantics.
pub(crate) fn is_package_plugin_specifier(
    fs: &dyn FsWithResolverProxy,
    plugin_path: &str,
    base_dir: &Utf8Path,
) -> bool {
    let path = Utf8Path::new(plugin_path);
    if path.is_absolute()
        || is_relative_specifier(plugin_path)
        || path
            .components()
            .any(|component| component.as_str() == "node_modules")
    {
        return false;
    }

    let base_path = base_dir.join(path);
    if fs.path_exists(&base_path) || fs.path_is_dir(&base_path) {
        return false;
    }

    let Ok((package_name, _)) = package_specifier_parts(plugin_path) else {
        return false;
    };

    if matches!(path.extension(), Some("grit" | "js" | "mjs")) {
        return !matches!(
            resolve_package_root(package_name, base_dir, fs),
            Err(ResolveError::NotFound)
        );
    }

    true
}

fn normalize_plugin_path(plugin_path: &str, base_dir: &Utf8Path) -> Option<String> {
    let path_buf = Utf8Path::new(plugin_path);
    if path_buf.is_absolute() {
        return None;
    }
    Some(normalize_path(&base_dir.join(path_buf)).to_string())
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
/// Can be either a path or package name string, or an object with options:
///
/// ```json
/// {
///   "plugins": [
///     "simple-plugin.grit",
///     "@scope/biome-plugin",
///     { "path": "scoped-plugin.grit", "includes": ["src/**/*.ts"] },
///     { "path": "./local-plugin.grit", "includes": ["src/**/*.ts"], "resolutionKind": "config" }
///   ]
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PluginConfiguration {
    /// A path or installed package name.
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

    pub fn resolved_package_specifier(&self) -> Option<&str> {
        match self {
            Self::Path(_) => None,
            Self::PathWithOptions(opts) => opts.resolved_package_specifier.as_deref(),
        }
    }

    fn resolution_kind(&self) -> PluginResolvePath {
        match self {
            Self::Path(_) => PluginResolvePath::Project,
            Self::PathWithOptions(opts) => opts.resolution_kind.unwrap_or_default(),
        }
    }
}

impl PartialEq for PluginConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
            && self.includes() == other.includes()
            && self.resolution_kind() == other.resolution_kind()
            && self.resolved_package_specifier() == other.resolved_package_specifier()
    }
}

impl Eq for PluginConfiguration {}

impl Hash for PluginConfiguration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path().hash(state);
        self.includes().hash(state);
        self.resolution_kind().hash(state);
        self.resolved_package_specifier().hash(state);
    }
}

impl Deserializable for PluginConfiguration {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
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

/// Plugin reference with additional options.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginWithOptions {
    /// The path or installed package name.
    #[deserializable(required)]
    pub path: String,

    /// A list of glob patterns. The plugin will only run on files matching
    /// these patterns. Use negated globs (e.g., `!**/*.test.ts`) for exclusions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<NormalizedGlob>>,

    /// Controls how the plugin is resolved.
    ///
    /// This only affects plugin resolution. It does not change how `includes`
    /// are interpreted.
    ///
    /// When omitted, relative paths and package names are resolved from the
    /// consuming project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_kind: Option<PluginResolvePath>,

    /// Package specifier captured when config-relative resolution replaces `path`
    /// with an absolute Biome manifest path.
    ///
    /// Configuration loading may resolve the package before `UpdateSettingsParams`
    /// crosses the workspace transport. Retaining the original specifier lets the
    /// server namespace package rules after `path` has become absolute.
    #[deserializable(skip)]
    #[serde(
        rename = "__resolvedPackageSpecifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(skip))]
    #[doc(hidden)]
    pub resolved_package_specifier: Option<String>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Hash, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PluginResolvePath {
    /// Resolve relative paths and package names from the consuming project.
    #[default]
    Project,
    /// Resolve relative paths and package names from the configuration file
    /// that declared the plugin.
    Config,
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_fs::MemoryFileSystem;
    use biome_json_parser::JsonParserOptions;

    #[test]
    fn normalize_relative_paths_makes_paths_base_dir_relative_and_normalized() {
        let fs = MemoryFileSystem::default();
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![
            PluginConfiguration::Path("./biome/../biome/my-plugin.grit".into()),
            PluginConfiguration::Path("other.grit".into()),
        ]);

        plugins.normalize_relative_paths(&fs, base_dir).unwrap();

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
        let fs = MemoryFileSystem::default();
        let base_dir = Utf8Path::new("base");
        let sep = std::path::MAIN_SEPARATOR;
        let absolute = format!("{sep}abs{sep}my-plugin.grit");
        let mut plugins = Plugins(vec![PluginConfiguration::Path(absolute.clone())]);

        plugins.normalize_relative_paths(&fs, base_dir).unwrap();

        assert_eq!(plugins.0[0].path(), &absolute);
    }

    #[test]
    fn normalize_relative_paths_with_options() {
        let fs = MemoryFileSystem::default();
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![PluginConfiguration::PathWithOptions(
            PluginWithOptions {
                path: "./my-plugin.grit".into(),
                includes: Some(vec!["src/**/*.ts".parse().unwrap()]),
                resolution_kind: None,
                resolved_package_specifier: None,
            },
        )]);

        plugins.normalize_relative_paths(&fs, base_dir).unwrap();

        let path = plugins.0[0].path();
        assert!(Utf8Path::new(path).starts_with(base_dir));
        assert!(Utf8Path::new(path).ends_with("my-plugin.grit"));

        // includes should be unchanged
        assert!(plugins.0[0].includes().is_some());
    }

    #[test]
    fn normalize_object_relative_paths_leaves_string_entries_unchanged() {
        let fs = MemoryFileSystem::default();
        let base_dir = Utf8Path::new("base");
        let mut plugins = Plugins(vec![
            PluginConfiguration::Path("./biome/../biome/my-plugin.grit".into()),
            PluginConfiguration::PathWithOptions(PluginWithOptions {
                path: "./default-plugin.grit".into(),
                includes: None,
                resolution_kind: None,
                resolved_package_specifier: None,
            }),
            PluginConfiguration::PathWithOptions(PluginWithOptions {
                path: "./other/../other/object-plugin.grit".into(),
                includes: None,
                resolution_kind: Some(PluginResolvePath::Config),
                resolved_package_specifier: None,
            }),
            PluginConfiguration::PathWithOptions(PluginWithOptions {
                path: "./project-plugin.grit".into(),
                includes: None,
                resolution_kind: Some(PluginResolvePath::Project),
                resolved_package_specifier: None,
            }),
        ]);

        plugins
            .normalize_object_relative_paths(&fs, base_dir)
            .unwrap();

        assert_eq!(plugins.0[0].path(), "./biome/../biome/my-plugin.grit");

        assert_eq!(plugins.0[1].path(), "./default-plugin.grit");

        let config_path = plugins.0[2].path();
        assert!(Utf8Path::new(config_path).starts_with(base_dir));
        let expected_suffix = Utf8Path::new("other").join("object-plugin.grit");
        assert!(Utf8Path::new(config_path).ends_with(expected_suffix.as_path()));

        assert_eq!(plugins.0[3].path(), "./project-plugin.grit");
    }

    #[test]
    fn normalize_relative_paths_leaves_package_specifiers_unresolved() {
        let fs = MemoryFileSystem::default();
        let base_dir = Utf8Path::new("/project");
        let mut plugins = Plugins(vec![PluginConfiguration::Path("@scope/plugin".into())]);

        plugins.normalize_relative_paths(&fs, base_dir).unwrap();

        assert_eq!(plugins.0[0].path(), "@scope/plugin");
    }

    #[test]
    fn normalize_relative_paths_ignores_paths_outside_the_base_directory() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "rules/1.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{ "name": "plugin" }"#,
        );
        let base_dir = Utf8Path::new("/project");
        let mut plugins = Plugins(vec![PluginConfiguration::Path("plugin".into())]);

        plugins.normalize_relative_paths(&fs, base_dir).unwrap();

        assert_eq!(plugins.0[0].path(), "plugin");
    }

    #[test]
    fn normalize_relative_paths_normalizes_bare_local_directories() {
        let fs = MemoryFileSystem::default();
        let base_dir = normalize_path(Utf8Path::new("/project"));
        let plugin_path = base_dir.join("my-plugin");
        fs.insert(
            plugin_path.join("biome-manifest.json"),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "rules/1.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        let mut plugins = Plugins(vec![PluginConfiguration::Path("my-plugin".into())]);

        plugins.normalize_relative_paths(&fs, &base_dir).unwrap();

        assert_eq!(plugins.0[0].path(), plugin_path.as_str());
    }

    #[test]
    fn normalize_config_relative_package_specifiers() {
        let fs = MemoryFileSystem::default();
        let base_dir = normalize_path(Utf8Path::new("/config"));
        let package_root = normalize_path(&base_dir.join("node_modules/@scope/plugin"));
        fs.insert(
            package_root.join("package.json"),
            r#"{ "name": "@scope/plugin" }"#,
        );
        let manifest_path = package_root.join("biome-manifest.json");
        fs.insert(
            manifest_path.clone(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "rules/1.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        let mut plugins = Plugins(vec![PluginConfiguration::PathWithOptions(
            PluginWithOptions {
                path: "@scope/plugin/one".into(),
                includes: None,
                resolution_kind: Some(PluginResolvePath::Config),
                resolved_package_specifier: None,
            },
        )]);

        plugins
            .normalize_object_relative_paths(&fs, &base_dir)
            .unwrap();

        assert_eq!(plugins.0[0].path(), manifest_path.as_str());
        assert_eq!(
            plugins.0[0].resolved_package_specifier(),
            Some("@scope/plugin/one")
        );

        let plugins: Plugins = serde_json::from_str(&serde_json::to_string(&plugins).unwrap())
            .expect("plugin configuration should survive workspace transport");
        assert_eq!(
            plugins.0[0].resolved_package_specifier(),
            Some("@scope/plugin/one")
        );
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
    fn equivalent_plugin_syntaxes_compare_equal() {
        let string: PluginConfiguration = serde_json::from_str(r#""my-plugin.grit""#).unwrap();
        let object: PluginConfiguration =
            serde_json::from_str(r#"{ "path": "my-plugin.grit" }"#).unwrap();
        let explicit_project: PluginConfiguration =
            serde_json::from_str(r#"{ "path": "my-plugin.grit", "resolutionKind": "project" }"#)
                .unwrap();

        assert_eq!(string, object);
        assert_eq!(string, explicit_project);
    }

    #[test]
    fn deserialize_object_with_config_resolve_path() {
        let config: PluginConfiguration =
            serde_json::from_str(r#"{ "path": "my-plugin.grit", "resolutionKind": "config" }"#)
                .unwrap();
        let PluginConfiguration::PathWithOptions(options) = config else {
            panic!("expected object syntax plugin");
        };
        assert_eq!(options.resolution_kind, Some(PluginResolvePath::Config));

        let config = deserialize_from_json_str::<PluginConfiguration>(
            r#"{ "path": "my-plugin.grit", "resolutionKind": "config" }"#,
            JsonParserOptions::default(),
            "",
        );
        let (config, diagnostics) = config.consume();
        assert!(diagnostics.is_empty());

        let Some(PluginConfiguration::PathWithOptions(options)) = config else {
            panic!("expected object syntax plugin");
        };
        assert_eq!(options.resolution_kind, Some(PluginResolvePath::Config));
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

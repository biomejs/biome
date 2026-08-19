#![deny(clippy::use_self)]

mod analyzer_grit_plugin;
mod diagnostics;
mod plugin_cache;
mod plugin_manifest;

#[cfg(feature = "js_plugin")]
mod analyzer_js_plugin;
#[cfg(feature = "js_plugin")]
mod thread_local;

mod configuration;

#[cfg(feature = "js_plugin")]
pub use analyzer_js_plugin::AnalyzerJsPlugin;

pub use analyzer_grit_plugin::AnalyzerGritPlugin;
pub use configuration::*;
pub use diagnostics::PluginDiagnostic;
pub use plugin_cache::*;
pub use plugin_manifest::PluginManifest;

use std::sync::Arc;

use biome_analyze::{AnalyzerPlugin, AnalyzerPluginVec};
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{ManifestName, normalize_path};
use biome_glob::{CandidatePath, NormalizedGlob};
use biome_json_parser::JsonParserOptions;
use biome_resolver::{FsWithResolverProxy, resolve_package_root};
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;

#[derive(Debug)]
pub struct BiomePlugin {
    pub analyzer_plugins: AnalyzerPluginVec,
}

impl BiomePlugin {
    /// Loads a plugin from the given `plugin_path`.
    ///
    /// The base path is used to resolve relative paths.
    /// The optional `includes` patterns restrict which files the plugin runs on.
    /// Note: `Some(&[])` (empty includes) means the plugin never matches any file.
    pub fn load(
        fs: Arc<dyn FsWithResolverProxy>,
        plugin_path: &str,
        base_path: &Utf8Path,
        includes: Option<&[NormalizedGlob]>,
    ) -> Result<(Self, Utf8PathBuf), PluginDiagnostic> {
        Self::load_with_package_name(fs, plugin_path, base_path, includes, None)
    }

    /// Loads a plugin and qualifies manifest rule names with `package_name`.
    pub fn load_with_package_name(
        fs: Arc<dyn FsWithResolverProxy>,
        plugin_path: &str,
        base_path: &Utf8Path,
        includes: Option<&[NormalizedGlob]>,
        package_name: Option<&str>,
    ) -> Result<(Self, Utf8PathBuf), PluginDiagnostic> {
        let package_name = package_name.map(str::to_owned).or_else(|| {
            configuration::is_package_plugin_specifier(fs.as_ref(), plugin_path, base_path)
                .then(|| plugin_path.to_string())
        });
        let plugin_path = resolve_plugin_path(fs.as_ref(), base_path, plugin_path)?;

        // If the plugin path references a `.grit` file directly, treat it as
        // a single-rule plugin instead of going through the manifest process:
        if plugin_path.extension() == Some("grit") && !fs.path_is_dir(&plugin_path) {
            let plugin = AnalyzerGritPlugin::load(fs.as_ref(), &plugin_path, includes)?;
            return Ok((
                Self {
                    analyzer_plugins: vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)],
                },
                plugin_path,
            ));
        }

        // TODO: plugin can have multiple analyser rules
        #[cfg(feature = "js_plugin")]
        if matches!(plugin_path.extension(), Some("js" | "mjs")) && !fs.path_is_dir(&plugin_path) {
            let plugin = AnalyzerJsPlugin::load(fs.clone(), &plugin_path, includes)?;
            return Ok((
                Self {
                    analyzer_plugins: vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)],
                },
                plugin_path,
            ));
        }

        let json_manifest_path = plugin_path.join(ManifestName::biome_manifest_json());
        let (manifest_path, parser_options) = if fs.path_is_file(&json_manifest_path) {
            (json_manifest_path, JsonParserOptions::default())
        } else {
            let jsonc_manifest_path = plugin_path.join(ManifestName::biome_manifest_jsonc());
            if !fs.path_is_file(&jsonc_manifest_path) {
                return Err(PluginDiagnostic::cant_resolve(json_manifest_path, None));
            }
            (
                jsonc_manifest_path,
                JsonParserOptions::default()
                    .with_allow_comments()
                    .with_allow_trailing_commas(),
            )
        };

        let manifest_content = fs.read_file_from_path(&manifest_path)?;
        let (manifest, errors) =
            deserialize_from_json_str::<PluginManifest>(&manifest_content, parser_options, "")
                .consume();

        let Some(manifest) = manifest.filter(|_| errors.is_empty()) else {
            return Err(PluginDiagnostic::invalid_manifest(
                markup!("Cannot load plugin manifest "<Emphasis>{manifest_path.to_string()}</Emphasis>),
                errors.into_iter().next(),
            ));
        };

        if manifest.rules.is_empty() {
            return Err(PluginDiagnostic::invalid_manifest(
                markup!("Plugin manifest "<Emphasis>{manifest_path.to_string()}</Emphasis>" must contain at least one rule"),
                None,
            ));
        }

        let mut rule_names = FxHashSet::default();
        let plugin = Self {
            analyzer_plugins: manifest
                .rules
                .into_iter()
                .map(|rule| {
                    let rule_path = resolve_manifest_rule_path(fs.as_ref(), &plugin_path, &rule)?;
                    let rule_name = rule_path.file_stem().unwrap_or_default();
                    if !rule_names.insert(rule_name.to_string()) {
                        return Err(PluginDiagnostic::invalid_manifest(
                            markup!(
                                "Plugin manifest contains multiple rules named "
                                <Emphasis>{rule_name}</Emphasis>
                            ),
                            None,
                        ));
                    }

                    if package_name.is_some()
                        && rule_name.chars().any(|character| {
                            character == ':' || character == '(' || character.is_whitespace()
                        })
                    {
                        return Err(PluginDiagnostic::invalid_manifest(
                            markup!(
                                "Package plugin rule name "
                                <Emphasis>{rule_name}</Emphasis>
                                " must not contain whitespace, "<Emphasis>":"</Emphasis>", or "<Emphasis>"("</Emphasis>
                            ),
                            None,
                        ));
                    }

                    let mut plugin = AnalyzerGritPlugin::load(fs.as_ref(), &rule_path, includes)?;
                    if let Some(package_name) = &package_name {
                        plugin = plugin.with_name(format!("{package_name}/{rule_name}"));
                    }
                    Ok(Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>))
                })
                .collect::<Result<_, _>>()?,
        };

        Ok((plugin, plugin_path))
    }
}

fn resolve_plugin_path(
    fs: &dyn FsWithResolverProxy,
    base_path: &Utf8Path,
    plugin_path: &str,
) -> Result<Utf8PathBuf, PluginDiagnostic> {
    let plugin_path = Utf8Path::new(plugin_path);
    if plugin_path.is_absolute()
        || (!base_path.as_str().is_empty() && plugin_path.starts_with(base_path))
    {
        Ok(normalize_path(plugin_path))
    } else if is_package_plugin_specifier(fs, plugin_path.as_str(), base_path) {
        resolve_package_root(plugin_path.as_str(), base_path, fs).map_err(|error| {
            PluginDiagnostic::cant_resolve_package(plugin_path.as_str(), base_path, error)
        })
    } else {
        Ok(normalize_path(&base_path.join(plugin_path)))
    }
}

fn resolve_manifest_rule_path(
    fs: &dyn FsWithResolverProxy,
    plugin_path: &Utf8Path,
    rule: &str,
) -> Result<Utf8PathBuf, PluginDiagnostic> {
    let invalid_segment = rule.split('/').any(|segment| {
        segment.is_empty()
            || segment == ".."
            || segment.contains('\\')
            || segment.as_bytes().get(1) == Some(&b':')
    });
    let rule_path = Utf8Path::new(rule);
    if rule_path.is_absolute() || invalid_segment {
        return Err(PluginDiagnostic::invalid_manifest(
            markup!(
                "Plugin rule path "<Emphasis>{rule}</Emphasis>" must be a normalized relative path"
            ),
            None,
        ));
    }
    if rule_path.extension() != Some("grit") {
        return Err(PluginDiagnostic::unsupported_rule_format(markup!(
            "Unsupported rule format for plugin rule "<Emphasis>{rule}</Emphasis>
        )));
    }

    let mut resolved_path = plugin_path.to_path_buf();
    for segment in rule.split('/') {
        if segment == "." {
            continue;
        }
        resolved_path.push(segment);
        if fs.path_is_symlink(&resolved_path) {
            return Err(PluginDiagnostic::invalid_manifest(
                markup!(
                    "Plugin rule path "<Emphasis>{rule}</Emphasis>" must not contain symbolic links"
                ),
                None,
            ));
        }
    }
    Ok(resolved_path)
}

/// Checks whether a file path matches the plugin's `includes` globs.
///
/// Returns `true` if `includes` is `None` (no restriction).
/// When `includes` is `Some`, delegates to `CandidatePath::matches_with_exceptions`.
pub(crate) fn file_matches_includes(includes: Option<&[NormalizedGlob]>, path: &Utf8Path) -> bool {
    let Some(includes) = includes else {
        return true;
    };
    CandidatePath::new(path).matches_with_exceptions(includes)
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_diagnostics::{Error, print_diagnostic_to_string};
    use biome_fs::MemoryFileSystem;
    #[cfg(unix)]
    use biome_fs::TemporaryFs;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;

    fn snap_diagnostic(test_name: &str, diagnostic: Error) {
        let content = print_diagnostic_to_string(&diagnostic);

        // Normalize Windows paths...
        let content = content.replace('\\', "/");

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }

    #[test]
    fn load_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    "version": 1,
    "rules": ["./rules/1.grit"]
}"#,
        );

        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn load_plugin_from_json_manifest() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "$schema": "./manifest_schema.json",
    "version": 1,
    "rules": ["rules/1.grit"]
}"#,
        );
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn load_plugin_from_package() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/@scope/plugin/package.json".into(),
            r#"{
    "name": "@scope/plugin",
    "exports": "./index.js"
}"#,
        );
        fs.insert(
            "/project/node_modules/@scope/plugin/biome-manifest.jsonc".into(),
            r#"{
    // Package manifests may use JSONC.
    "version": 1,
    "rules": ["rules/1.grit"],
}"#,
        );
        fs.insert(
            "/project/node_modules/@scope/plugin/rules/1.grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, path) =
            BiomePlugin::load(fs, "@scope/plugin", Utf8Path::new("/project"), None)
                .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
        assert_eq!(plugin.analyzer_plugins[0].name(), "@scope/plugin/1");
        assert_eq!(path, "/project/node_modules/@scope/plugin");
    }

    #[test]
    fn load_package_with_grit_extension() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin.grit/package.json".into(),
            r#"{ "name": "plugin.grit" }"#,
        );
        fs.insert(
            "/project/node_modules/plugin.grit/biome-manifest.json".into(),
            r#"{ "version": 1, "rules": ["rules/1.grit"] }"#,
        );
        fs.insert(
            "/project/node_modules/plugin.grit/rules/1.grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "plugin.grit", Utf8Path::new("/project"), None)
            .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn package_manifest_rejects_unsuppressible_rule_names() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{ "name": "plugin" }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/biome-manifest.json".into(),
            r#"{ "version": 1, "rules": ["rules/no(foo).grit"] }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/rules/no(foo).grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin", Utf8Path::new("/project"), None)
            .expect_err("package rule names should be valid suppression identifiers");
    }

    #[test]
    fn json_manifest_rejects_unknown_fields() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "rules": ["rules/1.grit"],
    "unknown": true
}"#,
        );
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("unknown manifest fields should be rejected");
    }

    #[test]
    fn jsonc_manifest_rejects_unknown_fields() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    // JSONC syntax remains supported.
    "version": 1,
    "rules": ["rules/1.grit"],
    "unknown": true,
}"#,
        );
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("unknown JSONC manifest fields should be rejected");
    }

    #[test]
    fn manifest_rejects_rule_path_traversal() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "rules": ["../outside.grit"]
}"#,
        );
        fs.insert("/outside.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("manifest rules should stay inside the plugin directory");
    }

    #[cfg(unix)]
    #[test]
    fn manifest_rejects_symlinked_rule_paths() {
        let mut fs = TemporaryFs::new("biome_plugin_loader_manifest_symlinks");
        let outside_rule = fs.create_file("outside/1.grit", r#"`hello`"#);
        fs.create_file(
            "directory-link/biome-manifest.json",
            r#"{ "version": 1, "rules": ["rules/1.grit"] }"#,
        );
        fs.create_file(
            "file-link/biome-manifest.json",
            r#"{ "version": 1, "rules": ["rules/1.grit"] }"#,
        );

        symlink(
            outside_rule.parent().unwrap(),
            fs.working_directory.join("directory-link/rules"),
        )
        .unwrap();
        std::fs::create_dir_all(fs.working_directory.join("file-link/rules")).unwrap();
        symlink(
            &outside_rule,
            fs.working_directory.join("file-link/rules/1.grit"),
        )
        .unwrap();

        for plugin_path in ["./directory-link", "./file-link"] {
            let os = Arc::new(fs.create_os()) as Arc<dyn FsWithResolverProxy>;
            BiomePlugin::load(os, plugin_path, &fs.working_directory, None)
                .expect_err("manifest rules should not contain symbolic links");
        }
    }

    #[test]
    fn manifest_rejects_empty_rules() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "rules": []
}"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("plugin manifests should contain a rule");
    }

    #[test]
    fn load_plugin_without_manifest() {
        let fs = MemoryFileSystem::default();
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let error = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_without_manifest", error.into());
    }

    #[test]
    fn load_plugin_with_wrong_version() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    "version": 2,
    "rules": ["rules/1.grit"]
}"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let error = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_with_wrong_version", error.into());
    }

    #[test]
    fn load_plugin_with_wrong_rule_extension() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    "version": 1,
    "rules": ["rules/1.js"]
}"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let error = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_with_wrong_rule_extension", error.into());
    }

    #[test]
    fn load_plugin_path_already_rooted_at_base_path() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "node_modules/@shared/config/grit/no-object-assign.grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, path) = BiomePlugin::load(
            fs,
            "node_modules/@shared/config/grit/no-object-assign.grit",
            Utf8Path::new("node_modules/@shared/config"),
            None,
        )
        .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
        assert_eq!(
            path,
            Utf8PathBuf::from("node_modules/@shared/config/grit/no-object-assign.grit")
        );
    }

    #[test]
    fn load_single_rule_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert("/my-plugin.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin.grit", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[cfg(feature = "js_plugin")]
    #[test]
    fn load_single_rule_js_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin.js".into(),
            r#"export default function useMyPlugin() {}"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin.js", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }
}

#![deny(clippy::use_self)]

mod analyzer_grit_plugin;
mod diagnostics;
mod plugin_cache;

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

use std::sync::Arc;

use biome_analyze::{AnalyzerPlugin, AnalyzerPluginVec};
use biome_console::markup;
use biome_fs::{FileSystemDiagnostic, FsErrorKind, ManifestName, normalize_path};
use biome_glob::{CandidatePath, NormalizedGlob};
use biome_manifest::{BiomeManifest, BiomeManifestError, ManifestEntry, ManifestPresets};
use biome_resolver::{
    FsWithResolverProxy, ResolveOptions, is_package_name, package_specifier_parts, resolve,
    resolve_package_root,
};
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;
use std::collections::BTreeMap;

const MAX_MANIFEST_IMPORT_DEPTH: usize = 10;

#[derive(Debug)]
pub struct BiomePlugin {
    pub analyzer_plugins: AnalyzerPluginVec,
}

/// A plugin reference resolved without reading or executing its rule sources.
#[derive(Debug, Eq, PartialEq)]
pub struct ResolvedPlugin {
    /// The resolved file or directory requested by the caller.
    /// Package references resolve to their Biome manifest.
    pub path: Utf8PathBuf,
    /// The package rule or preset subpath, such as `noFoo` or `presets/recommended`.
    /// Direct files and local all-rule manifest imports have no selection.
    pub selection: Option<String>,
    pub kind: ResolvedPluginKind,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ResolvedPluginKind {
    Grit,
    JavaScript,
    Manifest {
        /// The entry manifest containing the selected exports.
        path: Utf8PathBuf,
        /// Selected rules in loading order.
        rules: Vec<ResolvedPluginRule>,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct ResolvedPluginRule {
    pub path: Utf8PathBuf,
    /// The name used when loading the rule, including its package namespace.
    pub name: String,
    /// The public rule name without its package namespace.
    pub export_name: String,
    /// The public package namespace, or `None` for a locally declared rule.
    pub package: Option<String>,
    /// The manifest exporting the public name, which may re-export another package's rule.
    /// Package selections use the entry manifest; local imports use the imported package's manifest.
    pub exporting_manifest_path: Utf8PathBuf,
    /// The manifest declaring the rule's file, before any package re-exports.
    pub manifest_path: Utf8PathBuf,
}

impl From<BiomeManifestError> for PluginDiagnostic {
    fn from(value: BiomeManifestError) -> Self {
        match value {
            BiomeManifestError::FileSystem(error) => Self::FileSystem(error),
            BiomeManifestError::Invalid(error) => {
                let (path, source) = error.into_parts();
                Self::invalid_manifest(
                    markup!("Cannot load Biome manifest "<Emphasis>{path}</Emphasis>"."),
                    source,
                )
            }
        }
    }
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
        Self::load_with_package_specifier(fs, plugin_path, base_path, includes, None)
    }

    /// Loads a plugin using the original package specifier after config-relative resolution.
    pub fn load_with_package_specifier(
        fs: Arc<dyn FsWithResolverProxy>,
        plugin_path: &str,
        base_path: &Utf8Path,
        includes: Option<&[NormalizedGlob]>,
        package_specifier: Option<&str>,
    ) -> Result<(Self, Utf8PathBuf), PluginDiagnostic> {
        let resolved = resolve_plugin(fs.as_ref(), plugin_path, base_path, package_specifier)?;
        let analyzer_plugins = match resolved.kind {
            ResolvedPluginKind::Grit => {
                let plugin = AnalyzerGritPlugin::load(fs.as_ref(), &resolved.path, includes)?;
                vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)]
            }
            #[cfg(feature = "js_plugin")]
            ResolvedPluginKind::JavaScript => {
                let plugin = AnalyzerJsPlugin::load(fs, &resolved.path, includes)?;
                vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)]
            }
            #[cfg(not(feature = "js_plugin"))]
            ResolvedPluginKind::JavaScript => {
                return Err(PluginDiagnostic::unsupported_rule_format(markup!(
                    "Unsupported rule format for plugin rule "<Emphasis>{resolved.path.as_str()}</Emphasis>"."
                )));
            }
            ResolvedPluginKind::Manifest { rules, .. } => rules
                .into_iter()
                .map(|rule| {
                    let plugin = AnalyzerGritPlugin::load(fs.as_ref(), &rule.path, includes)?
                        .with_name(rule.name);
                    Ok(Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>))
                })
                .collect::<Result<_, PluginDiagnostic>>()?,
        };

        Ok((Self { analyzer_plugins }, resolved.path))
    }
}

/// Resolves a plugin and its selected rules without reading, compiling, or executing rule sources.
///
/// Relative paths resolve from `base_path`. `package_specifier` retains the original
/// package selection when `plugin_path` has already been resolved relative to a configuration.
/// Manifests are read and validated, but only selected rule files must exist.
/// JavaScript and TypeScript files resolve even when the `js_plugin` feature is disabled.
/// Successful resolution does not guarantee that the plugin can be loaded.
pub fn resolve_plugin(
    fs: &dyn FsWithResolverProxy,
    plugin_path: &str,
    base_path: &Utf8Path,
    package_specifier: Option<&str>,
) -> Result<ResolvedPlugin, PluginDiagnostic> {
    let package_specifier = package_specifier.or_else(|| {
        configuration::is_package_plugin_specifier(fs, plugin_path, base_path)
            .then_some(plugin_path)
    });
    let plugin_path = resolve_plugin_path(fs, base_path, plugin_path)?;

    if plugin_path.extension() == Some("grit") && !fs.path_is_dir(&plugin_path) {
        validate_plugin_file(fs, &plugin_path)?;
        return Ok(ResolvedPlugin {
            path: plugin_path,
            selection: None,
            kind: ResolvedPluginKind::Grit,
        });
    }

    if plugin_path
        .extension()
        .is_some_and(|extension| matches!(extension, "js" | "mjs" | "ts" | "mts"))
        && !fs.path_is_dir(&plugin_path)
    {
        validate_plugin_file(fs, &plugin_path)?;
        return Ok(ResolvedPlugin {
            path: plugin_path,
            selection: None,
            kind: ResolvedPluginKind::JavaScript,
        });
    }

    let (selection, package_name) = if let Some(specifier) = package_specifier {
        let (package_name, selection) = package_specifier_parts(specifier)
            .map_err(|error| PluginDiagnostic::cant_resolve_package(specifier, base_path, error))?;
        if selection.is_empty() {
            return Err(PluginDiagnostic::invalid_manifest(
                markup!(
                    "Plugin package "<Emphasis>{package_name}</Emphasis>
                    " must select a named rule or preset."
                ),
                None,
            ));
        }
        validate_rule_selection(selection)?;
        (Some(selection), Some(package_name))
    } else {
        (None, None)
    };
    let manifest_path = if ManifestName::is_manifest_file(&plugin_path) {
        plugin_path.clone()
    } else {
        find_biome_manifest(fs, &plugin_path)?
    };
    let manifest = BiomeManifest::load(fs, &manifest_path)?;
    let plugins = manifest.plugins.unwrap_or_default();
    let presets = plugins.presets.unwrap_or_default();
    let rules = collect_manifest_rules(
        fs,
        &manifest_path,
        plugins.rules,
        package_name,
        None,
        &mut FxHashSet::default(),
        0,
    )?;

    validate_manifest_presets(&presets, &rules, None)?;

    let selected_rules: Vec<&str> = if let Some(preset_name) =
        selection.and_then(|selection| selection.strip_prefix("presets/"))
    {
        let preset = presets.get(preset_name).ok_or_else(|| {
            PluginDiagnostic::invalid_manifest(
                markup!("Biome manifest does not export preset "<Emphasis>{preset_name}</Emphasis>"."),
                None,
            )
        })?;
        preset.iter().map(String::as_str).collect()
    } else if let Some(rule_name) = selection {
        let rule_key = exported_rule_key(&rules, rule_name).ok_or_else(|| {
            PluginDiagnostic::invalid_manifest(
                markup!("Biome manifest does not export rule "<Emphasis>{rule_name}</Emphasis>"."),
                None,
            )
        })?;
        vec![rule_key]
    } else {
        rules.keys().map(String::as_str).collect()
    };

    let rules = selected_rules
        .into_iter()
        .map(|rule_name| {
            debug_assert!(rules.contains_key(rule_name));
            let rule = &rules[rule_name];
            validate_plugin_file(fs, &rule.path)?;
            let (name, package, exporting_manifest_path) = if let Some(package_name) = package_name
            {
                (
                    format!("{package_name}/{}", rule.export_name),
                    Some(package_name.to_owned()),
                    manifest_path.clone(),
                )
            } else {
                (
                    rule.qualified_name
                        .as_ref()
                        .unwrap_or(&rule.export_name)
                        .clone(),
                    rule.package.clone(),
                    rule.exporting_manifest_path.clone(),
                )
            };
            Ok(ResolvedPluginRule {
                path: rule.path.clone(),
                name,
                export_name: rule.export_name.clone(),
                package,
                exporting_manifest_path,
                manifest_path: rule.manifest_path.clone(),
            })
        })
        .collect::<Result<_, PluginDiagnostic>>()?;

    Ok(ResolvedPlugin {
        path: plugin_path,
        selection: selection.map(str::to_owned),
        kind: ResolvedPluginKind::Manifest {
            path: manifest_path,
            rules,
        },
    })
}

fn validate_plugin_file(
    fs: &dyn FsWithResolverProxy,
    path: &Utf8Path,
) -> Result<(), PluginDiagnostic> {
    fs.path_kind(path)
        .and_then(|kind| {
            if kind.is_file() {
                Ok(())
            } else {
                Err(FileSystemDiagnostic {
                    path: path.to_string(),
                    severity: biome_diagnostics::Severity::Error,
                    error_kind: FsErrorKind::CantReadFile,
                    source: None,
                })
            }
        })
        .map_err(|source| PluginDiagnostic::cant_read_plugin_file(path.to_path_buf(), source))
}

struct ManifestRule {
    path: Utf8PathBuf,
    manifest_path: Utf8PathBuf,
    export_name: String,
    qualified_name: Option<String>,
    package: Option<String>,
    exporting_manifest_path: Utf8PathBuf,
}

fn collect_manifest_rules(
    fs: &dyn FsWithResolverProxy,
    manifest_path: &Utf8Path,
    entries: Vec<ManifestEntry>,
    package_name: Option<&str>,
    key_prefix: Option<&str>,
    active_manifests: &mut FxHashSet<Utf8PathBuf>,
    depth: usize,
) -> Result<BTreeMap<String, ManifestRule>, PluginDiagnostic> {
    if depth > MAX_MANIFEST_IMPORT_DEPTH {
        return Err(PluginDiagnostic::invalid_manifest(
            markup!("Biome manifest imports must not exceed "{MAX_MANIFEST_IMPORT_DEPTH}" levels."),
            None,
        ));
    }
    if !active_manifests.insert(manifest_path.to_path_buf()) {
        return Err(PluginDiagnostic::invalid_manifest(
            markup!("Biome manifest cycle detected at "<Emphasis>{manifest_path.to_string()}</Emphasis>"."),
            None,
        ));
    }

    let result = (|| {
        let manifest_dir = manifest_path.parent().unwrap_or(Utf8Path::new(""));
        let mut rules = BTreeMap::new();
        for entry in entries {
            match entry {
                ManifestEntry::Package(specifier) => {
                    let (package, selection) =
                        package_specifier_parts(&specifier).map_err(|_| {
                            PluginDiagnostic::invalid_manifest(
                                markup!(
                                    "Manifest rule import "<Emphasis>{specifier}</Emphasis>
                                    " must select a named rule or preset from a package."
                                ),
                                None,
                            )
                        })?;
                    validate_rule_selection(selection)?;
                    let imported_manifest = resolve_package_manifest(fs, manifest_dir, package)?;
                    let imported = BiomeManifest::load(fs, &imported_manifest)?;
                    let imported_plugins = imported.plugins.unwrap_or_default();
                    let imported_presets = imported_plugins.presets.unwrap_or_default();
                    let mut imported_rules = collect_manifest_rules(
                        fs,
                        &imported_manifest,
                        imported_plugins.rules,
                        Some(package),
                        Some(package),
                        active_manifests,
                        depth + 1,
                    )?;
                    validate_manifest_presets(&imported_presets, &imported_rules, Some(package))?;

                    let selected_rules =
                        if let Some(preset_name) = selection.strip_prefix("presets/") {
                            imported_presets.get(preset_name).ok_or_else(|| {
                            PluginDiagnostic::invalid_manifest(
                                markup!(
                                    "Biome manifest for "<Emphasis>{package}</Emphasis>
                                    " does not export preset "<Emphasis>{preset_name}</Emphasis>"."
                                ),
                                None,
                            )
                        })?.iter().map(String::as_str).collect::<Vec<_>>()
                        } else {
                            vec![selection]
                        };
                    for selected_rule in selected_rules {
                        let key = manifest_rule_key(&imported_rules, selected_rule, Some(package));
                        let mut rule = imported_rules.remove(&key).ok_or_else(|| {
                            PluginDiagnostic::invalid_manifest(
                                markup!(
                                    "Biome manifest for "<Emphasis>{package}</Emphasis>
                                    " does not export rule "<Emphasis>{selected_rule}</Emphasis>"."
                                ),
                                None,
                            )
                        })?;
                        let key = format!("{package}/{}", rule.export_name);
                        rule.qualified_name = Some(key.clone());
                        rule.package = Some(package.to_owned());
                        rule.exporting_manifest_path = imported_manifest.clone();
                        insert_named_rule(&mut rules, key, rule)?;
                    }
                }
                ManifestEntry::Paths(paths) => {
                    for (name, path) in paths {
                        validate_rule_name(&name)?;
                        let path = resolve_manifest_rule_path(fs, manifest_dir, &path)?;
                        let key = key_prefix
                            .map_or_else(|| name.clone(), |prefix| format!("{prefix}/{name}"));
                        let qualified_name =
                            package_name.map(|package| format!("{package}/{name}"));
                        insert_named_rule(
                            &mut rules,
                            key,
                            ManifestRule {
                                path,
                                manifest_path: manifest_path.to_path_buf(),
                                export_name: name,
                                qualified_name,
                                package: package_name.map(str::to_owned),
                                exporting_manifest_path: manifest_path.to_path_buf(),
                            },
                        )?;
                    }
                }
            }
        }
        Ok(rules)
    })();

    active_manifests.remove(manifest_path);
    result
}

fn validate_manifest_presets(
    presets: &ManifestPresets,
    rules: &BTreeMap<String, ManifestRule>,
    package: Option<&str>,
) -> Result<(), PluginDiagnostic> {
    for (preset_name, preset_rules) in presets.iter() {
        let mut names = FxHashSet::default();
        for rule_name in preset_rules {
            let key = manifest_rule_key(rules, rule_name, package);
            if !rules.contains_key(&key) {
                return Err(PluginDiagnostic::invalid_manifest(
                    markup!(
                        "Preset "<Emphasis>{preset_name}</Emphasis>
                        " references unknown rule "<Emphasis>{rule_name}</Emphasis>"."
                    ),
                    None,
                ));
            }
            if !names.insert(rule_name) {
                return Err(PluginDiagnostic::invalid_manifest(
                    markup!(
                        "Preset "<Emphasis>{preset_name}</Emphasis>
                        " contains duplicate rule "<Emphasis>{rule_name}</Emphasis>"."
                    ),
                    None,
                ));
            }
        }
    }
    Ok(())
}

fn manifest_rule_key(
    rules: &BTreeMap<String, ManifestRule>,
    rule_name: &str,
    package: Option<&str>,
) -> String {
    if rules.contains_key(rule_name) {
        rule_name.to_string()
    } else {
        package.map_or_else(
            || rule_name.to_string(),
            |package| format!("{package}/{rule_name}"),
        )
    }
}

fn exported_rule_key<'a>(
    rules: &'a BTreeMap<String, ManifestRule>,
    export_name: &str,
) -> Option<&'a str> {
    rules
        .iter()
        .find_map(|(key, rule)| (rule.export_name == export_name).then_some(key.as_str()))
}

fn insert_named_rule(
    rules: &mut BTreeMap<String, ManifestRule>,
    name: String,
    rule: ManifestRule,
) -> Result<(), PluginDiagnostic> {
    if rules.contains_key(&name)
        || rules
            .values()
            .any(|existing| existing.export_name == rule.export_name)
    {
        return Err(PluginDiagnostic::invalid_manifest(
            markup!("Biome manifest exports multiple rules named "<Emphasis>{rule.export_name}</Emphasis>"."),
            None,
        ));
    }
    rules.insert(name, rule);
    Ok(())
}

fn validate_rule_name(name: &str) -> Result<(), PluginDiagnostic> {
    if name.is_empty()
        || name.contains('/')
        || name
            .chars()
            .any(|character| character == ':' || character == '(' || character.is_whitespace())
    {
        return Err(PluginDiagnostic::invalid_manifest(
            markup!(
                "Package plugin rule name "<Emphasis>{name}</Emphasis>
                " must not contain slashes, whitespace, "<Emphasis>":"</Emphasis>", or "<Emphasis>"("</Emphasis>"."
            ),
            None,
        ));
    }
    Ok(())
}

fn validate_rule_selection(selection: &str) -> Result<(), PluginDiagnostic> {
    let name = selection.strip_prefix("presets/").unwrap_or(selection);
    if selection.is_empty() || name.is_empty() || name.contains('/') {
        return Err(PluginDiagnostic::invalid_manifest(
            markup!(
                "Plugin package selection "<Emphasis>{selection}</Emphasis>
                " must identify one rule or one preset."
            ),
            None,
        ));
    }
    Ok(())
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
        let (package_name, _) = package_specifier_parts(plugin_path.as_str()).map_err(|error| {
            PluginDiagnostic::cant_resolve_package(plugin_path.as_str(), base_path, error)
        })?;
        resolve_package_manifest(fs, base_path, package_name)
    } else {
        Ok(normalize_path(&base_path.join(plugin_path)))
    }
}

fn find_biome_manifest(
    fs: &dyn FsWithResolverProxy,
    plugin_path: &Utf8Path,
) -> Result<Utf8PathBuf, PluginDiagnostic> {
    if ManifestName::is_manifest_file(plugin_path) && fs.path_is_file(plugin_path) {
        return Ok(plugin_path.to_path_buf());
    }

    for file_name in ManifestName::file_names() {
        let manifest_path = plugin_path.join(file_name);
        if fs.path_is_file(&manifest_path) {
            return Ok(manifest_path);
        }
    }
    Err(PluginDiagnostic::cant_resolve(
        plugin_path.to_path_buf(),
        None,
    ))
}

fn resolve_package_manifest(
    fs: &dyn FsWithResolverProxy,
    base_path: &Utf8Path,
    specifier: &str,
) -> Result<Utf8PathBuf, PluginDiagnostic> {
    const RESOLVE_OPTIONS: ResolveOptions = ResolveOptions::new()
        .with_assume_relative()
        .with_condition_names(&["biome", "default"]);

    let resolved = resolve(specifier, base_path, fs, &RESOLVE_OPTIONS);
    if let Ok(path) = &resolved
        && ManifestName::is_manifest_file(path)
    {
        let package_root = resolve_package_root(specifier, base_path, fs)
            .map_err(|error| PluginDiagnostic::cant_resolve_package(specifier, base_path, error))?;
        validate_package_manifest_path(fs, &package_root, path)?;
        return Ok(path.clone());
    }

    if is_package_name(specifier) {
        let package_root = resolve_package_root(specifier, base_path, fs)
            .map_err(|error| PluginDiagnostic::cant_resolve_package(specifier, base_path, error))?;
        let manifest_path = find_biome_manifest(fs, &package_root)?;
        validate_package_manifest_path(fs, &package_root, &manifest_path)?;
        return Ok(manifest_path);
    }

    match resolved {
        Ok(path) => Err(PluginDiagnostic::invalid_manifest(
            markup!(
                "Package "<Emphasis>{specifier}</Emphasis>" must resolve to "
                <Emphasis>{ManifestName::biome_manifest_json()}</Emphasis>
                " or "<Emphasis>{ManifestName::biome_manifest_jsonc()}</Emphasis>
                ", but resolved to "<Emphasis>{path.to_string()}</Emphasis>"."
            ),
            None,
        )),
        Err(error) => Err(PluginDiagnostic::cant_resolve_package(
            specifier, base_path, error,
        )),
    }
}

fn validate_package_manifest_path(
    fs: &dyn FsWithResolverProxy,
    package_root: &Utf8Path,
    manifest_path: &Utf8Path,
) -> Result<(), PluginDiagnostic> {
    let relative_path = manifest_path.strip_prefix(package_root).map_err(|_| {
        PluginDiagnostic::invalid_manifest(
            markup!(
                "Biome manifest path "<Emphasis>{manifest_path.to_string()}</Emphasis>
                " must stay within the package directory."
            ),
            None,
        )
    })?;
    let mut candidate = package_root.to_path_buf();
    for component in relative_path.components() {
        candidate.push(component.as_str());
        if fs.path_is_symlink(&candidate) {
            return Err(PluginDiagnostic::invalid_manifest(
                markup!(
                    "Biome manifest path "<Emphasis>{manifest_path.to_string()}</Emphasis>
                    " must not contain symbolic links."
                ),
                None,
            ));
        }
    }
    Ok(())
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
                "Plugin rule path "<Emphasis>{rule}</Emphasis>" must be a normalized relative path."
            ),
            None,
        ));
    }
    let rule = if rule_path.extension().is_none() {
        format!("{rule}.grit")
    } else {
        rule.to_string()
    };
    let rule_path = Utf8Path::new(&rule);
    if rule_path.extension() != Some("grit") {
        return Err(PluginDiagnostic::unsupported_rule_format(markup!(
            "Unsupported rule format for plugin rule "<Emphasis>{rule}</Emphasis>"."
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
                    "Plugin rule path "<Emphasis>{rule}</Emphasis>" must not contain symbolic links."
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
    "plugins": {
        "rules": [{ "one": "./rules/1.grit" }]
    }
}"#,
        );

        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn resolve_grit_plugin_without_reading_or_compiling_source() {
        let fs = MemoryFileSystem::default();
        fs.insert("/my-plugin.grit".into(), "`unterminated");

        let resolved = resolve_plugin(&fs, "./my-plugin.grit", Utf8Path::new("/"), None)
            .expect("resolution should not compile the plugin");
        assert_eq!(resolved.path, "/my-plugin.grit");
        assert_eq!(resolved.selection, None);
        assert_eq!(resolved.kind, ResolvedPluginKind::Grit);
        assert!(matches!(
            AnalyzerGritPlugin::load(&fs, &resolved.path, None),
            Err(PluginDiagnostic::Compile(_))
        ));

        fs.insert("/my-plugin.grit".into(), vec![0xff]);
        assert_eq!(
            resolve_plugin(&fs, "./my-plugin.grit", Utf8Path::new("/"), None).unwrap(),
            resolved,
            "resolution should not read plugin source as UTF-8"
        );
    }

    #[test]
    fn resolve_plugin_rejects_missing_direct_and_selected_files() {
        let fs = MemoryFileSystem::default();
        let error = resolve_plugin(&fs, "./missing.grit", Utf8Path::new("/"), None)
            .expect_err("direct plugin files must exist");
        assert!(matches!(error, PluginDiagnostic::InvalidManifest(_)));
        assert!(error.to_string().contains("/missing.grit"));

        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "missing.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        for selection in ["plugin/one", "plugin/presets/recommended"] {
            let error = resolve_plugin(
                &fs,
                "/my-plugin/biome-manifest.json",
                Utf8Path::new("/"),
                Some(selection),
            )
            .expect_err("selected plugin files must exist");
            assert!(matches!(error, PluginDiagnostic::InvalidManifest(_)));
            assert!(error.to_string().contains("/my-plugin/missing.grit"));
        }

        fs.insert("/my-plugin/missing.grit/child".into(), "");
        resolve_plugin(&fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("selected plugin paths must be files, not directories");
    }

    #[test]
    fn resolve_config_only_manifest_has_no_rules() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "configs": [{ "recommended": "biome.json" }]
            }"#,
        );

        let resolved = resolve_plugin(&fs, "./my-plugin", Utf8Path::new("/"), None).unwrap();
        assert_eq!(resolved.path, "/my-plugin");
        assert_eq!(resolved.selection, None);
        assert_eq!(
            resolved.kind,
            ResolvedPluginKind::Manifest {
                path: "/my-plugin/biome-manifest.json".into(),
                rules: vec![],
            }
        );
        let (plugin, _) =
            BiomePlugin::load(Arc::new(fs), "./my-plugin", Utf8Path::new("/"), None).unwrap();
        assert!(plugin.analyzer_plugins.is_empty());
    }

    #[test]
    fn local_manifest_uses_exported_rule_names() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [
                        { "second": "second/1.grit" },
                        { "first": "first/1.grit" }
                    ],
                    "presets": { "recommended": ["first", "second"] }
                }
            }"#,
        );
        fs.insert("/my-plugin/first/1.grit".into(), r#"`hello`"#);
        fs.insert("/my-plugin/second/1.grit".into(), r#"`world`"#);

        let resolved = resolve_plugin(&fs, "./my-plugin", Utf8Path::new("/"), None).unwrap();
        assert_eq!(resolved.path, "/my-plugin");
        assert_eq!(resolved.selection, None);
        assert_eq!(
            resolved.kind,
            ResolvedPluginKind::Manifest {
                path: "/my-plugin/biome-manifest.json".into(),
                rules: vec![
                    ResolvedPluginRule {
                        path: "/my-plugin/first/1.grit".into(),
                        name: "first".into(),
                        export_name: "first".into(),
                        package: None,
                        exporting_manifest_path: "/my-plugin/biome-manifest.json".into(),
                        manifest_path: "/my-plugin/biome-manifest.json".into(),
                    },
                    ResolvedPluginRule {
                        path: "/my-plugin/second/1.grit".into(),
                        name: "second".into(),
                        export_name: "second".into(),
                        package: None,
                        exporting_manifest_path: "/my-plugin/biome-manifest.json".into(),
                        manifest_path: "/my-plugin/biome-manifest.json".into(),
                    },
                ],
            }
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect("rule exports should not depend on file stems");

        assert_eq!(plugin.analyzer_plugins.len(), 2);
        assert_eq!(plugin.analyzer_plugins[0].name(), "first");
        assert_eq!(plugin.analyzer_plugins[1].name(), "second");
    }

    #[test]
    fn load_plugin_from_json_manifest() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "$schema": "./manifest_schema.json",
    "version": 1,
    "plugins": {
        "rules": [{ "one": "rules/1.grit" }],
        "presets": { "recommended": ["one"] }
    }
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
    "plugins": {
        "rules": [{ "one": "rules/1.grit" }],
        "presets": { "recommended": ["one"] }
    },
}"#,
        );
        fs.insert(
            "/project/node_modules/@scope/plugin/rules/1.grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, path) =
            BiomePlugin::load(fs, "@scope/plugin/one", Utf8Path::new("/project"), None)
                .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
        assert_eq!(plugin.analyzer_plugins[0].name(), "@scope/plugin/one");
        assert_eq!(
            path,
            "/project/node_modules/@scope/plugin/biome-manifest.jsonc"
        );
    }

    #[test]
    fn load_plugin_from_biome_export() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{
    "name": "plugin",
    "exports": {
        "biome": "./manifests/biome-manifest.json",
        "default": "./index.js"
    }
}"#,
        );
        fs.insert(
            "/project/node_modules/plugin/manifests/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "exportedRule": "exportedRule.grit" }],
                    "presets": { "recommended": ["exportedRule"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/manifests/exportedRule.grit".into(),
            r#"`exported`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) =
            BiomePlugin::load(fs, "plugin/exportedRule", Utf8Path::new("/project"), None)
                .expect("the biome export should resolve a Biome manifest");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
        assert_eq!(plugin.analyzer_plugins[0].name(), "plugin/exportedRule");
    }

    #[test]
    fn package_manifest_reexports_selected_rules_and_presets() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{ "name": "plugin" }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "plugins": {
        "rules": [
            "@org/package/presets/recommended",
            "biome-styles-plugin/stylesRule",
            "default-only-plugin/defaultRule",
            "explicit-manifest/explicitRule",
            { "localRule": "local-rule.grit" }
        ],
        "presets": {
            "all": [
                "@org/package/orgRule",
                "@org/package/orgSecondRule",
                "biome-styles-plugin/stylesRule",
                "default-only-plugin/defaultRule",
                "explicit-manifest/explicitRule",
                "localRule"
            ]
        }
    }
}"#,
        );
        fs.insert(
            "/project/node_modules/@org/package/package.json".into(),
            r#"{
    "name": "@org/package",
    "exports": {
        "biome": "./biome-manifest.json",
        "default": "./index.js"
    }
}"#,
        );
        fs.insert(
            "/project/node_modules/@org/package/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "plugins": {
        "rules": [{
            "orgRule": "orgRule.grit",
            "orgSecondRule": "orgSecondRule.grit",
            "orgUnusedRule": "missing.grit"
        }],
        "presets": { "recommended": ["orgRule", "orgSecondRule"] }
    }
}"#,
        );
        fs.insert(
            "/project/node_modules/@org/package/orgRule.grit".into(),
            r#"`organization`"#,
        );
        fs.insert(
            "/project/node_modules/@org/package/orgSecondRule.grit".into(),
            r#"`organization second`"#,
        );
        fs.insert(
            "/project/node_modules/biome-styles-plugin/package.json".into(),
            r#"{ "name": "biome-styles-plugin", "main": "./biome-manifest.json" }"#,
        );
        fs.insert(
            "/project/node_modules/biome-styles-plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{
                        "stylesRule": "stylesRule.grit",
                        "unusedRule": "missing.grit"
                    }],
                    "presets": { "recommended": ["stylesRule"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/biome-styles-plugin/stylesRule.grit".into(),
            r#"`styles`"#,
        );
        fs.insert(
            "/project/node_modules/default-only-plugin/package.json".into(),
            r#"{
    "name": "default-only-plugin",
    "exports": { "default": "./biome-manifest.json" }
}"#,
        );
        fs.insert(
            "/project/node_modules/default-only-plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "defaultRule": "defaultRule.grit" }],
                    "presets": { "recommended": ["defaultRule"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/default-only-plugin/defaultRule.grit".into(),
            r#"`default`"#,
        );
        fs.insert(
            "/project/node_modules/explicit-manifest/package.json".into(),
            r#"{
    "name": "explicit-manifest",
    "exports": { "./biome-manifest": "./biome-manifest.json" }
}"#,
        );
        fs.insert(
            "/project/node_modules/explicit-manifest/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "explicitRule": "explicitRule.grit" }],
                    "presets": { "recommended": ["explicitRule"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/explicit-manifest/explicitRule.grit".into(),
            r#"`explicit`"#,
        );
        fs.insert(
            "/project/node_modules/plugin/local-rule.grit".into(),
            r#"`local`"#,
        );
        fs.insert(
            "/project/node_modules/local-rule.grit/package.json".into(),
            r#"{
    "name": "local-rule.grit",
    "exports": { "biome": "./packageRule.grit" }
}"#,
        );
        fs.insert(
            "/project/node_modules/local-rule.grit/packageRule.grit".into(),
            r#"`package`"#,
        );

        let resolved = resolve_plugin(&fs, "plugin/presets/all", Utf8Path::new("/project"), None)
            .expect("unused missing exports must not prevent resolution");
        assert_eq!(resolved.selection.as_deref(), Some("presets/all"));
        assert_eq!(
            resolved.path,
            "/project/node_modules/plugin/biome-manifest.json"
        );
        let ResolvedPluginKind::Manifest { path, rules } = resolved.kind else {
            panic!("expected manifest");
        };
        assert_eq!(path, resolved.path);
        let expected_rules = [
            ("orgRule", "@org/package", "orgRule.grit"),
            ("orgSecondRule", "@org/package", "orgSecondRule.grit"),
            ("stylesRule", "biome-styles-plugin", "stylesRule.grit"),
            ("defaultRule", "default-only-plugin", "defaultRule.grit"),
            ("explicitRule", "explicit-manifest", "explicitRule.grit"),
            ("localRule", "plugin", "local-rule.grit"),
        ];
        assert_eq!(rules.len(), expected_rules.len());
        for (rule, (name, package, file)) in rules.iter().zip(expected_rules) {
            assert_eq!(rule.name, format!("plugin/{name}"));
            assert_eq!(rule.export_name, name);
            assert_eq!(rule.package.as_deref(), Some("plugin"));
            assert_eq!(rule.exporting_manifest_path, resolved.path);
            assert_eq!(rule.path, format!("/project/node_modules/{package}/{file}"));
            assert_eq!(
                rule.manifest_path,
                format!("/project/node_modules/{package}/biome-manifest.json")
            );
        }

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(
            fs.clone(),
            "plugin/presets/all",
            Utf8Path::new("/project"),
            None,
        )
        .expect("manifest rule packages should resolve");

        assert_eq!(plugin.analyzer_plugins.len(), 6);
        assert_eq!(plugin.analyzer_plugins[0].name(), "plugin/orgRule");
        assert_eq!(plugin.analyzer_plugins[1].name(), "plugin/orgSecondRule");
        assert_eq!(plugin.analyzer_plugins[2].name(), "plugin/stylesRule");
        assert_eq!(plugin.analyzer_plugins[3].name(), "plugin/defaultRule");
        assert_eq!(plugin.analyzer_plugins[4].name(), "plugin/explicitRule");
        assert_eq!(plugin.analyzer_plugins[5].name(), "plugin/localRule");

        let (plugin, _) = BiomePlugin::load(fs, "plugin/orgRule", Utf8Path::new("/project"), None)
            .expect("re-exported rules should be selectable from the consuming package");
        assert_eq!(plugin.analyzer_plugins[0].name(), "plugin/orgRule");
    }

    #[test]
    fn package_manifest_rejects_reexported_rule_name_collisions() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{ "name": "plugin" }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [
                        "shared-plugin/useMyLogger",
                        { "useMyLogger": "./useMyLogger.grit" }
                    ],
                    "presets": {
                        "all": [
                            "useMyLogger",
                            "shared-plugin/useMyLogger"
                        ]
                    }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/useMyLogger.grit".into(),
            r#"`local logger`"#,
        );
        fs.insert(
            "/project/node_modules/shared-plugin/package.json".into(),
            r#"{ "name": "shared-plugin" }"#,
        );
        fs.insert(
            "/project/node_modules/shared-plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "useMyLogger": "./useMyLogger.grit" }],
                    "presets": { "recommended": ["useMyLogger"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/shared-plugin/useMyLogger.grit".into(),
            r#"`shared logger`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin/presets/all", Utf8Path::new("/project"), None)
            .expect_err("re-exported rules must have unique names");
    }

    #[test]
    fn package_manifest_rebinds_transitive_rule_exports() {
        let fs = MemoryFileSystem::default();
        for package in ["consumer", "middle", "leaf"] {
            fs.insert(
                format!("/project/node_modules/{package}/package.json").into(),
                format!(r#"{{ "name": "{package}" }}"#),
            );
        }
        fs.insert(
            "/project/node_modules/consumer/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": ["middle/presets/recommended"],
                    "presets": { "recommended": ["middle/noExternalLinks"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/middle/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": ["leaf/noExternalLinks"],
                    "presets": { "recommended": ["leaf/noExternalLinks"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/leaf/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "noExternalLinks": "noExternalLinks.grit" }]
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/leaf/noExternalLinks.grit".into(),
            r#"`<a href=$url></a>`"#,
        );
        fs.insert(
            "/project/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [
                        "middle/presets/recommended",
                        { "localRule": "local.grit" }
                    ]
                }
            }"#,
        );
        fs.insert("/project/local.grit".into(), vec![0xff]);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        for selection in ["consumer/noExternalLinks", "consumer/presets/recommended"] {
            let resolved =
                resolve_plugin(fs.as_ref(), selection, Utf8Path::new("/project"), None).unwrap();
            assert_eq!(
                resolved.selection.as_deref(),
                selection.strip_prefix("consumer/")
            );
            assert_eq!(
                resolved.kind,
                ResolvedPluginKind::Manifest {
                    path: "/project/node_modules/consumer/biome-manifest.json".into(),
                    rules: vec![ResolvedPluginRule {
                        path: "/project/node_modules/leaf/noExternalLinks.grit".into(),
                        name: "consumer/noExternalLinks".into(),
                        export_name: "noExternalLinks".into(),
                        package: Some("consumer".into()),
                        exporting_manifest_path:
                            "/project/node_modules/consumer/biome-manifest.json".into(),
                        manifest_path: "/project/node_modules/leaf/biome-manifest.json".into(),
                    }],
                }
            );
            let (plugin, _) =
                BiomePlugin::load(fs.clone(), selection, Utf8Path::new("/project"), None)
                    .expect("transitively re-exported rules should resolve");
            assert_eq!(
                plugin.analyzer_plugins[0].name(),
                "consumer/noExternalLinks"
            );
        }

        let resolved = resolve_plugin(
            fs.as_ref(),
            "./biome-manifest.json",
            Utf8Path::new("/project"),
            None,
        )
        .expect(
            "local imports should preserve public package exports without reading rule sources",
        );
        assert_eq!(resolved.selection, None);
        assert_eq!(
            resolved.kind,
            ResolvedPluginKind::Manifest {
                path: "/project/biome-manifest.json".into(),
                rules: vec![
                    ResolvedPluginRule {
                        path: "/project/local.grit".into(),
                        name: "localRule".into(),
                        export_name: "localRule".into(),
                        package: None,
                        exporting_manifest_path: "/project/biome-manifest.json".into(),
                        manifest_path: "/project/biome-manifest.json".into(),
                    },
                    ResolvedPluginRule {
                        path: "/project/node_modules/leaf/noExternalLinks.grit".into(),
                        name: "middle/noExternalLinks".into(),
                        export_name: "noExternalLinks".into(),
                        package: Some("middle".into()),
                        exporting_manifest_path: "/project/node_modules/middle/biome-manifest.json"
                            .into(),
                        manifest_path: "/project/node_modules/leaf/biome-manifest.json".into(),
                    },
                ],
            }
        );

        for selection in [
            "consumer/middle/noExternalLinks",
            "consumer/presets/team/recommended",
        ] {
            BiomePlugin::load(fs.clone(), selection, Utf8Path::new("/project"), None)
                .expect_err("consumer selections must not expose transitive paths");
        }
    }

    #[test]
    fn package_manifest_rejects_cycles() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{
    "name": "plugin",
    "exports": { "biome": "./biome-manifest.json" }
}"#,
        );
        fs.insert(
            "/project/node_modules/plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": ["plugin/rule"],
                    "presets": { "recommended": ["plugin/rule"] }
                }
            }"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin/rule", Utf8Path::new("/project"), None)
            .expect_err("Biome manifest imports should not contain cycles");
    }

    #[test]
    fn package_manifest_rejects_deep_imports() {
        let fs = MemoryFileSystem::default();
        for index in 0..=11 {
            let package = format!("plugin-{index}");
            let package_dir = format!("/project/node_modules/{package}");
            fs.insert(
                Utf8PathBuf::from(format!("{package_dir}/package.json")),
                format!(r#"{{ "name": "{package}" }}"#),
            );
            let rules = if index == 11 {
                r#"[{ "rule": "./rule.grit" }]"#.to_string()
            } else {
                format!(r#"["plugin-{}/rule"]"#, index + 1)
            };
            fs.insert(
                Utf8PathBuf::from(format!("{package_dir}/biome-manifest.json")),
                format!(
                    r#"{{
                        "version": 1,
                        "plugins": {{
                            "rules": {rules},
                            "presets": {{ "recommended": ["rule"] }}
                        }}
                    }}"#
                ),
            );
        }

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin-0/rule", Utf8Path::new("/project"), None)
            .expect_err("manifest imports should have a depth limit");
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
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "rules/1.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/plugin.grit/rules/1.grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "plugin.grit/one", Utf8Path::new("/project"), None)
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
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "no(foo)": "rules/no(foo).grit" }],
                    "presets": { "recommended": ["no(foo)"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/rules/no(foo).grit".into(),
            r#"`hello`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin/no(foo)", Utf8Path::new("/project"), None)
            .expect_err("package rule names should be valid suppression identifiers");
    }

    #[test]
    fn package_manifest_rejects_duplicate_rule_names() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{ "name": "plugin" }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [
                        { "one": "first/1.grit" },
                        { "one": "second/1.grit" }
                    ],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/first/1.grit".into(),
            r#"`hello`"#,
        );
        fs.insert(
            "/project/node_modules/plugin/second/1.grit".into(),
            r#"`world`"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin/one", Utf8Path::new("/project"), None)
            .expect_err("package manifests should contain unique rule names");
    }

    #[test]
    fn package_plugin_requires_a_named_export() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/project/node_modules/plugin/package.json".into(),
            r#"{ "name": "plugin" }"#,
        );
        fs.insert(
            "/project/node_modules/plugin/biome-manifest.json".into(),
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "one.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "plugin", Utf8Path::new("/project"), None)
            .expect_err("bare plugin package references should be rejected");
    }

    #[test]
    fn json_manifest_rejects_unknown_fields() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "plugins": {
        "rules": [{ "one": "rules/1.grit" }],
        "presets": { "recommended": ["one"] }
    },
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
    "plugins": {
        "rules": [{ "one": "rules/1.grit" }],
        "presets": { "recommended": ["one"] }
    },
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
    "plugins": {
        "rules": [{ "outside": "../outside.grit" }],
        "presets": { "recommended": ["outside"] }
    }
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
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "rules/1.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
        );
        fs.create_file(
            "file-link/biome-manifest.json",
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "one": "rules/1.grit" }],
                    "presets": { "recommended": ["one"] }
                }
            }"#,
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
    fn manifest_rejects_empty_content() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.json".into(),
            r#"{
    "version": 1,
    "plugins": {
        "rules": [],
        "presets": { "recommended": ["one"] }
    }
}"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"), None)
            .expect_err("Biome manifests should contain a rule or configuration");
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
    "plugins": {
        "rules": [{ "one": "rules/1.grit" }],
        "presets": { "recommended": ["one"] }
    }
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
    "plugins": {
        "rules": [{ "one": "rules/1.js" }],
        "presets": { "recommended": ["one"] }
    }
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

    #[test]
    fn resolve_js_plugin_without_reading_or_evaluating_source() {
        let fs = MemoryFileSystem::default();
        for extension in ["js", "mjs", "ts", "mts"] {
            let path = format!("/my-plugin.{extension}");
            fs.insert(path.clone().into(), "throw new Error('must not execute');");
            let resolved = resolve_plugin(&fs, &path, Utf8Path::new("/"), None)
                .expect("resolution should not execute JavaScript");
            assert_eq!(resolved.path, path);
            assert_eq!(resolved.selection, None);
            assert_eq!(resolved.kind, ResolvedPluginKind::JavaScript);

            fs.insert(path.clone().into(), vec![0xff]);
            assert_eq!(
                resolve_plugin(&fs, &path, Utf8Path::new("/"), None).unwrap(),
                resolved,
                "resolution should not read plugin source as UTF-8"
            );

            let error = resolve_plugin(
                &fs,
                &format!("./missing.{extension}"),
                Utf8Path::new("/"),
                None,
            )
            .expect_err("direct JavaScript plugin files must exist");
            assert!(matches!(error, PluginDiagnostic::InvalidManifest(_)));
        }
    }

    #[test]
    fn load_throwing_js_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin.js".into(),
            "throw new Error('must not execute');",
        );
        let error = BiomePlugin::load(Arc::new(fs), "./my-plugin.js", Utf8Path::new("/"), None)
            .expect_err(
                "ordinary loading should reject a throwing or unsupported JavaScript plugin",
            );
        #[cfg(feature = "js_plugin")]
        assert!(matches!(error, PluginDiagnostic::Compile(_)));
        #[cfg(not(feature = "js_plugin"))]
        {
            assert!(matches!(error, PluginDiagnostic::UnsupportedRuleFormat(_)));
            snap_diagnostic("load_js_plugin_without_runtime_support", error.into());
        }
    }

    #[cfg(feature = "js_plugin")]
    #[test]
    fn load_single_rule_ts_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin.ts".into(),
            r#"import { ast, defineRule } from "@biomejs/plugin-api";
            import type { AnyJsRoot } from "@biomejs/plugin-api";
            export const useMyPlugin = defineRule({
                query: ast("JS_MODULE"),
                run(root: AnyJsRoot): void {},
            });"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin.ts", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[cfg(feature = "js_plugin")]
    #[test]
    fn load_single_rule_js_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin.js".into(),
            r#"import { ast, defineRule } from "@biomejs/plugin-api";
            export const useMyPlugin = defineRule({
                query: ast("JS_MODULE"),
                run(root) {},
            });"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin.js", Utf8Path::new("/"), None)
            .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }
}

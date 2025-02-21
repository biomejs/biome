mod analyzer_grit_plugin;
mod diagnostics;
mod plugin_cache;
mod plugin_manifest;

pub use analyzer_grit_plugin::AnalyzerGritPlugin;
pub use diagnostics::PluginDiagnostic;
pub use plugin_cache::*;

use std::sync::Arc;

use biome_analyze::{AnalyzerPlugin, AnalyzerPluginVec};
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::FileSystem;
use biome_json_parser::JsonParserOptions;
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use plugin_manifest::PluginManifest;

#[derive(Debug)]
pub struct BiomePlugin {
    pub analyzer_plugins: AnalyzerPluginVec,
}

impl BiomePlugin {
    /// Loads a plugin from the given `plugin_path`.
    ///
    /// The base path is used to resolve relative paths.
    pub fn load(
        fs: &dyn FileSystem,
        plugin_path: &str,
        base_path: &Utf8Path,
    ) -> Result<Self, PluginDiagnostic> {
        let plugin_path = normalize_path(&base_path.join(plugin_path));

        // If the plugin path references a `.grit` file directly, treat it as
        // a single-rule plugin instead of going through the manifest process:
        if plugin_path
            .extension()
            .is_some_and(|extension| extension == "grit")
        {
            let plugin = AnalyzerGritPlugin::load(fs, &plugin_path)?;
            return Ok(Self {
                analyzer_plugins: vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)],
            });
        }

        let manifest_path = plugin_path.join("biome-manifest.jsonc");
        if !fs.path_is_file(&manifest_path) {
            return Err(PluginDiagnostic::cant_resolve(
                manifest_path.to_string(),
                None,
            ));
        }

        let manifest_content = fs.read_file_from_path(&manifest_path)?;
        let (manifest, errors) = deserialize_from_json_str::<PluginManifest>(
            &manifest_content,
            JsonParserOptions::default().with_allow_comments(),
            "",
        )
        .consume();

        let Some(manifest) = manifest else {
            return Err(PluginDiagnostic::invalid_manifest(
                markup!("Cannot load plugin manifest "<Emphasis>{manifest_path.to_string()}</Emphasis>),
                errors.into_iter().next(),
            ));
        };

        let plugin = Self {
            analyzer_plugins: manifest
                .rules
                .into_iter()
                .map(|rule| Utf8PathBuf::from_path_buf(rule).unwrap())
                .map(|rule| {
                    if rule.as_os_str().as_encoded_bytes().ends_with(b".grit") {
                        let plugin = AnalyzerGritPlugin::load(fs, &plugin_path.join(rule))?;
                        Ok(Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>))
                    } else {
                        Err(PluginDiagnostic::unsupported_rule_format(markup!(
                            "Unsupported rule format for plugin rule "
                            <Emphasis>{rule.to_string()}</Emphasis>
                        )))
                    }
                })
                .collect::<Result<_, _>>()?,
        };

        Ok(plugin)
    }
}

/// Normalizes the given `path` without requiring filesystem access.
///
/// This only normalizes `.` and `..` entries, but does not resolve symlinks.
fn normalize_path(path: &Utf8Path) -> Utf8PathBuf {
    let mut stack = Vec::new();

    for component in path.components() {
        match component {
            Utf8Component::ParentDir => {
                if stack.last().is_some_and(|last| *last == "..") {
                    stack.push("..");
                } else {
                    stack.pop();
                }
            }
            Utf8Component::CurDir => {}
            Utf8Component::RootDir => {
                stack.clear();
                stack.push("/");
            }
            Utf8Component::Normal(c) => stack.push(c),
            _ => {}
        }
    }

    let mut result = Utf8PathBuf::new();
    for part in stack {
        result.push(part);
    }

    result
}

#[cfg(test)]
mod test {
    use biome_diagnostics::{print_diagnostic_to_string, Error};
    use biome_fs::MemoryFileSystem;

    use super::*;

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
        let mut fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    "version": 1,
    "rules": ["rules/1.grit"]
}"#,
        );

        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let plugin = BiomePlugin::load(&fs, "./my-plugin", Utf8Path::new("/"))
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn load_plugin_without_manifest() {
        let mut fs = MemoryFileSystem::default();
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let error = BiomePlugin::load(&fs, "./my-plugin", Utf8Path::new("/"))
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_without_manifest", error.into());
    }

    #[test]
    fn load_plugin_with_wrong_version() {
        let mut fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    "version": 2,
    "rules": ["rules/1.grit"]
}"#,
        );

        let error = BiomePlugin::load(&fs, "./my-plugin", Utf8Path::new("/"))
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_with_wrong_version", error.into());
    }

    #[test]
    fn load_plugin_with_wrong_rule_extension() {
        let mut fs = MemoryFileSystem::default();
        fs.insert(
            "/my-plugin/biome-manifest.jsonc".into(),
            r#"{
    "version": 1,
    "rules": ["rules/1.js"]
}"#,
        );

        let error = BiomePlugin::load(&fs, "./my-plugin", Utf8Path::new("/"))
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_with_wrong_rule_extension", error.into());
    }

    #[test]
    fn load_single_rule_plugin() {
        let mut fs = MemoryFileSystem::default();
        fs.insert("/my-plugin.grit".into(), r#"`hello`"#);

        let plugin = BiomePlugin::load(&fs, "./my-plugin.grit", Utf8Path::new("/"))
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }
}

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

use std::sync::Arc;

use biome_analyze::{AnalyzerPlugin, AnalyzerPluginVec};
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::normalize_path;
use biome_json_parser::JsonParserOptions;
use biome_resolver::FsWithResolverProxy;
use camino::{Utf8Path, Utf8PathBuf};
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
        fs: Arc<dyn FsWithResolverProxy>,
        plugin_path: &str,
        base_path: &Utf8Path,
    ) -> Result<(Self, Utf8PathBuf), PluginDiagnostic> {
        let plugin_path = normalize_path(&base_path.join(plugin_path));

        // If the plugin path references a `.grit` file directly, treat it as
        // a single-rule plugin instead of going through the manifest process:
        if plugin_path
            .extension()
            .is_some_and(|extension| extension == "grit")
        {
            let plugin = AnalyzerGritPlugin::load(fs.as_ref(), &plugin_path)?;
            return Ok((
                Self {
                    analyzer_plugins: vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)],
                },
                plugin_path,
            ));
        }

        // TODO: plugin can have multiple analyser rules
        #[cfg(feature = "js_plugin")]
        if plugin_path
            .extension()
            .is_some_and(|extension| extension == "js" || extension == "mjs")
        {
            let plugin = AnalyzerJsPlugin::load(fs.clone(), &plugin_path)?;
            return Ok((
                Self {
                    analyzer_plugins: vec![Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)],
                },
                plugin_path,
            ));
        }

        let manifest_path = plugin_path.join("biome-manifest.jsonc");
        if !fs.path_is_file(&manifest_path) {
            return Err(PluginDiagnostic::cant_resolve(manifest_path, None));
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
                        let plugin =
                            AnalyzerGritPlugin::load(fs.as_ref(), &plugin_path.join(rule))?;
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

        Ok((plugin, plugin_path))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_diagnostics::{Error, print_diagnostic_to_string};
    use biome_fs::MemoryFileSystem;

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
    "rules": ["rules/1.grit"]
}"#,
        );

        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) =
            BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/")).expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn load_plugin_without_manifest() {
        let fs = MemoryFileSystem::default();
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let error = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"))
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
        let error = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"))
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
        let error = BiomePlugin::load(fs, "./my-plugin", Utf8Path::new("/"))
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_with_wrong_rule_extension", error.into());
    }

    #[test]
    fn load_single_rule_plugin() {
        let fs = MemoryFileSystem::default();
        fs.insert("/my-plugin.grit".into(), r#"`hello`"#);

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin.grit", Utf8Path::new("/"))
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
        let (plugin, _) = BiomePlugin::load(fs, "./my-plugin.js", Utf8Path::new("/"))
            .expect("Couldn't load plugin");

        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }
}

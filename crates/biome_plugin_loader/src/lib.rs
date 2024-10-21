mod analyzer_grit_plugin;
mod analyzer_plugin;
mod diagnostics;
mod plugin_manifest;

use std::path::Path;

use analyzer_grit_plugin::AnalyzerGritPlugin;
pub use analyzer_plugin::AnalyzerPlugin;
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_diagnostics::adapters::ResolveError;
use biome_fs::FileSystem;
use biome_json_parser::JsonParserOptions;
use diagnostics::PluginDiagnostic;
use plugin_manifest::PluginManifest;

#[derive(Debug)]
pub struct BiomePlugin {
    pub analyzer_plugins: Vec<Box<dyn AnalyzerPlugin>>,
}

impl BiomePlugin {
    /// Loads a plugin from the given `plugin_path`.
    ///
    /// Base paths are used to resolve relative paths and package specifiers.
    pub fn load(
        fs: &dyn FileSystem,
        plugin_path: &str,
        relative_resolution_base_path: &Path,
        external_resolution_base_path: &Path,
    ) -> Result<Self, PluginDiagnostic> {
        let plugin_path = if let Some(plugin_path) = plugin_path.strip_prefix("./") {
            relative_resolution_base_path.join(plugin_path)
        } else if plugin_path.starts_with('.') {
            relative_resolution_base_path.join(plugin_path)
        } else {
            fs.resolve_configuration(plugin_path, external_resolution_base_path)
                .map_err(|error| {
                    PluginDiagnostic::cant_resolve(
                        external_resolution_base_path.display().to_string(),
                        Some(ResolveError::from(error)),
                    )
                })?
                .into_path_buf()
        };

        // If the plugin path references a `.grit` file directly, treat it as
        // a single-rule plugin instead of going through the manifest process:
        if plugin_path
            .as_os_str()
            .as_encoded_bytes()
            .ends_with(b".grit")
        {
            let plugin = AnalyzerGritPlugin::load(fs, &plugin_path)?;
            return Ok(Self {
                analyzer_plugins: vec![Box::new(plugin) as Box<dyn AnalyzerPlugin>],
            });
        }

        let manifest_path = plugin_path.join("biome-manifest.jsonc");
        if !fs.path_is_file(&manifest_path) {
            return Err(PluginDiagnostic::cant_resolve(
                manifest_path.display().to_string(),
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
                markup!("Cannot load plugin manifest "<Emphasis>{manifest_path.display().to_string()}</Emphasis>),
                errors.into_iter().next(),
            ));
        };

        let plugin = Self {
            analyzer_plugins: manifest
                .rules
                .into_iter()
                .map(|rule| {
                    if rule.as_os_str().as_encoded_bytes().ends_with(b".grit") {
                        let plugin = AnalyzerGritPlugin::load(fs, &plugin_path.join(rule))?;
                        Ok(Box::new(plugin) as Box<dyn AnalyzerPlugin>)
                    } else {
                        Err(PluginDiagnostic::unsupported_rule_format(markup!(
                            "Unsupported rule format for plugin rule "
                            <Emphasis>{rule.display().to_string()}</Emphasis>
                        )))
                    }
                })
                .collect::<Result<_, _>>()?,
        };

        Ok(plugin)
    }
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

        let plugin = BiomePlugin::load(&fs, "./my-plugin", Path::new("/"), Path::new("/"))
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }

    #[test]
    fn load_plugin_without_manifest() {
        let mut fs = MemoryFileSystem::default();
        fs.insert("/my-plugin/rules/1.grit".into(), r#"`hello`"#);

        let error = BiomePlugin::load(&fs, "./my-plugin", Path::new("/"), Path::new("/"))
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

        let error = BiomePlugin::load(&fs, "./my-plugin", Path::new("/"), Path::new("/"))
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

        let error = BiomePlugin::load(&fs, "./my-plugin", Path::new("/"), Path::new("/"))
            .expect_err("Plugin loading should've failed");
        snap_diagnostic("load_plugin_with_wrong_rule_extension", error.into());
    }

    #[test]
    fn load_single_rule_plugin() {
        let mut fs = MemoryFileSystem::default();
        fs.insert("/my-plugin.grit".into(), r#"`hello`"#);

        let plugin = BiomePlugin::load(&fs, "./my-plugin.grit", Path::new("/"), Path::new("/"))
            .expect("Couldn't load plugin");
        assert_eq!(plugin.analyzer_plugins.len(), 1);
    }
}

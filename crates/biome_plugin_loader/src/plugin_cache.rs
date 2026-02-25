use biome_analyze::AnalyzerPluginVec;
use camino::Utf8PathBuf;
use papaya::HashMap;
use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::configuration::Plugins;
use crate::{BiomePlugin, PluginDiagnostic};

/// Cache for storing loaded plugins in memory.
///
/// Plugins are kept in a map from path to plugin instance. This allows for
/// convenient reloading of plugins if they are modified on disk.
#[derive(Debug, Default)]
pub struct PluginCache(HashMap<Utf8PathBuf, BiomePlugin, FxBuildHasher>);

impl PluginCache {
    /// Inserts a new plugin into the cache.
    pub fn insert_plugin(&self, path: Utf8PathBuf, plugin: BiomePlugin) {
        self.0.pin().insert(path, plugin);
    }

    /// Returns the loaded and matched analyzer plugins, deduped.
    pub fn get_analyzer_plugins(
        &self,
        plugin_configs: &Plugins,
    ) -> Result<AnalyzerPluginVec, Vec<PluginDiagnostic>> {
        let mut result = AnalyzerPluginVec::new();
        let mut seen = FxHashSet::default();
        let mut diagnostics: Vec<PluginDiagnostic> = Vec::new();

        let map = self.0.pin();
        for plugin_config in plugin_configs.iter() {
            let plugin_path = plugin_config.path();
            if seen.insert(plugin_path) {
                let path_buf = Utf8PathBuf::from(plugin_path);
                // Fast path: try exact key match first (O(1)).
                // Fall back to suffix match only if needed (e.g. relative vs absolute paths).
                let found = map.get(&path_buf).or_else(|| {
                    map.iter()
                        .find(|(path, _)| path.ends_with(path_buf.as_path()))
                        .map(|(_, plugin)| plugin)
                });
                match found {
                    Some(plugin) => {
                        result.extend_from_slice(&plugin.analyzer_plugins);
                    }
                    None => {
                        diagnostics.push(PluginDiagnostic::not_loaded(path_buf));
                    }
                }
            }
        }

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{PluginConfiguration, PluginPathWithOptions};

    #[test]
    fn cache_lookup_with_path_with_options() {
        let cache = PluginCache::default();

        // Insert a plugin keyed by its resolved path
        let path = Utf8PathBuf::from("/resolved/my-plugin.wasm");
        let plugin = BiomePlugin {
            analyzer_plugins: vec![],
        };
        cache.insert_plugin(path.clone(), plugin);

        // Look it up via a PathWithOptions configuration
        let plugins = Plugins(vec![PluginConfiguration::PathWithOptions(
            PluginPathWithOptions {
                path: "/resolved/my-plugin.wasm".into(),
                options: r#"{"maxLength": 100}"#.into(),
                rules: None,
            },
        )]);

        let result = cache.get_analyzer_plugins(&plugins);
        assert!(result.is_ok(), "Expected cache hit for PathWithOptions");
        assert!(result.unwrap().is_empty()); // empty because our test plugin has 0 analyzer_plugins
    }

    #[test]
    fn cache_miss_returns_error() {
        let cache = PluginCache::default();

        let plugins = Plugins(vec![PluginConfiguration::Path(
            "/missing/plugin.grit".into(),
        )]);

        let result = cache.get_analyzer_plugins(&plugins);
        assert!(result.is_err());
    }
}

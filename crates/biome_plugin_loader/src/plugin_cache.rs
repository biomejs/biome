use biome_analyze::AnalyzerPluginVec;
use camino::Utf8PathBuf;
use papaya::HashMap;
use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::configuration::{PluginConfiguration, Plugins};
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

    /// Returns the loaded and matched analyzer plugins, deduped
    pub fn get_analyzer_plugins(
        &self,
        plugin_configs: &Plugins,
    ) -> Result<AnalyzerPluginVec, Vec<PluginDiagnostic>> {
        let mut result = AnalyzerPluginVec::new();
        let mut seen = FxHashSet::default();
        let mut diagnostics: Vec<PluginDiagnostic> = Vec::new();

        let map = self.0.pin();
        for plugin_config in plugin_configs.iter() {
            match plugin_config {
                PluginConfiguration::Path(plugin_path) => {
                    if seen.insert(plugin_path) {
                        let path_buf = Utf8PathBuf::from(plugin_path);
                        match map
                            .iter()
                            .find(|(path, _)| path.ends_with(path_buf.as_path()))
                        {
                            Some((_, plugin)) => {
                                result.extend_from_slice(&plugin.analyzer_plugins);
                            }
                            None => {
                                diagnostics.push(PluginDiagnostic::not_loaded(path_buf));
                            }
                        }
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

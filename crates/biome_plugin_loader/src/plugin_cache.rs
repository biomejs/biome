use biome_analyze::AnalyzerPluginVec;
use camino::Utf8PathBuf;
use papaya::HashMap;
use rustc_hash::FxBuildHasher;

use crate::configuration::{PluginConfiguration, Plugins};
use crate::{BiomePlugin, PluginDiagnostic};

/// Cache for storing loaded plugins in memory.
///
/// Each configured instance is stored separately so the same plugin can use
/// different options in different overrides.
#[derive(Debug, Default)]
pub struct PluginCache(HashMap<PluginConfiguration, BiomePlugin, FxBuildHasher>);

impl PluginCache {
    /// Inserts a new plugin into the cache.
    pub fn insert_plugin(&self, configuration: &PluginConfiguration, plugin: BiomePlugin) {
        self.0.pin().insert(configuration.clone(), plugin);
    }

    /// Returns analyzer plugins for the configured entries in declaration order.
    ///
    /// Repeated equivalent configurations are emitted once. Configurations that differ by options
    /// remain distinct. Returns diagnostics instead of a partial result if any configured plugin is
    /// absent from the cache.
    pub fn get_analyzer_plugins(
        &self,
        plugin_configs: &Plugins,
    ) -> Result<AnalyzerPluginVec, Vec<PluginDiagnostic>> {
        let mut result = AnalyzerPluginVec::new();
        let mut seen = Vec::new();
        let mut diagnostics: Vec<PluginDiagnostic> = Vec::new();

        let map = self.0.pin();
        for plugin_config in plugin_configs.iter() {
            if seen.contains(&plugin_config) {
                continue;
            }
            seen.push(plugin_config);

            match map.get(plugin_config) {
                Some(plugin) => {
                    result.extend_from_slice(&plugin.analyzer_plugins);
                }
                None => {
                    diagnostics.push(PluginDiagnostic::not_loaded(Utf8PathBuf::from(
                        plugin_config.path(),
                    )));
                }
            }
        }

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        Ok(result)
    }
}

use biome_analyze::AnalyzerPluginVec;
use camino::Utf8PathBuf;
use papaya::HashMap;
use rustc_hash::FxBuildHasher;

use crate::BiomePlugin;

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

    /// Returns the loaded analyzer plugins.
    pub fn get_analyzer_plugins(&self) -> AnalyzerPluginVec {
        let mut plugins = AnalyzerPluginVec::new();
        for plugin in self.0.pin().values() {
            plugins.extend_from_slice(&plugin.analyzer_plugins);
        }
        plugins
    }
}

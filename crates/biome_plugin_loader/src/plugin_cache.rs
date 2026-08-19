use biome_analyze::AnalyzerPluginVec;
use camino::Utf8PathBuf;

use crate::configuration::{PluginConfiguration, Plugins};
use crate::{BiomePlugin, PluginDiagnostic};

/// Cache for storing loaded plugins in memory.
///
/// Each configured instance is stored separately so the same plugin can use
/// different options in different overrides.
#[derive(Debug, Default)]
pub struct PluginCache(Vec<(PluginConfiguration, BiomePlugin)>);

impl PluginCache {
    /// Inserts a new plugin into the cache.
    pub fn insert_plugin(&mut self, configuration: &PluginConfiguration, plugin: BiomePlugin) {
        match self
            .0
            .iter_mut()
            .find(|(cached_configuration, _)| cached_configuration == configuration)
        {
            Some((_, cached_plugin)) => *cached_plugin = plugin,
            None => self.0.push((configuration.clone(), plugin)),
        }
    }

    /// Returns the loaded and matched analyzer plugins, deduped
    pub fn get_analyzer_plugins(
        &self,
        plugin_configs: &Plugins,
    ) -> Result<AnalyzerPluginVec, Vec<PluginDiagnostic>> {
        let mut result = AnalyzerPluginVec::new();
        let mut seen = Vec::new();
        let mut diagnostics: Vec<PluginDiagnostic> = Vec::new();

        for plugin_config in plugin_configs.iter() {
            if seen.contains(&plugin_config) {
                continue;
            }
            seen.push(plugin_config);

            match self
                .0
                .iter()
                .find(|(cached_configuration, _)| cached_configuration == plugin_config)
            {
                Some((_, plugin)) => {
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

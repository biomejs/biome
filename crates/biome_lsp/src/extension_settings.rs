use std::str::FromStr;

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use tracing::debug;

pub(crate) const CONFIGURATION_SECTION: &str = "biome";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// The settings applied to the workspace by the LSP
pub struct WorkspaceSettings {
    /// Unstable features enabled
    #[serde(default)]
    pub unstable: bool,

    /// Enable rename capability
    /// Deprecated, use `experimental.rename` instead
    pub rename: Option<bool>,

    /// Only run Biome if a `biome.json` configuration file exists.
    pub require_configuration: Option<bool>,

    /// Path to the configuration file to prefer over the default `biome.json`.
    pub configuration_path: Option<String>,

    /// Experimental settings
    pub experimental: Option<ExperimentalSettings>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalSettings {
    /// Enable experimental symbol renaming
    pub rename: Option<bool>,
}

/// The `biome.*` extension settings
#[derive(Debug)]
pub(crate) struct ExtensionSettings {
    pub(crate) settings: WorkspaceSettings,
}

impl ExtensionSettings {
    pub(crate) fn new() -> Self {
        Self {
            settings: WorkspaceSettings::default(),
        }
    }

    pub(crate) fn set_workspace_settings(&mut self, value: Value) -> Result<(), Error> {
        let workspace_settings = serde_json::from_value(value)?;
        self.settings = workspace_settings;
        debug!(
            "Correctly stored the settings coming from the client: {:?}",
            self.settings
        );
        Ok(())
    }

    pub(crate) fn requires_configuration(&self) -> bool {
        self.settings.require_configuration.unwrap_or_default()
    }

    pub(crate) fn configuration_path(&self) -> Option<Utf8PathBuf> {
        match self.settings.configuration_path.as_deref() {
            // Ignore if empty as VS Code responses an empty string even if it's not set.
            Some(config_path) if !config_path.is_empty() => Utf8PathBuf::from_str(config_path).ok(),
            _ => None,
        }
    }
}

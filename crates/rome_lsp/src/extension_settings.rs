use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use tracing::trace;

pub(crate) const CONFIGURATION_SECTION: &str = "biome";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// The settings applied to the workspace by the LSP
pub struct WorkspaceSettings {
    /// Unstable features enabled
    #[serde(default)]
    pub unstable: bool,

    /// Enable rename capability
    pub rename: Option<bool>,

    /// Only run Biome if a `rome.json` configuration file exists.
    pub require_configuration: Option<bool>,
}

/// The `rome.*` extension settings
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
        trace!(
            "Correctly stored the settings coming from the client: {:?}",
            self.settings
        );
        Ok(())
    }

    pub(crate) fn requires_configuration(&self) -> bool {
        self.settings.require_configuration.unwrap_or_default()
    }
}

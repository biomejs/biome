use camino::{Utf8Path, Utf8PathBuf};

/// Identifies which source set a configuration value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceSource {
    /// The base/root configuration file being loaded
    BaseConfig {
        /// Absolute path to the config file
        path: Utf8PathBuf,
    },

    /// An extended configuration from the 'extends' array
    ExtendedConfig {
        /// The file that set this value
        path: Utf8PathBuf,

        /// How we got here (for nested extends)
        /// Example: ["main.json", "react.json", "react-base.json"]
        /// The last entry is the file that actually set the value
        resolution_path: Vec<Utf8PathBuf>,
    },

    /// An .editorconfig file
    EditorConfig {
        /// Path to the .editorconfig file
        path: Utf8PathBuf,
    },

    /// An override pattern from the 'overrides' array
    Override {
        /// Which configuration file contained this override
        config_source: Box<ProvenanceSource>,

        /// The index of this override in the overrides array
        index: usize,

        /// The glob patterns (for display purposes)
        includes: Vec<String>,
    },

    /// CLI argument (--indent-width=4)
    CliArgument {
        /// The argument string for reference
        argument: String,
    },

    /// Default/fallback value (not explicitly configured)
    Default,
}

impl ProvenanceSource {
    /// Get the config file path for this source
    /// This is the actual configuration file containing the value
    pub fn config_path(&self) -> Option<&Utf8Path> {
        match self {
            Self::BaseConfig { path } => Some(path),
            Self::ExtendedConfig { path, .. } => Some(path),
            Self::EditorConfig { path } => Some(path),
            Self::Override { config_source, .. } => config_source.config_path(),
            Self::CliArgument { .. } | Self::Default => None,
        }
    }

    /// Create a BaseConfig source
    pub fn base_config(path: Utf8PathBuf) -> Self {
        Self::BaseConfig { path }
    }

    /// Create an ExtendedConfig source
    pub fn extended_config(path: Utf8PathBuf, resolution_path: Vec<Utf8PathBuf>) -> Self {
        Self::ExtendedConfig {
            path,
            resolution_path,
        }
    }

    /// Create an EditorConfig source
    pub fn editor_config(path: Utf8PathBuf) -> Self {
        Self::EditorConfig { path }
    }

    /// Create an Override source
    pub fn override_source(
        config_source: ProvenanceSource,
        index: usize,
        includes: Vec<String>,
    ) -> Self {
        Self::Override {
            config_source: Box::new(config_source),
            index,
            includes,
        }
    }

    /// Create a CliArgument source
    pub fn cli_argument(argument: String) -> Self {
        Self::CliArgument { argument }
    }

    /// Create a Default source
    pub fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_config_source() {
        let path = Utf8PathBuf::from("/project/biome.json");
        let source = ProvenanceSource::base_config(path.clone());

        assert_eq!(source.config_path(), Some(path.as_path()));
    }

    #[test]
    fn test_extended_config_source() {
        let path = Utf8PathBuf::from("/project/base.json");
        let resolution = vec![
            Utf8PathBuf::from("/project/biome.json"),
            Utf8PathBuf::from("/project/base.json"),
        ];
        let source = ProvenanceSource::extended_config(path.clone(), resolution);

        assert_eq!(source.config_path(), Some(path.as_path()));
    }

    #[test]
    fn test_override_source() {
        let base_path = Utf8PathBuf::from("/project/biome.json");
        let base_source = ProvenanceSource::base_config(base_path.clone());

        let override_source =
            ProvenanceSource::override_source(base_source, 0, vec!["*.test.js".to_string()]);

        // Override should return the base config path
        assert_eq!(override_source.config_path(), Some(base_path.as_path()));
    }

    #[test]
    fn test_cli_argument_source() {
        let source = ProvenanceSource::cli_argument("--indent-width=4".to_string());
        assert_eq!(source.config_path(), None);
    }

    #[test]
    fn test_default_source() {
        let source = ProvenanceSource::default();
        assert_eq!(source.config_path(), None);
    }

    #[test]
    fn test_nested_override_path() {
        let base_path = Utf8PathBuf::from("/project/biome.json");
        let extended_path = Utf8PathBuf::from("/project/base.json");

        let extended_source = ProvenanceSource::extended_config(
            extended_path.clone(),
            vec![base_path.clone(), extended_path.clone()],
        );

        let override_source =
            ProvenanceSource::override_source(extended_source, 0, vec!["*.ts".to_string()]);

        // Should traverse through Override -> ExtendedConfig to get the path
        assert_eq!(override_source.config_path(), Some(extended_path.as_path()));
    }
}

//! A simple implementation of feature flags.

use biome_console::fmt::{Display, Formatter};
use biome_console::{markup, DebugDisplay, KeyValuePair};
use std::env;
use std::ops::Deref;
use std::sync::{LazyLock, OnceLock};

/// Returns `true` if this is an unstable build of Biome
pub fn is_unstable() -> bool {
    BIOME_VERSION.deref().is_none()
}

/// The internal version of Biome. This is usually supplied during the CI build
pub static BIOME_VERSION: LazyLock<Option<&str>> = LazyLock::new(|| option_env!("BIOME_VERSION"));

pub struct BiomeEnv {
    pub biome_log_path: BiomeEnvVariable,
    pub biome_log_prefix: BiomeEnvVariable,
    pub biome_config_path: BiomeEnvVariable,
}

pub static BIOME_ENV: OnceLock<BiomeEnv> = OnceLock::new();

impl BiomeEnv {
    fn new() -> Self {
        Self {
            biome_log_path: BiomeEnvVariable::new(
                "BIOME_LOG_PATH",
                "The directory where the Daemon logs will be saved.",
            ),
            biome_log_prefix: BiomeEnvVariable::new(
                "BIOME_LOG_PREFIX_NAME",
                "A prefix that's added to the name of the log. Default: `server.log.`",
            ),
            biome_config_path: BiomeEnvVariable::new(
                "BIOME_CONFIG_PATH",
                "A path to the configuration file",
            ),
        }
    }
}

pub struct BiomeEnvVariable {
    /// The name of the environment variable
    name: &'static str,
    /// The description of the variable.
    // This field will be used in the website to automate its generation
    description: &'static str,
}

impl BiomeEnvVariable {
    fn new(name: &'static str, description: &'static str) -> Self {
        Self { name, description }
    }

    /// It attempts to read the value of the variable
    pub fn value(&self) -> Option<String> {
        env::var(self.name).ok()
    }

    /// It returns the description of the variable
    pub fn description(&self) -> &'static str {
        self.description
    }

    /// It returns the name of the variable.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub fn biome_env() -> &'static BiomeEnv {
    BIOME_ENV.get_or_init(BiomeEnv::new)
}

impl Display for BiomeEnv {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self.biome_log_path.value() {
            None => {
                KeyValuePair(self.biome_log_path.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.biome_log_path.name, markup! {{DebugDisplay(value)}}).fmt(fmt)?;
            }
        };
        match self.biome_log_prefix.value() {
            None => {
                KeyValuePair(self.biome_log_prefix.name, markup! { <Dim>"unset"</Dim> })
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.biome_log_prefix.name, markup! {{DebugDisplay(value)}})
                    .fmt(fmt)?;
            }
        };

        match self.biome_config_path.value() {
            None => {
                KeyValuePair(self.biome_config_path.name, markup! { <Dim>"unset"</Dim> })
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.biome_config_path.name, markup! {{DebugDisplay(value)}})
                    .fmt(fmt)?;
            }
        };

        Ok(())
    }
}

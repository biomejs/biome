//! A simple implementation of feature flags.

#![deny(clippy::use_self)]

use biome_console::fmt::{Display, Formatter};
use biome_console::{DebugDisplay, KeyValuePair, markup};
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
    pub biome_log_prefix_name: BiomeEnvVariable,
    pub biome_config_path: BiomeEnvVariable,
    pub biome_threads: BiomeEnvVariable,
    pub biome_watcher_kind: BiomeEnvVariable,
    pub biome_watcher_polling_interval: BiomeEnvVariable,
    pub biome_log_level: BiomeEnvVariable,
    pub biome_log_kind: BiomeEnvVariable,
}

pub static BIOME_ENV: OnceLock<BiomeEnv> = OnceLock::new();

impl BiomeEnv {
    fn new() -> Self {
        Self {
            biome_log_path: BiomeEnvVariable::new(
                "BIOME_LOG_PATH",
                "The directory where the Daemon logs will be saved.",
            ),
            biome_log_prefix_name: BiomeEnvVariable::new(
                "BIOME_LOG_PREFIX_NAME",
                "A prefix that's added to the name of the log. Default: `server.log.`",
            ),
            biome_config_path: BiomeEnvVariable::new(
                "BIOME_CONFIG_PATH",
                "A path to the configuration file",
            ),
            biome_threads: BiomeEnvVariable::new(
                "BIOME_THREADS",
                "The number of threads to use in CI.",
            ),
            biome_watcher_kind: BiomeEnvVariable::new(
                "BIOME_WATCHER_KIND",
                "The kind of watcher to use. Possible values: polling, recommended, none. Default: recommended.",
            ),
            biome_watcher_polling_interval: BiomeEnvVariable::new(
                "BIOME_WATCHER_POLLING_INTERVAL",
                "The polling interval in milliseconds. This is only applicable when using the polling watcher. Default: 2000.",
            ),
            biome_log_level: BiomeEnvVariable::new(
                "BIOME_LOG_LEVEL",
                "The level of logging. Possible values: none, tracing, debug, info, warn, error. Default: info.",
            ),
            biome_log_kind: BiomeEnvVariable::new(
                "BIOME_LOG_KIND",
                "What the log should look like. Possible values: pretty, compact, json. Default: pretty.",
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
        let padding = 35usize;
        match self.biome_log_path.value() {
            None => {
                KeyValuePair::new(self.biome_log_path.name, markup! { <Dim>"unset"</Dim> })
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(self.biome_log_path.name, markup! {{DebugDisplay(value)}})
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
        };
        match self.biome_log_prefix_name.value() {
            None => {
                KeyValuePair::new(
                    self.biome_log_prefix_name.name,
                    markup! { <Dim>"unset"</Dim> },
                )
                .with_padding(padding)
                .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(
                    self.biome_log_prefix_name.name,
                    markup! {{DebugDisplay(value)}},
                )
                .with_padding(padding)
                .fmt(fmt)?;
            }
        };

        match self.biome_log_level.value() {
            None => {
                KeyValuePair::new(self.biome_log_level.name, markup! { <Dim>"unset"</Dim> })
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(self.biome_log_level.name, markup! {{DebugDisplay(value)}})
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
        };

        match self.biome_log_kind.value() {
            None => {
                KeyValuePair::new(self.biome_log_kind.name, markup! { <Dim>"unset"</Dim> })
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(self.biome_log_kind.name, markup! {{DebugDisplay(value)}})
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
        };

        match self.biome_config_path.value() {
            None => {
                KeyValuePair::new(self.biome_config_path.name, markup! { <Dim>"unset"</Dim> })
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(self.biome_config_path.name, markup! {{DebugDisplay(value)}})
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
        };

        match self.biome_threads.value() {
            None => {
                KeyValuePair::new(self.biome_threads.name, markup! { <Dim>"unset"</Dim> })
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(self.biome_threads.name, markup! {{DebugDisplay(value)}})
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
        };

        match self.biome_watcher_kind.value() {
            None => {
                KeyValuePair::new(self.biome_watcher_kind.name, markup! { <Dim>"unset"</Dim> })
                    .with_padding(padding)
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(
                    self.biome_watcher_kind.name,
                    markup! {{DebugDisplay(value)}},
                )
                .with_padding(padding)
                .fmt(fmt)?;
            }
        };

        match self.biome_watcher_polling_interval.value() {
            None => {
                KeyValuePair::new(
                    self.biome_watcher_polling_interval.name,
                    markup! { <Dim>"unset"</Dim> },
                )
                .with_padding(padding)
                .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair::new(
                    self.biome_watcher_polling_interval.name,
                    markup! {{DebugDisplay(value)}},
                )
                .with_padding(padding)
                .fmt(fmt)?;
            }
        };

        Ok(())
    }
}

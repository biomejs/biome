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

#[derive(Default)]
pub struct BiomeEnv {}

pub static BIOME_ENV: OnceLock<BiomeEnv> = OnceLock::new();

impl BiomeEnv {
    /// It attempts to read the value of the variable from the environment using [env::var]
    pub fn value_for(&self, name: &str) -> Option<String> {
        Self::ENV_VARIABLES
            .iter()
            .find(|variable| variable.name == name)
            .and_then(|variable| variable.value())
    }

    pub const ENV_VARIABLES: &[&BiomeEnvVariable] = &[
        &BiomeEnvVariable::new(
            "BIOME_DISTRIBUTION",
            "Override the detected distribution channel of Biome. Acceptable values: npm, homebrew or standalone",
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_PATH",
            "The directory where the logs of the Biome Daemon are stored.",
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_PREFIX_NAME",
            "A prefix that's added to the name of the log. Default: `server.log.`",
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_LEVEL",
            "The level of logging. Possible values: none, tracing, debug, info, warn, error. Default: info.",
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_KIND",
            "What the log should look like. Possible values: pretty, compact, json. Default: pretty.",
        ),
        &BiomeEnvVariable::new("BIOME_CONFIG_PATH", "A path to the configuration file"),
        &BiomeEnvVariable::new("BIOME_THREADS", "The number of threads to use in CI."),
        &BiomeEnvVariable::new(
            "BIOME_WATCHER_KIND",
            "The kind of watcher to use. Possible values: polling, recommended, none. Default: recommended.",
        ),
        &BiomeEnvVariable::new(
            "BIOME_WATCHER_POLLING_INTERVAL",
            "The polling interval in milliseconds. This is only applicable when using the polling watcher. Default: 2000.",
        ),
    ];
}

pub struct BiomeEnvVariable {
    /// The name of the environment variable
    name: &'static str,
    /// The description of the variable.
    // This field will be used in the website to automate its generation
    description: &'static str,
}

impl BiomeEnvVariable {
    const fn new(name: &'static str, description: &'static str) -> Self {
        Self { name, description }
    }

    /// It attempts to read the value of the variable from the environment using [env::var]
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
    BIOME_ENV.get_or_init(BiomeEnv::default)
}

impl Display for BiomeEnv {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let padding = 35usize;

        for variable in Self::ENV_VARIABLES {
            match variable.value() {
                None => {
                    KeyValuePair::new(variable.name, markup! { <Dim>"unset"</Dim> })
                        .with_padding(padding)
                        .fmt(fmt)?;
                }
                Some(value) => {
                    KeyValuePair::new(variable.name, markup! {{DebugDisplay(value)}})
                        .with_padding(padding)
                        .fmt(fmt)?;
                }
            }
        }

        Ok(())
    }
}

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
            r#"Overrides the installation source detected by `biome upgrade`.

The following installation sources are supported:

- `npm`: updates Biome by running an npm-compatible package manager command.
- `homebrew`: updates Biome by running `brew upgrade biome`.
- `standalone`: updates the standalone Biome binary directly.

Use this when `biome upgrade` cannot correctly detect how Biome was installed, such as a Homebrew installation under a custom prefix.

```shell
BIOME_DISTRIBUTION=homebrew biome upgrade
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_FILE",
            r#"Writes internal CLI logs from commands such as `check`, `lint`, `format`, and `ci` to the specified file.

Use this to save detailed CLI logs while investigating unexpected behavior.

```shell
BIOME_LOG_FILE=biome-debug.log BIOME_LOG_LEVEL=debug biome check .
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_PATH",
            r#"Directory where the Biome daemon stores its log files.

Use this to keep daemon logs in a project-specific directory while troubleshooting an editor integration.

```shell
BIOME_LOG_PATH="$PWD/.biome-logs" biome start
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_PREFIX_NAME",
            r#"Prefix added to daemon log file names. Defaults to `server.log`.

Use this to distinguish logs from different projects that share a log directory.

```shell
BIOME_LOG_PREFIX_NAME=my-project.log biome start
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_LEVEL",
            r#"Controls internal logging for commands such as `check`, `lint`, `format`, and `ci`. Defaults to `none`.

The following logging levels are supported:

- `none`: Disables internal logging.
- `info`: Shows general information about Biome's operation.
- `warn`: Shows warnings and errors.
- `error`: Shows only errors.
- `debug`: Shows detailed information useful for debugging.
- `tracing`: Shows the most detailed logs, including span timing.

Use this to inspect Biome's internal CLI activity while checking a project.

```shell
BIOME_LOG_LEVEL=debug biome check .
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_LOG_KIND",
            r#"Controls the internal log format for commands such as `check`, `lint`, `format`, and `ci`. Defaults to `pretty`.

The following log formats are supported:

- `pretty`: Displays human-readable, multiline logs with terminal styling.
- `compact`: Displays logs in a condensed, human-readable format.
- `json`: Displays machine-readable JSON logs.

Use this to collect machine-readable internal logs in CI. Set `BIOME_LOG_LEVEL` to enable logging.

```shell
BIOME_LOG_LEVEL=info BIOME_LOG_KIND=json biome ci .
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_CONFIG_PATH",
            r#"Path to a Biome configuration file or directory.

Use this to apply a shared configuration stored outside the current directory.

```shell
BIOME_CONFIG_PATH="$HOME/.config/biome/biome.json" biome check .
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_THREADS",
            r#"Number of worker threads used by `biome ci`. Defaults to automatic selection.

Use this to limit CPU usage in a constrained CI runner.

```shell
BIOME_THREADS=2 biome ci .
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_WATCHER_KIND",
            r#"Selects the file watcher used by the Biome daemon. Defaults to `recommended`.

The following file-watching strategies are supported:

- `recommended`: Uses the strategy recommended by the operating system.
- `polling`: Periodically checks for changes. This can be slower but may work better on mounted or network filesystems.
- `none`: Disables file watching. The daemon and editor integrations will not detect file changes.

Use polling when native file notifications are unreliable, such as on a network drive or mounted filesystem.

```shell
BIOME_WATCHER_KIND=polling biome start
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_WATCHER_POLLING_INTERVAL",
            r#"Polling interval, in milliseconds, used by the Biome daemon when `BIOME_WATCHER_KIND` is set to `polling`. Defaults to `2000` milliseconds (two seconds).

Use this to adjust how quickly the polling watcher detects changes.

```shell
BIOME_WATCHER_KIND=polling BIOME_WATCHER_POLLING_INTERVAL=1000 biome start
```"#,
        ),
        &BiomeEnvVariable::new(
            "BIOME_BINARY",
            r#"Overrides the Biome binary used by the `@biomejs/biome` npm package. If this variable is not set, the package automatically selects the correct binary for your platform.

Use this to run a system-installed or locally built binary through the `@biomejs/biome` package.

```shell
BIOME_BINARY=/usr/local/bin/biome npx @biomejs/biome check
```"#,
        ),
        &BiomeEnvVariable::new(
            "RUST_BACKTRACE",
            r#"Captures a backtrace if Biome panics, which can help identify where the panic occurred.

Use this when reporting a Biome panic to include a stack trace that can help locate the problem.

```shell
RUST_BACKTRACE=1 biome check .
```

:::note
`RUST_BACKTRACE` is a Rust environment variable, not a Biome-specific option. Biome supports it because it is built with Rust and uses Rust's backtrace support when reporting a panic.
:::"#,
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

//! Detection of how the running Biome binary was installed.

use biome_flags::Environment;
use biome_fs::normalize_path;
use camino::Utf8Path;

/// The channel that installed the running Biome binary.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum InstallSource {
    Homebrew,
    Npm,
    Standalone,
    Unknown,
}

impl From<&str> for InstallSource {
    fn from(source: &str) -> Self {
        match source {
            "homebrew" => Self::Homebrew,
            "npm" => Self::Npm,
            "standalone" => Self::Standalone,
            _ => Self::Unknown,
        }
    }
}

impl InstallSource {
    /// Detects the installation source of the binary at `current_exe`.
    ///
    /// When the `BIOME_DISTRIBUTION` environment variable is set, its value
    /// determines the source and the path is ignored. This allows users to
    /// override the detected source, for example for a Homebrew installation
    /// under a custom prefix. An unrecognized value gives
    /// [InstallSource::Unknown], so `biome upgrade` never attempts a
    /// cross-channel self-update.
    ///
    /// Otherwise, the source comes from the canonical path: a `node_modules`
    /// directory means npm, a Homebrew prefix means Homebrew, and any other
    /// path means a standalone binary.
    pub(crate) fn from_executable(environment: &dyn Environment, current_exe: &Utf8Path) -> Self {
        if let Some(install_source) = environment.var("BIOME_DISTRIBUTION") {
            return Self::from(install_source.as_str());
        }

        let canonical = current_exe
            .canonicalize_utf8()
            .unwrap_or_else(|_| current_exe.to_path_buf());

        if is_npm_install(&canonical) {
            Self::Npm
        } else if is_homebrew_install(&canonical) {
            Self::Homebrew
        } else {
            Self::Standalone
        }
    }
}

/// Determines whether Biome was installed using a npm-compatible package manager
fn is_npm_install(executable: &Utf8Path) -> bool {
    executable
        .components()
        .any(|component| component.as_str() == "node_modules")
}

/// Determines if Biome was installed with Homebrew
///
/// See https://docs.brew.sh/Installation for common Homebrew installation paths
///
/// - /opt/homebrew covers macOS on Apple Silicon
/// - /usr/local/Cellar covers macOS on Intel
/// - /home/linuxbrew/.linuxbrew/ covers Linux installations
/// - /Cellar/biome/ covers custom prefixes if users have set one
fn is_homebrew_install(executable: &Utf8Path) -> bool {
    let executable = normalize_path(executable);
    executable.starts_with("/opt/homebrew/")
        || executable.starts_with("/usr/local/Cellar/")
        || executable.starts_with("/home/linuxbrew/.linuxbrew/")
        || executable.as_str().contains("/Cellar/biome/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_flags::MemoryEnvironment;

    const NPM_PATH: &str = "/tmp/project/node_modules/@biomejs/cli-darwin-arm64/biome";

    fn detect(environment: &MemoryEnvironment, path: &str) -> InstallSource {
        InstallSource::from_executable(environment, Utf8Path::new(path))
    }

    #[test]
    fn detects_npm_install_from_path() {
        assert_eq!(
            detect(&MemoryEnvironment::default(), NPM_PATH),
            InstallSource::Npm
        );
    }

    #[test]
    fn detects_homebrew_install_from_path() {
        assert_eq!(
            detect(
                &MemoryEnvironment::default(),
                "/opt/homebrew/Cellar/biome/2.4.8/bin/biome"
            ),
            InstallSource::Homebrew
        );
    }

    #[test]
    fn defaults_to_standalone_install_source() {
        assert_eq!(
            detect(&MemoryEnvironment::default(), "/usr/local/bin/biome"),
            InstallSource::Standalone
        );
    }

    #[test]
    fn distribution_variable_overrides_path() {
        let environment = MemoryEnvironment::default().with_var("BIOME_DISTRIBUTION", "homebrew");

        assert_eq!(detect(&environment, NPM_PATH), InstallSource::Homebrew);
    }

    #[test]
    fn unrecognized_distribution_variable_is_unknown() {
        let environment = MemoryEnvironment::default().with_var("BIOME_DISTRIBUTION", "apt");

        assert_eq!(detect(&environment, NPM_PATH), InstallSource::Unknown);
    }
}

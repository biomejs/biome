use crate::{CliDiagnostic, CliSession, VERSION};
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Error, StdError};
use biome_fs::normalize_path;
use biome_package::node_semver::Version;
use camino::{Utf8Path, Utf8PathBuf};
use reqwest::blocking::Client;
use self_update::backends::github::Update;
use std::env;
use std::fmt;
use std::process::{Command, Stdio};
use std::time::Duration;

const BREW_BINARY_NAME: &str = "biome";
const GITHUB_REPO_OWNER: &str = "biomejs";
const GITHUB_REPO_NAME: &str = "biome";
const LATEST_VERSION_URL: &str = "https://biomejs.dev/api/versions/latest.txt";

/// Upgrade Biome to the latest version.
pub(crate) fn upgrade(session: CliSession) -> Result<(), CliDiagnostic> {
    ensure_upgrade_supported()?;

    let current_exe = Utf8PathBuf::from_path_buf(env::current_exe().map_err(CliDiagnostic::from)?)
        .map_err(|path| {
            CliDiagnostic::upgrade_error(
                format!(
                    "The current Biome executable path is not valid UTF-8: {}",
                    path.display()
                ),
                None,
            )
        })?;
    let install_source = detect_install_source(&current_exe);

    match install_source {
        InstallSource::Npm => Err(CliDiagnostic::upgrade_error(
            "`biome upgrade` is not available for binaries distributed through npm-compatible package managers. Upgrade Biome with the same package manager you used to install `@biomejs/biome`.",
            None,
        )),
        InstallSource::Homebrew => upgrade_with_homebrew(session),
        InstallSource::Standalone => upgrade_standalone(session),
        InstallSource::Unknown => Err(unknown_install_source_upgrade_error()),
    }
}

fn unknown_install_source_upgrade_error() -> CliDiagnostic {
    CliDiagnostic::upgrade_error(
        "`biome upgrade` couldn't determine how this binary was installed. Upgrade Biome with the same installer or package manager you originally used.",
        None,
    )
}

fn ensure_upgrade_supported() -> Result<(), CliDiagnostic> {
    if env::var_os("BIOME_BINARY").is_some() {
        return Err(CliDiagnostic::upgrade_error(
            "`biome upgrade` is not available when `BIOME_BINARY` is set. Unset `BIOME_BINARY` or upgrade the overridden binary directly.",
            None,
        ));
    }

    Ok(())
}

/// Delegate the upgrade process to Homebrew when Biome was with Homebrew
fn upgrade_with_homebrew(session: CliSession) -> Result<(), CliDiagnostic> {
    session.app.console.log(markup! {
        "Detected a Homebrew installation. Running "<Emphasis>"brew upgrade biome"</Emphasis>"..."
    });

    let status = Command::new("brew")
        .env("HOMEBREW_NO_AUTO_UPDATE", "1")
        .arg("upgrade")
        .arg(BREW_BINARY_NAME)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                CliDiagnostic::upgrade_error(
                    "Biome appears to be installed via Homebrew, but the `brew` executable could not be found in PATH.",
                    Some(Error::from(StdError::from(err))),
                )
            } else {
                CliDiagnostic::from(err)
            }
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(CliDiagnostic::upgrade_error(
            format!(
                "`brew upgrade biome` exited with {}.",
                ExitStatusDisplay(status)
            ),
            None,
        ))
    }
}

/// Upgrade the standalone Biome binary by downloading the latest release from GitHub and replacing
/// the current executable.
fn upgrade_standalone(session: CliSession) -> Result<(), CliDiagnostic> {
    let target =
        release_target().map_err(|err| CliDiagnostic::upgrade_error(err.to_string(), None))?;
    let identifier = release_asset_identifier(&target);
    let latest_version = latest_available_version()?;

    if !is_version_newer(VERSION, &latest_version)? {
        session.app.console.log(markup! {
            "Biome is already up to date at version "<Emphasis>{VERSION}</Emphasis>"."
        });
        return Ok(());
    }

    session.app.console.log(markup! {
        "Downloading the latest standalone Biome release..."
    });

    let status = Update::configure()
        .repo_owner(GITHUB_REPO_OWNER)
        .repo_name(GITHUB_REPO_NAME)
        .bin_name(binary_name_for_self_update())
        .target(&target)
        .identifier(&identifier)
        .current_version(VERSION)
        .target_version_tag(&release_tag_for_version(&latest_version))
        .show_output(false)
        .show_download_progress(true)
        .no_confirm(true)
        .build()
        .map_err(|err| {
            CliDiagnostic::upgrade_error(
                "Failed to prepare the standalone upgrade.",
                Some(Error::from(StdError::from(err))),
            )
        })?
        .update()
        .map_err(|err| {
            CliDiagnostic::upgrade_error(
                "Failed to apply the standalone upgrade.",
                Some(Error::from(StdError::from(err))),
            )
        })?;

    match status {
        self_update::Status::UpToDate(version) => {
            session.app.console.log(markup! {
                "Biome is already up to date at version "<Emphasis>{version}</Emphasis>"."
            });
        }
        self_update::Status::Updated(version) => {
            session.app.console.log(markup! {
                "Biome upgraded successfully to version "<Emphasis>{version}</Emphasis>"."
            });
            session.app.console.log(markup! {
                <Info>
                    "Your editor may still be using the previous Biome version. Reload it to pick up the updated binary."
                </Info>
            });
        }
    }

    Ok(())
}

fn latest_available_version() -> Result<String, CliDiagnostic> {
    let version = Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|err| {
            CliDiagnostic::upgrade_error(
                "Failed to create the HTTP client used to check the latest Biome version.",
                Some(Error::from(StdError::from(err))),
            )
        })?
        .get(LATEST_VERSION_URL)
        .send()
        .map_err(|err| {
            CliDiagnostic::upgrade_error(
                "Failed to request the latest Biome version.",
                Some(Error::from(StdError::from(err))),
            )
        })?
        .error_for_status()
        .map_err(|err| {
            CliDiagnostic::upgrade_error(
                "Failed to fetch the latest Biome version.",
                Some(Error::from(StdError::from(err))),
            )
        })?;

    let version = version.text().map_err(|err| {
        CliDiagnostic::upgrade_error(
            "Failed to read the latest Biome version response.",
            Some(Error::from(StdError::from(err))),
        )
    })?;

    let version = version.trim();
    parse_version(version)?;
    Ok(version.to_string())
}

fn release_tag_for_version(version: &str) -> String {
    format!("@biomejs/biome@{version}")
}

fn is_version_newer(current: &str, latest: &str) -> Result<bool, CliDiagnostic> {
    let current = parse_version(current)?;
    let latest = parse_version(latest)?;
    Ok(latest > current)
}

fn parse_version(version: &str) -> Result<Version, CliDiagnostic> {
    version.parse::<Version>().map_err(|err| {
        CliDiagnostic::upgrade_error(
            format!("Failed to parse the Biome version `{version}`."),
            Some(Error::from(StdError::from(err))),
        )
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum InstallSource {
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

/// Detect the installation source
///
/// This method detects the installation source based on the path to the binary.
///
/// If the `BIOME_DISTRIBUTION` environment variable is set, it will be used to determine the
/// installation source instead of path-based detection. This allows users to override the
/// detected installation source if necessary. Any unrecognized installation source is treated as
/// unknown so `biome upgrade` does not attempt a cross-channel self-update.
fn detect_install_source(current_exe: &Utf8Path) -> InstallSource {
    if let Some(install_source) = install_source_from_env() {
        return install_source;
    }

    let canonical = current_exe
        .canonicalize_utf8()
        .unwrap_or_else(|_| current_exe.to_path_buf());

    if is_npm_install(&canonical) {
        InstallSource::Npm
    } else if is_homebrew_install(&canonical) {
        InstallSource::Homebrew
    } else {
        InstallSource::Standalone
    }
}

/// Detect the installation source from the environment
fn install_source_from_env() -> Option<InstallSource> {
    let install_source = env::var_os("BIOME_DISTRIBUTION")?;
    let install_source = InstallSource::from(install_source.to_str()?);

    Some(install_source)
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

fn binary_name_for_self_update() -> &'static str {
    if cfg!(windows) { "biome.exe" } else { "biome" }
}

fn release_target() -> Result<String, UnsupportedPlatformError> {
    release_target_for_platform(env::consts::OS, env::consts::ARCH, is_musl()).map(String::from)
}

fn release_target_for_platform(
    os: &str,
    arch: &str,
    is_musl: bool,
) -> Result<&'static str, UnsupportedPlatformError> {
    match (os, arch, is_musl) {
        ("macos", "x86_64", _) => Ok("darwin-x64"),
        ("macos", "aarch64", _) => Ok("darwin-arm64"),
        ("linux", "x86_64", true) => Ok("linux-x64-musl"),
        ("linux", "x86_64", false) => Ok("linux-x64"),
        ("linux", "aarch64", true) => Ok("linux-arm64-musl"),
        ("linux", "aarch64", false) => Ok("linux-arm64"),
        ("windows", "x86_64", _) => Ok("win32-x64"),
        ("windows", "aarch64", _) => Ok("win32-arm64"),
        (os, arch, _) => Err(UnsupportedPlatformError::new(os, arch)),
    }
}

fn release_asset_identifier(target: &str) -> String {
    if target.starts_with("win32-") {
        format!("biome-{target}.exe")
    } else {
        format!("biome-{target}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct UnsupportedPlatformError {
    os: String,
    arch: String,
}

impl UnsupportedPlatformError {
    fn new(os: &str, arch: &str) -> Self {
        Self {
            os: os.to_string(),
            arch: arch.to_string(),
        }
    }
}

impl fmt::Display for UnsupportedPlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported platform: {}-{}", self.os, self.arch)
    }
}

impl std::error::Error for UnsupportedPlatformError {}

#[cfg(target_os = "linux")]
fn is_musl() -> bool {
    self_update::get_target().contains("musl")
}

#[cfg(not(target_os = "linux"))]
fn is_musl() -> bool {
    false
}

struct ExitStatusDisplay(std::process::ExitStatus);

impl fmt::Display for ExitStatusDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.code() {
            Some(code) => write!(f, "status code {code}"),
            None => f.write_str("a terminated process"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_diagnostics::PrintDescription;

    #[test]
    fn detects_npm_install_from_path() {
        assert_eq!(
            detect_install_source(Utf8Path::new(
                "/tmp/project/node_modules/@biomejs/cli-darwin-arm64/biome"
            )),
            InstallSource::Npm
        );
    }

    #[test]
    fn detects_homebrew_install_from_path() {
        assert_eq!(
            detect_install_source(Utf8Path::new("/opt/homebrew/Cellar/biome/2.4.8/bin/biome")),
            InstallSource::Homebrew
        );
    }

    #[test]
    fn defaults_to_standalone_install_source() {
        assert_eq!(
            detect_install_source(Utf8Path::new("/usr/local/bin/biome")),
            InstallSource::Standalone
        );
    }

    #[test]
    fn reports_unknown_install_source_upgrade_error() {
        let diagnostic = unknown_install_source_upgrade_error();

        match diagnostic {
            CliDiagnostic::UpgradeError(diagnostic) => {
                assert_eq!(
                    PrintDescription(&diagnostic).to_string(),
                    "`biome upgrade` couldn't determine how this binary was installed. Upgrade Biome with the same installer or package manager you originally used.",
                );
                assert!(diagnostic.source.is_none());
            }
            other => panic!("expected unknown install source upgrade error, got {other:?}"),
        }
    }
    #[test]
    fn resolves_supported_release_targets() {
        assert_eq!(
            release_target_for_platform("macos", "x86_64", false).unwrap(),
            "darwin-x64"
        );
        assert_eq!(
            release_target_for_platform("linux", "x86_64", true).unwrap(),
            "linux-x64-musl"
        );
        assert_eq!(
            release_target_for_platform("linux", "aarch64", false).unwrap(),
            "linux-arm64"
        );
        assert_eq!(
            release_target_for_platform("windows", "aarch64", false).unwrap(),
            "win32-arm64"
        );
    }

    #[test]
    fn rejects_unsupported_release_targets() {
        let err = release_target_for_platform("freebsd", "x86_64", false).unwrap_err();

        assert_eq!(err.to_string(), "unsupported platform: freebsd-x86_64");
    }

    #[test]
    fn computes_release_asset_identifier() {
        assert_eq!(release_asset_identifier("linux-x64"), "biome-linux-x64");
        assert_eq!(release_asset_identifier("win32-x64"), "biome-win32-x64.exe");
    }

    #[test]
    fn builds_release_tag_for_version() {
        assert_eq!(release_tag_for_version("2.4.8"), "@biomejs/biome@2.4.8");
    }

    #[test]
    fn compares_versions() {
        assert!(is_version_newer("2.4.7", "2.4.8").unwrap());
        assert!(!is_version_newer("2.4.8", "2.4.8").unwrap());
        assert!(!is_version_newer("2.4.9", "2.4.8").unwrap());
    }

    #[test]
    fn rejects_invalid_versions() {
        assert!(parse_version("not-a-version").is_err());
    }
}

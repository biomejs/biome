use crate::{CliDiagnostic, CliSession, VERSION};
use biome_console::{ConsoleExt, markup};
use biome_fs::normalize_path;
use biome_package::node_semver::Version;
use camino::{Utf8Path, Utf8PathBuf};
use self_update::backends::github::Update;
use std::env;
use std::fmt;
use std::process::{Command, Stdio};

const BREW_BINARY_NAME: &str = "biome";
const GITHUB_REPO_OWNER: &str = "biomejs";
const GITHUB_REPO_NAME: &str = "biome";
const LATEST_VERSION_URL: &str = "https://biomejs.dev/api/versions/latest.txt";

/// Upgrade Biome to the latest version.
pub(crate) fn upgrade(session: CliSession) -> Result<(), CliDiagnostic> {
    let current_exe = Utf8PathBuf::from_path_buf(env::current_exe().map_err(CliDiagnostic::from)?)
        .map_err(|path| {
            CliDiagnostic::upgrade_error(format!(
                "The current Biome executable path is not valid UTF-8: {}",
                path.display()
            ))
        })?;
    let install_source = detect_install_source(&current_exe);

    match install_source {
        InstallSource::Npm => Err(CliDiagnostic::upgrade_error(
            "`biome upgrade` is not available for binaries distributed through npm-compatible package managers. Upgrade Biome with the same package manager you used to install `@biomejs/biome`.",
        )),
        InstallSource::Homebrew => upgrade_with_homebrew(session),
        InstallSource::Standalone => upgrade_standalone(session),
    }
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
                )
            } else {
                CliDiagnostic::from(err)
            }
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(CliDiagnostic::upgrade_error(format!(
            "`brew upgrade biome` exited with {}.",
            ExitStatusDisplay(status)
        )))
    }
}

/// Upgrade the standalone Biome binary by downloading the latest release from GitHub and replacing
/// the current executable.
fn upgrade_standalone(session: CliSession) -> Result<(), CliDiagnostic> {
    let latest_version = latest_available_version()?;

    if !is_version_newer(VERSION, &latest_version)? {
        session.app.console.log(markup! {
            "Biome is already up to date at version "{VERSION}"."
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
        .target(&release_target())
        .identifier(&release_asset_identifier())
        .current_version(VERSION)
        .target_version_tag(&release_tag_for_version(&latest_version))
        .show_download_progress(true)
        .no_confirm(true)
        .build()
        .map_err(|err| CliDiagnostic::upgrade_error(err.to_string()))?
        .update()
        .map_err(|err| CliDiagnostic::upgrade_error(err.to_string()))?;

    match status {
        self_update::Status::UpToDate(version) => {
            session.app.console.log(markup! {
                "Biome is already up to date at version "{version}"."
            });
        }
        self_update::Status::Updated(version) => {
            session.app.console.log(markup! {
                "Biome upgraded successfully to version "{version}"."
            });
        }
    }

    Ok(())
}

fn latest_available_version() -> Result<String, CliDiagnostic> {
    let version = reqwest::blocking::Client::builder()
        .build()
        .map_err(|err| CliDiagnostic::upgrade_error(err.to_string()))?
        .get(LATEST_VERSION_URL)
        .send()
        .map_err(|err| CliDiagnostic::upgrade_error(err.to_string()))?
        .error_for_status()
        .map_err(|err| CliDiagnostic::upgrade_error(err.to_string()))?;

    let version = version
        .text()
        .map_err(|err| CliDiagnostic::upgrade_error(err.to_string()))?;

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
        CliDiagnostic::upgrade_error(format!(
            "Failed to parse the latest Biome version `{version}`: {err}"
        ))
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum InstallSource {
    Homebrew,
    Npm,
    Standalone,
}

/// Detect the installation source
///
/// This method detects the installation source based on the path to the binary.
///
/// If the `BIOME_DISTRIBUTION` environment variable is set, it will be used to determine the
/// installation source instead of path-based detection. This allows users to override the
/// detected installation source if necessary.
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
    match env::var_os("BIOME_DISTRIBUTION")?.to_str()? {
        "npm" => Some(InstallSource::Npm),
        "homebrew" => Some(InstallSource::Homebrew),
        "standalone" => Some(InstallSource::Standalone),
        _ => None,
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

fn binary_name_for_self_update() -> &'static str {
    if cfg!(windows) { "biome.exe" } else { "biome" }
}

fn release_target() -> String {
    match (env::consts::OS, env::consts::ARCH, is_musl()) {
        ("macos", "x86_64", _) => String::from("darwin-x64"),
        ("macos", "aarch64", _) => String::from("darwin-arm64"),
        ("linux", "x86_64", true) => String::from("linux-x64-musl"),
        ("linux", "x86_64", false) => String::from("linux-x64"),
        ("linux", "aarch64", true) => String::from("linux-arm64-musl"),
        ("linux", "aarch64", false) => String::from("linux-arm64"),
        ("windows", "x86_64", _) => String::from("win32-x64"),
        ("windows", "aarch64", _) => String::from("win32-arm64"),
        (os, arch, _) => format!("{os}-{arch}"),
    }
}

fn release_asset_identifier() -> String {
    let target = release_target();

    if cfg!(windows) {
        format!("biome-{target}.exe")
    } else {
        format!("biome-{target}")
    }
}

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
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

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
    fn defaults_to_standalone_install() {
        assert_eq!(
            detect_install_source(Utf8Path::new("/usr/local/bin/biome")),
            InstallSource::Standalone
        );
    }

    #[test]
    fn distribution_env_overrides_path_detection() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            env::set_var("BIOME_DISTRIBUTION", "npm");
        }

        assert_eq!(
            detect_install_source(Utf8Path::new("/opt/homebrew/Cellar/biome/2.4.8/bin/biome")),
            InstallSource::Npm
        );

        unsafe {
            env::remove_var("BIOME_DISTRIBUTION");
        }
    }

    #[test]
    fn computes_release_asset_identifier() {
        let identifier = release_asset_identifier();
        assert!(identifier.starts_with("biome-"));
        if cfg!(windows) {
            assert!(identifier.ends_with(".exe"));
        }
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

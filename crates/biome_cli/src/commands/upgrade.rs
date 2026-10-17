use crate::install_source::InstallSource;
use crate::{CliDiagnostic, CliSession, VERSION};
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Error, StdError};
use biome_package::node_semver::Version;
use camino::Utf8PathBuf;
use self_update::backends::github::Update;
use std::env;
use std::fmt;
use std::process::{Command, Stdio};
use std::time::Duration;

const BREW_BINARY_NAME: &str = "biome";
const GITHUB_API_BASE_URL: &str = "https://api.github.com";
const GITHUB_REPO_OWNER: &str = "biomejs";
const GITHUB_REPO_NAME: &str = "biome";
const GITHUB_RELEASE_TAG_PREFIX: &str = "@biomejs/biome@";
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
    let install_source = InstallSource::from_executable(session.environment.as_ref(), &current_exe);

    match install_source {
        InstallSource::Npm => Err(CliDiagnostic::upgrade_error(
            "`biome upgrade` is not available for binaries distributed through npm-compatible package managers. Upgrade Biome with the same package manager you used to install `@biomejs/biome`.",
            None,
        )),
        InstallSource::Homebrew => upgrade_with_homebrew(session),
        InstallSource::Standalone => upgrade_standalone(session, &current_exe),
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
fn upgrade_standalone(session: CliSession, current_exe: &Utf8Path) -> Result<(), CliDiagnostic> {
    let target =
        release_target().map_err(|err| CliDiagnostic::upgrade_error(err.to_string(), None))?;
    let latest_version = latest_available_version(LATEST_VERSION_URL)?;

    if !is_version_newer(VERSION, &latest_version)? {
        session.app.console.log(markup! {
            "Biome is already up to date at version "<Emphasis>{VERSION}</Emphasis>"."
        });
        return Ok(());
    }

    session.app.console.log(markup! {
        "Downloading the latest standalone Biome release..."
    });

    let status = install_release(
        GITHUB_API_BASE_URL,
        &target,
        VERSION,
        &latest_version,
        current_exe,
    )?;

    match status {
        self_update::VersionStatus::UpToDate(version) => {
            session.app.console.log(markup! {
                "Biome is already up to date at version "<Emphasis>{version}</Emphasis>"."
            });
        }
        self_update::VersionStatus::Updated(version) => {
            session.app.console.log(markup! {
                "Biome upgraded successfully to version "<Emphasis>{version}</Emphasis>"."
            });
            session.app.console.log(markup! {
                <Info>
                    "Your editor may still be using the previous Biome version. Reload it to pick up the updated binary."
                </Info>
            });
        }
        _ => {}
    }

    Ok(())
}

/// Download the release tagged with `latest_version` through the GitHub API at `api_base_url` and
/// install its binary for `target` at `install_path`.
fn install_release(
    api_base_url: &str,
    target: &str,
    current_version: &str,
    latest_version: &str,
    install_path: &Utf8Path,
) -> Result<self_update::VersionStatus, CliDiagnostic> {
    Update::configure()
        .api_base_url(api_base_url)
        .repo_owner(GITHUB_REPO_OWNER)
        .repo_name(GITHUB_REPO_NAME)
        .tag_prefix(GITHUB_RELEASE_TAG_PREFIX)
        .bin_name(binary_name_for_self_update())
        .target(target)
        .asset_identifier(release_asset_identifier(target))
        .current_version(current_version)
        .release_tag(release_tag_for_version(latest_version))
        .show_output(false)
        .show_download_progress(true)
        .no_confirm(true)
        .bin_install_path(install_path)
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
        })
}

fn latest_available_version(url: &str) -> Result<String, CliDiagnostic> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(30)))
        .timeout_connect(Some(Duration::from_secs(10)))
        .build();

    let mut response = ureq::Agent::from(config).get(url).call().map_err(|err| {
        CliDiagnostic::upgrade_error(
            "Failed to fetch the latest Biome version.",
            Some(Error::from(StdError::from(err))),
        )
    })?;

    let version = response.body_mut().read_to_string().map_err(|err| {
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
    format!("{GITHUB_RELEASE_TAG_PREFIX}{version}")
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
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;

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

    /// Serve `respond(path, base_url)` over HTTP on a random local port and return the base URL.
    /// Paths without a response get a `404`.
    fn start_http_server(respond: fn(&str, &str) -> Option<Vec<u8>>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let server_base_url = base_url.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut request_line = String::new();
                let mut reader = BufReader::new(&stream);
                reader.read_line(&mut request_line).unwrap();
                // Drain the headers; every request in these tests is a bodiless GET.
                let mut header = String::new();
                while reader.read_line(&mut header).unwrap() > 2 {
                    header.clear();
                }

                let path = request_line.split(' ').nth(1).unwrap_or_default();
                let (status, body) = match respond(path, &server_base_url) {
                    Some(body) => ("200 OK", body),
                    None => ("404 Not Found", Vec::new()),
                };
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(&body).unwrap();
            }
        });

        base_url
    }

    #[test]
    fn installs_latest_release_from_github() {
        let base_url = start_http_server(|path, base_url| match path {
            "/latest.txt" => Some(b"2.5.15\n".to_vec()),
            "/repos/biomejs/biome/releases/tags/%40biomejs%2Fbiome%402.5.15" => {
                let target = release_target().unwrap();
                let asset = release_asset_identifier(&target);
                Some(
                    format!(
                        r#"{{
                            "tag_name": "@biomejs/biome@2.5.15",
                            "created_at": "2026-10-01T00:00:00Z",
                            "assets": [{{ "name": "{asset}", "url": "{base_url}/assets/{asset}" }}]
                        }}"#
                    )
                    .into_bytes(),
                )
            }
            path if path.starts_with("/assets/") => Some(b"new biome".to_vec()),
            _ => None,
        });

        let dir = std::env::temp_dir().join(format!("biome-upgrade-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let install_path = Utf8PathBuf::from_path_buf(dir.join("biome")).unwrap();
        std::fs::write(&install_path, "old biome").unwrap();

        let latest_version = latest_available_version(&format!("{base_url}/latest.txt")).unwrap();
        assert_eq!(latest_version, "2.5.15");

        let status = install_release(
            &base_url,
            &release_target().unwrap(),
            "2.5.0",
            &latest_version,
            &install_path,
        )
        .unwrap();

        let installed = std::fs::read_to_string(&install_path).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(
            matches!(status, self_update::VersionStatus::Updated(version) if version == "2.5.15")
        );
        assert_eq!(installed, "new biome");
    }
}

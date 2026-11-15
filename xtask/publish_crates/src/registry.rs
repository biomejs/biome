use std::{
    collections::BTreeMap,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use ratelimit::{Ratelimiter, TryWaitError};
use serde::Deserialize;

const API_BASE: &str = "https://crates.io/api/v1/crates";
const INDEX_BASE: &str = "https://index.crates.io";
const USER_AGENT: &str = "Biome crate publisher (https://github.com/biomejs/biome)";
const MAX_REQUEST_ATTEMPTS: usize = 5;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(2 * 60);
const PROPAGATION_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const PROPAGATION_POLL: Duration = Duration::from_secs(5);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CrateVersion {
    pub(crate) checksum: String,
    pub(crate) yanked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CrateMetadata {
    pub(crate) versions: BTreeMap<String, CrateVersion>,
}

pub(crate) trait Registry {
    fn crate_metadata(&mut self, name: &str) -> Result<Option<CrateMetadata>>;
    fn wait_until_indexed(&mut self, name: &str, version: &str, checksum: &str) -> Result<()>;
    fn wait_until_published(&mut self, name: &str, version: &str, checksum: &str) -> Result<()>;
}

pub(crate) struct CratesIoRegistry {
    agent: ureq::Agent,
    requests: Ratelimiter,
}

impl CratesIoRegistry {
    pub(crate) fn new() -> Result<Self> {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(30)))
            .timeout_connect(Some(Duration::from_secs(10)))
            .build();
        let requests = Ratelimiter::builder(1)
            .period(Duration::from_secs(1))
            .max_tokens(1)
            .initial_available(1)
            .build()?;
        Ok(Self {
            agent: ureq::Agent::from(config),
            requests,
        })
    }

    fn wait_for_request(&self) -> Result<()> {
        loop {
            match self.requests.try_wait() {
                Ok(()) => return Ok(()),
                Err(TryWaitError::Insufficient(duration)) => thread::sleep(duration),
                Err(TryWaitError::ExceedsCapacity) => {
                    bail!("invalid crates.io metadata rate-limit bucket")
                }
                Err(error) => bail!("unexpected metadata rate-limit error: {error}"),
            }
        }
    }

    fn request<T, F>(&self, url: &str, mut parse: F) -> Result<Option<T>>
    where
        F: FnMut(&str) -> Result<T>,
    {
        let mut last_error = None;
        let deadline = Instant::now() + REQUEST_TIMEOUT;
        for attempt in 0..MAX_REQUEST_ATTEMPTS {
            if Instant::now() >= deadline {
                break;
            }
            self.wait_for_request()?;
            let mut delay = retry_delay(attempt);
            match self.agent.get(url).header("User-Agent", USER_AGENT).call() {
                Ok(mut response) => {
                    let status = response.status().as_u16();
                    if status == 404 {
                        return Ok(None);
                    }
                    if status == 429
                        && let Some(retry_after) = response
                            .headers()
                            .get("retry-after")
                            .and_then(|value| value.to_str().ok())
                            .and_then(parse_retry_after)
                    {
                        delay = retry_after;
                    }
                    match response.body_mut().read_to_string() {
                        Ok(body) => {
                            if (200..300).contains(&status) {
                                match parse(&body)
                                    .with_context(|| format!("failed to parse response from {url}"))
                                {
                                    Ok(value) => return Ok(Some(value)),
                                    Err(error) => last_error = Some(error),
                                }
                            } else {
                                let error = anyhow::anyhow!(
                                    "request to {url} failed with HTTP {status}: {}",
                                    body.trim()
                                );
                                if !is_transient_status(status) {
                                    return Err(error);
                                }
                                last_error = Some(error);
                            }
                        }
                        Err(error) => {
                            last_error = Some(anyhow::Error::new(error).context(format!(
                                "failed to read response from {url} after HTTP {status}"
                            )));
                        }
                    }
                }
                Err(error) => {
                    last_error =
                        Some(anyhow::Error::new(error).context(format!("request to {url} failed")));
                }
            }
            if attempt + 1 < MAX_REQUEST_ATTEMPTS
                && Instant::now()
                    .checked_add(delay)
                    .is_some_and(|next_attempt| next_attempt < deadline)
            {
                thread::sleep(delay);
            } else {
                break;
            }
        }
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("request to {url} failed")))
    }

    fn wait_for_sparse_index(&self, name: &str, version: &str, checksum: &str) -> Result<()> {
        let url = format!("{INDEX_BASE}/{}", sparse_index_path(name)?);
        let deadline = Instant::now() + PROPAGATION_TIMEOUT;
        loop {
            if let Some(entries) = self.request(&url, |body| parse_sparse_index(name, body))? {
                for entry in entries {
                    if entry.version == version {
                        if entry.checksum != checksum {
                            bail!(
                                "sparse index checksum mismatch for {name} {version}: found {}, expected {checksum}",
                                entry.checksum
                            );
                        }
                        if entry.yanked {
                            bail!("{name} {version} is yanked in the sparse index");
                        }
                        return Ok(());
                    }
                }
            }
            if Instant::now() >= deadline {
                bail!("timed out waiting for {name} {version} in the sparse index");
            }
            thread::sleep(PROPAGATION_POLL);
        }
    }
}

impl Registry for CratesIoRegistry {
    fn crate_metadata(&mut self, name: &str) -> Result<Option<CrateMetadata>> {
        let url = format!("{API_BASE}/{name}");
        self.request(&url, |body| parse_api_metadata(name, body))
    }

    fn wait_until_indexed(&mut self, name: &str, version: &str, checksum: &str) -> Result<()> {
        self.wait_for_sparse_index(name, version, checksum)
    }

    fn wait_until_published(&mut self, name: &str, version: &str, checksum: &str) -> Result<()> {
        let deadline = Instant::now() + PROPAGATION_TIMEOUT;
        loop {
            if let Some(metadata) = self.crate_metadata(name)?
                && let Some(remote_version) = metadata.versions.get(version)
            {
                if remote_version.checksum != checksum {
                    bail!(
                        "crates.io checksum mismatch for {name} {version}: found {}, expected {checksum}",
                        remote_version.checksum
                    );
                }
                if remote_version.yanked {
                    bail!("{name} {version} is yanked on crates.io");
                }
                break;
            }
            if Instant::now() >= deadline {
                bail!("timed out waiting for {name} {version} on crates.io");
            }
            thread::sleep(PROPAGATION_POLL);
        }
        self.wait_for_sparse_index(name, version, checksum)
    }
}

#[derive(Deserialize)]
struct ApiCrateResponse {
    versions: Vec<ApiVersion>,
}

#[derive(Deserialize)]
struct ApiVersion {
    #[serde(rename = "num")]
    number: String,
    checksum: String,
    yanked: bool,
}

#[derive(Deserialize)]
struct IndexVersion {
    #[serde(rename = "vers")]
    version: String,
    #[serde(rename = "cksum")]
    checksum: String,
    yanked: bool,
}

fn parse_api_metadata(name: &str, body: &str) -> Result<CrateMetadata> {
    let response: ApiCrateResponse = serde_json::from_str(body)
        .with_context(|| format!("invalid crates.io metadata for {name}"))?;
    Ok(CrateMetadata {
        versions: response
            .versions
            .into_iter()
            .map(|version| {
                (
                    version.number,
                    CrateVersion {
                        checksum: version.checksum,
                        yanked: version.yanked,
                    },
                )
            })
            .collect(),
    })
}

fn parse_sparse_index(name: &str, body: &str) -> Result<Vec<IndexVersion>> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .with_context(|| format!("invalid sparse index entry for {name}"))
        })
        .collect()
}

fn sparse_index_path(name: &str) -> Result<String> {
    if name.is_empty() || !name.is_ascii() {
        bail!("invalid crates.io crate name {name:?}");
    }
    let mut name = name.to_string();
    name.make_ascii_lowercase();
    match name.len() {
        1 => Ok(format!("1/{name}")),
        2 => Ok(format!("2/{name}")),
        3 => Ok(format!(
            "3/{}/{name}",
            name.get(..1)
                .context("invalid three-character crate name")?
        )),
        _ => Ok(format!(
            "{}/{}/{name}",
            name.get(..2).context("invalid crate-name prefix")?,
            name.get(2..4).context("invalid crate-name prefix")?
        )),
    }
}

fn is_transient_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

fn parse_retry_after(value: &str) -> Option<Duration> {
    value.parse().ok().map(Duration::from_secs)
}

fn retry_delay(attempt: usize) -> Duration {
    Duration::from_secs(1 << attempt.min(5))
}

#[cfg(test)]
mod tests {
    use std::{
        io::{BufRead, BufReader, Write},
        net::{SocketAddr, TcpListener},
        thread,
        time::Duration,
    };

    use super::{
        CratesIoRegistry, is_transient_status, parse_api_metadata, parse_retry_after,
        parse_sparse_index, sparse_index_path,
    };

    fn serve_responses(
        responses: Vec<(usize, &'static str)>,
    ) -> (SocketAddr, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for (content_length, body) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line == "\r\n" || line.is_empty() {
                        break;
                    }
                }
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\nConnection: close\r\n\r\n{body}"
                )
                .unwrap();
            }
        });
        (address, server)
    }

    #[test]
    fn classifies_transient_responses() {
        assert!(is_transient_status(429));
        assert!(is_transient_status(503));
        assert!(!is_transient_status(400));
        assert_eq!(parse_retry_after("120"), Some(Duration::from_secs(120)));
        assert_eq!(parse_retry_after("invalid"), None);
    }

    #[test]
    fn retries_incomplete_response_bodies() {
        let (address, server) = serve_responses(vec![(10, "short"), (8, "complete")]);
        let registry = CratesIoRegistry::new().unwrap();

        let body = registry
            .request(&format!("http://{address}"), |body| Ok(body.to_string()))
            .unwrap();

        assert_eq!(body.as_deref(), Some("complete"));
        server.join().unwrap();
    }

    #[test]
    fn retries_malformed_api_metadata() {
        let valid = r#"{"versions":[{"num":"1.0.0","checksum":"abc","yanked":false}]}"#;
        let (address, server) = serve_responses(vec![(1, "{"), (valid.len(), valid)]);
        let registry = CratesIoRegistry::new().unwrap();

        let metadata = registry
            .request(&format!("http://{address}"), |body| {
                parse_api_metadata("probe", body)
            })
            .unwrap()
            .unwrap();

        assert_eq!(metadata.versions["1.0.0"].checksum, "abc");
        server.join().unwrap();
    }

    #[test]
    fn retries_malformed_sparse_index_entries() {
        let valid = r#"{"vers":"1.0.0","cksum":"abc","yanked":false}
"#;
        let (address, server) = serve_responses(vec![(2, "{\n"), (valid.len(), valid)]);
        let registry = CratesIoRegistry::new().unwrap();

        let entries = registry
            .request(&format!("http://{address}"), |body| {
                parse_sparse_index("probe", body)
            })
            .unwrap()
            .unwrap();

        assert_eq!(entries[0].version, "1.0.0");
        assert_eq!(entries[0].checksum, "abc");
        server.join().unwrap();
    }

    #[test]
    fn builds_sparse_index_paths() {
        assert_eq!(sparse_index_path("a").unwrap(), "1/a");
        assert_eq!(sparse_index_path("ab").unwrap(), "2/ab");
        assert_eq!(sparse_index_path("abc").unwrap(), "3/a/abc");
        assert_eq!(sparse_index_path("Serde").unwrap(), "se/rd/serde");
        assert_eq!(
            sparse_index_path("biome_text_size").unwrap(),
            "bi/om/biome_text_size"
        );
        assert!(sparse_index_path("").is_err());
        assert!(sparse_index_path("é").is_err());
    }
}

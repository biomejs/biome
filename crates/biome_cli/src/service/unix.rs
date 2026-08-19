use crate::logging::LogOptions;
use biome_lsp::{ServerConnection, ServerFactory};
use biome_package::node_semver::Version;
use biome_service::WatcherOptions;
use camino::{Utf8Path, Utf8PathBuf};
use std::{
    convert::Infallible,
    env, fs,
    io::{self, ErrorKind},
    os::unix::fs::FileTypeExt,
    time::Duration,
};
use tokio::{
    io::Interest,
    net::{
        UnixListener, UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
    process::{Child, Command},
    time,
};
use tracing::{Instrument, debug, info, warn};

/// Returns the filesystem path of the global socket used to communicate with
/// the server daemon
fn get_socket_name() -> Utf8PathBuf {
    biome_fs::ensure_cache_dir().join(format!("biome-socket-{}", biome_configuration::VERSION))
}

pub(crate) fn enumerate_pipes() -> io::Result<impl Iterator<Item = (String, Utf8PathBuf)>> {
    enumerate_pipes_in(biome_fs::ensure_cache_dir())
}

fn enumerate_pipes_in(
    path: Utf8PathBuf,
) -> io::Result<impl Iterator<Item = (String, Utf8PathBuf)>> {
    fs::read_dir(&path).map(|iter| {
        iter.filter_map(|entry| {
            let entry = Utf8PathBuf::from_path_buf(entry.ok()?.path()).ok()?;
            let file_name = entry.file_name()?;

            let version = file_name.strip_prefix("biome-socket")?;
            if version.is_empty() {
                Some((String::new(), entry))
            } else {
                Some((version.strip_prefix('-')?.to_string(), entry))
            }
        })
    })
}

async fn purge_old_sockets_in(cache_dir: &Utf8Path, current_version: &str) {
    let Ok(current_version) = current_version.parse::<Version>() else {
        warn!("Skipping stale socket cleanup because the current version is invalid");
        return;
    };

    let sockets = match enumerate_pipes_in(cache_dir.to_path_buf()) {
        Ok(sockets) => sockets.collect::<Vec<_>>(),
        Err(err) => {
            warn!("Could not enumerate stale daemon sockets: {err}");
            return;
        }
    };

    for (version, path) in sockets {
        let Ok(version) = version.parse::<Version>() else {
            continue;
        };

        if version >= current_version {
            continue;
        }

        let file_type = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata.file_type(),
            Err(err) if err.kind() == ErrorKind::NotFound => continue,
            Err(err) => {
                warn!("Could not inspect stale daemon socket {path}: {err}");
                continue;
            }
        };

        if !file_type.is_socket() {
            continue;
        }

        match time::timeout(Duration::from_millis(100), UnixStream::connect(&path)).await {
            Ok(Ok(_)) => {}
            Ok(Err(err)) if err.kind() == ErrorKind::ConnectionRefused => {
                if let Err(err) = fs::remove_file(&path)
                    && err.kind() != ErrorKind::NotFound
                {
                    warn!("Could not remove stale daemon socket {path}: {err}");
                } else {
                    info!("Removed stale socket at {path}");
                }
            }
            Ok(Err(err)) if err.kind() == ErrorKind::NotFound => {}
            Ok(Err(err)) => warn!("Could not connect to stale daemon socket {path}: {err}"),
            Err(_) => warn!("Timed out connecting to stale daemon socket {path}"),
        }
    }
}

async fn purge_old_sockets() {
    purge_old_sockets_in(&biome_fs::ensure_cache_dir(), biome_configuration::VERSION).await;
}

/// Try to connect to the global socket and wait for the connection to become ready
async fn try_connect() -> io::Result<UnixStream> {
    let socket_name = get_socket_name();
    info!("Trying to connect to socket {}", socket_name.as_str());
    let stream = UnixStream::connect(socket_name).await?;
    stream
        .ready(Interest::READABLE | Interest::WRITABLE)
        .await?;
    Ok(stream)
}

/// Spawn the daemon server process in the background
fn spawn_daemon(
    stop_on_disconnect: bool,
    watcher_configuration: WatcherOptions,
    log_options: LogOptions,
) -> io::Result<Child> {
    let binary = env::current_exe()?;

    let mut cmd = Command::new(binary);
    debug!("command {:?}", &cmd);
    cmd.arg("__run_server");

    cmd.arg(format!(
        "--watcher-kind={}",
        watcher_configuration.watcher_kind
    ));
    cmd.arg(format!(
        "--watcher-polling-interval={}",
        watcher_configuration.polling_interval
    ));

    if stop_on_disconnect {
        cmd.arg("--stop-on-disconnect");
    }
    cmd.arg(format!("--log-path={}", log_options.log_path.as_str()));

    cmd.arg(format!(
        "--log-prefix-name={}",
        log_options.log_prefix_name.as_str()
    ));

    // Create a new session for the process and make it the leader, this will
    // ensures that the child process is fully detached from its parent and will
    // continue running in the background even after the parent process exits
    //
    // SAFETY: This closure runs in the forked child process before it starts
    // executing, this is a highly unsafe environment because the process isn't
    // running yet so seemingly innocuous operation like allocating memory may
    // hang indefinitely.
    // The only thing we do here is issuing a syscall, which is safe to do in
    // this state but still "unsafe" in Rust semantics because it's technically
    // mutating the shared global state of the process
    unsafe {
        cmd.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }

    let child = cmd.spawn()?;
    Ok(child)
}

/// Open a connection to the daemon server process, returning [None] if the
/// server is not running
pub(crate) async fn open_socket() -> io::Result<Option<(OwnedReadHalf, OwnedWriteHalf)>> {
    match try_connect().await {
        Ok(socket) => Ok(Some(socket.into_split())),
        Err(err)
            // The OS will return `ConnectionRefused` if the socket file exists
            // but no server process is listening on it
            if matches!(
                err.kind(),
                ErrorKind::NotFound | ErrorKind::ConnectionRefused
            ) =>
        {
            Ok(None)
        }
        Err(err) => Err(err),
    }
}

/// Ensure the server daemon is running and ready to receive connections
///
/// Returns false if the daemon process was already running or true if it had
/// to be started
pub(crate) async fn ensure_daemon(
    stop_on_disconnect: bool,
    watcher_configuration: WatcherOptions,
    log_options: LogOptions,
) -> io::Result<bool> {
    let mut current_child: Option<Child> = None;
    let mut last_error = None;

    // Try to initialize the connection a few times
    for _ in 0..10 {
        // Try to open a connection on the global socket
        match try_connect().await {
            // The connection is open and ready
            Ok(_) => {
                return Ok(current_child.is_some());
            }

            // There's no process listening on the global socket
            Err(err)
                if matches!(
                    err.kind(),
                    ErrorKind::NotFound | ErrorKind::ConnectionRefused
                ) =>
            {
                last_error = Some(err);

                if let Some(current_child) = &mut current_child {
                    // If we have a handle to the daemon process, wait for a few
                    // milliseconds for it to exit, or retry the connection
                    tokio::select! {
                        result = current_child.wait() => {
                            let _status = result?;
                            return Err(io::Error::new(
                                io::ErrorKind::ConnectionReset,
                                "the server process exited before the connection could be established",
                            ));
                        }
                        _ = time::sleep(Duration::from_millis(50)) => {}
                    }
                } else {
                    // Spawn the daemon process and wait a few milliseconds for
                    // it to become ready then retry the connection
                    current_child = Some(spawn_daemon(
                        stop_on_disconnect,
                        watcher_configuration.clone(),
                        log_options.clone(),
                    )?);
                    time::sleep(Duration::from_millis(50)).await;
                }
            }

            Err(err) => return Err(err),
        }
    }

    // If the connection couldn't be opened after 10 tries fail with the last
    // error message from the OS, or a generic error message otherwise
    Err(last_error.unwrap_or_else(|| io::Error::other("could not connect to the daemon socket")))
}

/// Ensure the server daemon is running and ready to receive connections and
/// print the global socket name in the standard output
pub(crate) async fn print_socket() -> io::Result<()> {
    ensure_daemon(true, WatcherOptions::default(), LogOptions::default()).await?;
    println!("{}", get_socket_name().as_str());
    Ok(())
}

/// Start listening on the global socket and accepting connections with the
/// provided [ServerFactory]
pub(crate) async fn run_daemon(factory: ServerFactory) -> io::Result<Infallible> {
    let path = get_socket_name();

    info!("Trying to connect to socket {path}");

    purge_old_sockets().await;

    // Try to remove the socket file if it already exists
    if path.exists() {
        info!("Remove socket {path}");
        fs::remove_file(&path)?;
    }

    let listener = UnixListener::bind(path)?;

    loop {
        let (stream, _) = listener.accept().await?;
        let connection = factory.create();
        let span = tracing::trace_span!("run_server");
        tokio::spawn(run_server(connection, stream).instrument(span.or_current()));
    }
}

/// Async task driving a single client connection
async fn run_server(connection: ServerConnection, stream: UnixStream) {
    let (read, write) = stream.into_split();
    connection.accept(read, write).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        os::unix::fs::symlink,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static NEXT_TEST_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TestDirectory(Utf8PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let path = env::temp_dir().join(format!(
                "biome-socket-test-{}-{}",
                std::process::id(),
                NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(Utf8PathBuf::from_path_buf(path).unwrap())
        }

        fn socket(&self, version: &str) -> Utf8PathBuf {
            self.0.join(format!("biome-socket-{version}"))
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).ok();
        }
    }

    fn create_stale_socket(path: &Utf8Path) {
        drop(UnixListener::bind(path).unwrap());
    }

    #[tokio::test]
    async fn removes_stale_older_sockets() {
        let directory = TestDirectory::new();
        let stable = directory.socket("1.9.0");
        let prerelease = directory.socket("2.0.0-beta.1");
        create_stale_socket(&stable);
        create_stale_socket(&prerelease);

        purge_old_sockets_in(&directory.0, "2.0.0").await;

        assert!(!stable.exists());
        assert!(!prerelease.exists());
    }

    #[tokio::test]
    async fn preserves_active_older_sockets() {
        let directory = TestDirectory::new();
        let socket = directory.socket("1.9.0");
        let _listener = UnixListener::bind(&socket).unwrap();

        purge_old_sockets_in(&directory.0, "2.0.0").await;

        assert!(socket.exists());
    }

    #[tokio::test]
    async fn preserves_equal_and_newer_sockets() {
        let directory = TestDirectory::new();
        let equal = directory.socket("2.0.0");
        let newer = directory.socket("2.0.1");
        let same_precedence = directory.socket("2.0.0+build.1");
        create_stale_socket(&equal);
        create_stale_socket(&newer);
        create_stale_socket(&same_precedence);

        purge_old_sockets_in(&directory.0, "2.0.0").await;

        assert!(equal.exists());
        assert!(newer.exists());
        assert!(same_precedence.exists());
    }

    #[tokio::test]
    async fn preserves_unparseable_and_non_socket_entries() {
        let directory = TestDirectory::new();
        let invalid = directory.socket("not-a-version");
        let legacy = directory.0.join("biome-socket");
        let regular_file = directory.socket("1.9.0");
        let target = directory.0.join("target");
        let symlink_path = directory.socket("1.8.0");
        create_stale_socket(&invalid);
        create_stale_socket(&legacy);
        fs::write(&regular_file, "not a socket").unwrap();
        fs::write(&target, "not a socket").unwrap();
        symlink(&target, &symlink_path).unwrap();

        purge_old_sockets_in(&directory.0, "2.0.0").await;

        assert!(invalid.exists());
        assert!(legacy.exists());
        assert!(regular_file.exists());
        assert!(symlink_path.exists());
    }

    #[tokio::test]
    async fn skips_cleanup_for_an_unparseable_current_version() {
        let directory = TestDirectory::new();
        let socket = directory.socket("1.9.0");
        create_stale_socket(&socket);

        purge_old_sockets_in(&directory.0, "not-a-version").await;

        assert!(socket.exists());
    }
}

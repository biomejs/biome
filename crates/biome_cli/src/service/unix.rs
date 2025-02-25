use biome_lsp::{ServerConnection, ServerFactory};
use camino::Utf8PathBuf;
use std::{
    convert::Infallible,
    env, fs,
    io::{self, ErrorKind},
    time::Duration,
};
use tokio::{
    io::Interest,
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf},
        UnixListener, UnixStream,
    },
    process::{Child, Command},
    time,
};
use tracing::{debug, info, Instrument};

/// Returns the filesystem path of the global socket used to communicate with
/// the server daemon
fn get_socket_name() -> Utf8PathBuf {
    biome_fs::ensure_cache_dir().join(format!("biome-socket-{}", biome_configuration::VERSION))
}

pub(crate) fn enumerate_pipes() -> io::Result<impl Iterator<Item = (String, Utf8PathBuf)>> {
    fs::read_dir(biome_fs::ensure_cache_dir()).map(|iter| {
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
    config_path: Option<Utf8PathBuf>,
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: Option<String>,
) -> io::Result<Child> {
    let binary = env::current_exe()?;

    let mut cmd = Command::new(binary);
    debug!("command {:?}", &cmd);
    cmd.arg("__run_server");

    if stop_on_disconnect {
        cmd.arg("--stop-on-disconnect");
    }
    if let Some(config_path) = config_path {
        cmd.arg(format!("--config-path={}", config_path));
    }
    if let Some(log_path) = log_path {
        cmd.arg(format!("--log-path={}", log_path));
    }

    if let Some(log_file_name_prefix) = log_file_name_prefix {
        cmd.arg(format!("--log-prefix-name={log_file_name_prefix}"));
    }

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
    config_path: Option<Utf8PathBuf>,
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: Option<String>,
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
                        config_path.clone(),
                        log_path.clone(),
                        log_file_name_prefix.clone(),
                    )?);
                    time::sleep(Duration::from_millis(50)).await;
                }
            }

            Err(err) => return Err(err),
        }
    }

    // If the connection couldn't be opened after 10 tries fail with the last
    // error message from the OS, or a generic error message otherwise
    Err(last_error.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "could not connect to the daemon socket",
        )
    }))
}

/// Ensure the server daemon is running and ready to receive connections and
/// print the global socket name in the standard output
pub(crate) async fn print_socket() -> io::Result<()> {
    ensure_daemon(true, None, None, None).await?;
    println!("{}", get_socket_name().as_str());
    Ok(())
}

/// Start listening on the global socket and accepting connections with the
/// provided [ServerFactory]
pub(crate) async fn run_daemon(
    factory: ServerFactory,
    config_path: Option<Utf8PathBuf>,
) -> io::Result<Infallible> {
    let path = get_socket_name();

    info!("Trying to connect to socket {}", path.as_str());

    // Try to remove the socket file if it already exists
    if path.exists() {
        info!("Remove socket folder {}", path.as_str());
        fs::remove_file(&path)?;
    }

    let listener = UnixListener::bind(path)?;

    loop {
        let (stream, _) = listener.accept().await?;
        let connection = factory.create(config_path.clone());
        let span = tracing::trace_span!("run_server");
        tokio::spawn(run_server(connection, stream).instrument(span.or_current()));
    }
}

/// Async task driving a single client connection
async fn run_server(connection: ServerConnection, stream: UnixStream) {
    let (read, write) = stream.into_split();
    connection.accept(read, write).await;
}

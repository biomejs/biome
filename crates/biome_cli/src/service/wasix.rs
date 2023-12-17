use std::{
    convert::Infallible,
    env, fs,
    io::{self, ErrorKind},
    path::PathBuf,
    time::Duration,
};

use biome_lsp::{ServerConnection, ServerFactory};
use tokio::{
    net::UnixStream,
    process::{Child, Command},
    time,
};
use tracing::debug;

#[cfg(not(target_os = "wasi"))]
use tokio::net::{
    unix::{OwnedReadHalf, OwnedWriteHalf},
    UnixListener,
};

/// Returns the filesystem path of the global socket used to communicate with
/// the server daemon
fn get_socket_name() -> PathBuf {
    env::temp_dir().join(format!("biome-socket-{}", biome_service::VERSION))
}

pub(crate) fn enumerate_pipes() -> io::Result<impl Iterator<Item = String>> {
    fs::read_dir(env::temp_dir()).map(|iter| {
        iter.filter_map(|entry| {
            let entry = entry.ok()?.path();
            let file_name = entry.file_name()?;
            let file_name = file_name.to_str()?;

            let version = file_name.strip_prefix("biome-socket")?;
            if version.is_empty() {
                Some(String::new())
            } else {
                Some(version.strip_prefix('-')?.to_string())
            }
        })
    })
}

/// Try to connect to the global socket and wait for the connection to become ready
async fn try_connect() -> io::Result<()> {
    Ok(())
}

/// Spawn the daemon server process in the background
fn spawn_daemon(stop_on_disconnect: bool, config_path: Option<PathBuf>) -> io::Result<Child> {
    let binary = env::current_exe()?;

    let mut cmd = Command::new(binary);
    debug!("command {:?}", &cmd);
    cmd.arg("__run_server");

    if stop_on_disconnect {
        cmd.arg("--stop-on-disconnect");
    }
    if let Some(config_path) = config_path {
        cmd.arg(format!("--config-path={}", config_path.display()));
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
    // unsafe {
    //     cmd.pre_exec(|| {
    //         libc::setsid();
    //         Ok(())
    //     });
    // }

    let child = cmd.spawn()?;
    Ok(child)
}

/// Open a connection to the daemon server process, returning [None] if the
/// server is not running
pub(crate) async fn open_socket() -> io::Result<Option<((), ())>> {
    io::Result::Ok(None::<((), ())>)
}

/// Ensure the server daemon is running and ready to receive connections
///
/// Returns false if the daemon process was already running or true if it had
/// to be started
pub(crate) async fn ensure_daemon(
    stop_on_disconnect: bool,
    config_path: Option<PathBuf>,
) -> io::Result<bool> {
    Ok(false)
}

/// Ensure the server daemon is running and ready to receive connections and
/// print the global socket name in the standard output
pub(crate) async fn print_socket() -> io::Result<()> {
    Ok(())
}

/// Start listening on the global socket and accepting connections with the
/// provided [ServerFactory]
pub(crate) async fn run_daemon(
    factory: ServerFactory,
    config_path: Option<PathBuf>,
) -> io::Result<Infallible> {
    loop {}
}

/// Async task driving a single client connection
async fn run_server(connection: ServerConnection, stream: UnixStream) {}

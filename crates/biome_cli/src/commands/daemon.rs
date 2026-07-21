use crate::logging::LogOptions;
use crate::{
    CliDiagnostic, CliSession, open_transport,
    service::{self, ensure_daemon, open_socket, run_daemon},
};
use biome_console::{ConsoleExt, markup};
use biome_fs::OsFileSystem;
use biome_lsp::ServerFactory;
use biome_service::{
    TransportError, Watcher, WatcherOptions, WorkspaceError, workspace::WorkspaceClient,
};
use camino::Utf8PathBuf;
use std::{env, fs, process};
use tokio::io;
use tokio::runtime::Runtime;
use tracing::subscriber::Interest;
use tracing::{Instrument, Metadata, debug_span, metadata::LevelFilter};
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    Layer,
    layer::{Context, Filter},
    prelude::*,
    registry,
};
use tracing_tree::HierarchicalLayer;

pub(crate) fn start(
    session: CliSession,
    watcher_options: WatcherOptions,
    log_options: LogOptions,
) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    let did_spawn = rt.block_on(ensure_daemon(false, watcher_options, log_options))?;

    if did_spawn {
        session.app.console.log(markup! {
            "The Biome server was successfully started"
        });
    } else {
        session.app.console.log(markup! {
            "The Biome server was already running"
        });
    }

    Ok(())
}

pub(crate) fn stop(session: CliSession) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;

    if let Some(transport) = open_transport(rt)? {
        let client = WorkspaceClient::new(transport, Box::new(OsFileSystem::default()))?;
        match client.shutdown() {
            // The `ChannelClosed` error is expected since the server can
            // shutdown before sending a response
            Ok(()) | Err(WorkspaceError::TransportError(TransportError::ChannelClosed)) => {}
            Err(err) => return Err(CliDiagnostic::from(err)),
        };

        session.app.console.log(markup! {
            "The Biome server was successfully stopped"
        });
    } else {
        session.app.console.log(markup! {
            "The Biome server was not running"
        });
    }

    Ok(())
}

pub(crate) fn run_server(
    stop_on_disconnect: bool,
    watcher_options: WatcherOptions,
    log_options: LogOptions,
) -> Result<(), CliDiagnostic> {
    setup_tracing_subscriber(
        log_options.log_path.clone(),
        log_options.log_prefix_name.clone(),
    );

    let span = debug_span!(
        "Running Server",
        pid = std::process::id(),
        log_path = &log_options.log_path.as_str(),
        log_file_name_prefix = &log_options.log_prefix_name.as_str(),
    );

    let (watcher, instruction_channel) = Watcher::new(watcher_options)?;

    let rt = Runtime::new()?;
    let factory = ServerFactory::new(stop_on_disconnect, instruction_channel.sender.clone());
    let cancellation = factory.cancellation();

    let workspace = factory.workspace();
    let db_state = factory.db_state();
    rt.spawn_blocking(move || {
        workspace.start_watcher(&db_state, watcher);
    });

    rt.block_on(async move {
        tokio::select! {
            res = run_daemon(factory).instrument(span) => {
                match res {
                    Ok(never) => match never {},
                    Err(err) => Err(err.into()),
                }
            }
            _ = cancellation.notified() => {
                tracing::info!("Received shutdown signal");
                // Do a forced exit, as there should be no need to wait for
                // other tasks to finish in the daemon.
                process::exit(0);
            }
        }
    })
}

pub(crate) fn print_socket() -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    rt.block_on(service::print_socket())?;
    Ok(())
}

pub(crate) fn lsp_proxy(
    watcher_options: WatcherOptions,
    log_options: LogOptions,
) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    rt.block_on(start_lsp_proxy(watcher_options, log_options))?;

    Ok(())
}

async fn forward_lsp<I, O, R, W>(
    mut input: I,
    mut output: O,
    mut socket_read: R,
    mut socket_write: W,
) where
    I: io::AsyncRead + Unpin,
    O: io::AsyncWrite + Unpin,
    R: io::AsyncRead + Unpin,
    W: io::AsyncWrite + Unpin,
{
    tokio::select! {
        result = io::copy(&mut input, &mut socket_write) => {
            if result.is_ok() {
                let _ = io::AsyncWriteExt::flush(&mut socket_write).await;
            }
        }
        result = io::copy(&mut socket_read, &mut output) => {
            if result.is_ok() {
                let _ = io::AsyncWriteExt::flush(&mut output).await;
            }
        }
    }
}

/// Start a proxy process.
/// Receives a process via `stdin` and then copy the content to the LSP socket.
/// Copy to the process on `stdout` when the LSP responds to a message
async fn start_lsp_proxy(
    watcher_options: WatcherOptions,
    log_options: LogOptions,
) -> Result<(), CliDiagnostic> {
    ensure_daemon(true, watcher_options, log_options).await?;

    match open_socket().await? {
        Some((owned_read_half, owned_write_half)) => {
            forward_lsp(io::stdin(), io::stdout(), owned_read_half, owned_write_half).await;

            // Tokio standard I/O uses blocking reads that cannot be cancelled.
            // Exit so a pending read cannot keep a disconnected proxy alive.
            process::exit(0);
        }
        None => Ok(()),
    }
}

pub(crate) fn read_most_recent_log_file(
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: String,
) -> io::Result<Option<String>> {
    let biome_log_path = log_path.unwrap_or(default_biome_log_path());

    let most_recent = fs::read_dir(biome_log_path)?
        .flatten()
        .filter(|file| file.file_type().is_ok_and(|ty| ty.is_file()))
        .filter_map(|file| {
            match file
                .file_name()
                .to_str()?
                .split_once(log_file_name_prefix.as_str())
            {
                Some((_, date_part)) if date_part.split('-').count() == 4 => Some(file.path()),
                _ => None,
            }
        })
        .max();

    match most_recent {
        Some(file) => Ok(Some(fs::read_to_string(file)?)),
        None => Ok(None),
    }
}

/// Set up the [tracing]-based logging system for the server
/// The events received by the subscriber are filtered at the `info` level,
/// then printed using the [HierarchicalLayer] layer, and the resulting text
/// is written to log files rotated on a hourly basis (in
/// `biome-logs/server.log.yyyy-MM-dd-HH` files inside the system temporary
/// directory)
fn setup_tracing_subscriber(log_path: Utf8PathBuf, log_file_name_prefix: String) {
    fs::create_dir_all(&log_path).expect("Failed to create log directory for the daemon.");

    let appender_builder = tracing_appender::rolling::RollingFileAppender::builder();
    let file_appender = appender_builder
        .filename_prefix(log_file_name_prefix)
        .max_log_files(7)
        .rotation(Rotation::HOURLY)
        .build(log_path)
        .expect("Failed to start the logger for the daemon.");

    registry()
        .with(
            HierarchicalLayer::default()
                .with_indent_lines(true)
                .with_indent_amount(2)
                .with_bracketed_fields(true)
                .with_targets(true)
                .with_ansi(false)
                .with_writer(file_appender)
                .with_filter(LoggingFilter),
        )
        .init();
}

pub fn default_biome_log_path() -> Utf8PathBuf {
    match env::var_os("BIOME_LOG_PATH") {
        Some(directory) => Utf8PathBuf::from(directory.as_os_str().to_str().unwrap()),
        None => biome_fs::ensure_cache_dir().join("biome-logs"),
    }
}

/// Tracing filter enabling:
/// - All spans and events at level info or higher
/// - All spans and events at level debug in crates whose name starts with `biome`
struct LoggingFilter;

/// Tracing filter used for spans emitted by `biome*` crates
const SELF_FILTER: LevelFilter = if cfg!(debug_assertions) {
    LevelFilter::TRACE
} else {
    LevelFilter::DEBUG
};

impl LoggingFilter {
    fn is_enabled(&self, meta: &Metadata<'_>) -> bool {
        let filter = if meta.target().starts_with("biome") {
            SELF_FILTER
        } else {
            return false;
        };

        meta.level() <= &filter
    }
}

impl<S> Filter<S> for LoggingFilter {
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        self.is_enabled(meta)
    }

    fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest {
        if self.is_enabled(meta) {
            Interest::always()
        } else {
            Interest::never()
        }
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(SELF_FILTER)
    }
}

#[cfg(test)]
mod tests {
    use super::forward_lsp;
    use std::time::Duration;
    use tokio::io::{duplex, empty, sink, split};
    use tokio::time::timeout;

    #[tokio::test]
    async fn forwarding_stops_when_the_daemon_disconnects() {
        let (input, _input_writer) = duplex(64);
        let (socket, remote) = duplex(64);
        let (socket_read, socket_write) = split(socket);
        drop(remote);

        timeout(
            Duration::from_secs(1),
            forward_lsp(input, sink(), socket_read, socket_write),
        )
        .await
        .expect("forwarding should stop after the daemon disconnects");
    }

    #[tokio::test]
    async fn forwarding_stops_when_the_editor_disconnects() {
        let (socket, _remote) = duplex(64);
        let (socket_read, socket_write) = split(socket);

        timeout(
            Duration::from_secs(1),
            forward_lsp(empty(), sink(), socket_read, socket_write),
        )
        .await
        .expect("forwarding should stop after the editor disconnects");
    }
}

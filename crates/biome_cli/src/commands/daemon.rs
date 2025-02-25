use crate::{
    open_transport,
    service::{self, ensure_daemon, open_socket, run_daemon},
    CliDiagnostic, CliSession,
};
use biome_console::{markup, ConsoleExt};
use biome_fs::OsFileSystem;
use biome_lsp::ServerFactory;
use biome_service::{workspace::WorkspaceClient, TransportError, WorkspaceError};
use camino::{Utf8Path, Utf8PathBuf};
use std::{env, fs};
use tokio::io;
use tokio::runtime::Runtime;
use tracing::subscriber::Interest;
use tracing::{debug_span, metadata::LevelFilter, Instrument, Metadata};
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    layer::{Context, Filter},
    prelude::*,
    registry, Layer,
};
use tracing_tree::HierarchicalLayer;

pub(crate) fn start(
    session: CliSession,
    config_path: Option<Utf8PathBuf>,
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: Option<String>,
) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    let did_spawn = rt.block_on(ensure_daemon(
        false,
        config_path,
        log_path,
        log_file_name_prefix,
    ))?;

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
    config_path: Option<Utf8PathBuf>,
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: Option<String>,
) -> Result<(), CliDiagnostic> {
    setup_tracing_subscriber(log_path.as_deref(), log_file_name_prefix.as_deref());

    let rt = Runtime::new()?;
    let factory = ServerFactory::new(stop_on_disconnect);
    let cancellation = factory.cancellation();
    let span = debug_span!("Running Server",
        pid = std::process::id(),
        config_path = ?config_path.as_ref(),
        log_path = ?log_path.as_ref(),
        log_file_name_prefix = &log_file_name_prefix.as_deref(),
    );

    rt.block_on(async move {
        tokio::select! {
            res = run_daemon(factory, config_path).instrument(span) => {
                match res {
                    Ok(never) => match never {},
                    Err(err) => Err(err.into()),
                }
            }
            _ = cancellation.notified() => {
                tracing::info!("Received shutdown signal");
                Ok(())
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
    config_path: Option<Utf8PathBuf>,
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: Option<String>,
) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    rt.block_on(start_lsp_proxy(
        &rt,
        config_path,
        log_path,
        log_file_name_prefix,
    ))?;

    Ok(())
}

/// Start a proxy process.
/// Receives a process via `stdin` and then copy the content to the LSP socket.
/// Copy to the process on `stdout` when the LSP responds to a message
async fn start_lsp_proxy(
    rt: &Runtime,
    config_path: Option<Utf8PathBuf>,
    log_path: Option<Utf8PathBuf>,
    log_file_name_prefix: Option<String>,
) -> Result<(), CliDiagnostic> {
    ensure_daemon(true, config_path, log_path, log_file_name_prefix).await?;

    match open_socket().await? {
        Some((mut owned_read_half, mut owned_write_half)) => {
            // forward stdin to socket
            let mut stdin = io::stdin();
            let input_handle = rt.spawn(async move {
                loop {
                    match io::copy(&mut stdin, &mut owned_write_half).await {
                        Ok(b) => {
                            if b == 0 {
                                return Ok(());
                            }
                        }
                        Err(err) => return Err(err),
                    };
                }
            });

            // receive socket response to stdout
            let mut stdout = io::stdout();
            let out_put_handle = rt.spawn(async move {
                loop {
                    match io::copy(&mut owned_read_half, &mut stdout).await {
                        Ok(b) => {
                            if b == 0 {
                                return Ok(());
                            }
                        }
                        Err(err) => return Err(err),
                    };
                }
            });

            let _ = input_handle.await;
            let _ = out_put_handle.await;
            Ok(())
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
fn setup_tracing_subscriber(log_path: Option<&Utf8Path>, log_file_name_prefix: Option<&str>) {
    let biome_log_path = log_path.map_or_else(
        || biome_fs::ensure_cache_dir().join("biome-logs"),
        |path| path.to_path_buf(),
    );
    let appender_builder = tracing_appender::rolling::RollingFileAppender::builder();
    let file_appender = appender_builder
        .filename_prefix(log_file_name_prefix.map_or(String::from("server.log"), Into::into))
        .max_log_files(7)
        .rotation(Rotation::HOURLY)
        .build(biome_log_path)
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
            LevelFilter::INFO
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

//! This is the main binary of Biome.
//!
//! If you're curious about how to use it, check Biome's [website]
//!
//! [website]: https://biomejs.dev

use biome_cli::{
    biome_command, open_transport, setup_panic_handler, to_color_mode, BiomeCommand, CliDiagnostic,
    CliSession,
};
use biome_console::{markup, ConsoleExt, EnvConsole};
use biome_diagnostics::{set_bottom_frame, Diagnostic, PrintDiagnostic};
use biome_fs::OsFileSystem;
use biome_service::workspace;
use std::process::{ExitCode, Termination};
use tokio::runtime::Runtime;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl")
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn main() -> ExitCode {
    setup_panic_handler();
    set_bottom_frame(main as usize);

    let mut console = EnvConsole::default();
    let command = biome_command().fallback_to_usage().run();

    console.set_color(to_color_mode(command.get_color()));

    let is_verbose = command.is_verbose();
    let result = run_workspace(&mut console, command);
    match result {
        Err(termination) => {
            if termination.tags().is_verbose() && is_verbose {
                console.error(markup! {{PrintDiagnostic::verbose(&termination)}})
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&termination)}})
            }
            termination.report()
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn run_workspace(console: &mut EnvConsole, command: BiomeCommand) -> Result<(), CliDiagnostic> {
    // If the `--use-server` CLI flag is set, try to open a connection to an
    // existing Biome server socket
    let fs = Box::new(OsFileSystem::default());
    let workspace = if command.should_use_server() {
        let runtime = Runtime::new()?;
        match open_transport(runtime)? {
            Some(transport) => workspace::client(transport, fs)?,
            None => return Err(CliDiagnostic::server_not_running()),
        }
    } else {
        workspace::server(fs)
    };

    let session = CliSession::new(&*workspace, console)?;
    session.run(command)
}

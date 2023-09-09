//! This is the main binary of Biome.
//!
//! If you're curious about how to use it, check Biome's [website]
//!
//! [website]: https://biomejs.dev

use biome_cli::{
    biome_command, open_transport, setup_panic_handler, to_color_mode, BiomeCommand, CliDiagnostic,
    CliSession,
};
use biome_diagnostics::{set_bottom_frame, PrintDiagnostic};
use bpaf::{Args, ParseFailure};
use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_service::workspace;
use std::process::{ExitCode, Termination};
use tokio::runtime::Runtime;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> ExitCode {
    setup_panic_handler();
    set_bottom_frame(main as usize);

    let mut console = EnvConsole::default();
    let command = biome_command().run_inner(Args::current_args());
    match command {
        Ok(command) => {
            let color_mode = to_color_mode(command.get_color());
            console.set_color(color_mode);
            let is_verbose = command.is_verbose();
            let result = run_workspace(&mut console, command);
            match result {
                Err(termination) => {
                    console.error(markup! {
						{if is_verbose { PrintDiagnostic::verbose(&termination) } else { PrintDiagnostic::simple(&termination) }}
            		});

                    termination.report()
                }
                Ok(_) => ExitCode::SUCCESS,
            }
        }
        Err(failure) => {
            return if let ParseFailure::Stdout(help, _) = &failure {
                console.log(markup! {{help.to_string()}});
                ExitCode::FAILURE
            } else {
                let diagnostic = CliDiagnostic::parse_error_bpaf(failure);
                console.error(markup! { {PrintDiagnostic::simple(&diagnostic)}});
                ExitCode::FAILURE
            }
        }
    }
}

fn run_workspace(console: &mut EnvConsole, command: BiomeCommand) -> Result<(), CliDiagnostic> {
    // If the `--use-server` CLI flag is set, try to open a connection to an
    // existing Biome server socket
    let workspace = if command.should_use_server() {
        let runtime = Runtime::new()?;
        match open_transport(runtime)? {
            Some(transport) => workspace::client(transport)?,
            None => return Err(CliDiagnostic::server_not_running()),
        }
    } else {
        workspace::server()
    };

    let session = CliSession::new(&*workspace, console)?;
    session.run(command)
}

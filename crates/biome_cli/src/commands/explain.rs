use biome_analyze::RuleMetadata;
use biome_console::{markup, ConsoleExt};
use biome_flags::biome_env;
use biome_service::documentation::Doc;

use crate::commands::daemon::default_biome_log_path;
use crate::{CliDiagnostic, CliSession};

fn print_rule(session: CliSession, metadata: &RuleMetadata) {
    session.app.console.log(markup! {
        {metadata}
    });
}

pub(crate) fn explain(session: CliSession, doc: Doc) -> Result<(), CliDiagnostic> {
    match doc {
        Doc::Rule(metadata) => {
            print_rule(session, &metadata);
            Ok(())
        }
        Doc::DaemonLogs => {
            let cache_dir = biome_env()
                .biome_log_path
                .value()
                .unwrap_or(default_biome_log_path().to_string());
            session.app.console.error(markup! {
                <Info>"The daemon logs are available in the directory: \n"</Info>
            });
            session.app.console.log(markup! {{cache_dir}});
            Ok(())
        }
        Doc::Unknown(arg) => Err(CliDiagnostic::unexpected_argument(arg, "explain")),
    }
}

use crate::commands::daemon::default_biome_log_path;
use crate::{CliDiagnostic, CliSession};
use biome_console::{ConsoleExt, markup};
use biome_flags::biome_env;
use biome_service::documentation::{Doc, ExplainRule};

fn print_rule(session: CliSession, metadata: &ExplainRule) {
    session.app.console.log(markup! {
        {metadata}
    });
}

pub(crate) fn explain(session: CliSession, doc: Doc) -> Result<(), CliDiagnostic> {
    match doc {
        Doc::Rule(explain_rule) => {
            print_rule(session, &explain_rule);
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

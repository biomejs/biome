use biome_analyze::RuleMetadata;
use biome_console::{markup, ConsoleExt};
use biome_service::documentation::Doc;

use crate::commands::daemon::biome_log_dir;
use crate::{CliDiagnostic, CliSession};

fn print_rule(session: CliSession, metadata: &RuleMetadata) {
    session.app.console.log(markup! {
        "# "{metadata.name}"\n"
    });

    if let Some(kind) = &metadata.fix_kind {
        session.app.console.log(markup! {
            "Fix is "{kind}".\n"
        });
    } else {
        session.app.console.log(markup! {
            "No fix available.\n"
        });
    }

    let docs = metadata
        .docs
        .lines()
        .map(|line| line.trim_start())
        .collect::<Vec<_>>()
        .join("\n");

    session.app.console.log(markup! {
        "This rule is "{if metadata.recommended {"recommended."} else {"not recommended."}}
        "\n\n"
        "# Description\n"
        {docs}
    });
}

pub(crate) fn explain(session: CliSession, doc: Doc) -> Result<(), CliDiagnostic> {
    match doc {
        Doc::Rule(metadata) => {
            print_rule(session, &metadata);
            Ok(())
        }
        Doc::DaemonLogs => {
            let cache_dir = biome_log_dir().display().to_string();
            session.app.console.error(markup! {
                <Info>"The daemon logs are available in the directory: \n"</Info>
            });
            session.app.console.log(markup! {{cache_dir}});
            Ok(())
        }
        Doc::Unknown(arg) => Err(CliDiagnostic::unexpected_argument(arg, "explain")),
    }
}

use biome_analyze::RuleMetadata;
use biome_console::{markup, ConsoleExt};
use biome_service::explain::Explain;

use crate::{CliDiagnostic, CliSession};

fn print_rule(session: CliSession, metadata: &RuleMetadata) -> Result<(), CliDiagnostic> {
    session.app.console.log(markup! {
        "# "{metadata.name}
        "\n\n"
        {if let Some(fix) = &metadata.fix_kind {format!("Fix is {}.", fix.to_string())} else {"No fix available.".to_string()}}
        "\n\n"
        "This rule is "{if metadata.recommended {"recommended."} else {"not recommended."}}
        "\n\n"
        "# Description\n"
        {metadata.docs}
    });

    Ok(())
}

pub(crate) fn explain(session: CliSession, explain: Explain) -> Result<(), CliDiagnostic> {
    match explain {
        Explain::Rule(metadata) => print_rule(session, &metadata),
        Explain::Unknown(arg) => Err(CliDiagnostic::unexpected_argument(arg, "explain")),
    }
}

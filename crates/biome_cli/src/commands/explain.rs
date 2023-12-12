use biome_analyze::RuleMetadata;
use biome_console::{markup, ConsoleExt};
use biome_service::explain::Explain;

use crate::{CliDiagnostic, CliSession};

fn print_rule(session: CliSession, metadata: &RuleMetadata) {
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
}

pub(crate) fn explain(session: CliSession, explain: Explain) -> Result<(), CliDiagnostic> {
    if let Some(metadata) = explain.rule {
        print_rule(session, &metadata);
    }

    Ok(())
}

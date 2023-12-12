use biome_analyze::RuleMetadata;
use biome_console::{markup, ConsoleExt};
use biome_service::explanations::Explanations;

use crate::{CliDiagnostic, CliSession};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub(crate) struct ExplainCommandPayload {
    pub(crate) explanations: Explanations,
}

pub(crate) fn explain(
    session: CliSession,
    payload: ExplainCommandPayload,
) -> Result<(), CliDiagnostic> {
    let ExplainCommandPayload { explanations } = payload;

    if let Some(metadata) = explanations.rule {
        print_rule(session, &metadata);
    }

    Ok(())
}

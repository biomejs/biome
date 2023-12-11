use biome_analyze::RuleMetadata;
use biome_service::explanations::Explanations;

use crate::{CliDiagnostic, CliSession};

#[allow(dead_code)]
fn print_rule(metadata: &RuleMetadata) {
    println!("# {}", metadata.name);
    println!();

    print!("This rule is ");
    if metadata.recommended {
        println!("recommended.");
    } else {
        println!("not recommended.");
    }
    println!();

    if let Some(fix) = &metadata.fix_kind {
        println!("Fix is {}.", fix.to_string());
    } else {
        println!("No fix available.")
    }
    println!();

    println!("# Description");
    println!("{}", metadata.docs);
}

#[allow(dead_code)]
pub(crate) struct ExplainCommandPayload {
    pub(crate) explanations: Explanations,
}

pub(crate) fn explain(
    _session: CliSession,
    payload: ExplainCommandPayload,
) -> Result<(), CliDiagnostic> {
    let ExplainCommandPayload { explanations } = payload;

    if let Some(metadata) = explanations.rule {
        print_rule(&metadata);
    }

    Ok(())
}

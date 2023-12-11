use biome_analyze::RuleMetadata;
use biome_service::explanations::Explanations;

use crate::{CliDiagnostic, CliSession};

#[allow(dead_code)]
fn print_rule(rule: &RuleMetadata) {
    println!("# {} v{}", rule.name, rule.version);

    if let Some(deprecated) = rule.deprecated {
        println!("Deprecated: {}", deprecated);
    }

    if rule.recommended {
        println!("Recommended: Yes");
    } else {
        println!("Recommended: No");
    }

    if let Some(fix_kind) = &rule.fix_kind {
        println!("Fix: {}", fix_kind.to_string());
    }

    println!("{}", rule.docs);
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

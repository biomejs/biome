use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::*;
use biome_rowan::{TextRange, TextSize};

use crate::parser::TailwindParser;

pub(crate) fn expected_candidate(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("candidate", range, p).into_diagnostic(p)
}

pub(crate) fn expected_variant(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variant", range, p).into_diagnostic(p)
}

pub(crate) fn expected_value(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("value", range, p).into_diagnostic(p)
}

pub(crate) fn expected_modifier(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("modifier", range, p).into_diagnostic(p)
}

/// The diagnostic for the whitespace byte at `start` that CSS doesn't allow: `\x0B` or `\x0C`.
pub(crate) fn invalid_whitespace(start: TextSize) -> ParseDiagnostic {
    ParseDiagnostic::new(
        "The CSS standard only allows tabs, whitespace, carriage return and line feed whitespace.",
        start..start + TextSize::from(1),
    )
    .with_hint("Use a regular whitespace character instead.")
}

/// A candidate spelled important twice: with the legacy leading `!` and
/// the trailing `!` (`!flex!`). Tailwind rejects the combination.
pub(crate) fn duplicate_important(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "A candidate can't be marked important with both a leading and a trailing `!`.",
        range,
    )
    .with_hint("Keep only the trailing `!` (`flex!`); the leading form is legacy syntax.")
}

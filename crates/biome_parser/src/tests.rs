use super::{ParserContext, diagnostic::ParseDiagnostic};
use biome_rowan::{TextRange, TextSize, raw_language::RawLanguageKind};

fn diagnostic(message: &str, start: u32) -> ParseDiagnostic {
    ParseDiagnostic::new(message, TextRange::empty(TextSize::from(start)))
}

#[test]
fn finish_sorts_diagnostics_stably_by_start() {
    let mut context = ParserContext::<RawLanguageKind>::new();
    context.diagnostics.push(diagnostic("later", 10));
    context.diagnostics.push(diagnostic("earlier", 2));
    context.diagnostics.push(diagnostic("same start", 2));
    context
        .diagnostics
        .push(ParseDiagnostic::new("no span", None::<TextRange>));

    let (_, diagnostics) = context.finish();
    let messages: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.to_string())
        .collect();
    assert_eq!(messages, ["no span", "earlier", "same start", "later"]);
}

#[test]
fn rewind_removes_late_diagnostics_at_earlier_positions() {
    let mut context = ParserContext::<RawLanguageKind>::new();
    context.diagnostics.push(diagnostic("retained", 10));
    let checkpoint = context.checkpoint();
    context.diagnostics.push(diagnostic("discarded", 2));
    context.rewind(checkpoint);

    let (_, diagnostics) = context.finish();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].message.to_string(), "retained");
}

#[test]
fn finish_without_diagnostics() {
    let context = ParserContext::<RawLanguageKind>::new();
    assert!(context.finish().1.is_empty());
}

#![cfg(test)]

use crate::parse_grit;

/// This pattern should be parseable, but previously caused explosive memory usage
#[test]
fn parse_language_declaration() {
    let code = "language js\n";
    let parse = parse_grit(code);
    assert!(parse.diagnostics().is_empty());
}

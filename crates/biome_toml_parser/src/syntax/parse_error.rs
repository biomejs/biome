use crate::parser::TomlParser;
use crate::syntax::MAX_NESTING_DEPTH;
use biome_parser::diagnostic::{ParseDiagnostic, expected_node};
use biome_parser::prelude::*;
use biome_rowan::TextRange;

pub(crate) fn expected_key(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("key", range, p)
}

pub(crate) fn expected_value(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("value", range, p)
}

pub(crate) fn expected_key_value(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("key/value pair", range, p)
}

pub(crate) fn expected_key_value_or_table_header(
    p: &TomlParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_node("key/value pair or table header", range, p)
}

/// A value on the line after its key, as in `key =` followed by a new line.
pub(crate) fn expected_value_on_same_line(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    expected_value(p, range).with_hint("The value must be on the same line as the key.")
}

/// A key/value pair or table header that doesn't start a new line.
pub(crate) fn expected_newline(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!(
            "Expected a newline but instead found `{}`.",
            &p.source().text()[range]
        ),
        range,
    )
    .with_hint("Each key/value pair and table header must be on its own line.")
}

/// A closing bracket of a table header on a new line, as in `[table` followed
/// by `]` on the next line.
pub(crate) fn token_on_next_line(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!(
            "`{}` must be on the same line as the previous token.",
            &p.source().text()[range]
        ),
        range,
    )
}

/// An `=` on the line after its key.
pub(crate) fn equals_on_next_line(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("`=` must be on the same line as the key.", range)
}

/// A key at the end of a line without `=` and a value.
pub(crate) fn expected_equals_and_value(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected `=` and a value after this key.", range)
        .with_hint("Key/value pairs use the `key = value` format.")
}

pub(crate) fn multiline_string_key(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Multi-line strings can't be used as keys.", range)
        .with_hint("Use a basic string (`\"key\"`) or a literal string (`'key'`).")
}

pub(crate) fn nesting_too_deep(p: &TomlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!(
            "Arrays and inline tables can't be nested more than {MAX_NESTING_DEPTH} levels deep."
        ),
        range,
    )
}

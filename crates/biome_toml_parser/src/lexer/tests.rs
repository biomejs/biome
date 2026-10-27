#![cfg(test)]

use super::{TomlLexContext, TomlLexer};
use biome_parser::lexer::Lexer;
use biome_toml_syntax::TomlSyntaxKind::{self, *};
use biome_toml_syntax::TextSize;

/// Lexes `source` in `context` and returns the kinds and texts of all tokens.
fn lex(source: &str, context: TomlLexContext) -> (Vec<(TomlSyntaxKind, &str)>, usize) {
    let mut lexer = TomlLexer::from_str(source);
    let mut tokens = Vec::new();
    let mut reconstructed = String::new();

    while lexer.next_token(context) != EOF {
        let range = lexer.current_range();
        tokens.push((lexer.current(), &source[range]));
        reconstructed.push_str(&source[range]);
    }

    assert_eq!(reconstructed, source, "the lexer must be lossless");
    (tokens, lexer.finish().len())
}

fn assert_tokens(source: &str, context: TomlLexContext, expected: &[(TomlSyntaxKind, &str)]) {
    let (tokens, diagnostics) = lex(source, context);
    assert_eq!(tokens, expected, "tokens of {source:?}");
    assert_eq!(diagnostics, 0, "diagnostics of {source:?}");
}

/// Asserts that `source` is lexed as a single value token of `kind`.
fn assert_value(source: &str, kind: TomlSyntaxKind) {
    assert_tokens(source, TomlLexContext::Value, &[(kind, source)]);
}

/// Asserts that `source` is lexed as a single token with at least one diagnostic.
fn assert_invalid_value(source: &str) {
    let (tokens, diagnostics) = lex(source, TomlLexContext::Value);
    assert_eq!(tokens.len(), 1, "tokens of {source:?}: {tokens:?}");
    assert!(diagnostics > 0, "expected a diagnostic for {source:?}");
}

#[test]
fn key_context() {
    assert_tokens(
        "a.\"b\".'c' = 1",
        TomlLexContext::Key,
        &[
            (TOML_BARE_KEY_LITERAL, "a"),
            (DOT, "."),
            (TOML_BASIC_STRING_LITERAL, "\"b\""),
            (DOT, "."),
            (TOML_LITERAL_STRING_LITERAL, "'c'"),
            (WHITESPACE, " "),
            (EQ, "="),
            (WHITESPACE, " "),
            (TOML_BARE_KEY_LITERAL, "1"),
        ],
    );
}

#[test]
fn key_context_value_like_keys() {
    assert_tokens(
        "3.14 true 2021-01-01",
        TomlLexContext::Key,
        &[
            (TOML_BARE_KEY_LITERAL, "3"),
            (DOT, "."),
            (TOML_BARE_KEY_LITERAL, "14"),
            (WHITESPACE, " "),
            (TOML_BARE_KEY_LITERAL, "true"),
            (WHITESPACE, " "),
            (TOML_BARE_KEY_LITERAL, "2021-01-01"),
        ],
    );
}

#[test]
fn double_brackets_depend_on_context() {
    assert_tokens(
        "[[a]]",
        TomlLexContext::Key,
        &[
            (L_DOUBLE_BRACK, "[["),
            (TOML_BARE_KEY_LITERAL, "a"),
            (R_DOUBLE_BRACK, "]]"),
        ],
    );
    assert_tokens(
        "[[1]]",
        TomlLexContext::Value,
        &[
            (L_BRACK, "["),
            (L_BRACK, "["),
            (TOML_INTEGER_LITERAL, "1"),
            (R_BRACK, "]"),
            (R_BRACK, "]"),
        ],
    );
}

#[test]
fn trivia() {
    assert_tokens(
        " \t# comment\r\n\n",
        TomlLexContext::Key,
        &[
            (WHITESPACE, " \t"),
            (COMMENT, "# comment"),
            (NEWLINE, "\r\n"),
            (NEWLINE, "\n"),
        ],
    );
}

#[test]
fn bom() {
    assert_tokens(
        "\u{feff}a",
        TomlLexContext::Key,
        &[(UNICODE_BOM, "\u{feff}"), (TOML_BARE_KEY_LITERAL, "a")],
    );
}

#[test]
fn strings() {
    assert_value(r#""a \"b\" \u00E9 \U0001F600 \x41 \e""#, TOML_BASIC_STRING_LITERAL);
    assert_value("'C:\\path'", TOML_LITERAL_STRING_LITERAL);
    assert_value("\"\"\"\na\n\"b\"\"\"\"\"", TOML_ML_BASIC_STRING_LITERAL);
    assert_value("\"\"\"a \\  \n  b\"\"\"", TOML_ML_BASIC_STRING_LITERAL);
    assert_value("'''a''b'''''", TOML_ML_LITERAL_STRING_LITERAL);
    assert_value("\"\"", TOML_BASIC_STRING_LITERAL);
    assert_value("''", TOML_LITERAL_STRING_LITERAL);
}

#[test]
fn invalid_strings() {
    assert_invalid_value(r#""\q""#);
    assert_invalid_value(r#""\uD800""#);
    assert_invalid_value(r#""\u12""#);
    assert_invalid_value("\"a\u{1}\"");
    assert_invalid_value("'a\u{7f}'");
    assert_invalid_value("\"\"\"a \\ b\"\"\"");
    assert_invalid_value("'''a''''''");
    assert_invalid_value("\"unterminated");
    assert_invalid_value("'''unterminated");
}

#[test]
fn unterminated_string_stops_at_newline() {
    let (tokens, diagnostics) = lex("\"abc\nd", TomlLexContext::Value);
    assert_eq!(
        tokens,
        [
            (TOML_BASIC_STRING_LITERAL, "\"abc"),
            (NEWLINE, "\n"),
            (ERROR_TOKEN, "d"),
        ]
    );
    assert_eq!(diagnostics, 2);
}

#[test]
fn integers() {
    for source in [
        "0",
        "+0",
        "-0",
        "42",
        "1_000",
        "9223372036854775807",
        "-9223372036854775808",
        "0xDEAD_beef",
        "0x7FFFFFFFFFFFFFFF",
        "0o755",
        "0b1101",
        "0x00",
    ] {
        assert_value(source, TOML_INTEGER_LITERAL);
    }
}

#[test]
fn invalid_integers() {
    for source in [
        "01",
        "1__0",
        "1_",
        "_1",
        "-0x1",
        "+0b1",
        "0X1",
        "0x",
        "0xG",
        "0o8",
        "0b2",
        "9223372036854775808",
        "-9223372036854775809",
        "0x8000000000000000",
    ] {
        assert_invalid_value(source);
    }
}

#[test]
fn floats() {
    for source in [
        "1.0", "+1.0", "-0.01", "5e+22", "1e06", "-2E-2", "6.626e-34", "1_0.0_1", "0.0", "inf",
        "+inf", "-inf", "nan", "+nan", "-nan",
    ] {
        assert_value(source, TOML_FLOAT_LITERAL);
    }
}

#[test]
fn invalid_floats() {
    for source in [
        ".5", "1.", "1.e5", "1e", "1e+", "01.5", "1._5", "1.5_", "1e_5", "Inf", "NaN", "1.2.3",
    ] {
        assert_invalid_value(source);
    }
}

#[test]
fn booleans() {
    assert_value("true", TRUE_KW);
    assert_value("false", FALSE_KW);
    assert_invalid_value("True");
    assert_invalid_value("fals");
}

#[test]
fn date_times() {
    assert_value("1979-05-27T07:32:00Z", TOML_OFFSET_DATE_TIME_LITERAL);
    assert_value("1979-05-27t07:32:00.999-07:00", TOML_OFFSET_DATE_TIME_LITERAL);
    assert_value("1979-05-27 07:32z", TOML_OFFSET_DATE_TIME_LITERAL);
    assert_value("1979-05-27T07:32:00", TOML_LOCAL_DATE_TIME_LITERAL);
    assert_value("1979-05-27 07:32", TOML_LOCAL_DATE_TIME_LITERAL);
    assert_value("1979-05-27", TOML_LOCAL_DATE_LITERAL);
    assert_value("2000-02-29", TOML_LOCAL_DATE_LITERAL);
    assert_value("07:32:00", TOML_LOCAL_TIME_LITERAL);
    assert_value("07:32", TOML_LOCAL_TIME_LITERAL);
    assert_value("23:59:60.5", TOML_LOCAL_TIME_LITERAL);
}

#[test]
fn leap_seconds() {
    assert_value("2016-12-31T23:59:60Z", TOML_OFFSET_DATE_TIME_LITERAL);
    assert_value("2015-06-30T23:59:60.5z", TOML_OFFSET_DATE_TIME_LITERAL);
    // 23:59:60 UTC on 2016-12-31.
    assert_value("2016-12-31T18:59:60-05:00", TOML_OFFSET_DATE_TIME_LITERAL);
    assert_value("2017-01-01T08:59:60+09:00", TOML_OFFSET_DATE_TIME_LITERAL);
    // 23:59:60 UTC on 2016-02-29, the last day of February in a leap year.
    assert_value("2016-03-01T08:59:60+09:00", TOML_OFFSET_DATE_TIME_LITERAL);
    // The UTC instant is unknown without an offset.
    assert_value("2000-01-01T12:30:60", TOML_LOCAL_DATE_TIME_LITERAL);
    assert_value("12:30:60", TOML_LOCAL_TIME_LITERAL);

    assert_invalid_value("2000-01-01T12:30:60Z");
    assert_invalid_value("2016-12-31T23:58:60Z");
    assert_invalid_value("2016-12-30T23:59:60Z");
    assert_invalid_value("2016-12-31T23:59:60+01:00");
    assert_invalid_value("2016-12-31T23:59:60-00:01");
    assert_invalid_value("2016-03-02T08:59:60+09:00");
}

#[test]
fn date_followed_by_comment() {
    assert_tokens(
        "1979-05-27 # c",
        TomlLexContext::Value,
        &[
            (TOML_LOCAL_DATE_LITERAL, "1979-05-27"),
            (WHITESPACE, " "),
            (COMMENT, "# c"),
        ],
    );
}

#[test]
fn invalid_date_times() {
    for source in [
        "1900-02-29",
        "2023-13-01",
        "2023-01-32",
        "2023-04-31",
        "24:00:00",
        "00:60:00",
        "00:00:61",
        "07:32.5",
        "07:32:00.",
        "1979-05-27T07:32:00+24:00",
        "1979-05-27T07:32:00+0700",
        "1979-05-27X07:32:00",
        "1979-5-27",
    ] {
        assert_invalid_value(source);
    }
}

#[test]
fn unexpected_characters() {
    let (tokens, diagnostics) = lex("\u{c}\u{3000}!", TomlLexContext::Key);
    assert_eq!(
        tokens,
        [
            (ERROR_TOKEN, "\u{c}"),
            (ERROR_TOKEN, "\u{3000}"),
            (ERROR_TOKEN, "!"),
        ]
    );
    assert_eq!(diagnostics, 3);
}

#[test]
fn lone_carriage_return() {
    let (tokens, diagnostics) = lex("a\rb", TomlLexContext::Key);
    assert_eq!(
        tokens,
        [
            (TOML_BARE_KEY_LITERAL, "a"),
            (NEWLINE, "\r"),
            (TOML_BARE_KEY_LITERAL, "b"),
        ]
    );
    assert_eq!(diagnostics, 1);
}

#[test]
fn re_lex_keeps_line_break_flag() {
    use biome_parser::lexer::ReLexer;

    let mut lexer = TomlLexer::from_str("a = \nb");
    for context in [
        TomlLexContext::Key,
        TomlLexContext::Key,
        TomlLexContext::Key,
        TomlLexContext::Value,
        TomlLexContext::Value,
        TomlLexContext::Value,
    ] {
        lexer.next_token(context);
    }
    assert_eq!(lexer.current(), ERROR_TOKEN);
    assert!(lexer.has_preceding_line_break());

    assert_eq!(lexer.re_lex(TomlLexContext::Key), TOML_BARE_KEY_LITERAL);
    assert!(lexer.has_preceding_line_break());
    assert_eq!(lexer.current_start(), TextSize::from(5));
    assert!(lexer.finish().is_empty());
}

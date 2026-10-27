//! Deeply nested values must produce a diagnostic instead of overflowing the
//! stack.

use biome_toml_parser::parse_toml;

fn assert_parses_with_error(source: &str) {
    let parse = parse_toml(source);
    assert!(parse.has_errors());
    assert_eq!(parse.syntax().to_string(), source);
}

#[test]
fn deeply_nested_arrays() {
    let depth = 100_000;
    let source = format!("a = {}0{}\nb = 1\n", "[".repeat(depth), "]".repeat(depth));
    assert_parses_with_error(&source);
}

#[test]
fn deeply_nested_inline_tables() {
    let depth = 100_000;
    let source = format!(
        "a = {}0{}\nb = 1\n",
        "{ a = ".repeat(depth),
        " }".repeat(depth)
    );
    assert_parses_with_error(&source);
}

#[test]
fn deeply_nested_unterminated_arrays() {
    let source = format!("a = {}\nb = 1\n", "[".repeat(100_000));
    assert_parses_with_error(&source);
}

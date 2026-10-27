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

/// Returns the root items as `kind name` strings, with the keys of each table's
/// entries indented below it.
fn outline(source: &str) -> Vec<String> {
    use biome_rowan::{AstNode, AstSeparatedList};
    use biome_toml_syntax::{AnyTomlKeyValue, AnyTomlRootItem, TomlKey};

    fn key_text(key: &TomlKey) -> String {
        key.parts()
            .iter()
            .map(|part| part.unwrap().syntax().text_trimmed().to_string())
            .collect::<Vec<_>>()
            .join(".")
    }

    let parse = parse_toml(source);
    let mut outline = Vec::new();
    for item in parse.tree().items() {
        let entries = match &item {
            AnyTomlRootItem::TomlKeyValue(key_value) => {
                outline.push(format!("pair {}", key_text(&key_value.key().unwrap())));
                continue;
            }
            AnyTomlRootItem::TomlTable(table) => {
                outline.push(format!("table {}", key_text(&table.key().unwrap())));
                table.entries()
            }
            AnyTomlRootItem::TomlArrayTable(table) => {
                outline.push(format!("array table {}", key_text(&table.key().unwrap())));
                table.entries()
            }
            AnyTomlRootItem::TomlBogus(bogus) => {
                outline.push(format!("bogus {}", bogus.syntax().text_trimmed()));
                continue;
            }
        };
        for entry in entries {
            match entry {
                AnyTomlKeyValue::TomlKeyValue(key_value) => {
                    outline.push(format!("  pair {}", key_text(&key_value.key().unwrap())));
                }
                AnyTomlKeyValue::TomlBogus(bogus) => {
                    outline.push(format!("  bogus {}", bogus.syntax().text_trimmed()));
                }
            }
        }
    }
    outline
}

#[test]
fn recovery_after_too_deep_unterminated_array() {
    let source = format!("a = {}\nb = 1\n[t]\nc = 2\n", "[".repeat(129));
    assert_parses_with_error(&source);
    assert_eq!(
        outline(&source),
        ["pair a", "pair b", "table t", "  pair c"]
    );
}

#[test]
fn recovery_after_too_deep_unterminated_inline_table() {
    let source = format!("a = {}\n[t]\nc = 2\n", "{ a = ".repeat(129));
    assert_parses_with_error(&source);
    assert_eq!(outline(&source), ["pair a", "table t", "  pair c"]);
}

#[test]
fn recovery_after_unterminated_array_with_trailing_comma() {
    let source = "f = [1,\n[[g]]\nh = 1\n[t]\ni = [\n  2,\n  [t2]\nj = 3\n";
    assert_parses_with_error(source);
    assert_eq!(
        outline(source),
        [
            "pair f",
            "array table g",
            "  pair h",
            "table t",
            "  pair i",
            "table t2",
            "  pair j"
        ]
    );
}

#[test]
fn recovery_after_unterminated_array_before_assignment() {
    let source = "f = [1,\n\"g\" = 2\nh = [\n  true = 3\n";
    assert_parses_with_error(source);
    assert_eq!(
        outline(source),
        ["pair f", "pair \"g\"", "pair h", "pair true"]
    );
}

#[test]
fn nested_arrays_on_their_own_lines_stay_elements() {
    let source = "a = [\n  1,\n  [[2]]\n]\nb = [\n  [1],\n  [2]\n]\nc = [\n  [x]\n]\n";
    let parse = parse_toml(source);
    assert_eq!(outline(source)[..2], ["pair a", "pair b"]);
    // `[x]` is an invalid value, not a header, because `]` follows it.
    assert_eq!(outline(source)[2], "pair c");
    assert_eq!(parse.diagnostics().len(), 1);
}

#[test]
fn recovery_after_unterminated_array_before_dotted_assignment() {
    let source = "a = [1,\n\"b\".c = 2\nd = [\n  true.e = 3\nf = [1.5,\n  1.5 = 4\n";
    assert_parses_with_error(source);
    assert_eq!(
        outline(source),
        [
            "pair a",
            "pair \"b\".c",
            "pair d",
            "pair true.e",
            "pair f",
            "pair 1.5"
        ]
    );
}

#[test]
fn recovery_after_too_deep_array_before_header() {
    let source = format!("a = {}1,\n[[t]]\nb = 2\n", "[".repeat(129));
    assert_parses_with_error(&source);
    assert_eq!(outline(&source), ["pair a", "array table t", "  pair b"]);
}

#[test]
fn values_that_look_like_keys_stay_elements() {
    let source = "a = [\n  1.5,\n  \"b\",\n  true,\n  1979-05-27T07:32:00Z\n]\n";
    let parse = parse_toml(source);
    assert!(!parse.has_errors(), "{:?}", parse.diagnostics());
}

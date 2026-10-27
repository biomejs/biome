//! Formats the valid TOML 1.1.0 documents of
//! [toml-test](https://github.com/toml-lang/toml-test), vendored by
//! `biome_toml_parser`.
//!
//! Formatting must keep every document valid, must not change any decoded
//! value, and must be idempotent, with every combination of layout options.

use biome_formatter::{Expand, IndentStyle, LineWidth};
use biome_rowan::AstSeparatedList;
use biome_toml_formatter::{TomlFormatOptions, format_node};
use biome_toml_parser::parse_toml;
use biome_toml_syntax::{
    AnyTomlKeyValue, AnyTomlRootItem, AnyTomlValue, TomlKey, TomlKeyValue, TomlSyntaxKind,
};
use serde_json::{Map, Value, json};
use std::fmt::Write;
use std::path::Path;

const SUITE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../biome_toml_parser/tests/toml-test"
);

#[test]
fn formatting_preserves_toml_test_documents() {
    let suite = Path::new(SUITE);
    let files = std::fs::read_to_string(suite.join("files-toml-1.1.0")).unwrap();
    let option_sets = [
        TomlFormatOptions::default(),
        TomlFormatOptions::default().with_expand(Expand::Always),
        TomlFormatOptions::default()
            .with_expand(Expand::Never)
            .with_indent_style(IndentStyle::Space)
            .with_line_width(LineWidth::try_from(20).unwrap()),
    ];
    let mut failures = String::new();
    let mut count = 0;

    for file in files
        .lines()
        .filter(|file| file.starts_with("valid/") && file.ends_with(".toml"))
    {
        count += 1;
        let source = std::fs::read_to_string(suite.join(file)).unwrap();
        let expected = decode_document(&parse_toml(&source).tree());

        for options in &option_sets {
            let formatted = format(&source, options);
            let parse = parse_toml(&formatted);
            if parse.has_errors() {
                writeln!(
                    failures,
                    "{file}: formatted document has errors with {options:?}:\n{formatted}\n{:#?}",
                    parse.diagnostics()
                )
                .unwrap();
                continue;
            }

            if comments(&parse.syntax()) != comments(&parse_toml(&source).syntax()) {
                writeln!(
                    failures,
                    "{file}: formatting changed the comments or their order with {options:?}:\n{formatted}"
                )
                .unwrap();
            }

            let actual = decode_document(&parse.tree());
            if actual != expected {
                writeln!(
                    failures,
                    "{file}: formatting changed the values with {options:?}:\n{formatted}"
                )
                .unwrap();
            }

            let reformatted = format(&formatted, options);
            if reformatted != formatted {
                writeln!(
                    failures,
                    "{file}: formatting isn't idempotent with {options:?}:\n{formatted}\n---\n{reformatted}"
                )
                .unwrap();
            }
        }
    }

    assert!(count > 200, "the suite is incomplete");
    assert!(failures.is_empty(), "{failures}");
}

/// Documents that exercise formatting edge cases. Each must keep its values
/// and comments and be idempotent, like the toml-test documents.
const EDGE_CASES: &[&str] = &[
    // U+2028 and U+2029 aren't line breaks in TOML.
    "# note\u{2028}a = 1\nb = 2\n",
    "# note\u{2029}a = 1\nb = 2\n",
    "x = \"a\u{2028}b\"\ny = \"\"\"a\u{2029}b\"\"\"\n",
    "# biome-ignore format: preserve\nx   =   \"a\u{2028}b\"\n",
    "# biome-ignore format: preserve\nx   =   \"\"\"a\u{2029}\nb\"\"\"\n",
    // Entries of an inline table that start lines after commas.
    "x = { a = 1, # a\nb = 2, # b\nc = 3 }\n",
    "x = { a = 1,\nb = 2 }\n",
    "x = { a = 1\n, b = 2 }\n",
    "x = { a = [1, # one\n2], b = 3 }\n",
    // Comments separated by commas that the formatter moves.
    "a = [1 # one\n, # two\n2]\n",
    "a = [\n# biome-ignore format: preserve\n1 # one\n, # two\n2\n]\n",
    "# biome-ignore format: preserve\na   =   1 # one\n# two\nb = 2\n",
    "# biome-ignore format: preserve\n[  t  ] # one\nc = 1 # two\n",
];

#[test]
fn formatting_preserves_edge_cases() {
    let options = [
        TomlFormatOptions::default(),
        TomlFormatOptions::default().with_expand(Expand::Always),
        TomlFormatOptions::default().with_line_width(LineWidth::try_from(10).unwrap()),
    ];
    for source in EDGE_CASES {
        let original = parse_toml(source);
        assert!(
            !original.has_errors(),
            "{source:?}: {:#?}",
            original.diagnostics()
        );
        for options in &options {
            let formatted = format(source, options);
            let parse = parse_toml(&formatted);
            assert!(
                !parse.has_errors(),
                "{source:?} formatted to an invalid document:\n{formatted}"
            );
            assert!(
                decode_document(&parse.tree()) == decode_document(&original.tree()),
                "{source:?}: formatting changed the values:\n{formatted}"
            );
            assert_eq!(
                comments(&parse.syntax()),
                comments(&original.syntax()),
                "{source:?}: formatting changed the comments:\n{formatted}"
            );
            assert_eq!(format(&formatted, options), formatted, "{source:?}");
        }
    }
}

/// Returns the text of every comment in document order, with the token that
/// precedes it, so moving a comment past a token changes the result. Commas are
/// ignored because the formatter moves them before trailing comments.
fn comments(root: &biome_toml_syntax::TomlSyntaxNode) -> Vec<(String, String)> {
    let mut comments = Vec::new();
    let mut previous = String::new();
    for token in root.descendants_tokens(biome_rowan::Direction::Next) {
        for piece in token.leading_trivia().pieces() {
            if piece.is_comments() {
                comments.push((previous.clone(), piece.text().to_string()));
            }
        }
        if token.kind() != TomlSyntaxKind::COMMA {
            previous = token.text_trimmed().to_string();
        }
        for piece in token.trailing_trivia().pieces() {
            if piece.is_comments() {
                comments.push((previous.clone(), piece.text().to_string()));
            }
        }
    }
    comments
}

/// Formats documents with errors: the invalid documents of the suite and every
/// prefix of the valid ones. Formatting may refuse a document with errors, but
/// must not panic, and must not drop or invent source text when it succeeds:
/// reformatting the output must reproduce it.
#[test]
fn formatting_documents_with_errors_doesnt_panic() {
    let suite = Path::new(SUITE);
    let files = std::fs::read_to_string(suite.join("files-toml-1.1.0")).unwrap();
    let options = TomlFormatOptions::default();

    for file in files.lines().filter(|file| file.ends_with(".toml")) {
        let Ok(source) = String::from_utf8(std::fs::read(suite.join(file)).unwrap()) else {
            continue;
        };

        let mut inputs = vec![source.as_str()];
        if file.starts_with("valid/") {
            inputs.extend(source.char_indices().skip(1).map(|(end, _)| &source[..end]));
        }

        for input in inputs {
            let parse = parse_toml(input);
            let Ok(formatted) = format_node(options.clone(), &parse.syntax()) else {
                continue;
            };
            let printed = formatted.print().unwrap().into_code();
            assert_eq!(
                strip_whitespace(&printed),
                strip_whitespace(input),
                "{file}: formatting changed more than whitespace:\n{input}\n---\n{printed}"
            );
        }
    }
}

/// Removes whitespace and commas, which the formatter can add or remove.
fn strip_whitespace(text: &str) -> String {
    text.chars()
        .filter(|char| !char.is_whitespace() && *char != ',')
        .collect()
}

fn format(source: &str, options: &TomlFormatOptions) -> String {
    let parse = parse_toml(source);
    format_node(options.clone(), &parse.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code()
}

fn decode_document(root: &biome_toml_syntax::TomlRoot) -> Value {
    let mut document = Map::new();

    for item in root.items() {
        match item {
            AnyTomlRootItem::TomlKeyValue(key_value) => insert_key_value(&mut document, &key_value),
            AnyTomlRootItem::TomlTable(table) => {
                let names = key_names(&table.key().unwrap());
                let target = navigate(&mut document, &names);
                for entry in table.entries() {
                    if let AnyTomlKeyValue::TomlKeyValue(key_value) = entry {
                        insert_key_value(target, &key_value);
                    }
                }
            }
            AnyTomlRootItem::TomlArrayTable(table) => {
                let names = key_names(&table.key().unwrap());
                let (last, parents) = names.split_last().unwrap();
                let parent = navigate(&mut document, parents);
                let array = parent
                    .entry(last.clone())
                    .or_insert_with(|| Value::Array(Vec::new()))
                    .as_array_mut()
                    .unwrap();
                array.push(Value::Object(Map::new()));
                let target = array.last_mut().unwrap().as_object_mut().unwrap();
                for entry in table.entries() {
                    if let AnyTomlKeyValue::TomlKeyValue(key_value) = entry {
                        insert_key_value(target, &key_value);
                    }
                }
            }
            AnyTomlRootItem::TomlBogus(_) => unreachable!(),
        }
    }

    Value::Object(document)
}

fn key_names(key: &TomlKey) -> Vec<String> {
    key.parts()
        .iter()
        .map(|part| part.unwrap().name().unwrap().to_string())
        .collect()
}

/// Returns the table at `names`, creating tables and descending into the last
/// element of arrays of tables.
fn navigate<'a>(table: &'a mut Map<String, Value>, names: &[String]) -> &'a mut Map<String, Value> {
    let mut table = table;
    for name in names {
        let entry = table
            .entry(name.clone())
            .or_insert_with(|| Value::Object(Map::new()));
        table = match entry {
            Value::Array(array) => array.last_mut().unwrap().as_object_mut().unwrap(),
            value => value.as_object_mut().unwrap(),
        };
    }
    table
}

fn insert_key_value(table: &mut Map<String, Value>, key_value: &TomlKeyValue) {
    let names = key_names(&key_value.key().unwrap());
    let (last, parents) = names.split_last().unwrap();
    let target = navigate(table, parents);
    target.insert(last.clone(), decode_value(&key_value.value().unwrap()));
}

fn decode_value(value: &AnyTomlValue) -> Value {
    match value {
        AnyTomlValue::TomlString(string) => {
            json!({ "type": "string", "value": string.decoded_value().unwrap().text() })
        }
        AnyTomlValue::TomlInteger(integer) => {
            let token = integer.value_token().unwrap();
            let text = token.text_trimmed().replace('_', "");
            let value = if let Some(hex) = text.strip_prefix("0x") {
                i64::from_str_radix(hex, 16).unwrap()
            } else if let Some(octal) = text.strip_prefix("0o") {
                i64::from_str_radix(octal, 8).unwrap()
            } else if let Some(binary) = text.strip_prefix("0b") {
                i64::from_str_radix(binary, 2).unwrap()
            } else {
                text.parse::<i64>().unwrap()
            };
            json!({ "type": "integer", "value": value.to_string() })
        }
        AnyTomlValue::TomlFloat(float) => {
            let token = float.value_token().unwrap();
            json!({ "type": "float", "value": token.text_trimmed().replace('_', "") })
        }
        AnyTomlValue::TomlBoolean(boolean) => {
            let token = boolean.value_token().unwrap();
            json!({ "type": "bool", "value": token.text_trimmed() })
        }
        AnyTomlValue::TomlDateTime(date_time) => {
            let token = date_time.value_token().unwrap();
            let kind = match token.kind() {
                TomlSyntaxKind::TOML_OFFSET_DATE_TIME_LITERAL => "datetime",
                TomlSyntaxKind::TOML_LOCAL_DATE_TIME_LITERAL => "datetime-local",
                TomlSyntaxKind::TOML_LOCAL_DATE_LITERAL => "date-local",
                TomlSyntaxKind::TOML_LOCAL_TIME_LITERAL => "time-local",
                kind => unreachable!("{kind:?}"),
            };
            json!({ "type": kind, "value": token.text_trimmed() })
        }
        AnyTomlValue::TomlArray(array) => Value::Array(
            array
                .elements()
                .iter()
                .map(|element| decode_value(&element.unwrap()))
                .collect(),
        ),
        AnyTomlValue::TomlInlineTable(inline_table) => {
            let mut table = Map::new();
            for entry in inline_table.entries().iter() {
                if let AnyTomlKeyValue::TomlKeyValue(key_value) = entry.unwrap() {
                    insert_key_value(&mut table, &key_value);
                }
            }
            Value::Object(table)
        }
        AnyTomlValue::TomlBogusValue(_) => unreachable!(),
    }
}

/// Checks the formatting of the files listed in `TOML_FORMAT_FILES`, separated
/// by `:`, such as the downloaded benchmark inputs.
#[ignore]
#[test]
fn formatting_preserves_files() {
    let files = std::env::var("TOML_FORMAT_FILES").unwrap();
    for file in files.split(':') {
        let source = std::fs::read_to_string(file).unwrap();
        let original = parse_toml(&source);
        assert!(!original.has_errors(), "{file}");
        for options in [
            TomlFormatOptions::default(),
            TomlFormatOptions::default().with_expand(Expand::Always),
        ] {
            let formatted = format(&source, &options);
            let parse = parse_toml(&formatted);
            assert!(!parse.has_errors(), "{file}: {:#?}", parse.diagnostics());
            assert!(
                decode_document(&parse.tree()) == decode_document(&original.tree()),
                "{file}"
            );
            assert_eq!(
                comments(&parse.syntax()),
                comments(&original.syntax()),
                "{file}"
            );
            assert_eq!(format(&formatted, &options), formatted, "{file}");
            println!(
                "{file}: {} -> {} bytes, {} changed lines",
                source.len(),
                formatted.len(),
                source
                    .lines()
                    .zip(formatted.lines())
                    .filter(|(a, b)| a != b)
                    .count()
            );
        }
    }
}

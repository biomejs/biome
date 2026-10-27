//! Runs the TOML 1.1.0 cases of [toml-test](https://github.com/toml-lang/toml-test).
//!
//! Valid documents must parse without diagnostics and decode to the values in
//! the sibling `.json` file. Invalid documents must produce a diagnostic.

use biome_rowan::AstSeparatedList;
use biome_toml_parser::parse_toml;
use biome_toml_syntax::{
    AnyTomlKeyValue, AnyTomlRootItem, AnyTomlValue, TomlKey, TomlKeyValue, TomlSyntaxKind,
};
use serde_json::{Map, Value, json};
use std::fmt::Write;
use std::path::Path;

const SUITE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/toml-test");

#[test]
fn toml_test_suite() {
    let suite = Path::new(SUITE);
    let files = std::fs::read_to_string(suite.join("files-toml-1.1.0")).unwrap();
    let mut failures = String::new();
    let mut valid = 0;
    let mut invalid = 0;

    for file in files.lines().filter(|file| file.ends_with(".toml")) {
        let bytes = std::fs::read(suite.join(file)).unwrap();

        if file.starts_with("valid/") {
            valid += 1;
            let source = match String::from_utf8(bytes) {
                Ok(source) => source,
                Err(_) => {
                    writeln!(failures, "{file}: valid document isn't UTF-8").unwrap();
                    continue;
                }
            };

            let parse = parse_toml(&source);
            assert_eq!(
                parse.syntax().to_string(),
                source,
                "{file}: tree isn't lossless"
            );

            if parse.has_errors() {
                let messages: Vec<_> = parse
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| format!("{:?}", diagnostic))
                    .collect();
                writeln!(failures, "{file}: rejected valid document: {messages:#?}").unwrap();
                continue;
            }

            let expected_path = suite.join(file.replace(".toml", ".json"));
            let expected: Value =
                serde_json::from_str(&std::fs::read_to_string(expected_path).unwrap()).unwrap();
            let actual = decode_document(&parse.tree());
            if let Err(error) = compare(&expected, &actual, "$") {
                writeln!(
                    failures,
                    "{file}: decoded value mismatch at {error}\nexpected: {expected}\nactual:   {actual}"
                )
                .unwrap();
            }
        } else {
            invalid += 1;
            // Invalid UTF-8 can't be represented as `&str` and is rejected
            // before parsing.
            let Ok(source) = String::from_utf8(bytes) else {
                continue;
            };

            let parse = parse_toml(&source);
            assert_eq!(
                parse.syntax().to_string(),
                source,
                "{file}: tree isn't lossless"
            );

            if !parse.has_errors() {
                writeln!(failures, "{file}: accepted invalid document").unwrap();
            }
        }
    }

    assert!(valid > 200 && invalid > 400, "the suite is incomplete");
    assert!(failures.is_empty(), "toml-test failures:\n{failures}");
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

fn compare(expected: &Value, actual: &Value, path: &str) -> Result<(), String> {
    match (expected, actual) {
        (Value::Object(expected), Value::Object(actual))
            if expected.contains_key("type") && expected.contains_key("value") =>
        {
            let kind = expected["type"].as_str().unwrap();
            if actual.get("type").and_then(Value::as_str) != Some(kind) {
                return Err(format!("{path}: expected type {kind}"));
            }
            let expected_value = expected["value"].as_str().unwrap();
            let actual_value = actual["value"].as_str().unwrap();
            let equal = match kind {
                "float" => {
                    let expected = parse_float(expected_value);
                    let actual = parse_float(actual_value);
                    (expected.is_nan() && actual.is_nan()) || expected == actual
                }
                "datetime" | "datetime-local" | "date-local" | "time-local" => {
                    normalize_date_time(expected_value) == normalize_date_time(actual_value)
                }
                _ => expected_value == actual_value,
            };
            if equal {
                Ok(())
            } else {
                Err(format!(
                    "{path}: expected {expected_value:?}, found {actual_value:?}"
                ))
            }
        }
        (Value::Object(expected), Value::Object(actual)) => {
            if expected.len() != actual.len() {
                return Err(format!(
                    "{path}: expected keys {:?}, found {:?}",
                    expected.keys().collect::<Vec<_>>(),
                    actual.keys().collect::<Vec<_>>()
                ));
            }
            for (key, expected) in expected {
                let Some(actual) = actual.get(key) else {
                    return Err(format!("{path}: missing key {key:?}"));
                };
                compare(expected, actual, &format!("{path}.{key}"))?;
            }
            Ok(())
        }
        (Value::Array(expected), Value::Array(actual)) => {
            if expected.len() != actual.len() {
                return Err(format!("{path}: array length differs"));
            }
            for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
                compare(expected, actual, &format!("{path}[{index}]"))?;
            }
            Ok(())
        }
        _ => Err(format!("{path}: structure differs")),
    }
}

fn parse_float(text: &str) -> f64 {
    match text {
        "inf" | "+inf" => f64::INFINITY,
        "-inf" => f64::NEG_INFINITY,
        "nan" | "+nan" | "-nan" => f64::NAN,
        text => text.parse().unwrap(),
    }
}

/// Normalizes the representation choices TOML allows: the date/time
/// delimiter, the case of `Z`, omitted seconds, and trailing zeros of the
/// fractional seconds.
fn normalize_date_time(text: &str) -> String {
    let mut text = text.to_ascii_uppercase();
    if text.len() > 10 && text.as_bytes()[4] == b'-' {
        text.replace_range(10..11, "T");
    }

    let time_start = if text.as_bytes().get(2) == Some(&b':') {
        Some(0)
    } else if text.len() > 10 {
        Some(11)
    } else {
        None
    };

    if let Some(start) = time_start {
        // `HH:MM` without seconds.
        if text.len() == start + 5 || !text[start + 5..].starts_with(':') {
            text.insert_str(start + 5, ":00");
        }

        let fraction_start = start + 8;
        if text[fraction_start..].starts_with('.') {
            let digits_end = text[fraction_start + 1..]
                .find(|char: char| !char.is_ascii_digit())
                .map_or(text.len(), |index| fraction_start + 1 + index);
            let digits = text[fraction_start + 1..digits_end].trim_end_matches('0');
            let replacement = if digits.is_empty() {
                String::new()
            } else {
                format!(".{digits}")
            };
            text.replace_range(fraction_start..digits_end, &replacement);
        }
    }

    text
}

/// Parses every prefix of every valid document. Truncation exercises the error
/// recovery of every construct; parsing must not panic and must stay lossless.
#[test]
fn truncated_documents_parse_losslessly() {
    let suite = Path::new(SUITE);
    let files = std::fs::read_to_string(suite.join("files-toml-1.1.0")).unwrap();

    for file in files
        .lines()
        .filter(|file| file.starts_with("valid/") && file.ends_with(".toml"))
    {
        let source = std::fs::read_to_string(suite.join(file)).unwrap();
        for (end, _) in source.char_indices().skip(1) {
            let prefix = &source[..end];
            let parse = parse_toml(prefix);
            assert_eq!(
                parse.syntax().to_string(),
                prefix,
                "{file}: prefix of {end} bytes"
            );
        }
    }
}

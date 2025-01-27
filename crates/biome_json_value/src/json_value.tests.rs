use biome_deserialize::json::deserialize_from_json_str;
use biome_json_parser::JsonParserOptions;

use super::*;

#[test]
fn test_string_escaped_newline() {
    let deserialized = deserialize_from_json_str(
        r#"{ "foo": "multi\nline" }"#,
        JsonParserOptions::default(),
        "test_json",
    );
    let object: JsonObject = deserialized.into_deserialized().expect("should parse");
    assert_eq!(
        object.get("foo").unwrap().as_string().unwrap(),
        &JsonString::from("multi\nline")
    )
}

#[test]
fn test_string_escaped_unicode_sequence() {
    let deserialized = deserialize_from_json_str(
        r#"{ "foo": "\u1234" }"#,
        JsonParserOptions::default(),
        "test_json",
    );
    let object: JsonObject = deserialized.into_deserialized().expect("should parse");
    assert_eq!(
        object.get("foo").unwrap().as_string().unwrap(),
        &JsonString::from("\u{1234}")
    )
}

#[test]
fn test_string_combined_sequences() {
    let deserialized = deserialize_from_json_str(
        r#"{ "foo": "Here's some escape sequences:\n\u1234 \\ \/" }"#,
        JsonParserOptions::default(),
        "test_json",
    );
    let object: JsonObject = deserialized.into_deserialized().expect("should parse");
    assert_eq!(
        object.get("foo").unwrap().as_string().unwrap(),
        &JsonString::from("Here's some escape sequences:\n\u{1234} \\ /")
    )
}

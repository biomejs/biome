use biome_json_parser::{JsonParserOptions, parse_json};
use biome_provenance::{ProvenanceContext, ProvenanceTrackable};
use biome_provenance_macros::ProvenanceTrackable;
use camino::Utf8PathBuf;

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct SimpleStruct {
    name: String,
    count: u32,
    enabled: bool,
}

#[test]
fn test_simple_struct() {
    let json = r#"{"name": "test", "count": 42, "enabled": true}"#;
    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = SimpleStruct::from_source_with_provenance(&value, &mut ctx).unwrap();

    assert_eq!(result.name, "test");
    assert_eq!(result.count, 42);
    assert_eq!(result.enabled, true);

    let entries = ctx.into_entries();
    assert_eq!(entries.len(), 3); // 3 fields captured
}

#[test]
fn test_missing_field() {
    let json = r#"{"name": "test", "count": 42}"#; // missing "enabled"
    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = SimpleStruct::from_source_with_provenance(&value, &mut ctx);

    // Should succeed with default value for missing field
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.name, "test");
    assert_eq!(result.count, 42);
    assert_eq!(result.enabled, false); // Default value
}

#[test]
fn test_invalid_type() {
    let json = r#"{"name": "test", "count": "invalid", "enabled": true}"#;
    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = SimpleStruct::from_source_with_provenance(&value, &mut ctx);

    // Should fail because "count" can't be parsed as u32
    assert!(result.is_none());
}

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct NestedStruct {
    outer_field: String,
    inner: InnerStruct,
}

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct InnerStruct {
    inner_value: String,
    inner_count: u32,
}

#[test]
fn test_nested_struct() {
    let json = r#"{
        "outer_field": "outer",
        "inner": {
            "inner_value": "nested",
            "inner_count": 99
        }
    }"#;

    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = NestedStruct::from_source_with_provenance(&value, &mut ctx).unwrap();

    assert_eq!(result.outer_field, "outer");
    assert_eq!(result.inner.inner_value, "nested");
    assert_eq!(result.inner.inner_count, 99);

    let entries = ctx.into_entries();
    // Should capture: outerField, inner.innerValue, inner.innerCount
    assert_eq!(entries.len(), 3);

    // Check paths
    assert!(entries.iter().any(|e| e.field_path == "outer_field"));
    assert!(entries.iter().any(|e| e.field_path == "inner.inner_value"));
    assert!(entries.iter().any(|e| e.field_path == "inner.inner_count"));
}

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct VecStruct {
    items: Vec<String>,
    numbers: Vec<u32>,
}

#[test]
fn test_vec_fields() {
    let json = r#"{
        "items": ["apple", "banana", "cherry"],
        "numbers": [1, 2, 3]
    }"#;

    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = VecStruct::from_source_with_provenance(&value, &mut ctx).unwrap();

    assert_eq!(result.items, vec!["apple", "banana", "cherry"]);
    assert_eq!(result.numbers, vec![1, 2, 3]);

    let entries = ctx.into_entries();
    // Should capture: items[0], items[1], items[2], numbers[0], numbers[1], numbers[2]
    assert_eq!(entries.len(), 6);
}

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct OptionalStruct {
    required_field: String,
    optional_field: Option<String>,
}

#[test]
fn test_option_some() {
    let json = r#"{"required_field": "req", "optional_field": "opt"}"#;
    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = OptionalStruct::from_source_with_provenance(&value, &mut ctx).unwrap();

    assert_eq!(result.required_field, "req");
    assert_eq!(result.optional_field, Some("opt".to_string()));
}

#[test]
fn test_option_missing() {
    let json = r#"{"required_field": "req"}"#; // optional_field is missing
    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("test.json"), 0);
    let result = OptionalStruct::from_source_with_provenance(&value, &mut ctx).unwrap();

    assert_eq!(result.required_field, "req");
    assert_eq!(result.optional_field, None); // Should use default (None)
}

//! End-to-end integration test demonstrating provenance tracking
//! with a simplified configuration structure

use biome_json_parser::{JsonParserOptions, parse_json};
use biome_provenance::{ProvenanceContext, ProvenanceIndex, ProvenanceTrackable, ProvenanceValue};
use biome_provenance_macros::ProvenanceTrackable;
use camino::Utf8PathBuf;

/// Simplified formatter configuration for demonstration
#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct FormatterConfig {
    enabled: Option<bool>,
    indent_width: Option<u32>,
    indent_style: Option<String>,
    line_width: Option<u32>,
}

/// Simplified linter configuration
#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct LinterConfig {
    enabled: Option<bool>,
    rules: Option<RulesConfig>,
}

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct RulesConfig {
    recommended: Option<bool>,
}

/// Simplified Biome configuration
#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct BiomeConfig {
    formatter: Option<FormatterConfig>,
    linter: Option<LinterConfig>,
    files: Option<FilesConfig>,
}

#[derive(Debug, Default, ProvenanceTrackable, PartialEq)]
struct FilesConfig {
    ignore: Option<Vec<String>>,
    include: Option<Vec<String>>,
}

#[test]
fn test_simple_config_provenance() {
    let json = r#"{
        "formatter": {
            "enabled": true,
            "indent_width": 4,
            "indent_style": "space"
        }
    }"#;

    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("biome.json"), 0);
    let config = BiomeConfig::from_source_with_provenance(&value, &mut ctx).unwrap();

    // Verify config was parsed correctly
    assert!(config.formatter.is_some());
    let formatter = config.formatter.as_ref().unwrap();
    assert_eq!(formatter.enabled, Some(true));
    assert_eq!(formatter.indent_width, Some(4));
    assert_eq!(formatter.indent_style, Some("space".to_string()));

    // Verify provenance was captured
    let index = ProvenanceIndex::new(ctx.into_entries());

    // Check formatter.enabled provenance
    let entry = index.query("formatter.enabled").unwrap();
    assert_eq!(entry.source_value, ProvenanceValue::Boolean(true));
    assert_eq!(entry.source_path, Utf8PathBuf::from("biome.json"));

    // Check formatter.indent_width provenance
    let entry = index.query("formatter.indent_width").unwrap();
    assert_eq!(entry.source_value, ProvenanceValue::Number("4".to_string()));

    // Check formatter.indent_style provenance
    let entry = index.query("formatter.indent_style").unwrap();
    assert_eq!(
        entry.source_value,
        ProvenanceValue::String("space".to_string())
    );
}

#[test]
fn test_nested_config_provenance() {
    let json = r#"{
        "linter": {
            "enabled": true,
            "rules": {
                "recommended": true
            }
        }
    }"#;

    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("biome.json"), 0);
    let config = BiomeConfig::from_source_with_provenance(&value, &mut ctx).unwrap();

    // Verify config
    assert!(config.linter.is_some());
    let linter = config.linter.as_ref().unwrap();
    assert_eq!(linter.enabled, Some(true));
    assert!(linter.rules.is_some());
    assert_eq!(linter.rules.as_ref().unwrap().recommended, Some(true));

    // Verify provenance
    let index = ProvenanceIndex::new(ctx.into_entries());

    let entry = index.query("linter.enabled").unwrap();
    assert_eq!(entry.source_value, ProvenanceValue::Boolean(true));

    let entry = index.query("linter.rules.recommended").unwrap();
    assert_eq!(entry.source_value, ProvenanceValue::Boolean(true));
}

#[test]
fn test_array_config_provenance() {
    let json = r#"{
        "files": {
            "ignore": ["node_modules", "dist", ".git"],
            "include": ["src/**/*.ts", "tests/**/*.ts"]
        }
    }"#;

    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("biome.json"), 0);
    let config = BiomeConfig::from_source_with_provenance(&value, &mut ctx).unwrap();

    // Verify config
    let files = config.files.as_ref().unwrap();
    assert_eq!(
        files.ignore,
        Some(vec![
            "node_modules".to_string(),
            "dist".to_string(),
            ".git".to_string()
        ])
    );
    assert_eq!(
        files.include,
        Some(vec!["src/**/*.ts".to_string(), "tests/**/*.ts".to_string()])
    );

    // Verify provenance for array elements
    let index = ProvenanceIndex::new(ctx.into_entries());

    let entry = index.query("files.ignore[0]").unwrap();
    assert_eq!(
        entry.source_value,
        ProvenanceValue::String("node_modules".to_string())
    );

    let entry = index.query("files.ignore[1]").unwrap();
    assert_eq!(
        entry.source_value,
        ProvenanceValue::String("dist".to_string())
    );

    let entry = index.query("files.include[0]").unwrap();
    assert_eq!(
        entry.source_value,
        ProvenanceValue::String("src/**/*.ts".to_string())
    );
}

#[test]
fn test_merge_order() {
    // Simulate extends chain: base.json (order=0) extended by biome.json (order=1)

    // Base config
    let base_json = r#"{
        "formatter": {
            "indent_width": 2,
            "indent_style": "tab"
        }
    }"#;

    let parsed = parse_json(base_json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx_base = ProvenanceContext::new(Utf8PathBuf::from("base.json"), 0);
    let _base_config = BiomeConfig::from_source_with_provenance(&value, &mut ctx_base).unwrap();
    let mut base_entries = ctx_base.into_entries();

    // Extended config (overrides indent_width)
    let extended_json = r#"{
        "formatter": {
            "indent_width": 4
        }
    }"#;

    let parsed = parse_json(extended_json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx_extended = ProvenanceContext::new(Utf8PathBuf::from("biome.json"), 1);
    let _extended_config =
        BiomeConfig::from_source_with_provenance(&value, &mut ctx_extended).unwrap();
    let extended_entries = ctx_extended.into_entries();

    // Merge entries (in real implementation, this would happen during config loading)
    base_entries.extend(extended_entries);
    let index = ProvenanceIndex::new(base_entries);

    // Query should return the entry with highest merge_order (most recent)
    let entry = index.query("formatter.indent_width").unwrap();
    assert_eq!(entry.source_value, ProvenanceValue::Number("4".to_string()));
    assert_eq!(entry.source_path, Utf8PathBuf::from("biome.json"));
    assert_eq!(entry.merge_order, 1);

    // indent_style only exists in base config
    let entry = index.query("formatter.indent_style").unwrap();
    assert_eq!(
        entry.source_value,
        ProvenanceValue::String("tab".to_string())
    );
    assert_eq!(entry.source_path, Utf8PathBuf::from("base.json"));
    assert_eq!(entry.merge_order, 0);
}

#[test]
fn test_complete_config() {
    let json = r#"{
        "formatter": {
            "enabled": true,
            "indent_width": 4,
            "indent_style": "space",
            "line_width": 80
        },
        "linter": {
            "enabled": true,
            "rules": {
                "recommended": true
            }
        },
        "files": {
            "ignore": ["node_modules", "dist"],
            "include": ["src/**/*.ts"]
        }
    }"#;

    let parsed = parse_json(json, JsonParserOptions::default());
    let root = parsed.tree();
    let value = root.value().ok().unwrap();

    let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("biome.json"), 0);
    let config = BiomeConfig::from_source_with_provenance(&value, &mut ctx).unwrap();

    // Verify all sections parsed
    assert!(config.formatter.is_some());
    assert!(config.linter.is_some());
    assert!(config.files.is_some());

    let index = ProvenanceIndex::new(ctx.into_entries());
    let all_entries = index.all_entries();

    // We should have entries for:
    // - formatter.enabled
    // - formatter.indent_width
    // - formatter.indent_style
    // - formatter.line_width
    // - linter.enabled
    // - linter.rules.recommended
    // - files.ignore[0], files.ignore[1]
    // - files.include[0]
    // = 9 total entries
    assert_eq!(all_entries.len(), 9);

    // Verify we can query any field
    assert!(index.query("formatter.line_width").is_some());
    assert!(index.query("linter.rules.recommended").is_some());
    assert!(index.query("files.ignore[1]").is_some());

    // Verify non-existent field returns None
    assert!(index.query("non.existent.field").is_none());
}

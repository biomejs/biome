use biome_toml_parser::parse_toml;
use biome_toml_syntax::{
    AnyTomlItem,
    TomlSyntaxKind::{R_BRACK, TOML_ARRAY, TOML_INLINE_TABLE},
};

#[test]
fn deeply_nested_values_parse_without_overflowing_the_stack() {
    let mut value = "0".to_string();
    for depth in 0..256 {
        value = if depth % 2 == 0 {
            format!("[{value}]")
        } else {
            format!("{{ nested = {value} }}")
        };
    }
    let source = format!("value = {value}\nafter = true");
    let parsed = parse_toml(&source);

    assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
    assert_eq!(
        parsed
            .tree()
            .items()
            .into_iter()
            .filter(|item| matches!(item, AnyTomlItem::TomlKeyValue(_)))
            .count(),
        2
    );
}

#[test]
fn unterminated_inline_table_preserves_the_next_item() {
    let parsed = parse_toml("broken = { a = 1\nafter = true");

    assert!(parsed.has_errors());
    assert_eq!(
        parsed
            .tree()
            .items()
            .into_iter()
            .filter(|item| matches!(item, AnyTomlItem::TomlKeyValue(_)))
            .count(),
        2
    );
}

#[test]
fn unterminated_array_preserves_the_next_item() {
    let parsed = parse_toml("broken = [1\nafter = true");

    assert!(parsed.has_errors());
    assert_eq!(
        parsed
            .tree()
            .items()
            .into_iter()
            .filter(|item| matches!(item, AnyTomlItem::TomlKeyValue(_)))
            .count(),
        2
    );
}

#[test]
fn unterminated_empty_array_preserves_the_next_item() {
    for source in [
        "broken = [\nafter = true",
        "broken = [\n\"after\" = true",
        "broken = [\nafter.key = true",
    ] {
        let parsed = parse_toml(source);

        assert!(parsed.has_errors(), "source: {source:?}");
        assert_eq!(
            parsed
                .tree()
                .items()
                .into_iter()
                .filter(|item| matches!(item, AnyTomlItem::TomlKeyValue(_)))
                .count(),
            2,
            "source: {source:?}"
        );
    }
}

#[test]
fn line_broken_dotted_key_preserves_the_next_item() {
    let parsed = parse_toml("broken.\nafter = true");

    assert!(parsed.has_errors());
    assert_eq!(
        parsed
            .tree()
            .items()
            .into_iter()
            .filter(|item| matches!(item, AnyTomlItem::TomlKeyValue(_)))
            .count(),
        2
    );
}

#[test]
fn malformed_containers_preserve_the_next_item() {
    for source in ["broken = [}\nafter = true", "broken = {]\nafter = true"] {
        let parsed = parse_toml(source);

        assert!(parsed.has_errors(), "source: {source:?}");
        assert_eq!(
            parsed
                .tree()
                .items()
                .into_iter()
                .filter(|item| matches!(item, AnyTomlItem::TomlKeyValue(_)))
                .count(),
            2,
            "source: {source:?}"
        );
    }
}

#[test]
fn malformed_nested_container_preserves_the_parent_delimiter() {
    let parsed = parse_toml("value = [{ key = 1 ]");

    assert!(parsed.has_errors());
    let array = parsed
        .syntax()
        .descendants()
        .find(|node| node.kind() == TOML_ARRAY)
        .unwrap();
    assert!(
        array
            .children_with_tokens()
            .any(|element| element.kind() == R_BRACK)
    );
}

#[test]
fn unterminated_containers_preserve_the_next_table() {
    for source in [
        "broken = [1\n[table]\nafter = true",
        "broken = [\n[table]\nafter = true",
        "broken = [1,\n[table]\nafter = true",
        "broken = { value = 1\n[table]\nafter = true",
        "broken = {\n[table]\nafter = true",
        "broken = { value = 1,\n[table]\nafter = true",
    ] {
        let parsed = parse_toml(source);

        assert!(parsed.has_errors(), "source: {source:?}");
        assert_eq!(
            parsed
                .tree()
                .items()
                .into_iter()
                .filter(|item| matches!(item, AnyTomlItem::TomlTable(_)))
                .count(),
            1,
            "source: {source:?}"
        );
    }
}

#[test]
fn accepts_nested_arrays_that_resemble_table_headers() {
    let parsed = parse_toml("value = [\n[true],\n[1.2],\n[1979-05-27],\n[\"table\"],\n[[1]],\n]");

    assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
}

#[test]
fn accepts_multiline_inline_tables_with_trailing_commas() {
    let parsed = parse_toml(
        "value = {\n  # comment\n  first = 1, # comment\n  nested = { second = 2, },\n}\nafter = true",
    );

    assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
    assert_eq!(
        parsed
            .syntax()
            .descendants()
            .filter(|node| node.kind() == TOML_INLINE_TABLE)
            .count(),
        2
    );
}

#[test]
fn accepts_conformance_boundaries() {
    for source in [
        "\u{feff}value = true",
        "values = [\n  1,\n  2, # comment\n]",
        r#"value = """"one quote"""""#,
        "value = ''''one quote''''",
        r#"value = """
Closing with five quotes
""""""#,
    ] {
        let parsed = parse_toml(source);
        assert!(!parsed.has_errors(), "source: {source:?}");
    }
}

#[test]
fn malformed_inline_table_entry_preserves_the_typed_table() {
    let parsed = parse_toml("inline = { a = 1, [b], c = 2 }");

    assert!(parsed.has_errors());
    assert_eq!(
        parsed
            .syntax()
            .descendants()
            .filter(|node| node.kind() == TOML_INLINE_TABLE)
            .count(),
        1
    );
}

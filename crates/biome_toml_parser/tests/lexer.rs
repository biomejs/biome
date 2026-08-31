use biome_toml_parser::parse_toml;
use biome_toml_syntax::TomlSyntaxKind::TOML_KEY_VALUE;

#[test]
fn rejects_forbidden_control_characters_and_bare_carriage_returns() {
    for source in [
        "value = 1\rnext = 2",
        "# comment\u{7f}\nvalue = 1",
        "value = \"\u{7f}\"",
    ] {
        assert!(parse_toml(source).has_errors(), "source: {source:?}");
    }
}

#[test]
fn accepts_whitespace_before_multiline_string_continuations() {
    let source = "value = \"\"\"first\\   \n  second\"\"\"";
    let parsed = parse_toml(source);

    assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
}

#[test]
fn escaped_newline_in_single_line_string_preserves_the_next_item() {
    let source = "bad = \"value\\\nafter = true";
    let parsed = parse_toml(source);

    assert!(parsed.has_errors());
    assert_eq!(
        parsed
            .syntax()
            .descendants()
            .filter(|node| node.kind() == TOML_KEY_VALUE)
            .count(),
        2
    );
}

#[test]
fn rejects_signs_in_date_time_components() {
    for source in [
        "value = +001-01-01",
        "value = +1:00:00",
        "value = 1979-05-27T00:00:00++1:00",
    ] {
        assert!(parse_toml(source).has_errors(), "source: {source:?}");
    }
}

#[test]
fn accepts_escape_and_hexadecimal_string_escapes() {
    for source in [
        r#"value = "\e""#,
        r#"value = "\xE9""#,
        r#""\x6b\x65\x79" = true"#,
        r#"value = """\e\x1B""""#,
    ] {
        let parsed = parse_toml(source);
        assert!(!parsed.has_errors(), "source: {source:?}");
    }
}

#[test]
fn rejects_malformed_hexadecimal_escapes() {
    for source in [r#"value = "\x""#, r#"value = "\x0""#, r#"value = "\xGG""#] {
        assert!(parse_toml(source).has_errors(), "source: {source:?}");
    }
}

#[test]
fn accepts_times_without_seconds() {
    for source in [
        "value = 07:32",
        "value = 1979-05-27T07:32",
        "value = 1979-05-27 07:32Z",
        "value = 1979-05-27 07:32-07:00",
    ] {
        let parsed = parse_toml(source);
        assert!(!parsed.has_errors(), "source: {source:?}");
    }
}

#[test]
fn rejects_fractional_minutes() {
    for source in [
        "value = 07:32.5",
        "value = 1979-05-27T07:32.5",
        "value = 1979-05-27 07:32.5Z",
        "value = 1979-05-27 07:32.5-07:00",
    ] {
        assert!(parse_toml(source).has_errors(), "source: {source:?}");
    }
}

#[test]
fn validates_offset_leap_second_instants() {
    for source in [
        "value = 1990-12-31T23:59:60Z",
        "value = 1990-12-31T15:59:60-08:00",
        "value = 1991-01-01T00:59:60+01:00",
    ] {
        let parsed = parse_toml(source);
        assert!(!parsed.has_errors(), "source: {source:?}");
    }

    for source in [
        "value = 1979-05-27T00:00:60Z",
        "value = 1990-12-31T23:58:60Z",
        "value = 1990-12-31T23:59:60+01:00",
    ] {
        assert!(parse_toml(source).has_errors(), "source: {source:?}");
    }
}

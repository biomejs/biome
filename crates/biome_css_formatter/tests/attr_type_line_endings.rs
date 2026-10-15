use biome_css_formatter::{context::CssFormatOptions, format_node};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_formatter::LineEnding;
use biome_languages::CssFileSource;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

fn format_round_trip(source: &str, file_source: CssFileSource, line_ending: LineEnding) -> String {
    assert!(source.contains("\r\n"));
    let parsed = parse_css(source, file_source, CssParserOptions::default());
    assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
    assert_eq!(parsed.syntax().to_string(), source);
    assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));

    let options = CssFormatOptions::new(file_source).with_line_ending(line_ending);
    let output = format_node(options.clone(), &parsed.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();
    let normalized = output.replace("\r\n", "\n");
    assert!(!normalized.contains('\r'), "{output:?}");
    assert_eq!(output, normalized.replace('\n', line_ending.as_str()));

    let reparsed = parse_css(&output, file_source, CssParserOptions::default());
    assert!(!reparsed.has_errors(), "{:?}", reparsed.diagnostics());
    assert_eq!(reparsed.syntax().to_string(), output);
    assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
    let second = format_node(options, &reparsed.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code();
    assert_eq!(output, second);
    output
}

fn assert_stored_type_line_ending(line_ending: LineEnding) {
    for (source, preserved) in [
        ("$t: type(\r\n<length>);\r\n", "type(\n<length>)"),
        ("$t: type(  \r\n<length>  );\r\n", "type(  \n<length>  )"),
    ] {
        let output = format_round_trip(source, CssFileSource::scss(), line_ending);
        assert!(output.contains(&preserved.replace('\n', line_ending.as_str())));
    }
}

#[test]
fn stored_type_crlf_source_uses_lf_output() {
    assert_stored_type_line_ending(LineEnding::Lf);
}

#[test]
fn stored_type_crlf_source_uses_crlf_output() {
    assert_stored_type_line_ending(LineEnding::Crlf);
}

#[test]
fn stored_type_preserves_unicode_separator_data() {
    let source = "$t: type(\r\n<length\u{2028}part\u{2029}end>);\r\n";
    for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
        let output = format_round_trip(source, CssFileSource::scss(), line_ending);
        assert!(output.contains("<length\u{2028}part\u{2029}end>"));
        assert_eq!(output.matches('\u{2028}').count(), 1);
        assert_eq!(output.matches('\u{2029}').count(), 1);
    }
}

#[test]
fn stored_type_keeps_inner_and_outer_comments_once() {
    let source = concat!(
        "$t: /* before */ type(\r\n",
        "/* inside */<length>) /* after */;\r\n",
        ".a { width: attr(data-width $t, 10px); }\r\n",
    );
    for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
        let output = format_round_trip(source, CssFileSource::scss(), line_ending);
        for comment in ["/* before */", "/* inside */", "/* after */"] {
            assert_eq!(output.matches(comment).count(), 1, "{output}");
        }
        assert!(output.find("/* before */").unwrap() < output.find("type(").unwrap());
        assert!(output.find("<length>)").unwrap() < output.find("/* after */").unwrap());
        assert!(output.contains(&format!(
            "type({}/* inside */<length>)",
            line_ending.as_str()
        )));
    }
}

#[test]
fn inline_css_type_keeps_structured_formatting() {
    let source = ".a { width: attr(data-width type(\r\n  <length>\r\n), 10px); }\r\n";
    for file_source in [CssFileSource::css(), CssFileSource::scss()] {
        for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
            let output = format_round_trip(source, file_source, line_ending);
            assert!(output.contains("attr(data-width type(<length>), 10px)"));
        }
    }
}

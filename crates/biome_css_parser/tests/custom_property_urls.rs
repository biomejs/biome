use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssSyntaxKind, CssUrlFunction};
use biome_languages::CssFileSource;
use biome_rowan::AstNode;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[test]
fn raw_custom_property_urls_use_url_nodes() {
    for value in [
        "url(data:image/svg+xml,%3Csvg/%3E)",
        "url($asset)",
        "url(/icons/$asset.svg)",
        "url(/*payload*/mark.svg)",
        "url(//cdn.example/icon.svg)",
        "url([mark].svg)",
        "UrL(/icons/mark.svg)",
        r"u\72l(/icons/mark.svg)",
    ] {
        let source = format!(".icon {{ --image: {value}; color: red; }}");
        let parsed = parse_css(&source, CssFileSource::scss(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
        let url = parsed.syntax().descendants().find_map(CssUrlFunction::cast);
        let url = url.unwrap_or_else(|| panic!("expected a URL node for {value}"));
        assert!(url.value().unwrap().as_css_url_value_raw().is_some());
        assert!(
            !url.syntax()
                .descendants()
                .any(|node| node.kind() == CssSyntaxKind::SCSS_EXPRESSION)
        );
    }
}

#[test]
fn custom_property_url_fallback_keeps_raw_expressions() {
    for value in ["url()", "url(var(--image))", r#"url($asset + ".svg")"#] {
        let source = format!(".icon {{ --image: {value}; color: red; }}");
        let parsed = parse_css(&source, CssFileSource::scss(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
        assert!(parsed.syntax().descendants().all(|node| !matches!(
            node.kind(),
            CssSyntaxKind::CSS_URL_FUNCTION | CssSyntaxKind::SCSS_VARIABLE
        )));
    }
}

#[test]
fn raw_custom_property_urls_only_parse_explicit_interpolation() {
    for value in [
        "url(/icons/#{$asset}.svg)",
        "url($literal-#{$asset}.svg)",
        "url(/*payload*/#{$asset}.svg)",
    ] {
        let source = format!(".icon {{ --image: {value}; color: red; }}");
        let parsed = parse_css(&source, CssFileSource::scss(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
        let url = parsed.syntax().descendants().find_map(CssUrlFunction::cast);
        let url = url.unwrap_or_else(|| panic!("expected a URL node for {value}"));
        assert!(
            url.value()
                .unwrap()
                .as_scss_interpolated_url_value()
                .is_some()
        );
        assert_eq!(
            url.syntax()
                .descendants()
                .filter(|node| node.kind() == CssSyntaxKind::SCSS_INTERPOLATION)
                .count(),
            1,
            "{value}"
        );
    }
}

use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::CssSyntaxKind;
use biome_diagnostics::print_diagnostic_to_string;
use biome_languages::CssFileSource;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

const INTERPOLATION_CASES: &[&str] = &[
    ".a { value: #{name}; }",
    ".a { value: prefix-#{name}-suffix; }",
    ".a { value: #{name}(1); }",
    ".a { value: url(\"a.png\" #{modifier}(woff2)); }",
    "@supports #{condition} {}",
    "@supports (display: grid) and #{condition} {}",
    "@media #{query} {}",
    "@media #{query} and not (color) {}",
    "@media screen and #{query} {}",
    "@media not #{query} {}",
    "@media (#{query} and (color)) {}",
    "@media (#{feature}: 1px) {}",
    "@media (min-#{feature}: 1px) {}",
    "@media (width: #{value}) {}",
    "@container (#{feature}: 1px) {}",
    ":lang(#{locale}) {}",
    ":lang(en-#{locale}) {}",
];

#[test]
fn unsupported_interpolation_reaches_exclusive_gates() {
    for source in INTERPOLATION_CASES {
        let source = format!("{source}\n.after {{ color: red; }}");
        let parsed = parse_css(
            &source,
            CssFileSource::css(),
            CssParserOptions::default().report_scss_exclusive_syntax(),
        );
        let diagnostics: Vec<_> = parsed
            .diagnostics()
            .iter()
            .map(|diagnostic| print_diagnostic_to_string(&diagnostic.clone().into()))
            .collect();
        assert_eq!(diagnostics.len(), 1, "{source}\n{diagnostics:#?}");
        assert!(
            diagnostics[0].contains("SCSS only feature"),
            "{source}\n{diagnostics:#?}"
        );
        assert!(parsed.syntax().descendants().any(|node| {
            node.kind() == CssSyntaxKind::CSS_QUALIFIED_RULE
                && node.text_trimmed() == ".after { color: red; }"
        }));

        let default = parse_css(&source, CssFileSource::css(), CssParserOptions::default());
        assert_eq!(default.diagnostics().len(), parsed.diagnostics().len());
        assert_eq!(
            format!("{:#?}", default.syntax()),
            format!("{:#?}", parsed.syntax()),
            "diagnostic reporting must preserve the recovery tree for {source}"
        );
        assert!(default.diagnostics().iter().all(|diagnostic| {
            !print_diagnostic_to_string(&diagnostic.clone().into()).contains("SCSS")
        }));
    }
}

#[test]
fn supported_interpolation_keeps_valid_trees() {
    for source in INTERPOLATION_CASES {
        let parsed = parse_css(source, CssFileSource::scss(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}\n{:?}", parsed.diagnostics());
        assert!(
            !has_bogus_nodes_or_empty_slots(&parsed.syntax()),
            "{source}\n{:#?}",
            parsed.syntax()
        );
    }
}

#[test]
fn css_fallbacks_remain_valid() {
    for source in [
        ":lang(en, \"de\") {}",
        "@media (width: 1px) {}",
        "@supports future(thing) {}",
        ".a { --raw: \"#{value}\"; content: \"#{value}\"; }",
        ".a { &__item { color: red; } }",
        "@unknown fn(#{value});",
    ] {
        let parsed = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}\n{:?}", parsed.diagnostics());
        assert!(
            !has_bogus_nodes_or_empty_slots(&parsed.syntax()),
            "{source}\n{:#?}",
            parsed.syntax()
        );
    }
}

#[test]
fn malformed_interpolation_preserves_following_rules() {
    for source in [
        ".a { value: #{}; }",
        "@supports #{} {}",
        "@media (#{}: 1px) {}",
    ] {
        let source = format!("{source}\n.after {{ color: red; }}");
        for source_type in [CssFileSource::css(), CssFileSource::scss()] {
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(parsed.has_errors(), "{source}");
            assert!(
                parsed.syntax().descendants().any(|node| {
                    node.kind() == CssSyntaxKind::CSS_QUALIFIED_RULE
                        && node.text_trimmed() == ".after { color: red; }"
                }),
                "{source}\n{:#?}",
                parsed.syntax()
            );
        }
    }
}

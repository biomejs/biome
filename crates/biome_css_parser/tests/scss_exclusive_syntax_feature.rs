use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssQualifiedRule, CssSyntaxKind};
use biome_languages::CssFileSource;
use biome_rowan::AstNode;

const SCSS_VARIABLE_DECLARATION: &str = "$color: red;";
const SCSS_VARIABLE_VALUE: &str = ".selector { color: $color; }";
const SCSS_DIMENSION_INTERPOLATED_VALUE: &str = ".selector { width: 10px#{suffix}; }";
const SCSS_NUMBER_INTERPOLATED_VALUE: &str = ".selector { width: 10#{unit}; }";
const SCSS_PARENTHESIZED_QUERY_VALUE: &str = "@media (max-width: ($device - 1px)) {}";
const SCSS_NEGATIVE_QUERY_VALUE: &str = "@media (min-width: -$gap + 800px) {}";
const SCSS_POSITIVE_QUERY_VALUE: &str = "@media (min-width: +$gap + 800px) {}";

fn diagnostic_text(parse: &biome_css_parser::CssParse) -> String {
    format!("{:?}", parse.diagnostics())
}

#[test]
fn css_files_do_not_report_scss_exclusive_syntax_without_parser_option() {
    for source in [
        SCSS_VARIABLE_DECLARATION,
        SCSS_VARIABLE_VALUE,
        SCSS_PARENTHESIZED_QUERY_VALUE,
        SCSS_NEGATIVE_QUERY_VALUE,
        SCSS_POSITIVE_QUERY_VALUE,
    ] {
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let diagnostics = diagnostic_text(&parse);

        assert!(
            !diagnostics.contains("SCSS"),
            "expected no SCSS-specific diagnostics without the parser option, got: {diagnostics}"
        );
        assert!(
            !parse.diagnostics().is_empty(),
            "expected parsing to keep reporting invalid CSS syntax"
        );
    }
}

#[test]
fn reporting_scss_exclusive_syntax_only_changes_diagnostic_text() {
    for source in [
        SCSS_VARIABLE_VALUE,
        SCSS_NUMBER_INTERPOLATED_VALUE,
        SCSS_DIMENSION_INTERPOLATED_VALUE,
        SCSS_PARENTHESIZED_QUERY_VALUE,
        SCSS_NEGATIVE_QUERY_VALUE,
        SCSS_POSITIVE_QUERY_VALUE,
    ] {
        let default_parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let reporting_parse = parse_css(
            source,
            CssFileSource::css(),
            CssParserOptions::default().report_scss_exclusive_syntax(),
        );

        assert_eq!(
            format!("{:#?}", default_parse.syntax()),
            format!("{:#?}", reporting_parse.syntax()),
            "expected parser option to preserve CSS recovery tree for {source}"
        );
        assert_eq!(
            default_parse.diagnostics().len(),
            reporting_parse.diagnostics().len(),
            "expected parser option to preserve diagnostic count for {source}"
        );

        let default_diagnostics = diagnostic_text(&default_parse);
        let reporting_diagnostics = diagnostic_text(&reporting_parse);

        assert!(
            !default_diagnostics.contains("SCSS"),
            "expected default parser option to keep generic diagnostics, got: {default_diagnostics}"
        );
        assert!(
            reporting_diagnostics.contains("SCSS"),
            "expected reporting parser option to emit SCSS diagnostics, got: {reporting_diagnostics}"
        );
    }
}

#[test]
fn scss_query_values_report_one_unsupported_syntax_diagnostic() {
    for source in [
        SCSS_PARENTHESIZED_QUERY_VALUE,
        SCSS_NEGATIVE_QUERY_VALUE,
        SCSS_POSITIVE_QUERY_VALUE,
        "@container (width > (1px)) {}",
    ] {
        let parse = parse_css(
            source,
            CssFileSource::css(),
            CssParserOptions::default().report_scss_exclusive_syntax(),
        );

        assert_eq!(parse.diagnostics().len(), 1, "{source}");
        assert!(diagnostic_text(&parse).contains("SCSS"), "{source}");
    }
}

#[test]
fn scss_only_entries_recover_with_one_diagnostic() {
    for source in [
        "@media (#{$query} and (color)) { .inside {} }",
        "@media (#{$query} or (color)) { .inside {} }",
        "@font-face { font: { family: serif; } src: url(font.woff2); }",
        "a { value: [(1 + 2)]; color: red; }",
        "a { value: [1]; color: red; }",
        "a { value: [$value]; color: red; }",
        "@media (width: 10px + 1px) { .inside {} }",
        "@media (width: $value + 1px) { .inside {} }",
    ] {
        let source = format!("{source} .after {{ color: blue; }}");
        let parsed = parse_css(
            &source,
            CssFileSource::css(),
            CssParserOptions::default().report_scss_exclusive_syntax(),
        );
        assert_eq!(parsed.diagnostics().len(), 1, "{source}: {parsed:#?}");
        assert!(diagnostic_text(&parsed).contains("SCSS"), "{source}");
        assert_eq!(parsed.syntax().text_with_trivia().to_string(), source);
        assert!(
            parsed
                .syntax()
                .descendants()
                .all(|node| node.kind() != CssSyntaxKind::CSS_BOGUS_BLOCK)
        );
        assert!(
            parsed
                .syntax()
                .descendants()
                .filter_map(CssQualifiedRule::cast)
                .any(|rule| rule.prelude().syntax().text_trimmed() == ".after"
                    && !format!("{rule:#?}").contains("missing (required)"))
        );
        let default = parse_css(&source, CssFileSource::css(), CssParserOptions::default());
        assert_eq!(default.diagnostics().len(), parsed.diagnostics().len());
        assert_eq!(
            format!("{:#?}", default.syntax()),
            format!("{:#?}", parsed.syntax())
        );
        assert!(!diagnostic_text(&default).contains("SCSS"));
    }
}

#[test]
fn scss_query_recovery_preserves_blocks_after_a_missing_parenthesis() {
    let source = include_str!("css_test_suite/error/scss-exclusive-query-recovery.css");
    for file_source in [CssFileSource::css(), CssFileSource::scss()] {
        let parsed = parse_css(
            source,
            file_source,
            CssParserOptions::default().report_scss_exclusive_syntax(),
        );
        assert!(parsed.has_errors());
        assert_eq!(parsed.syntax().text_with_trivia().to_string(), source);
        let rules: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(CssQualifiedRule::cast)
            .collect();
        assert_eq!(rules.len(), 2, "{file_source:?}: {:#?}", parsed.tree());
        assert_eq!(rules[0].prelude().syntax().text_trimmed(), ".inside");
        assert_eq!(rules[1].prelude().syntax().text_trimmed(), ".after");
        assert_eq!(
            rules[1].syntax().grand_parent().unwrap().kind(),
            CssSyntaxKind::CSS_ROOT
        );
        for rule in rules {
            assert!(!format!("{rule:#?}").contains("missing (required)"));
        }
        let default = parse_css(source, file_source, CssParserOptions::default());
        assert_eq!(default.diagnostics().len(), parsed.diagnostics().len());
        assert_eq!(
            format!("{:#?}", default.syntax()),
            format!("{:#?}", parsed.syntax())
        );
    }
}

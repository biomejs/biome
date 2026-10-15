use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssAttrFunction, CssGenericProperty, CssSyntaxKind};
use biome_languages::CssFileSource;
use biome_rowan::AstNode;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[test]
fn dynamic_attr_types_keep_the_name_and_type_slots_distinct() {
    for (arguments, name, kind) in [
        (
            "data-width $unit, 10px",
            "data-width",
            CssSyntaxKind::SCSS_VARIABLE,
        ),
        ("$name$unit", "$name", CssSyntaxKind::SCSS_VARIABLE),
        ("ns|$name $unit", "ns|$name", CssSyntaxKind::SCSS_VARIABLE),
        (
            "#{$name} #{$unit}2",
            "#{$name}",
            CssSyntaxKind::SCSS_INTERPOLATED_IDENTIFIER,
        ),
        (
            "data-width r#{$suffix}",
            "data-width",
            CssSyntaxKind::SCSS_INTERPOLATED_IDENTIFIER,
        ),
    ] {
        let source = format!(".a {{ width: attr({arguments}); }}");
        let parsed = parse_css(&source, CssFileSource::scss(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
        let function = parsed
            .syntax()
            .descendants()
            .find_map(CssAttrFunction::cast)
            .unwrap();
        assert_eq!(function.attr_name().syntax().text_trimmed(), name);
        assert_eq!(function.attr_type().unwrap().syntax().kind(), kind);
    }
}

#[test]
fn namespaced_attr_types_keep_the_name_and_type_slots_distinct() {
    for name in ["data-width", "$name", "#{$name}"] {
        let source = format!(".a {{ width: attr({name} units.$unit, 10px); }}");
        let parsed = parse_css(&source, CssFileSource::scss(), CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
        let function = parsed
            .syntax()
            .descendants()
            .find_map(CssAttrFunction::cast)
            .unwrap();
        assert_eq!(function.attr_name().syntax().text_trimmed(), name);
        assert_eq!(
            function.attr_type().unwrap().syntax().kind(),
            CssSyntaxKind::SCSS_NAMESPACED_VARIABLE
        );
    }
}

#[test]
fn invalid_attr_types_preserve_following_rules_and_source() {
    for (source_type, arguments) in [
        (CssFileSource::css(), "data-width $unit, 10px"),
        (CssFileSource::css(), "data-width units.$unit, 10px"),
        (CssFileSource::css(), "data-width #{$unit}, 10px"),
        (CssFileSource::scss(), "data-width $, 10px"),
        (CssFileSource::scss(), "data-width #{}, 10px"),
        (CssFileSource::scss(), "data-width $unit + 1px, 10px"),
    ] {
        let source =
            format!(".a {{ width: attr({arguments}); color: red; }} .after {{ display: block; }}");
        let parsed = parse_css(
            &source,
            source_type,
            CssParserOptions::default().report_scss_exclusive_syntax(),
        );
        assert!(parsed.has_errors(), "{source}");
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|node| node.kind() == CssSyntaxKind::CSS_QUALIFIED_RULE
                    && node.text_trimmed() == ".after { display: block; }")
        );
    }
}

#[test]
fn attr_type_feature_gate_preserves_typed_ownership() {
    for (unit, kind, scss_only) in [
        ("$unit", CssSyntaxKind::SCSS_VARIABLE, true),
        ("units.$unit", CssSyntaxKind::SCSS_NAMESPACED_VARIABLE, true),
        (
            "#{$unit}",
            CssSyntaxKind::SCSS_INTERPOLATED_IDENTIFIER,
            true,
        ),
        (
            "r#{$suffix}",
            CssSyntaxKind::SCSS_INTERPOLATED_IDENTIFIER,
            true,
        ),
        (
            "-#{$unit}",
            CssSyntaxKind::SCSS_INTERPOLATED_IDENTIFIER,
            true,
        ),
        ("px", CssSyntaxKind::CSS_REGULAR_ATTR_UNIT, false),
        ("type(<length>)", CssSyntaxKind::CSS_TYPE_FUNCTION, false),
    ] {
        let source = format!(
            ".a {{ width: attr(data-width {unit},10px); color: red; }} .after {{ display: block; }}"
        );
        for (file_source, is_scss) in [(CssFileSource::css(), false), (CssFileSource::scss(), true)]
        {
            for options in [
                CssParserOptions::default(),
                CssParserOptions::default().report_scss_exclusive_syntax(),
            ] {
                let parsed = parse_css(&source, file_source, options);
                let unsupported = scss_only && !is_scss;
                assert_eq!(
                    parsed.diagnostics().len(),
                    if unsupported { 1 } else { 0 },
                    "{file_source:?} {source}: {:?}",
                    parsed.diagnostics()
                );
                assert_eq!(parsed.syntax().to_string(), source);
                let properties: Vec<_> = parsed
                    .syntax()
                    .descendants()
                    .filter_map(CssGenericProperty::cast)
                    .collect();
                assert_eq!(properties.len(), 3, "{file_source:?} {source}");
                for (property, name) in properties.iter().zip(["width", "color", "display"]) {
                    assert_eq!(property.name().unwrap().syntax().text_trimmed(), name);
                }
                let value = properties[0].value().unwrap();
                let function = value
                    .syntax()
                    .descendants()
                    .find_map(CssAttrFunction::cast)
                    .unwrap();
                assert_eq!(function.attr_name().syntax().text_trimmed(), "data-width");
                assert!(!has_bogus_nodes_or_empty_slots(
                    function.attr_name().syntax()
                ));
                let attr_type = function.attr_type().unwrap();
                assert_eq!(
                    attr_type.syntax().kind(),
                    if unsupported {
                        CssSyntaxKind::CSS_BOGUS_PROPERTY_VALUE
                    } else {
                        kind
                    }
                );
                assert_eq!(attr_type.syntax().text_trimmed(), unit);
                let fallback = function.fallback_value().unwrap();
                assert_eq!(fallback.comma_token().unwrap().text_trimmed(), ",");
                assert_eq!(fallback.value().unwrap().syntax().text_trimmed(), "10px");
                assert!(!has_bogus_nodes_or_empty_slots(fallback.syntax()));
                assert_eq!(function.r_paren_token().unwrap().text_trimmed(), ")");
                if !unsupported {
                    assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
                }
            }
        }
    }
}

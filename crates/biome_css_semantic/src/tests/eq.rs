use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::css::CssFileSource;

use crate::model::SemanticModel;
use crate::semantic_model;

fn build_model(source: &str) -> SemanticModel {
    let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    semantic_model(&root)
}

#[test]
fn whitespace_change_is_eq() {
    assert_eq!(
        build_model("p { color: red; }"),
        build_model("p  {  color:  red;  }"),
        "whitespace should not affect semantic eq"
    );
}

#[test]
fn comment_change_is_eq() {
    assert_eq!(
        build_model("p { color: red; }"),
        build_model("p { /* a comment */ color: red; }"),
        "comments should not affect semantic eq"
    );
}

#[test]
fn value_change_same_variant_is_eq() {
    assert_eq!(
        build_model("p { color: red; }"),
        build_model("p { color: blue; }"),
        "different values of the same variant should be equal"
    );
}

#[test]
fn selector_change_is_not_eq() {
    assert_ne!(
        build_model("p { color: red; }"),
        build_model("span { color: red; }"),
        "different selectors should produce different models"
    );
}

#[test]
fn property_name_change_is_not_eq() {
    assert_ne!(
        build_model("p { color: red; }"),
        build_model("p { font-size: 12px; }"),
        "different property names should produce different models"
    );
}

#[test]
fn declaration_count_change_is_not_eq() {
    assert_ne!(
        build_model("p { color: red; }"),
        build_model("p { color: red; font-size: 12px; }"),
        "different declaration counts should produce different models"
    );
}

#[test]
fn rule_count_change_is_not_eq() {
    assert_ne!(
        build_model("p { color: red; }"),
        build_model("p { color: red; } span { color: blue; }"),
        "different rule counts should produce different models"
    );
}

#[test]
fn nested_vs_flat_is_not_eq() {
    assert_ne!(
        build_model("p { color: red; }"),
        build_model("p { .child { color: red; } }"),
        "flat vs nested structure should differ (different declaration counts on parent)"
    );
}

#[test]
fn specificity_change_is_not_eq() {
    assert_ne!(
        build_model(".foo { color: red; }"),
        build_model("#foo { color: red; }"),
        "different specificity should produce different models"
    );
}

#[test]
fn global_variable_name_change_is_not_eq() {
    assert_ne!(
        build_model(":root { --color: red; }"),
        build_model(":root { --size: 20px; }"),
        "different custom property names should produce different models"
    );
}

#[test]
fn global_variable_count_change_is_not_eq() {
    assert_ne!(
        build_model(":root { --color: red; }"),
        build_model(":root { --color: red; --size: 20px; }"),
        "different custom property counts should produce different models"
    );
}

#[test]
fn at_property_name_change_is_not_eq() {
    assert_ne!(
        build_model(
            r#"@property --foo { syntax: "<color>"; inherits: true; initial-value: red; }"#
        ),
        build_model(
            r#"@property --bar { syntax: "<color>"; inherits: true; initial-value: red; }"#
        ),
        "different @property names should produce different models"
    );
}

#[test]
fn at_property_syntax_change_is_not_eq() {
    assert_ne!(
        build_model(
            r#"@property --foo { syntax: "<color>"; inherits: true; initial-value: red; }"#
        ),
        build_model(
            r#"@property --foo { syntax: "<length>"; inherits: true; initial-value: 10px; }"#
        ),
        "different @property syntax should produce different models"
    );
}

#[test]
fn at_property_inherits_change_is_not_eq() {
    assert_ne!(
        build_model(
            r#"@property --foo { syntax: "<color>"; inherits: true; initial-value: red; }"#
        ),
        build_model(
            r#"@property --foo { syntax: "<color>"; inherits: false; initial-value: red; }"#
        ),
        "different @property inherits should produce different models"
    );
}

#[test]
fn same_nested_structure_value_change_is_eq() {
    assert_eq!(
        build_model(".parent { color: red; .child { font-size: 12px; } }"),
        build_model(".parent { color: blue; .child { font-size: 16px; } }"),
        "value-only changes in nested rules should be equal"
    );
}

#[test]
fn nested_selector_change_is_not_eq() {
    assert_ne!(
        build_model(".parent { .child { color: red; } }"),
        build_model(".parent { .other { color: red; } }"),
        "different nested selectors should produce different models"
    );
}

#[test]
fn nested_declaration_count_change_is_not_eq() {
    assert_ne!(
        build_model(".parent { .child { color: red; } }"),
        build_model(".parent { .child { color: red; font-size: 12px; } }"),
        "different nested declaration counts should produce different models"
    );
}

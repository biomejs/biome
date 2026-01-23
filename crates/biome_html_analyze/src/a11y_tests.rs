use super::*;
use biome_html_parser::parse_html;
use biome_html_syntax::HtmlRoot;
use biome_rowan::AstNode;

/// Helper to parse HTML and extract the first element
fn parse_first_element(html: &str) -> AnyHtmlElement {
    let parsed = parse_html(html, Default::default());
    let root = HtmlRoot::cast(parsed.syntax()).unwrap();
    root.syntax()
        .descendants()
        .find_map(AnyHtmlElement::cast)
        .expect("No element found in parsed HTML")
}

// ============================================================================
// Tests for is_aria_hidden_value_truthy (via get_truthy_aria_hidden_attribute)
// ============================================================================

#[test]
fn test_aria_hidden_true_is_truthy() {
    let element = parse_first_element(r#"<div aria-hidden="true"></div>"#);
    assert!(get_truthy_aria_hidden_attribute(&element).is_some());
}

#[test]
fn test_aria_hidden_false_is_not_truthy() {
    let element = parse_first_element(r#"<div aria-hidden="false"></div>"#);
    assert!(get_truthy_aria_hidden_attribute(&element).is_none());
}

#[test]
fn test_aria_hidden_false_case_insensitive() {
    let element = parse_first_element(r#"<div aria-hidden="FALSE"></div>"#);
    assert!(get_truthy_aria_hidden_attribute(&element).is_none());
}

#[test]
fn test_aria_hidden_empty_is_truthy() {
    let element = parse_first_element(r#"<div aria-hidden=""></div>"#);
    assert!(get_truthy_aria_hidden_attribute(&element).is_some());
}

#[test]
fn test_aria_hidden_absent_returns_none() {
    let element = parse_first_element(r#"<div></div>"#);
    assert!(get_truthy_aria_hidden_attribute(&element).is_none());
}

// ============================================================================
// Tests for is_aria_hidden_true (strict check)
// ============================================================================

#[test]
fn test_strict_aria_hidden_true() {
    let element = parse_first_element(r#"<div aria-hidden="true"></div>"#);
    assert!(is_aria_hidden_true(&element));
}

#[test]
fn test_strict_aria_hidden_false() {
    let element = parse_first_element(r#"<div aria-hidden="false"></div>"#);
    assert!(!is_aria_hidden_true(&element));
}

#[test]
fn test_strict_aria_hidden_true_uppercase_not_matched() {
    let element = parse_first_element(r#"<div aria-hidden="TRUE"></div>"#);
    assert!(!is_aria_hidden_true(&element));
}

// ============================================================================
// Tests for has_accessible_name
// ============================================================================

#[test]
fn test_has_accessible_name_aria_label() {
    let element = parse_first_element(r#"<div aria-label="Label"></div>"#);
    assert!(has_accessible_name(&element));
}

#[test]
fn test_has_accessible_name_aria_labelledby() {
    let element = parse_first_element(r#"<div aria-labelledby="other-id"></div>"#);
    assert!(has_accessible_name(&element));
}

#[test]
fn test_has_accessible_name_title() {
    let element = parse_first_element(r#"<div title="Title"></div>"#);
    assert!(has_accessible_name(&element));
}

#[test]
fn test_has_accessible_name_empty_aria_label() {
    let element = parse_first_element(r#"<div aria-label=""></div>"#);
    assert!(!has_accessible_name(&element));
}

#[test]
fn test_has_accessible_name_whitespace_only() {
    let element = parse_first_element(r#"<div aria-label="   "></div>"#);
    assert!(!has_accessible_name(&element));
}

#[test]
fn test_has_accessible_name_none() {
    let element = parse_first_element(r#"<div></div>"#);
    assert!(!has_accessible_name(&element));
}

// ============================================================================
// Tests for attribute_value_equals_ignore_case
// ============================================================================

#[test]
fn test_attribute_value_case_insensitive() {
    let element = parse_first_element(r#"<input type="HIDDEN" />"#);
    let attr = element.find_attribute_by_name("type").unwrap();
    assert!(attribute_value_equals_ignore_case(&attr, "hidden"));
}

#[test]
fn test_attribute_value_exact_match() {
    let element = parse_first_element(r#"<input type="hidden" />"#);
    let attr = element.find_attribute_by_name("type").unwrap();
    assert!(attribute_value_equals_ignore_case(&attr, "hidden"));
}

#[test]
fn test_attribute_value_no_match() {
    let element = parse_first_element(r#"<input type="text" />"#);
    let attr = element.find_attribute_by_name("type").unwrap();
    assert!(!attribute_value_equals_ignore_case(&attr, "hidden"));
}

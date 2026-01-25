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

mod aria_hidden_truthy {
    use super::*;

    #[test]
    fn true_value_is_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="true"></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_some());
    }

    #[test]
    fn false_value_is_not_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="false"></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn false_uppercase_is_not_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="FALSE"></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn false_mixed_case_is_not_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="FaLsE"></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn empty_string_is_not_truthy() {
        let element = parse_first_element(r#"<div aria-hidden=""></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn absent_attribute_returns_none() {
        let element = parse_first_element(r#"<div></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn whitespace_only_is_not_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="   "></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn arbitrary_string_is_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="yes"></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_some());
    }

    #[test]
    fn numeric_string_is_truthy() {
        let element = parse_first_element(r#"<div aria-hidden="1"></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_some());
    }

    #[test]
    fn false_with_whitespace_is_not_truthy() {
        // " false " is trimmed to "false", so it's not truthy
        let element = parse_first_element(r#"<div aria-hidden=" false "></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }

    #[test]
    fn on_self_closing_element() {
        let element = parse_first_element(r#"<img aria-hidden="true" />"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_some());
    }

    #[test]
    fn other_attributes_ignored() {
        let element =
            parse_first_element(r#"<div class="hidden" data-hidden="true" hidden></div>"#);
        assert!(get_truthy_aria_hidden_attribute(&element).is_none());
    }
}

// ============================================================================
// Tests for is_aria_hidden_true (strict check)
// ============================================================================

mod aria_hidden_strict {
    use super::*;

    #[test]
    fn exact_true_matches() {
        let element = parse_first_element(r#"<div aria-hidden="true"></div>"#);
        assert!(is_aria_hidden_true(&element));
    }

    #[test]
    fn false_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden="false"></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn uppercase_true_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden="TRUE"></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn mixed_case_true_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden="True"></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn empty_string_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden=""></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn absent_attribute_does_not_match() {
        let element = parse_first_element(r#"<div></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn true_with_whitespace_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden=" true "></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn yes_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden="yes"></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }

    #[test]
    fn one_does_not_match() {
        let element = parse_first_element(r#"<div aria-hidden="1"></div>"#);
        assert!(!is_aria_hidden_true(&element));
    }
}

// ============================================================================
// Tests for has_accessible_name
// ============================================================================

mod accessible_name {
    use super::*;

    #[test]
    fn aria_label_provides_name() {
        let element = parse_first_element(r#"<div aria-label="Label"></div>"#);
        assert!(has_accessible_name(&element));
    }

    #[test]
    fn aria_labelledby_provides_name() {
        let element = parse_first_element(r#"<div aria-labelledby="other-id"></div>"#);
        assert!(has_accessible_name(&element));
    }

    #[test]
    fn title_provides_name() {
        let element = parse_first_element(r#"<div title="Title"></div>"#);
        assert!(has_accessible_name(&element));
    }

    #[test]
    fn empty_aria_label_no_name() {
        let element = parse_first_element(r#"<div aria-label=""></div>"#);
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn whitespace_aria_label_no_name() {
        let element = parse_first_element(r#"<div aria-label="   "></div>"#);
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn tab_and_newline_only_no_name() {
        let element = parse_first_element("<div aria-label=\"\t\n\"></div>");
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn no_attributes_no_name() {
        let element = parse_first_element(r#"<div></div>"#);
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn empty_title_no_name() {
        let element = parse_first_element(r#"<div title=""></div>"#);
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn empty_aria_labelledby_no_name() {
        let element = parse_first_element(r#"<div aria-labelledby=""></div>"#);
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn multiple_sources_first_wins() {
        let element =
            parse_first_element(r#"<div aria-label="Label" aria-labelledby="id" title="T"></div>"#);
        assert!(has_accessible_name(&element));
    }

    #[test]
    fn only_title_empty_others() {
        let element =
            parse_first_element(r#"<div aria-label="" aria-labelledby="" title="Title"></div>"#);
        assert!(has_accessible_name(&element));
    }

    #[test]
    fn unrelated_attributes_no_name() {
        let element = parse_first_element(r#"<div class="label" data-label="x" name="y"></div>"#);
        assert!(!has_accessible_name(&element));
    }

    #[test]
    fn on_self_closing_element() {
        let element = parse_first_element(r#"<img aria-label="Image description" />"#);
        assert!(has_accessible_name(&element));
    }

    #[test]
    fn aria_label_with_only_leading_trailing_whitespace() {
        let element = parse_first_element(r#"<div aria-label="  valid label  "></div>"#);
        assert!(has_accessible_name(&element));
    }
}

// ============================================================================
// Tests for attribute_value_equals_ignore_case
// ============================================================================

mod attribute_value_equals {
    use super::*;

    #[test]
    fn exact_lowercase_match() {
        let element = parse_first_element(r#"<input type="hidden" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(attribute_value_equals_ignore_case(&attr, "hidden"));
    }

    #[test]
    fn uppercase_value_matches_lowercase() {
        let element = parse_first_element(r#"<input type="HIDDEN" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(attribute_value_equals_ignore_case(&attr, "hidden"));
    }

    #[test]
    fn lowercase_value_matches_uppercase() {
        let element = parse_first_element(r#"<input type="hidden" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(attribute_value_equals_ignore_case(&attr, "HIDDEN"));
    }

    #[test]
    fn mixed_case_matches() {
        let element = parse_first_element(r#"<input type="HiDdEn" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(attribute_value_equals_ignore_case(&attr, "hidden"));
    }

    #[test]
    fn different_value_no_match() {
        let element = parse_first_element(r#"<input type="text" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(!attribute_value_equals_ignore_case(&attr, "hidden"));
    }

    #[test]
    fn empty_value_matches_empty() {
        let element = parse_first_element(r#"<input type="" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(attribute_value_equals_ignore_case(&attr, ""));
    }

    #[test]
    fn empty_value_no_match_non_empty() {
        let element = parse_first_element(r#"<input type="" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(!attribute_value_equals_ignore_case(&attr, "hidden"));
    }

    #[test]
    fn partial_match_no_match() {
        let element = parse_first_element(r#"<input type="hidden-field" />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(!attribute_value_equals_ignore_case(&attr, "hidden"));
    }

    #[test]
    fn value_with_whitespace_no_match() {
        let element = parse_first_element(r#"<input type=" hidden " />"#);
        let attr = element.find_attribute_by_name("type").unwrap();
        assert!(!attribute_value_equals_ignore_case(&attr, "hidden"));
    }
}

// ============================================================================
// Tests for has_non_empty_attribute
// ============================================================================

mod has_non_empty_attribute_tests {
    use super::*;

    #[test]
    fn attribute_with_value() {
        let element = parse_first_element(r#"<div data-test="value"></div>"#);
        assert!(has_non_empty_attribute(&element, "data-test"));
    }

    #[test]
    fn attribute_empty_value() {
        let element = parse_first_element(r#"<div data-test=""></div>"#);
        assert!(!has_non_empty_attribute(&element, "data-test"));
    }

    #[test]
    fn attribute_whitespace_only() {
        let element = parse_first_element(r#"<div data-test="   "></div>"#);
        assert!(!has_non_empty_attribute(&element, "data-test"));
    }

    #[test]
    fn attribute_absent() {
        let element = parse_first_element(r#"<div></div>"#);
        assert!(!has_non_empty_attribute(&element, "data-test"));
    }

    #[test]
    fn different_attribute_name() {
        let element = parse_first_element(r#"<div data-other="value"></div>"#);
        assert!(!has_non_empty_attribute(&element, "data-test"));
    }

    #[test]
    fn value_with_leading_trailing_whitespace() {
        let element = parse_first_element(r#"<div data-test="  value  "></div>"#);
        assert!(has_non_empty_attribute(&element, "data-test"));
    }
}

// ============================================================================
// Tests for is_hidden_from_screen_reader
// ============================================================================

mod hidden_from_screen_reader {
    use super::*;
    use biome_html_syntax::element_ext::AnyHtmlTagElement;

    fn parse_tag_element(html: &str) -> AnyHtmlTagElement {
        let parsed = parse_html(html, Default::default());
        let root = HtmlRoot::cast(parsed.syntax()).unwrap();
        root.syntax()
            .descendants()
            .find_map(AnyHtmlTagElement::cast)
            .expect("No tag element found")
    }

    #[test]
    fn aria_hidden_true_is_hidden() {
        let element = parse_tag_element(r#"<div aria-hidden="true"></div>"#);
        assert!(is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn aria_hidden_false_not_hidden() {
        let element = parse_tag_element(r#"<div aria-hidden="false"></div>"#);
        assert!(!is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn input_type_hidden_is_hidden() {
        let element = parse_tag_element(r#"<input type="hidden" />"#);
        assert!(is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn input_type_hidden_case_insensitive() {
        let element = parse_tag_element(r#"<input type="HIDDEN" />"#);
        assert!(is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn input_type_text_not_hidden() {
        let element = parse_tag_element(r#"<input type="text" />"#);
        assert!(!is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn input_without_type_not_hidden() {
        let element = parse_tag_element(r#"<input />"#);
        assert!(!is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn div_without_aria_hidden_not_hidden() {
        let element = parse_tag_element(r#"<div></div>"#);
        assert!(!is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn non_input_with_type_hidden_not_hidden() {
        let element = parse_tag_element(r#"<div type="hidden"></div>"#);
        assert!(!is_hidden_from_screen_reader(&element));
    }

    #[test]
    fn button_type_hidden_not_hidden() {
        // type="hidden" only applies to input elements
        let element = parse_tag_element(r#"<button type="hidden"></button>"#);
        assert!(!is_hidden_from_screen_reader(&element));
    }
}

// ============================================================================
// Tests for type-specific variants
// ============================================================================

mod type_specific_variants {
    use super::*;
    use biome_html_syntax::{HtmlElement, HtmlSelfClosingElement};

    fn parse_html_element(html: &str) -> HtmlElement {
        let parsed = parse_html(html, Default::default());
        let root = HtmlRoot::cast(parsed.syntax()).unwrap();
        root.syntax()
            .descendants()
            .find_map(HtmlElement::cast)
            .expect("No HtmlElement found")
    }

    fn parse_self_closing_element(html: &str) -> HtmlSelfClosingElement {
        let parsed = parse_html(html, Default::default());
        let root = HtmlRoot::cast(parsed.syntax()).unwrap();
        root.syntax()
            .descendants()
            .find_map(HtmlSelfClosingElement::cast)
            .expect("No HtmlSelfClosingElement found")
    }

    #[test]
    fn html_element_truthy_aria_hidden() {
        let element = parse_html_element(r#"<div aria-hidden="true"></div>"#);
        assert!(html_element_has_truthy_aria_hidden(&element));
    }

    #[test]
    fn html_element_false_aria_hidden() {
        let element = parse_html_element(r#"<div aria-hidden="false"></div>"#);
        assert!(!html_element_has_truthy_aria_hidden(&element));
    }

    #[test]
    fn html_element_no_aria_hidden() {
        let element = parse_html_element(r#"<div></div>"#);
        assert!(!html_element_has_truthy_aria_hidden(&element));
    }

    #[test]
    fn self_closing_truthy_aria_hidden() {
        let element = parse_self_closing_element(r#"<img aria-hidden="true" />"#);
        assert!(html_self_closing_element_has_truthy_aria_hidden(&element));
    }

    #[test]
    fn self_closing_false_aria_hidden() {
        let element = parse_self_closing_element(r#"<img aria-hidden="false" />"#);
        assert!(!html_self_closing_element_has_truthy_aria_hidden(&element));
    }

    #[test]
    fn self_closing_has_accessible_name() {
        let element = parse_self_closing_element(r#"<img aria-label="Description" />"#);
        assert!(html_self_closing_element_has_accessible_name(&element));
    }

    #[test]
    fn self_closing_no_accessible_name() {
        let element = parse_self_closing_element(r#"<img />"#);
        assert!(!html_self_closing_element_has_accessible_name(&element));
    }

    #[test]
    fn self_closing_has_non_empty_attribute() {
        let element = parse_self_closing_element(r#"<img alt="description" />"#);
        assert!(html_self_closing_element_has_non_empty_attribute(
            &element, "alt"
        ));
    }

    #[test]
    fn self_closing_empty_attribute() {
        let element = parse_self_closing_element(r#"<img alt="" />"#);
        assert!(!html_self_closing_element_has_non_empty_attribute(
            &element, "alt"
        ));
    }

    #[test]
    fn self_closing_missing_attribute() {
        let element = parse_self_closing_element(r#"<img />"#);
        assert!(!html_self_closing_element_has_non_empty_attribute(
            &element, "alt"
        ));
    }
}

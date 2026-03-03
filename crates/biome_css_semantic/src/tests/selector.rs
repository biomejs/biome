use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::CssFileSource;

use crate::semantic_model;

#[test]
fn test_resolve_selector_no_parents() {
    let css = "div { color: red; }";
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rule = model.rules().first().unwrap();
    let root = model.root();
    assert_eq!(rule.selectors.len(), 1);
    assert_eq!(rule.selectors[0].text(&root).to_string(), "div");
}

#[test]
fn test_resolve_selector_simple_parent() {
    let css = r#"
            .parent {
                .child {
                    color: red;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".parent");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".parent .child" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), ".child");
}

#[test]
fn test_resolve_selector_with_ampersand() {
    let css = r#"
            a {
                &:hover {
                    color: orange;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), "a");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to "a:hover" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), "&:hover");
}

#[test]
fn test_resolve_selector_multiple_parents() {
    let css = r#"
            .grandparent {
                .parent {
                    .child {
                        color: blue;
                    }
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let grandparent_rule = model.rules().first().unwrap();
    assert_eq!(grandparent_rule.selectors.len(), 1);
    assert_eq!(
        grandparent_rule.selectors[0].text(&root).to_string(),
        ".grandparent"
    );

    let parent_rule_id = grandparent_rule.child_ids.first().unwrap();
    let parent_rule = model.get_rule_by_id(parent_rule_id).unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    // TODO: Should resolve to ".grandparent .parent" once nested selector resolution is implemented
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".parent");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".grandparent .parent .child" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), ".child");
}

#[test]
fn test_resolve_selector_with_multi_ampersand() {
    let css = r#"
            p {
                & + & {
                    margin-top: 10px;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), "p");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to "p + p" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), "& + &");
}

#[test]
fn test_resolve_selector_no_ampersand_with_parents() {
    let css = r#"
            .list {
                li {
                    font-size: 14px;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".list");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".list li" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), "li");
}

#[test]
#[ignore]
fn test_resolve_selector_with_complex_parent() {
    let css = r#"
            .menu > ul {
                > li {
                    display: inline-block;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(
        parent_rule.selectors[0].text(&root).to_string(),
        ".menu > ul"
    );

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".menu > ul > li" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), "> li");
}

#[test]
fn test_resolve_selector_with_nested_ampersands() {
    let css = r#"
            .btn {
                &--primary:hover &__icon {
                    fill: blue;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".btn");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".btn--primary:hover .btn__icon" once nested selector resolution is implemented
    assert_eq!(
        child_rule.selectors[0].text(&root).to_string(),
        "&--primary:hover &__icon"
    );
}

#[test]
fn test_resolve_selector_with_multiple_parents_and_ampersand() {
    let css = r#"
            .grandparent {
                .parent {
                    &--modified {
                        color: green;
                    }
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let grandparent_rule = model.rules().first().unwrap();
    assert_eq!(grandparent_rule.selectors.len(), 1);
    assert_eq!(
        grandparent_rule.selectors[0].text(&root).to_string(),
        ".grandparent"
    );

    let parent_rule_id = grandparent_rule.child_ids.first().unwrap();
    let parent_rule = model.get_rule_by_id(parent_rule_id).unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    // TODO: Should resolve to ".grandparent .parent" once nested selector resolution is implemented
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".parent");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".grandparent .parent--modified" once nested selector resolution is implemented
    assert_eq!(
        child_rule.selectors[0].text(&root).to_string(),
        "&--modified"
    );
}

#[test]
fn test_descendant_combinator() {
    let css = ".foo .bar { color: red; }";
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let rule = model.rules().first().unwrap();

    assert_eq!(rule.selectors.len(), 1);
    assert_eq!(rule.selectors[0].text(&root).to_string(), ".foo .bar");
}

#[test]
fn test_child_combinator() {
    let css = r#"
            .foo > .bar { color: red; }
            .foo || .bar { color: blue; }
            .foo + .bar { color: green; }
            .foo ~ .bar { color: yellow; }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let rules = model.rules();

    assert_eq!(rules.len(), 4);

    assert_eq!(rules[0].selectors.len(), 1);
    assert_eq!(rules[0].selectors[0].text(&root).to_string(), ".foo > .bar");

    assert_eq!(rules[1].selectors.len(), 1);
    assert_eq!(
        rules[1].selectors[0].text(&root).to_string(),
        ".foo || .bar"
    );

    assert_eq!(rules[2].selectors.len(), 1);
    assert_eq!(rules[2].selectors[0].text(&root).to_string(), ".foo + .bar");

    assert_eq!(rules[3].selectors.len(), 1);
    assert_eq!(rules[3].selectors[0].text(&root).to_string(), ".foo ~ .bar");
}

#[test]
fn test_selector_list_with_nesting() {
    let css = r#"
            .a, .b {
                div {
                    color: red;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rules = model.rules();
    assert_eq!(parent_rules.len(), 1);

    let parent_rule = &parent_rules[0];
    assert_eq!(parent_rule.selectors.len(), 2);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".a");
    assert_eq!(parent_rule.selectors[1].text(&root).to_string(), ".b");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    // TODO: Should expand to 2 selectors ".a div" and ".b div" once nested selector resolution is implemented
    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(child_rule.selectors[0].text(&root).to_string(), "div");
}

#[test]
fn test_ampersand_nesting_selector() {
    let css = r#"
            .foo {
                .bar &:hover {
                    color: red;
                }
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(parent_rule.selectors[0].text(&root).to_string(), ".foo");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    // TODO: Should resolve to ".bar .foo:hover" once nested selector resolution is implemented
    assert_eq!(
        child_rule.selectors[0].text(&root).to_string(),
        ".bar &:hover"
    );
}

#[test]
fn test_attribute_class_id_selector() {
    let css = r#"
            type[attribute].class#id, div {
                color: red;
            }
        "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let root = model.root();
    let rules = model.rules();
    assert_eq!(rules.len(), 1);

    let rule = &rules[0];
    assert_eq!(rule.selectors.len(), 2);
    assert_eq!(
        rule.selectors[0].text(&root).to_string(),
        "type[attribute].class#id"
    );
    assert_eq!(rule.selectors[1].text(&root).to_string(), "div");
}

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
    assert_eq!(rule.selectors[0].resolved().to_string(), "div");
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".parent");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".parent .child"
    );
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), "a");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), "a:hover");
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
        grandparent_rule.selectors[0].resolved().to_string(),
        ".grandparent"
    );

    let parent_rule_id = grandparent_rule.child_ids.first().unwrap();
    let parent_rule = model.get_rule_by_id(parent_rule_id).unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(
        parent_rule.selectors[0].resolved().to_string(),
        ".grandparent .parent"
    );

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".grandparent .parent .child"
    );
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), "p");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), "p + p");
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".list");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), ".list li");
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
        parent_rule.selectors[0].resolved().to_string(),
        ".menu > ul"
    );

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".menu > ul > li"
    );
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".btn");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".btn--primary:hover .btn__icon"
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
        grandparent_rule.selectors[0].resolved().to_string(),
        ".grandparent"
    );

    let parent_rule_id = grandparent_rule.child_ids.first().unwrap();
    let parent_rule = model.get_rule_by_id(parent_rule_id).unwrap();
    assert_eq!(parent_rule.selectors.len(), 1);
    assert_eq!(
        parent_rule.selectors[0].resolved().to_string(),
        ".grandparent .parent"
    );

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".grandparent .parent--modified"
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
    assert_eq!(rule.selectors[0].resolved().to_string(), ".foo .bar");
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
    assert_eq!(rules[0].selectors[0].resolved().to_string(), ".foo > .bar");

    assert_eq!(rules[1].selectors.len(), 1);
    assert_eq!(rules[1].selectors[0].resolved().to_string(), ".foo || .bar");

    assert_eq!(rules[2].selectors.len(), 1);
    assert_eq!(rules[2].selectors[0].resolved().to_string(), ".foo + .bar");

    assert_eq!(rules[3].selectors.len(), 1);
    assert_eq!(rules[3].selectors[0].resolved().to_string(), ".foo ~ .bar");
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".a");
    assert_eq!(parent_rule.selectors[1].resolved().to_string(), ".b");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 2);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), ".a div");
    assert_eq!(child_rule.selectors[1].resolved().to_string(), ".b div");
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
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".foo");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();

    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".bar .foo:hover"
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
        rule.selectors[0].resolved().to_string(),
        "type[attribute].class#id"
    );
    assert_eq!(rule.selectors[1].resolved().to_string(), "div");
}

#[test]
fn test_universal_selector() {
    let css = "* { box-sizing: border-box; }";
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rule = model.rules().first().unwrap();
    assert_eq!(rule.selectors.len(), 1);
    assert_eq!(rule.selectors[0].resolved().to_string(), "*");
}

#[test]
fn test_id_selector() {
    let css = "#app { display: flex; }";
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rule = model.rules().first().unwrap();
    assert_eq!(rule.selectors.len(), 1);
    assert_eq!(rule.selectors[0].resolved().to_string(), "#app");
}

#[test]
fn test_pseudo_element_selector() {
    let css = r#"
        p::before { content: "»"; }
        a::after  { content: "«"; }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rules = model.rules();
    assert_eq!(rules.len(), 2);
    assert_eq!(rules[0].selectors[0].resolved().to_string(), "p::before");
    assert_eq!(rules[1].selectors[0].resolved().to_string(), "a::after");
}

#[test]
fn test_nested_pseudo_element() {
    let css = r#"
        a {
            &::before {
                content: "→";
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), "a");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), "a::before");
}

#[test]
fn test_selector_list_with_ampersand() {
    // .a, .b { &:hover {} } → ".a:hover" and ".b:hover"
    let css = r#"
        .a, .b {
            &:hover {
                color: red;
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors.len(), 2);
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".a");
    assert_eq!(parent_rule.selectors[1].resolved().to_string(), ".b");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 2);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), ".a:hover");
    assert_eq!(child_rule.selectors[1].resolved().to_string(), ".b:hover");
}

#[test]
fn test_nested_sibling_combinator() {
    // parent { & ~ .sibling {} } → "parent ~ .sibling"
    let css = r#"
        h2 {
            & ~ p {
                margin-top: 0;
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), "h2");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(child_rule.selectors[0].resolved().to_string(), "h2 ~ p");
}

#[test]
fn test_nested_inside_media() {
    // @media nesting: the @media rule is skipped when looking for the parent selector,
    // so `.card` inside `@media` still resolves against the outer `.card` selector.
    let css = r#"
        .card {
            @media (min-width: 600px) {
                .title {
                    font-size: 2rem;
                }
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let card_rule = model.rules().first().unwrap();
    assert_eq!(card_rule.selectors[0].resolved().to_string(), ".card");

    // The @media rule is a child of .card.
    let media_rule_id = card_rule.child_ids.first().unwrap();
    let media_rule = model.get_rule_by_id(media_rule_id).unwrap();

    // The .title rule is nested inside @media.
    let title_rule_id = media_rule.child_ids.first().unwrap();
    let title_rule = model.get_rule_by_id(title_rule_id).unwrap();

    // @media is transparent for selector resolution: .title resolves as ".card .title".
    assert_eq!(title_rule.selectors.len(), 1);
    assert_eq!(
        title_rule.selectors[0].resolved().to_string(),
        ".card .title"
    );
}

#[test]
fn test_nested_inside_supports() {
    // @supports is skipped like @media.
    let css = r#"
        .grid {
            @supports (display: grid) {
                &--full {
                    grid-column: 1 / -1;
                }
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let grid_rule = model.rules().first().unwrap();
    assert_eq!(grid_rule.selectors[0].resolved().to_string(), ".grid");

    let supports_rule_id = grid_rule.child_ids.first().unwrap();
    let supports_rule = model.get_rule_by_id(supports_rule_id).unwrap();

    let inner_rule_id = supports_rule.child_ids.first().unwrap();
    let inner_rule = model.get_rule_by_id(inner_rule_id).unwrap();

    assert_eq!(inner_rule.selectors.len(), 1);
    assert_eq!(
        inner_rule.selectors[0].resolved().to_string(),
        ".grid--full"
    );
}

#[test]
fn test_multiple_top_level_rules() {
    let css = r#"
        h1 { color: red; }
        h2 { color: blue; }
        h3 { color: green; }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rules = model.rules();
    assert_eq!(rules.len(), 3);
    assert_eq!(rules[0].selectors[0].resolved().to_string(), "h1");
    assert_eq!(rules[1].selectors[0].resolved().to_string(), "h2");
    assert_eq!(rules[2].selectors[0].resolved().to_string(), "h3");
}

#[test]
fn test_ampersand_suffix_nesting() {
    // BEM modifier: .block { &--modifier {} } → ".block--modifier"
    let css = r#"
        .block {
            &--modifier {
                font-weight: bold;
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let parent_rule = model.rules().first().unwrap();
    assert_eq!(parent_rule.selectors[0].resolved().to_string(), ".block");

    let child_rule_id = parent_rule.child_ids.first().unwrap();
    let child_rule = model.get_rule_by_id(child_rule_id).unwrap();
    assert_eq!(child_rule.selectors.len(), 1);
    assert_eq!(
        child_rule.selectors[0].resolved().to_string(),
        ".block--modifier"
    );
}

#[test]
fn test_deeply_nested_ampersand() {
    // Three levels: .a { .b { &.c {} } } → ".a .b.c"
    let css = r#"
        .a {
            .b {
                &.c {
                    color: red;
                }
            }
        }
    "#;
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let a_rule = model.rules().first().unwrap();
    assert_eq!(a_rule.selectors[0].resolved().to_string(), ".a");

    let b_rule_id = a_rule.child_ids.first().unwrap();
    let b_rule = model.get_rule_by_id(b_rule_id).unwrap();
    assert_eq!(b_rule.selectors[0].resolved().to_string(), ".a .b");

    let c_rule_id = b_rule.child_ids.first().unwrap();
    let c_rule = model.get_rule_by_id(c_rule_id).unwrap();
    assert_eq!(c_rule.selectors.len(), 1);
    assert_eq!(c_rule.selectors[0].resolved().to_string(), ".a .b.c");
}

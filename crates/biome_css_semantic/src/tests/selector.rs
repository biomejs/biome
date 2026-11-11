use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::CssFileSource;

use crate::model::{ResolvedSelector, Selector};
use crate::semantic_model;

#[test]
fn test_resolve_selector_no_parents() {
    let css = "div { color: red; }";
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rules = model.rules();
    let rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let grandparent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let grandparent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let rule = rules.first().unwrap();

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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let rule = rules.first().unwrap();
    assert_eq!(rule.selectors.len(), 1);
    assert_eq!(rule.selectors[0].resolved().to_string(), "*");
}

#[test]
fn test_id_selector() {
    let css = "#app { display: flex; }";
    let parse = parse_css(css, CssFileSource::css(), CssParserOptions::default());
    let root = parse.tree();
    let model = semantic_model(&root);

    let rules = model.rules();
    let rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let card_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let grid_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let parent_rule = rules.first().unwrap();
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

    let rules = model.rules();
    let a_rule = rules.first().unwrap();
    assert_eq!(a_rule.selectors[0].resolved().to_string(), ".a");

    let b_rule_id = a_rule.child_ids.first().unwrap();
    let b_rule = model.get_rule_by_id(b_rule_id).unwrap();
    assert_eq!(b_rule.selectors[0].resolved().to_string(), ".a .b");

    let c_rule_id = b_rule.child_ids.first().unwrap();
    let c_rule = model.get_rule_by_id(c_rule_id).unwrap();
    assert_eq!(c_rule.selectors.len(), 1);
    assert_eq!(c_rule.selectors[0].resolved().to_string(), ".a .b.c");
}

fn resolved_deepest_scss_selector(source: &str, depth: usize) -> String {
    deepest_scss_selector(source, depth).resolved().to_string()
}

fn deepest_scss_selector(source: &str, depth: usize) -> Selector {
    let parse = parse_css(source, CssFileSource::scss(), CssParserOptions::default());
    assert!(parse.diagnostics().is_empty(), "{:#?}", parse.diagnostics());

    let root = parse.tree();
    let model = semantic_model(&root);
    let mut rule = model
        .rules()
        .into_iter()
        .next()
        .expect("expected a root style rule");

    for _ in 0..depth {
        let child_id = rule
            .child_ids
            .first()
            .expect("expected a nested style rule");
        rule = model
            .get_rule_by_id(child_id)
            .expect("expected the nested rule in the semantic model");
    }

    rule.selectors
        .first()
        .expect("expected the nested rule selector")
        .clone()
}

/// Returns each compound of the selector as `combinator|simple selectors`, with simple selectors
/// formatted as `kind:text`.
fn compounds(source: &str, depth: usize) -> Vec<String> {
    let selector = deepest_scss_selector(source, depth);
    selector
        .resolved()
        .compounds()
        .map(|compound| {
            let combinator = compound.combinator().map_or("", |(_, text)| text.text());
            let simple_selectors: Vec<_> = compound
                .simple_selectors()
                .map(|simple| {
                    let text: String = simple.tokens().iter().map(|(_, t)| t.text()).collect();
                    format!("{:?}:{text}", simple.kind())
                })
                .collect();
            format!("{combinator}|{}", simple_selectors.join(" "))
        })
        .collect()
}

#[test]
fn test_compounds_split_at_top_level_combinators() {
    assert_eq!(
        compounds("ul > li.a:not(.b > .c)[x~=y]::before {}", 0),
        [
            "|Type:ul",
            ">|Type:li Class:.a PseudoClass::not(.b>.c) Attribute:[x~=y] PseudoElement:::before",
        ]
    );
    assert_eq!(
        compounds("a:nth-child(2n + 1) b:is(.c .d) {}", 0),
        [
            "|Type:a PseudoClass::nth-child(2n+1)",
            " |Type:b PseudoClass::is(.c .d)"
        ]
    );
}

#[test]
fn test_compounds_keep_scss_suffix_on_parent_selector() {
    assert_eq!(
        compounds(".a { &--x + &--y#id {} }", 1),
        ["|Class:.a--x", "+|Class:.a--y Id:#id"]
    );
}

#[test]
fn test_compounds_classify_scss_selectors() {
    assert_eq!(
        compounds("%p.a#{$b} {}", 0),
        ["|Placeholder:%p Class:.a Interpolation:#{$b}"]
    );
}

#[test]
fn test_subject_is_last_compound() {
    let selector = deepest_scss_selector(".a { ul li:hover {} }", 1);
    let subject = selector
        .resolved()
        .subject()
        .expect("expected a non-empty selector");
    assert_eq!(subject.combinator().map(|(_, text)| text.text()), Some(" "));
    let names: Vec<_> = subject
        .simple_selectors()
        .map(|simple| simple.name())
        .collect();
    assert_eq!(names, [Some("li"), Some("hover")]);
}

#[test]
fn test_empty_selector_has_no_subject() {
    let selector = ResolvedSelector(Vec::new());
    assert_eq!(selector.compounds().count(), 0);
    assert!(selector.subject().is_none());
}

#[test]
fn test_normalize_sorts_and_lowercases() {
    let normalize = |source| deepest_scss_selector(source, 0).resolved().normalize();
    assert_eq!(normalize("DIV.b.a > A:hover {}"), "div.a.b > a:hover");
    assert_eq!(normalize(":not(.b.a).c {}"), ".c:not(.b.a)");
    assert_eq!(normalize("a:is(.b > .c) {}"), "a:is(.b > .c)");
}

#[test]
fn test_at_root_query_style_rule_semantics() {
    for (source, expected) in [
        (".outer { @at-root { .inner {} } }", ".inner"),
        (
            ".outer { @at-root (with: rule) { .inner {} } }",
            ".outer .inner",
        ),
        (
            ".outer { @at-root (without: rule) { .inner {} } }",
            ".inner",
        ),
        (".outer { @at-root (with: media) { .inner {} } }", ".inner"),
        (
            ".outer { @at-root (without: media) { .inner {} } }",
            ".outer .inner",
        ),
        (
            ".outer { @at-root (WITH: ALL) { .inner {} } }",
            ".outer .inner",
        ),
        (".outer { @at-root (WITHOUT: ALL) { .inner {} } }", ".inner"),
    ] {
        assert_eq!(resolved_deepest_scss_selector(source, 2), expected);
    }
}

#[test]
fn test_resolve_scss_partial_combinator_with_left_selector() {
    let resolved = resolved_deepest_scss_selector(".sidebar > { .error {} }", 1);
    assert_eq!(resolved, ".sidebar > .error");
}

#[test]
fn test_resolve_scss_partial_combinator_without_left_selector() {
    let resolved = resolved_deepest_scss_selector(".card { + { .media {} } }", 2);
    assert_eq!(resolved, ".card + .media");
}

#[test]
fn test_resolve_scss_partial_combinator_with_nesting_selector() {
    let resolved = resolved_deepest_scss_selector(".navbar { & ~ { .container {} } }", 2);
    assert_eq!(resolved, ".navbar ~ .container");
}

#[test]
fn test_requires_scss_evaluation() {
    let requires = |source| {
        deepest_scss_selector(source, 0)
            .resolved()
            .requires_scss_evaluation()
    };
    assert!(requires("%placeholder {}"));
    assert!(requires(".a-#{$b} {}"));
    assert!(requires("[title=\"#{$x}\"] {}"));
    assert!(!requires("[data-x=\"50%\"] {}"));
    assert!(!requires("#id {}"));
}

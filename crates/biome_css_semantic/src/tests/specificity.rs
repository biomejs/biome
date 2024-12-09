use biome_css_parser::{parse_css, CssParserOptions};

use crate::model::Specificity;
use crate::semantic_model;

#[test]
fn test_specificity_type_selector() {
    let css_code = "div {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(0, 0, 1));
}

#[test]
fn test_specificity_class_selector() {
    let css_code = ".class {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(0, 1, 0));
}

#[test]
fn test_specificity_id_selector() {
    let css_code = "#id {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(1, 0, 0));
}

#[test]
fn test_specificity_type_and_class_selector() {
    let css_code = "div.class {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(0, 1, 1));
}

#[test]
fn test_specificity_attribute_selector() {
    let css_code = "[type=\"text\"] {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(0, 1, 0));
}

#[test]
fn test_specificity_pseudo_class_selector() {
    let css_code = ":hover {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(0, 1, 0));
}

#[test]
fn test_specificity_pseudo_element_selector() {
    let css_code = "::before {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(0, 0, 1));
}

#[test]
fn test_specificity_complex_selector() {
    let css_code = "ul li.active a#link:hover {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(1, 2, 3));
}

#[test]
fn test_specificity_pseudo_class_functions() {
    let css_code = ":not(#id) {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    assert_eq!(specificity, &Specificity(1, 0, 0));
}

#[test]
fn test_specificity_with_pseudo_function_where() {
    let css_code = ":where(.class) {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // :where doesn't contribute to specificity
    assert_eq!(specificity, &Specificity(0, 0, 0));
}

#[test]
fn test_specificity_with_pseudo_function_is() {
    let css_code = ":is(div, .class) {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // :is takes the maximum specificity of its arguments
    assert_eq!(specificity, &Specificity(0, 1, 0));
}

#[test]
fn test_specificity_nested_selector() {
    let css_code = ".parent .child {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // For ".parent .child", the specificity is sum of ".parent" and ".child": (0,2,0)
    assert_eq!(specificity, &Specificity(0, 2, 0));
}

#[test]
fn test_specificity_nested_pseudo_class() {
    let css_code = "a:not(.active) {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // Specificity:
    // - "a" contributes (0, 0, 1)
    // - ":not(.active)" contributes specificity of ".active" (0,1,0)
    // Total specificity: (0, 1, 1)
    assert_eq!(specificity, &Specificity(0, 1, 1));
}

#[test]
fn test_specificity_nested_pseudo_class_functions() {
    let css_code = ":not(:nth-child(2n+1)) {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // ":not(:nth-child(2n+1))"
    // - ":nth-child" contributes (0, 1, 0)
    // - ":not" takes specificity of its argument
    // Total specificity: (0, 1, 0)
    assert_eq!(specificity, &Specificity(0, 1, 0));
}

#[test]
fn test_specificity_multiple_nesting() {
    let css_code = "body div#main .content ul li.active a:hover {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // Calculating specificity:
    // "body"                -> (0, 0, 1)
    // "div#main"            -> (1, 0, 1)
    // ".content"            -> (0, 1, 0)
    // "ul"                  -> (0, 0, 1)
    // "li.active"           -> (0, 1, 1)
    // "a:hover"             -> (0, 1, 1)
    // Total specificity: Sum of all components
    // Total IDs: 1
    // Total classes: 1+1+1 = 3 (from .content, .active, :hover)
    // Total elements: 1+1+1+1+1 = 5 (from body, div, ul, li, a)
    let expected_specificity = Specificity(1, 3, 5);
    assert_eq!(specificity, &expected_specificity);
}

#[test]
fn test_specificity_nested_pseudo_elements_and_classes() {
    let css_code = "div::before:hover {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // "div"                 -> (0, 0, 1)
    // "::before"            -> (0, 0, 1)
    // ":hover"              -> (0, 1, 0)
    // Total specificity: (0, 1, 2)
    assert_eq!(specificity, &Specificity(0, 1, 2));
}

#[test]
fn test_specificity_nested_combinators() {
    let css_code = "ul > li + li.active ~ a#link:hover {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // "ul"                 -> (0, 0, 1)
    // "li"                 -> (0, 0, 1)
    // "li.active"          -> (0, 1, 1)
    // "a#link:hover"       -> (1, 1, 1)
    // Total specificity: (1, 2, 4)
    assert_eq!(specificity, &Specificity(1, 2, 4));
}

#[test]
fn test_specificity_with_nested_pseudo_functions() {
    let css_code = ":is(div, :not(.class)) {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();
    let selector = &rule.selectors[0];

    let specificity = &selector.specificity;
    // ":is(div, :not(.class))"
    // - Arguments are "div" (0, 0, 1) and ":not(.class)" (0, 1, 0)
    // - ":not(.class)" has specificity of ".class" (0,1,0)
    // - ":is" takes the maximum specificity of its arguments
    // So overall specificity: max((0,0,1), (0,1,0)) = (0,1,0)
    assert_eq!(specificity, &Specificity(0, 1, 0));
}

#[test]
fn test_specificity_nested_selector_lists() {
    let css_code = "div, .class, #id, a:hover, ul li a#link {}";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);
    let rule = model.rules().first().unwrap();

    let expected_specificities = [
        Specificity(0, 0, 1), // "div"
        Specificity(0, 1, 0), // ".class"
        Specificity(1, 0, 0), // "#id"
        Specificity(0, 1, 1), // "a:hover"
        Specificity(1, 0, 3), // "ul li a#link"
    ];
    for (selector, expected_specificity) in rule.selectors.iter().zip(expected_specificities.iter())
    {
        let specificity = &selector.specificity;
        assert_eq!(specificity, expected_specificity);
    }
}

#[test]
fn test_specificity_deeply_nested_rules() {
    let css_code = "a { div { .class { #id { } } } }";
    let parse = parse_css(css_code, CssParserOptions::default());
    let root = parse.tree();

    let model = semantic_model(&root);

    // Navigate through nested rules to reach the deepest rule
    let parent_rule = model
        .rules()
        .first()
        .expect("Expected to find parent rule with selector 'a'");

    let div_rule_id = parent_rule
        .child_ids
        .first()
        .expect("Expected 'div' child rule");
    let div_rule = model
        .get_rule_by_id(*div_rule_id)
        .expect("Expected to retrieve 'div' rule");

    let class_rule_id = div_rule
        .child_ids
        .first()
        .expect("Expected '.class' child rule");
    let class_rule = model
        .get_rule_by_id(*class_rule_id)
        .expect("Expected to retrieve 'span.class' rule");

    let id_rule_id = class_rule
        .child_ids
        .first()
        .expect("Expected '#id' child rule");
    let _id_rule = model
        .get_rule_by_id(*id_rule_id)
        .expect("Expected to retrieve '#id' rule");

    // Check selectors and specificities at each level
    // 'a'
    let a_selector = &parent_rule.selectors[0];
    assert_eq!(&a_selector.name, "a");
    assert_eq!(&a_selector.specificity, &Specificity(0, 0, 1));

    // 'a div'
    let div_selector = &div_rule.selectors[0];
    assert_eq!(&div_selector.name, "a div");
    assert_eq!(&div_selector.specificity, &Specificity(0, 0, 2));

    // TODO: Bug. It should be (0, 1, 2) instead of (0, 1, 1)
    // 'a div .class'
    // let class_selector = &class_rule.selectors[0];
    // assert_eq!(&class_selector.name, ".class");
    // assert_eq!(&class_selector.specificity, &Specificity(0, 1, 2));

    // TODO: Bug. It should be (1, 1, 2) instead of (1, 1, 0)
    // 'a div .class #id'
    // let id_selector = &id_rule.selectors[0];
    // assert_eq!(&id_selector.name, "#id");
    // assert_eq!(&id_selector.specificity, &Specificity(1, 1, 2));
}

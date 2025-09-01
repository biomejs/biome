pub mod builder;
pub mod model;
pub mod specificity;

use biome_css_syntax::CssRoot;
use biome_rowan::AstNode;
use builder::SemanticModelBuilder;
use model::SemanticModel;

use crate::events::SemanticEventExtractor;

pub fn semantic_model(root: &CssRoot) -> SemanticModel {
    let mut extractor = SemanticEventExtractor::default();
    let mut builder = SemanticModelBuilder::new(root.clone());

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            biome_css_syntax::WalkEvent::Enter(node) => {
                extractor.enter(&node);
            }
            biome_css_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }

    while let Some(e) = extractor.pop() {
        builder.push_event(e);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use biome_css_parser::CssParserOptions;
    use biome_css_parser::parse_css;

    #[test]
    fn test_simple_ruleset() {
        let parse = parse_css(
            r#"p {
  font-family: verdana;
  font-size: 20px;
}"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rule = model.rules().first().unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);
        assert_eq!(rule.child_ids.len(), 0);
        assert_eq!(rule.parent_id, None);
    }
    #[test]
    fn test_nested_selector() {
        let parse = parse_css(
            r#".parent {
  color: blue;

  .child {
    color: red;
  }
}"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rule = model.rules().first().unwrap();
        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 1);
        assert_eq!(rule.child_ids.len(), 1);

        let child_id = rule.child_ids.first().unwrap();
        let child = model.get_rule_by_id(child_id).unwrap();

        assert_eq!(child.selectors.len(), 1);
        assert_eq!(child.declarations.len(), 1);
        assert_eq!(child.child_ids.len(), 0);
        assert_eq!(child.parent_id, Some(rule.id));
    }

    #[test]
    fn test_nested_sub_selector() {
        let parse = parse_css(
            r#"a {
        &:hover {
            color: orange;
        }
}"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rule = model.rules().first().unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 0);
        assert_eq!(rule.child_ids.len(), 1);

        let child_id = rule.child_ids.first().unwrap();
        let child = model.get_rule_by_id(child_id).unwrap();
        assert_eq!(child.selectors.len(), 1);
        assert_eq!(child.declarations.len(), 1);
        assert_eq!(child.child_ids.len(), 0);
        assert_eq!(child.parent_id, Some(rule.id));
    }

    #[test]
    fn test_nested_at_media() {
        let parse = parse_css(
            r#"a {
        @media {
            color: orange;
        }
}"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rule = model.rules().first().unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 0);
        assert_eq!(rule.child_ids.len(), 1);

        let child_id = rule.child_ids.first().unwrap();
        let child = model.get_rule_by_id(child_id).unwrap();
        assert_eq!(child.selectors.len(), 0);
        assert_eq!(child.declarations.len(), 1);
        assert_eq!(child.child_ids.len(), 0);
        assert_eq!(child.parent_id, Some(rule.id));
    }

    #[test]
    fn test_global_custom_variables() {
        let parse = parse_css(
            r#"@property --item-size {
  syntax: "<percentage>";
  inherits: true;
  initial-value: 40%;
}

:root {
  --custom-color: red;
  --custom-size: 20px;
}
  "#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let global_custom_variables = model.global_custom_variables();

        assert_eq!(global_custom_variables.len(), 3);

        let item_size = global_custom_variables.contains_key("--item-size");
        let custom_color = global_custom_variables.contains_key("--custom-color");
        let custom_size = global_custom_variables.contains_key("--custom-size");

        assert!(item_size);
        assert!(custom_color);
        assert!(custom_size);
    }

    #[test]
    fn test_empty_at_property() {
        let parse = parse_css(r#"@property --item-size {}"#, CssParserOptions::default());

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let global_custom_variables = model.global_custom_variables();

        assert_eq!(global_custom_variables.len(), 1);

        let item_size = global_custom_variables.contains_key("--item-size");

        assert!(item_size);
    }

    #[ignore]
    #[test]
    fn quick_test() {
        let parse = parse_css(
            r#"@property --item-size {
  syntax: "<percentage>";
  inherits: true;
  initial-value: 40%;
}"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        dbg!(&model.rules());
        dbg!(&model.global_custom_variables());
    }
}

#[cfg(test)]
mod specificity_tests {
    use crate::model::{SemanticModel, Specificity};
    use biome_css_parser::{CssParserOptions, parse_css};

    fn to_semantic_model(source: &str) -> SemanticModel {
        let parse = parse_css(source, CssParserOptions::default());
        let root = parse.tree();
        super::semantic_model(&root)
    }

    #[test]
    fn selector() {
        let source = "div";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1))
    }

    #[test]
    fn selector_id() {
        let source = "#div";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(1, 0, 0))
    }

    #[test]
    fn selector_class() {
        let source = ".div";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 1, 0))
    }

    #[test]
    fn selector_combinations() {
        let source = "#div .div {} #div .div div {} .div .div {}";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(
            specificity.next().unwrap(),
            Specificity(1, 1, 0),
            "#div .div"
        );

        assert_eq!(
            specificity.next().unwrap(),
            Specificity(1, 1, 1),
            "#div .div div"
        );
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 2, 0),
            ".div .div"
        );
    }

    #[test]
    fn nested_selector() {
        let source = r#"div {
        & > span {}
    } "#;
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "div");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 0, 2),
            "& > span"
        );
    }

    #[test]
    fn nested_selectors_with_media_query() {
        let source = r#"div {
  display: flex;

  & > p {
    justify-content: start;
  }

  @media (orientation: portrait) {
    & > p {
      justify-content: center;
    }
  }
}"#;
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "div");
        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 2), "& > p");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 0, 2),
            "@media (orientation: portrait) & > p"
        );
    }

    #[test]
    fn nested_selectors() {
        let source = r#"
#div {
    .div {
        div {
        }
    }
}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(1, 0, 0), "#div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 0), ".div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 1), "div");
    }

    #[test]
    fn nested_selectors_multiple_parents() {
        let source = r#"
#div {
    .div {
        div {
            & > p {}
            & & > p {}
            & & & > p {}
        }
    }
}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(1, 0, 0), "#div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 0), ".div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 1), "div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 2), "& > p");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 1), "& & > p");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(1, 0, 1),
            "& & & > p"
        );
    }

    #[test]
    fn comma_separated_with_parent() {
        let source = r#"div, span { & > p {} }}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        assert_eq!(specificity.len(), 3);

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "div");
        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "span");
        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 2), "& > p");
    }

    #[test]
    fn pseudo_selector() {
        let source = r#":is(#fake#fake#fake#fake#fake#fake, *) g {}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(6, 0, 1));
    }
}

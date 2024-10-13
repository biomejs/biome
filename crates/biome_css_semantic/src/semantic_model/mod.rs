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
    use biome_css_parser::parse_css;
    use biome_css_parser::CssParserOptions;
    use biome_rowan::TextRange;

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
        let child = model.get_rule_by_id(*child_id).unwrap();

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
        let child = model.get_rule_by_id(*child_id).unwrap();
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
        let child = model.get_rule_by_id(*child_id).unwrap();
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

    #[test]
    fn test_get_rule_by_range() {
        let parse = parse_css(
            r#"p {color: red; font-size: 12px;}"#,
            CssParserOptions::default(),
        );
        let root = parse.tree();
        let model = super::semantic_model(&root);

        // range of the declaration 'red'
        let range = TextRange::new(10.into(), 13.into());
        let rule = model.get_rule_by_range(range).unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);

        assert_eq!(rule.selectors[0].name, "p");
        assert_eq!(rule.declarations[0].property.name, "color");
        assert_eq!(rule.declarations[0].value.text, "red");

        assert_eq!(rule.declarations[1].property.name, "font-size");
        assert_eq!(rule.declarations[1].value.text, "12px");

        let range = TextRange::new(0.into(), 1.into());
        let rule = model.get_rule_by_range(range).unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);

        assert_eq!(rule.selectors[0].name, "p");
        assert_eq!(rule.declarations[0].property.name, "color");
        assert_eq!(rule.declarations[0].value.text, "red");

        assert_eq!(rule.declarations[1].property.name, "font-size");
        assert_eq!(rule.declarations[1].value.text, "12px");
    }

    #[test]
    fn test_nested_get_rule_by_range() {
        let parse = parse_css(
            r#"p { --foo: red; font-size: 12px;
            .child { color: var(--foo)}
            }"#,
            CssParserOptions::default(),
        );
        let root = parse.tree();
        let model = super::semantic_model(&root);

        // range of the declaration 'blue' in '.child'
        let range = TextRange::new(60.into(), 64.into());
        let rule = model.get_rule_by_range(range).unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 1);

        assert_eq!(rule.selectors[0].name, ".child");

        assert_eq!(rule.declarations[0].property.name, "color");
        assert_eq!(rule.declarations[0].value.text, "var(--foo)");

        let parent = model.get_rule_by_id(rule.parent_id.unwrap()).unwrap();
        assert_eq!(parent.selectors.len(), 1);
        assert_eq!(parent.declarations.len(), 2);

        assert_eq!(parent.selectors[0].name, "p");
        assert_eq!(parent.declarations[0].property.name, "--foo");
        assert_eq!(parent.declarations[0].value.text, "red");

        assert_eq!(parent.declarations[1].property.name, "font-size");
        assert_eq!(parent.declarations[1].value.text, "12px");
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

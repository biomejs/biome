pub mod builder;
pub mod model;

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
                builder.push_node(&node);
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
        assert_eq!(rule.children.len(), 1);
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
        assert_eq!(rule.children.len(), 1);

        let child = rule.children.first().unwrap();
        assert_eq!(child.selectors.len(), 1);
        assert_eq!(child.declarations.len(), 1);
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
        assert_eq!(rule.children.len(), 1);

        let child = rule.children.first().unwrap();
        assert_eq!(child.selectors.len(), 0);
        assert_eq!(child.declarations.len(), 1);
    }

    #[test]
    fn test_global_css_variables() {
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
        let global_custom_variables = model.global_css_variables();

        assert_eq!(global_custom_variables.len(), 3);

        let item_size = global_custom_variables.contains_key("--item-size");
        let custom_color = global_custom_variables.contains_key("--custom-color");
        let custom_size = global_custom_variables.contains_key("--custom-size");

        assert!(item_size);
        assert!(custom_color);
        assert!(custom_size);
    }

    #[test]
    fn debug() {
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
        dbg!(&model.global_css_variables());
    }
}

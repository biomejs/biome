mod builder;
mod model;

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
    use biome_rowan::TextRange;

    #[test]
    fn test_simple_ruleset() {
        let parse = parse_css(r#".class { color: red; }"#, CssParserOptions::default());

        let root = parse.tree();
        let model = super::semantic_model(&root);

        let selector_range = TextRange::new(0.into(), 7.into());
        assert_eq!(model.selectors(selector_range).unwrap().len(), 1);
        assert_eq!(model.selectors(selector_range).unwrap()[0].0, ".class");

        assert_eq!(model.declarations(selector_range).unwrap().len(), 1);
        let declaration = &model.declarations(selector_range).unwrap()[0];
        assert_eq!(declaration.property, "color");
        assert_eq!(declaration.value, "red");
    }

    #[test]
    fn test_semantic_model() {
        let parse = parse_css(
            r#".foo .bar { color: red; text-align: center; }"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);

        let selector_range = TextRange::new(0.into(), 10.into());

        assert_eq!(model.selectors(selector_range).unwrap().len(), 2);
        assert_eq!(model.selectors(selector_range).unwrap()[0].0, ".foo");
        assert_eq!(model.selectors(selector_range).unwrap()[1].0, ".bar");

        assert_eq!(model.declarations(selector_range).unwrap().len(), 2);
        assert_eq!(
            model.declarations(selector_range).unwrap()[0].property,
            "color"
        );
        assert_eq!(model.declarations(selector_range).unwrap()[0].value, "red");
        assert_eq!(
            model.declarations(selector_range).unwrap()[1].property,
            "text-align"
        );
        assert_eq!(
            model.declarations(selector_range).unwrap()[1].value,
            "center"
        );
    }
}

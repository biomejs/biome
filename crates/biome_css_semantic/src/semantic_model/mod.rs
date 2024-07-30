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

    #[test]
    fn test_semantic_model() {
        use biome_css_parser::CssParserOptions;
        let parse = parse_css(
            r#".class .class { color: red; color: green; } .foo .foo { color: green; }"#,
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        dbg!(&model);
    }
}

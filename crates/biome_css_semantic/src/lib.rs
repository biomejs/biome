mod events;
mod model;

use biome_css_syntax::CssRoot;
use biome_rowan::AstNode;

use crate::events::CssSemanticEventExtractor;

pub fn create_css_semantic_model(root: CssRoot) {
    // let mut builder = CssSemanticModelBuilder::new(root.clone());
    let mut extractor = CssSemanticEventExtractor::default();

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            biome_rowan::WalkEvent::Enter(node) => {
                // builder.push_node(&node);
                extractor.enter(&node);
            }
            biome_rowan::WalkEvent::Leave(node) => {
                extractor.leave(&node);
            }
        }
    }

    while let Some(_e) = extractor.pop() {
        // builder.push_event(e);
    }
    // builder.build();
    todo!()
}

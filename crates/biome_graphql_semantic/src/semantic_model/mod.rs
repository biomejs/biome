mod binding;
mod builder;
mod model;
mod reference;

use biome_graphql_syntax::GraphqlRoot;
use biome_rowan::AstNode;

pub use binding::*;
pub use builder::*;
pub use model::*;
pub use reference::*;

use crate::SemanticEventExtractor;

/// Build the complete [SemanticModel] of a parsed file.
/// For a push based model to build the [SemanticModel], see [SemanticModelBuilder].
pub fn semantic_model(root: &GraphqlRoot) -> SemanticModel {
    let mut extractor = SemanticEventExtractor::default();
    let mut builder = SemanticModelBuilder::new(root.clone());

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            biome_graphql_syntax::WalkEvent::Enter(node) => {
                builder.push_node(&node);
                extractor.enter(&node);
            }
            biome_graphql_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }

    while let Some(e) = extractor.pop() {
        builder.push_event(e);
    }

    builder.build()
}

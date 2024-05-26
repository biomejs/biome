mod events;
mod model;

use biome_css_syntax::CssRoot;
use biome_rowan::AstNode;

pub fn create_css_semantic_model(root: CssRoot) {
    let root = root.syntax();
    for node in root.preorder() {
        match node {
            biome_rowan::WalkEvent::Enter(_) => todo!(),
            biome_rowan::WalkEvent::Leave(_) => todo!(),
        }
    }
    todo!()
}

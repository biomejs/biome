use biome_js_syntax::{JsSyntaxNode, JsSyntaxToken};

pub struct JsSyntaxMatchPair {
    pub member_name: JsSyntaxToken,
    pub matching_array_element: JsSyntaxNode,
}

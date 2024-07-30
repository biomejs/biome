use biome_css_syntax::{AnyCssRule, AnyCssSelector, CssRoot, CssSyntaxKind, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

use super::model::{SemanticModel, SemanticModelData};

/// Builds the [SemanticModel] consuming [SemanticEvent] and [GraphqlSyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvent] and build all the
/// data necessary to build a semantic model, that is allocated with an
/// [std::rc::Rc] and stored inside the [SemanticModel].
pub struct SemanticModelBuilder {
    root: CssRoot,
    node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    selectors: Vec<AnyCssSelector>,
    rules: Vec<AnyCssRule>,
}

impl SemanticModelBuilder {
    pub fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            selectors: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            node_by_range: self.node_by_range,
            selectors: self.selectors,
            rules: self.rules,
        };
        SemanticModel::new(data)
    }

    #[inline]
    pub fn push_node(&mut self, node: &CssSyntaxNode) {
        use CssSyntaxKind::*;
        if matches!(node.kind(), CSS_QUALIFIED_RULE | CSS_SELECTOR_LIST) {
            self.node_by_range.insert(node.text_range(), node.clone());
        }
    }
}

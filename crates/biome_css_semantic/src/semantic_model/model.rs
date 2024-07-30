use std::rc::Rc;

use biome_css_syntax::{AnyCssRule, CssRoot, CssSelectorList, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct SemanticModel {
    pub(crate) data: Rc<SemanticModelData>,
}

impl SemanticModel {
    pub(crate) fn new(data: SemanticModelData) -> Self {
        Self {
            data: Rc::new(data),
        }
    }
}

#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: CssRoot,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    pub(crate) selectors: Vec<CssSelectorList>,
    pub(crate) rules: Vec<AnyCssRule>,
}

impl SemanticModelData {
    pub(crate) fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            selectors: Vec::new(),
            rules: Vec::new(),
        }
    }
}

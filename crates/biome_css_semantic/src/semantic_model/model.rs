use std::rc::Rc;

use biome_css_syntax::{CssRoot, CssSyntaxNode};
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
    pub(crate) selectors: FxHashMap<TextRange, Vec<(String, TextRange)>>,
    pub(crate) properties: FxHashMap<TextRange, Vec<(String, TextRange)>>,
}

impl SemanticModelData {
    pub(crate) fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            selectors: FxHashMap::default(),
            properties: FxHashMap::default(),
        }
    }
}

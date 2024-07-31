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

    pub fn root(&self) -> &CssRoot {
        &self.data.root
    }

    pub fn node_by_range(&self, range: TextRange) -> Option<&CssSyntaxNode> {
        self.data.node_by_range.get(&range)
    }

    pub fn selectors(&self, range: TextRange) -> Option<&Vec<(String, TextRange)>> {
        self.data.selectors.get(&range)
    }

    pub fn declarations(&self, range: TextRange) -> Option<&Vec<Declaration>> {
        self.data.declarations.get(&range)
    }
}

#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: CssRoot,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    pub(crate) selectors: FxHashMap<TextRange, Vec<(String, TextRange)>>,
    pub(crate) declarations: FxHashMap<TextRange, Vec<Declaration>>,
}

#[derive(Debug)]
pub struct Declaration {
    pub property: String,
    pub value: String,
    pub property_range: TextRange,
    pub value_range: TextRange,
}

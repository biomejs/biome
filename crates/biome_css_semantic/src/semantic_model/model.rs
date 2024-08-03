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

    pub fn rules(&self) -> &[Rule] {
        &self.data.rules
    }
}

#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: CssRoot,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
    pub children: Vec<Rule>,
    pub range: TextRange,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: String,
    pub property_range: TextRange,
    pub value_range: TextRange,
}

#[derive(Debug, Clone)]
pub struct Selector {
    pub name: String,
    pub range: TextRange,
    pub specificity: Specificity,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity(pub u32, pub u32, pub u32);

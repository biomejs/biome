use std::rc::Rc;

use biome_css_syntax::{CssRoot, CssSyntaxNode};
use biome_rowan::{TextRange, TextSize, TokenText};
use rust_lapper::Lapper;
use rustc_hash::FxHashMap;

pub struct CssSemanticModel {
    pub(crate) data: Rc<CssSemanticModelData>,
}

pub struct CssSemanticModelData {
    pub(crate) root: CssRoot,
    // All scopes of this model
    pub(crate) scopes: Vec<CssSemanticModelScopeData>,
    pub(crate) scope_by_range: Lapper<usize, usize>,
    // Maps the start of a node range to a scope id
    pub(crate) scope_hoisted_to_by_range: FxHashMap<TextSize, usize>,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    // Maps any range start in the code to its bindings (usize points to bindings vec)
    pub(crate) declared_at_by_start: FxHashMap<TextSize, usize>,
    // List of all the declarations
    pub(crate) bindings: Vec<SemanticModelBindingData>,
    // Index bindings by range start
    pub(crate) bindings_by_start: FxHashMap<TextSize, usize>,
    /// All references that could not be resolved
    pub(crate) unresolved_references: Vec<SemanticModelUnresolvedReference>,
}

#[derive(Debug)]
pub(crate) struct CssSemanticModelScopeData {
    // The scope range
    pub(crate) range: TextRange,
    // The parent scope of this scope
    pub(crate) parent: Option<usize>,
    // All children scope of this scope
    pub(crate) children: Vec<usize>,
    pub(crate) selectors: Vec<CssSyntaxNode>,
    // All bindings of this scope (points to CssSemanticModelData::bindings)
    pub(crate) bindings: Vec<usize>,
    // Map pointing to the [bindings] vec of each bindings by its name
    pub(crate) bindings_by_name: FxHashMap<TokenText, usize>,
}

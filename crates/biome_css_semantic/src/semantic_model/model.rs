use std::rc::Rc;

use biome_css_syntax::{CssRoot, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

/// The façade for all semantic information of a CSS document.
///
/// This struct provides access to the root, rules, and individual nodes of the CSS document.
/// It holds a reference-counted pointer to the internal `SemanticModelData`.
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

    /// Retrieves a node by its text range.
    pub fn node_by_range(&self, range: TextRange) -> Option<&CssSyntaxNode> {
        self.data.node_by_range.get(&range)
    }

    /// Returns a slice of all rules in the CSS document.
    pub fn rules(&self) -> &[Rule] {
        &self.data.rules
    }
}

/// Contains the internal data of a `SemanticModel`.
///
/// This struct holds the root of the CSS document, a mapping of nodes by their range,
/// and a list of all rules in the document.
#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: CssRoot,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    // List of all the rules
    pub(crate) rules: Vec<Rule>,
}

/// Represents a CSS rule, including its selectors, declarations, and nested rules.
#[derive(Debug)]
pub struct Rule {
    /// The selectors associated with this rule.
    pub selectors: Vec<Selector>,
    /// The declarations within this rule.
    pub declarations: Vec<Declaration>,
    /// Any nested rules within this rule.
    pub children: Vec<Rule>,
    /// The text range of this rule in the source document.
    pub range: TextRange,
}

/// Represents a CSS declaration (property-value pair).
#[derive(Debug, Clone)]
pub struct Declaration {
    /// The property name.
    pub property: String,
    /// The property value.
    pub value: String,
    /// The text range of the property in the source document.
    pub property_range: TextRange,
    /// The text range of the value in the source document.
    pub value_range: TextRange,
}

/// Represents a CSS selector.
#[derive(Debug, Clone)]
pub struct Selector {
    /// The name of the selector.
    pub name: String,
    /// The text range of the selector in the source document.
    pub range: TextRange,
    /// The specificity of the selector.
    pub specificity: Specificity,
}

/// Represents the specificity of a CSS selector.
///
/// This specificity is represented as a tuple of three `u32` values,
/// corresponding to (ID selectors, class selectors, type selectors).
/// More details https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Specificity(pub u32, pub u32, pub u32);

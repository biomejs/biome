use biome_css_syntax::{CssRoot, CssSyntaxKind, CssSyntaxNode};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;

use crate::events::SemanticEvent;

use super::model::{Declaration, SemanticModel, SemanticModelData};

/// Builds the [SemanticModel] consuming [SemanticEvent] and [GraphqlSyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvent] and build all the
/// data necessary to build a semantic model, that is allocated with an
/// [std::rc::Rc] and stored inside the [SemanticModel].
pub struct SemanticModelBuilder {
    root: CssRoot,
    node_by_range: FxHashMap<TextRange, CssSyntaxNode>,
    selectors: FxHashMap<TextRange, Vec<(String, TextRange)>>,
    declarations: FxHashMap<TextRange, Vec<Declaration>>,
}

impl SemanticModelBuilder {
    pub fn new(root: CssRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            selectors: FxHashMap::default(),
            declarations: FxHashMap::default(),
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            node_by_range: self.node_by_range,
            selectors: self.selectors,
            declarations: self.declarations,
        };
        SemanticModel::new(data)
    }

    #[inline]
    pub fn push_node(&mut self, node: &CssSyntaxNode) {
        use CssSyntaxKind::*;
        if matches!(
            node.kind(),
            CSS_SELECTOR_LIST | CSS_DECLARATION | CSS_DECLARATION_OR_RULE_LIST
        ) {
            self.node_by_range.insert(node.text_range(), node.clone());
        }
    }

    #[inline]
    pub fn push_event(&mut self, event: SemanticEvent) {
        match event {
            SemanticEvent::SelectorDeclaration {
                range,
                name,
                selector_range,
            } => {
                self.selectors
                    .entry(range)
                    .or_default()
                    .push((name, selector_range));
            }
            SemanticEvent::PropertyDeclaration {
                ruleset_range,
                property,
                property_range,
                value,
                value_range,
            } => {
                self.declarations
                    .entry(ruleset_range)
                    .or_default()
                    .push(Declaration {
                        property,
                        value,
                        property_range,
                        value_range,
                    });
            }
        }
    }
}

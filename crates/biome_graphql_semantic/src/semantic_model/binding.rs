use std::rc::Rc;

use biome_graphql_syntax::{
    GraphqlDirective, GraphqlDirectiveDefinition, GraphqlFragmentDefinition, GraphqlFragmentSpread,
    GraphqlNameBinding, GraphqlNameReference, GraphqlSyntaxNode,
};
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};

use crate::SemanticModel;

use super::{
    model::{SemanticIndex, SemanticModelData},
    reference::Reference,
};

/// Internal type with all the semantic data of a specific binding
#[derive(Debug)]
pub(crate) struct SemanticModelBinding {
    pub index: SemanticIndex,
    pub range: TextRange,
}

/// Provides access to all semantic data of a specific binding.
pub struct Binding {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) index: SemanticIndex,
}

impl std::fmt::Debug for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding").field("id", &self.index).finish()
    }
}

impl Binding {
    /// Returns the syntax node associated with this binding.
    pub fn syntax(&self) -> &GraphqlSyntaxNode {
        let binding = &self.data.bindings[self.index.0];
        &self.data.node_by_range[&binding.range]
    }

    /// Returns the typed AST node associated with this binding.
    pub fn tree(&self) -> GraphqlNameBinding {
        let node = self.syntax();
        let binding = GraphqlNameBinding::cast_ref(node);
        debug_assert!(binding.is_some());
        binding.unwrap()
    }

    /// Returns an iterator to all references of this binding.
    pub fn all_references(&self) -> Vec<Reference> {
        self.data.bindings_to_references[self.index.0]
            .iter()
            .map(|&x| Reference {
                data: self.data.clone(),
                index: x.into(),
            })
            .collect::<Vec<_>>()
    }
}

pub trait ReferenceExtensions {
    fn all_references(&self, model: &SemanticModel) -> Vec<Reference>;
}

impl ReferenceExtensions for GraphqlNameBinding {
    fn all_references(&self, model: &SemanticModel) -> Vec<Reference> {
        model.as_binding(self).all_references()
    }
}

pub trait IsBindingAstNode {
    type ReferenceAstNode;
    fn all_reference_nodes(&self, model: &SemanticModel) -> Vec<Self::ReferenceAstNode>;
}

impl IsBindingAstNode for GraphqlNameBinding {
    type ReferenceAstNode = GraphqlNameReference;
    fn all_reference_nodes(&self, model: &SemanticModel) -> Vec<Self::ReferenceAstNode> {
        self.all_references(model)
            .iter()
            .map(|r| r.tree())
            .collect()
    }
}

impl IsBindingAstNode for GraphqlDirectiveDefinition {
    type ReferenceAstNode = GraphqlDirective;
    fn all_reference_nodes(&self, model: &SemanticModel) -> Vec<Self::ReferenceAstNode> {
        let Ok(name) = self.name() else {
            return vec![];
        };
        name.all_reference_nodes(model)
            .into_iter()
            .filter_map(|r| r.syntax().parent()?.cast())
            .collect()
    }
}

impl IsBindingAstNode for GraphqlFragmentDefinition {
    type ReferenceAstNode = GraphqlFragmentSpread;
    fn all_reference_nodes(&self, model: &SemanticModel) -> Vec<Self::ReferenceAstNode> {
        let Ok(name) = self.name() else {
            return vec![];
        };
        name.all_reference_nodes(model)
            .into_iter()
            .filter_map(|r| r.syntax().parent()?.cast())
            .collect()
    }
}

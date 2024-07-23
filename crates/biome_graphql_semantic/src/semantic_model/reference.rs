use std::rc::Rc;

use biome_graphql_syntax::{
    AnyGraphqlTypeDefinition, GraphqlDirective, GraphqlDirectiveDefinition,
    GraphqlEnumTypeDefinition, GraphqlEnumTypeExtension, GraphqlFragmentDefinition,
    GraphqlFragmentSpread, GraphqlInterfaceTypeDefinition, GraphqlInterfaceTypeExtension,
    GraphqlNameBinding, GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
    GraphqlOperationDefinition, GraphqlScalarTypeDefinition, GraphqlScalarTypeExtension,
    GraphqlTypeCondition, GraphqlUnionTypeDefinition, GraphqlUnionTypeExtension,
    GraphqlVariableBinding, GraphqlVariableReference,
};
use biome_graphql_syntax::{GraphqlNameReference, GraphqlSyntaxNode};
use biome_rowan::TextRange;
use biome_rowan::{AstNode, SyntaxNodeCast};

use crate::{SemanticIndex, SemanticModel};

use super::{binding::Binding, model::SemanticModelData};

/// Internal type with all the semantic data of a specific reference
#[derive(Debug)]
pub(crate) struct SemanticModelReference {
    pub(crate) index: SemanticIndex,
    pub(crate) range: TextRange,
}

/// Provides all information regarding to a specific reference.
#[derive(Debug)]
pub struct Reference {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) index: SemanticIndex,
}

impl Reference {
    /// Returns the range of this reference
    pub fn range(&self) -> &TextRange {
        let reference = &self.data.references[self.index.0];
        &reference.range
    }

    /// Returns the node of this reference
    pub fn syntax(&self) -> &GraphqlSyntaxNode {
        &self.data.node_by_range[self.range()]
    }

    pub fn tree(&self) -> GraphqlNameReference {
        let node = self.syntax();

        let reference = GraphqlNameReference::cast_ref(node);
        debug_assert!(reference.is_some());
        reference.unwrap()
    }

    /// Returns the binding of this reference
    pub fn all_bindings(&self) -> Vec<Binding> {
        self.data.references_to_bindings[self.index.0]
            .iter()
            .map(|&x| Binding {
                data: self.data.clone(),
                index: x.into(),
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct SemanticModelUnresolvedReference {
    pub(crate) range: TextRange,
}

#[derive(Debug)]
pub struct UnresolvedReference {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) id: usize,
}

impl UnresolvedReference {
    pub fn syntax(&self) -> &GraphqlSyntaxNode {
        let reference = &self.data.unresolved_references[self.id];
        &self.data.node_by_range[&reference.range]
    }

    pub fn tree(&self) -> GraphqlNameReference {
        GraphqlNameReference::unwrap_cast(self.syntax().clone())
    }

    pub fn range(&self) -> &TextRange {
        let reference = &self.data.unresolved_references[self.id];
        &reference.range
    }
}

#[derive(Debug)]
pub struct SemanticModelUnresolvedVariableReference {
    pub(crate) range: TextRange,
    pub(crate) referenced_operation: Option<TextRange>,
}

#[derive(Debug)]
pub struct UnresolvedVariableReference {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) id: usize,
    pub(crate) referenced_operation: Option<TextRange>,
}

impl UnresolvedVariableReference {
    pub fn syntax(&self) -> &GraphqlSyntaxNode {
        let reference = &self.data.unresolved_variable_references[self.id];
        &self.data.node_by_range[&reference.range]
    }

    pub fn tree(&self) -> GraphqlVariableReference {
        GraphqlVariableReference::unwrap_cast(self.syntax().clone())
    }

    pub fn range(&self) -> &TextRange {
        let reference = &self.data.unresolved_variable_references[self.id];
        &reference.range
    }

    pub fn referenced_operation(&self) -> Option<GraphqlOperationDefinition> {
        self.referenced_operation
            .as_ref()
            .and_then(|range| self.data.node_by_range[range].clone().cast())
    }
}

pub trait BindingExtensions {
    fn binding(&self, model: &SemanticModel) -> Option<Binding>;
}

impl BindingExtensions for GraphqlNameReference {
    fn binding(&self, model: &SemanticModel) -> Option<Binding> {
        model.binding(self)
    }
}

pub trait HasDeclarationAstNode {
    type DeclarationAstNode;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode>;
}

impl HasDeclarationAstNode for GraphqlNameReference {
    type DeclarationAstNode = GraphqlNameBinding;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let binding = model.binding(self)?;
        let name_binding = binding.syntax().clone().cast()?;
        Some(name_binding)
    }
}

impl HasDeclarationAstNode for GraphqlFragmentSpread {
    type DeclarationAstNode = GraphqlFragmentDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let fragment_definition = name_binding.syntax().parent()?.cast()?;
        Some(fragment_definition)
    }
}

impl HasDeclarationAstNode for GraphqlDirective {
    type DeclarationAstNode = GraphqlDirectiveDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let directive_definition = name_binding.syntax().parent()?.cast()?;
        Some(directive_definition)
    }
}

impl HasDeclarationAstNode for GraphqlTypeCondition {
    type DeclarationAstNode = AnyGraphqlTypeDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let type_name = self.ty().ok()?;
        let name_binding = type_name.binding_node(model)?;
        let definition = name_binding.syntax().parent()?.cast()?;
        Some(definition)
    }
}

impl HasDeclarationAstNode for GraphqlScalarTypeExtension {
    type DeclarationAstNode = GraphqlScalarTypeDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let definition = name_binding.syntax().parent()?.cast()?;
        Some(definition)
    }
}

impl HasDeclarationAstNode for GraphqlObjectTypeExtension {
    type DeclarationAstNode = GraphqlObjectTypeDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let definition = name_binding.syntax().parent()?.cast()?;
        Some(definition)
    }
}

impl HasDeclarationAstNode for GraphqlInterfaceTypeExtension {
    type DeclarationAstNode = GraphqlInterfaceTypeDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let definition = name_binding.syntax().parent()?.cast()?;
        Some(definition)
    }
}

impl HasDeclarationAstNode for GraphqlUnionTypeExtension {
    type DeclarationAstNode = GraphqlUnionTypeDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let definition = name_binding.syntax().parent()?.cast()?;
        Some(definition)
    }
}

impl HasDeclarationAstNode for GraphqlEnumTypeExtension {
    type DeclarationAstNode = GraphqlEnumTypeDefinition;
    fn binding_node(&self, model: &SemanticModel) -> Option<Self::DeclarationAstNode> {
        let name = self.name().ok()?;
        let name_binding = name.binding_node(model)?;
        let definition = name_binding.syntax().parent()?.cast()?;
        Some(definition)
    }
}

pub trait HasDeclarationAstNodes {
    type DeclarationAstNode;
    fn binding_nodes(&self, model: &SemanticModel) -> Vec<Self::DeclarationAstNode>;
}

impl HasDeclarationAstNodes for GraphqlVariableReference {
    type DeclarationAstNode = GraphqlVariableBinding;
    fn binding_nodes(&self, model: &SemanticModel) -> Vec<Self::DeclarationAstNode> {
        model
            .bindings(self)
            .iter()
            .filter_map(|binding| binding.syntax().clone().cast())
            .collect()
    }
}

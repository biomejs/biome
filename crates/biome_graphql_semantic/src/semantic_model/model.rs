use std::rc::Rc;

use biome_graphql_syntax::{
    GraphqlNameBinding, GraphqlNameReference, GraphqlRoot, GraphqlSyntaxNode,
    GraphqlVariableReference,
};
use biome_rowan::{AstNode, TextRange, TextSize};
use rustc_hash::FxHashMap;

use crate::{
    semantic_model::reference::UnresolvedReference, Reference, SemanticModelReference,
    SemanticModelUnresolvedVariableReference, UnresolvedVariableReference,
};

use super::{
    binding::{Binding, SemanticModelBinding},
    reference::SemanticModelUnresolvedReference,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct SemanticIndex(pub(crate) usize);

impl From<usize> for SemanticIndex {
    fn from(v: usize) -> Self {
        SemanticIndex(v)
    }
}

/// Contains all the data of the [SemanticModel] and only lives behind an [Rc].
///
/// That allows any returned struct (like [Scope], [Binding])
/// to outlive the [SemanticModel], and to not include lifetimes.
#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: GraphqlRoot,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, GraphqlSyntaxNode>,
    // List of all the declarations
    pub(crate) bindings: Vec<SemanticModelBinding>,
    // Index bindings by range start
    pub(crate) bindings_by_start: FxHashMap<TextSize, usize>,
    // Map from each binding to its references
    pub(crate) bindings_to_references: Vec<Vec<usize>>,
    // List of all the references
    pub(crate) references: Vec<SemanticModelReference>,
    // Map from each reference to its binding
    pub(crate) references_to_bindings: Vec<Vec<usize>>,
    // Index references by range start
    pub(crate) references_by_start: FxHashMap<TextSize, usize>,
    /// All references that could not be resolved
    pub(crate) unresolved_references: Vec<SemanticModelUnresolvedReference>,
    /// All variable references that could not be resolved
    pub(crate) unresolved_variable_references: Vec<SemanticModelUnresolvedVariableReference>,
}

impl PartialEq for SemanticModelData {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for SemanticModelData {}

/// The façade for all semantic information.
/// - Bindings
/// - References
///
/// See `SemanticModelData` for more information about the internals.
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

    pub fn all_bindings(&self) -> impl Iterator<Item = Binding> + '_ {
        self.data.bindings.iter().map(|x| Binding {
            data: self.data.clone(),
            index: x.index,
        })
    }

    pub fn all_references(&self) -> impl Iterator<Item = Reference> + '_ {
        self.data.references.iter().map(|x| Reference {
            data: self.data.clone(),
            index: x.index,
        })
    }

    /// Returns the [Binding] of a reference.
    /// Can also be called from "binding" extension method.
    ///
    /// ```rust
    /// use biome_graphql_parser::parse_graphql;
    /// use biome_rowan::{AstNode, SyntaxNodeCast};
    /// use biome_graphql_syntax::GraphqlNameReference;
    /// use biome_graphql_semantic::{semantic_model, BindingExtensions};
    ///
    /// let r = parse_graphql("{ ...fragment }");
    /// let model = semantic_model(&r.tree());
    ///
    /// let fragment_reference = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(|x| x.cast::<GraphqlNameReference>())
    ///     .find(|x| x.to_trimmed_string() == "fragment")
    ///     .unwrap();
    ///
    /// let fragment_binding = model.binding(&fragment_reference);
    /// // or
    /// let fragment_binding = fragment_reference.binding(&model);
    /// ```
    pub fn binding(&self, reference: &GraphqlNameReference) -> Option<Binding> {
        let range = reference.syntax().text_range_with_trivia();
        let reference_id = self.data.references_by_start.get(&range.start())?;
        debug_assert!(self.data.references_to_bindings[*reference_id].len() <= 1);
        self.data.references_to_bindings[*reference_id]
            .iter()
            .map(|&x| Binding {
                data: self.data.clone(),
                index: x.into(),
            })
            .next()
    }

    /// Returns the [Binding] of a variable reference.
    /// Since a variable can be referenced in a fragment, which in turn can be referenced
    /// by multiple operations, and then defined in those operations, this method returns
    /// a list of bindings.
    ///
    /// ```rust
    /// use biome_graphql_parser::parse_graphql;
    /// use biome_rowan::{AstNode, SyntaxNodeCast};
    /// use biome_graphql_syntax::GraphqlVariableReference;
    /// use biome_graphql_semantic::semantic_model;
    ///
    /// let r = parse_graphql("{ field(arg: $var) }");
    /// let model = semantic_model(&r.tree());
    ///
    /// let fragment_reference = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(|x| x.cast::<GraphqlVariableReference>())
    ///     .find(|x| x.to_trimmed_string() == "$var")
    ///     .unwrap();
    ///
    /// let fragment_bindings = model.bindings(&fragment_reference);
    /// ```
    pub fn bindings(&self, reference: &GraphqlVariableReference) -> Vec<Binding> {
        let range = reference.syntax().text_range_with_trivia();
        let Some(reference_id) = self.data.references_by_start.get(&range.start()) else {
            return Vec::new();
        };
        self.data.references_to_bindings[*reference_id]
            .iter()
            .map(|&x| Binding {
                data: self.data.clone(),
                index: x.into(),
            })
            .collect::<Vec<_>>()
    }

    /// Returns an iterator of all the unresolved references in the program
    pub fn all_unresolved_references(&self) -> impl Iterator<Item = UnresolvedReference> + '_ {
        (0..self.data.unresolved_references.len()).map(move |id| UnresolvedReference {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns an iterator of all the unresolved variable references in the program
    pub fn all_unresolved_variable_references(
        &self,
    ) -> impl Iterator<Item = UnresolvedVariableReference> + '_ {
        self.data
            .unresolved_variable_references
            .iter()
            .enumerate()
            .map(move |(id, reference)| UnresolvedVariableReference {
                data: self.data.clone(),
                referenced_operation: reference.referenced_operation,
                id,
            })
    }

    pub fn as_binding(&self, binding: &GraphqlNameBinding) -> Binding {
        let range = binding.syntax().text_range_with_trivia();
        let id = self.data.bindings_by_start[&range.start()];
        Binding {
            data: self.data.clone(),
            index: id.into(),
        }
    }
}

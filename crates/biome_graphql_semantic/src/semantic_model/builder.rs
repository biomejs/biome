use biome_graphql_syntax::{GraphqlRoot, GraphqlSyntaxKind, GraphqlSyntaxNode};
use biome_rowan::{TextRange, TextSize};
use rustc_hash::FxHashMap;

use crate::{SemanticEvent, SemanticModelReference, SemanticModelUnresolvedVariableReference};

use super::{
    binding::SemanticModelBinding,
    model::{SemanticModel, SemanticModelData},
    reference::SemanticModelUnresolvedReference,
};

/// Builds the [SemanticModel] consuming [SemanticEvent] and [GraphqlSyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvent] and build all the
/// data necessary to build a semantic model, that is allocated with an
/// [std::rc::Rc] and stored inside the [SemanticModel].
pub struct SemanticModelBuilder {
    root: GraphqlRoot,
    node_by_range: FxHashMap<TextRange, GraphqlSyntaxNode>,
    bindings: Vec<SemanticModelBinding>,
    /// maps a binding range start to its index inside SemanticModelBuilder::bindings vec
    bindings_by_start: FxHashMap<TextSize, usize>,
    // Map from each binding to its references
    bindings_to_references: Vec<Vec<usize>>,
    // List of all the references
    references: Vec<SemanticModelReference>,
    /// maps a reference range start to its index inside SemanticModelBuilder::references vec
    references_by_start: FxHashMap<TextSize, usize>,
    // Map from each reference to its binding
    references_to_bindings: Vec<Vec<usize>>,
    unresolved_references: Vec<SemanticModelUnresolvedReference>,
    unresolved_variable_references: Vec<SemanticModelUnresolvedVariableReference>,
}

impl SemanticModelBuilder {
    pub fn new(root: GraphqlRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            bindings: Vec::new(),
            bindings_by_start: FxHashMap::default(),
            bindings_to_references: Vec::new(),
            references: Vec::new(),
            references_by_start: FxHashMap::default(),
            references_to_bindings: Vec::new(),
            unresolved_references: Vec::new(),
            unresolved_variable_references: Vec::new(),
        }
    }

    #[inline]
    pub fn push_node(&mut self, node: &GraphqlSyntaxNode) {
        use GraphqlSyntaxKind::*;
        if matches!(
            node.kind(),
            GRAPHQL_NAME_BINDING
                | GRAPHQL_NAME_REFERENCE
                | GRAPHQL_VARIABLE_BINDING
                | GRAPHQL_VARIABLE_REFERENCE
                | GRAPHQL_OPERATION_DEFINITION
        ) {
            self.node_by_range
                .insert(node.text_range_with_trivia(), node.clone());
        }
    }

    #[inline]
    pub fn push_event(&mut self, e: SemanticEvent) {
        use std::collections::hash_map::Entry;
        use SemanticEvent::*;
        match e {
            Declaration { range } => {
                let binding_id = self.bindings.len();
                self.bindings.push(SemanticModelBinding {
                    index: binding_id.into(),
                    range,
                });
                self.bindings_by_start.insert(range.start(), binding_id);
                self.bindings_to_references.push(Vec::new());
            }
            Reference {
                range,
                declared_at: declaration_at,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at.start()];
                let reference_id =
                    if let Entry::Vacant(e) = self.references_by_start.entry(range.start()) {
                        let reference_id = self.references.len();

                        self.references.push(SemanticModelReference {
                            index: reference_id.into(),
                            range,
                        });

                        e.insert(reference_id);
                        self.references_to_bindings.push(Vec::new());
                        reference_id
                    } else {
                        self.references_by_start[&range.start()]
                    };
                self.references_to_bindings[reference_id].push(binding_id);
                self.bindings_to_references[binding_id].push(reference_id);
            }

            UnresolvedReference { range } => {
                let node = &self.node_by_range[&range];
                let name = node.text_trimmed().to_string();
                if !Self::is_builtin_type(&name) && !Self::is_builtin_directive(&name) {
                    self.unresolved_references
                        .push(SemanticModelUnresolvedReference { range })
                }
            }
            UnresolvedVariableReference {
                range,
                referenced_operation,
            } => {
                self.unresolved_variable_references
                    .push(SemanticModelUnresolvedVariableReference {
                        range,
                        referenced_operation,
                    })
            }
        }
    }

    #[inline]
    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            node_by_range: self.node_by_range,
            bindings: self.bindings,
            bindings_by_start: self.bindings_by_start,
            bindings_to_references: self.bindings_to_references,
            references: self.references,
            references_by_start: self.references_by_start,
            references_to_bindings: self.references_to_bindings,
            unresolved_references: self.unresolved_references,
            unresolved_variable_references: self.unresolved_variable_references,
        };
        SemanticModel::new(data)
    }

    fn is_builtin_type(name: &str) -> bool {
        matches!(name, "String" | "Int" | "Float" | "Boolean" | "ID")
    }

    fn is_builtin_directive(name: &str) -> bool {
        matches!(name, "skip" | "include" | "deprecated" | "specifiedBy")
    }
}

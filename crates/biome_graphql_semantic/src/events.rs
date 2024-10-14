//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use biome_graphql_syntax::{
    AnyGraphqlTypeDefinition, AnyGraphqlTypeExtension, GraphqlFragmentDefinition,
    GraphqlNameBinding, GraphqlNameReference, GraphqlOperationDefinition, GraphqlSyntaxKind,
    GraphqlSyntaxNode, GraphqlVariableBinding, GraphqlVariableReference, TextRange,
};
use biome_rowan::{AstNode, TokenText};
use rustc_hash::FxHashMap;
use std::collections::{HashSet, VecDeque};
use GraphqlSyntaxKind::*;

/// Events emitted by the [SemanticEventExtractor].
/// These events are later made into the Semantic Model.
#[derive(Debug, Eq, PartialEq)]
pub enum SemanticEvent {
    /// Tracks where a new symbol declaration is found.
    /// Generated for:
    /// - Variable Declarations
    /// - Type Definitions
    /// - Directive Definitions
    /// - Operation Definitions
    Declaration { range: TextRange },

    /// Tracks where a symbol is referenced regardless of its declaration position.
    /// Generated for:
    /// - All reference identifiers
    Reference {
        range: TextRange,
        declared_at: TextRange,
    },

    /// Tracks references that do no have any matching binding
    /// Generated for:
    /// - Unmatched reference identifiers
    UnresolvedReference { range: TextRange },

    /// Tracks variable references that do no have any matching binding
    /// Generated for:
    /// - Unmatched variable reference identifiers
    UnresolvedVariableReference {
        range: TextRange,
        referenced_operation: Option<TextRange>,
    },
}

impl SemanticEvent {
    pub fn range(&self) -> TextRange {
        match self {
            Self::Declaration { range, .. }
            | Self::Reference { range, .. }
            | Self::UnresolvedReference { range, .. }
            | Self::UnresolvedVariableReference { range, .. } => *range,
        }
    }
}

/// Extracts [SemanticEvent] from [GraphqlSyntaxNode].
///
/// The extraction is not entirely pull based, nor entirely push based.
/// This happens because some nodes can generate multiple events.
///
/// For a simpler way to extract [SemanticEvent] see [semantic_events].
///
/// To use the [SemanticEventExtractor] one must push the current node, following
/// the pre-order of the tree, and must pull events until `pop` returns [None].
///
/// ```rust
/// use biome_graphql_parser::*;
/// use biome_graphql_syntax::*;
/// use biome_graphql_semantic::*;
/// let tree = parse_graphql("query { hero }");
/// let mut extractor = SemanticEventExtractor::new();
/// for e in tree.syntax().preorder() {
///     match e {
///         WalkEvent::Enter(node) => extractor.enter(&node),
///         WalkEvent::Leave(node) => extractor.leave(&node),
///         _ => {}
///     }
///
///     while let Some(e) = extractor.pop() {
///         dbg!(e);
///     }
/// }
/// ```
#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    /// Event queue
    stash: VecDeque<SemanticEvent>,
    /// Every available bindings and their range
    bindings: FxHashMap<BindingName, BindingInfo>,

    scopes: Vec<Scope>,
    /// Every available bindings and their range
    references: FxHashMap<BindingName, Vec<ReferenceInfo>>,

    current_scope: Option<Scope>,
}

/// A scope created by an operation or a fragment.
#[derive(Debug)]
struct Scope {
    scope_id: usize,
    range: TextRange,
    /// Every references in this scope, used to track referenced fragments
    references: FxHashMap<BindingName, Vec<ReferenceInfo>>,
    /// Every implicit variables references in this scope, which might be referenced directly in
    /// this scope, or by any referenced fragments
    implicit_variables_references: FxHashMap<TokenText, HashSet<VariableReferenceInfo>>,
    /// Operation definition has variables definitions
    variables_definitions: Option<FxHashMap<TokenText, VariableBindingInfo>>,
}

/// A binding name is either a type or a value.
///
/// A value refer can refer to a directive or an operation.
/// A type can refer to a type definition like a scalar or an object.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum BindingName {
    Type(TokenText),
    Value(TokenText),
}

#[derive(Debug, Clone)]
struct BindingInfo {
    /// range of the name
    range: TextRange,
    /// If this is a variable binding, it will be defined in an operation scope
    scope_id: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ReferenceInfo {
    /// range of the name
    range: TextRange,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct VariableReferenceInfo {
    range: TextRange,
    name: TokenText,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct VariableBindingInfo {
    range: TextRange,
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self::default()
    }

    /// See [SemanticEvent] for a more detailed description of which events [GraphqlSyntaxNode] generates.
    #[inline]
    pub fn enter(&mut self, node: &GraphqlSyntaxNode) {
        match node.kind() {
            GRAPHQL_NAME_BINDING => {
                self.enter_identifier_binding(&GraphqlNameBinding::unwrap_cast(node.clone()));
            }

            GRAPHQL_NAME_REFERENCE => {
                self.enter_identifier_usage(&GraphqlNameReference::unwrap_cast(node.clone()));
            }

            GRAPHQL_VARIABLE_BINDING => {
                self.enter_variable_binding(&GraphqlVariableBinding::unwrap_cast(node.clone()));
            }

            GRAPHQL_VARIABLE_REFERENCE => {
                self.enter_variable_usage(&GraphqlVariableReference::unwrap_cast(node.clone()));
            }

            GRAPHQL_OPERATION_DEFINITION => {
                self.push_operation_scope(&GraphqlOperationDefinition::unwrap_cast(node.clone()));
            }

            GRAPHQL_FRAGMENT_DEFINITION => {
                self.push_fragment_scope(&GraphqlFragmentDefinition::unwrap_cast(node.clone()));
            }

            _ => {}
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
    pub fn leave(&mut self, node: &GraphqlSyntaxNode) {
        match node.kind() {
            GRAPHQL_ROOT => {
                self.resolve_references();

                self.resolve_variables_references();
            }
            GRAPHQL_OPERATION_DEFINITION | GRAPHQL_FRAGMENT_DEFINITION => {
                self.leave_scope();
            }
            _ => {}
        }
    }

    /// Return any previous extracted [SemanticEvent].
    #[inline]
    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }

    fn enter_identifier_binding(&mut self, node: &GraphqlNameBinding) {
        let Ok(name_token) = node.value_token() else {
            return;
        };
        let name = name_token.token_text_trimmed();

        let range = node.syntax().text_range_with_trivia();
        let Some(parent) = node.syntax().parent() else {
            // every node aside from the root should have a parent, so this should never happen
            return;
        };
        self.stash.push_back(SemanticEvent::Declaration { range });
        if AnyGraphqlTypeDefinition::can_cast(parent.kind()) {
            self.push_binding(
                BindingName::Type(name),
                BindingInfo {
                    range,
                    scope_id: self.get_current_scope_id(),
                },
            );
        } else {
            self.push_binding(
                BindingName::Value(name),
                BindingInfo {
                    range,
                    scope_id: self.get_current_scope_id(),
                },
            );
        }
    }

    fn enter_identifier_usage(&mut self, node: &GraphqlNameReference) {
        let Ok(name_token) = node.value_token() else {
            return;
        };
        let name = name_token.token_text_trimmed();
        let range = node.syntax().text_range_with_trivia();
        let Some(parent) = node.syntax().parent() else {
            // every node aside from the root should have a parent, so this should never happen
            return;
        };
        let binding_info = ReferenceInfo { range };
        let binding_name = match parent.kind() {
            GRAPHQL_FIELD_DEFINITION
            | GRAPHQL_IMPLEMENTS_INTERFACE_LIST
            | GRAPHQL_INPUT_VALUE_DEFINITION
            | GRAPHQL_LIST_TYPE
            | GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION
            | GRAPHQL_TYPE_CONDITION
            | GRAPHQL_UNION_MEMBER_TYPE_LIST
            | GRAPHQL_VARIABLE_DEFINITION => BindingName::Type(name),
            _ if AnyGraphqlTypeExtension::can_cast(parent.kind()) => BindingName::Type(name),
            _ => BindingName::Value(name),
        };
        self.push_reference(binding_name.clone(), binding_info.clone());
        if let Some(scope) = &mut self.current_scope {
            scope
                .references
                .entry(binding_name)
                .or_default()
                .push(binding_info);
        }
    }

    fn enter_variable_binding(&mut self, node: &GraphqlVariableBinding) {
        let Some(scope) = &mut self.current_scope else {
            return;
        };
        // We should be inside an operation scope
        let Some(variables_definitions) = &mut scope.variables_definitions else {
            return;
        };
        let Ok(name) = node.name() else {
            return;
        };
        let Ok(name_token) = name.value_token() else {
            return;
        };
        let name_token = name_token.token_text_trimmed();
        let range = node.range();
        variables_definitions.insert(name_token, VariableBindingInfo { range });
        self.stash.push_back(SemanticEvent::Declaration { range });
    }

    fn enter_variable_usage(&mut self, node: &GraphqlVariableReference) {
        let Some(scope) = &mut self.current_scope else {
            return;
        };
        let Ok(name_token) = node.name() else {
            return;
        };

        let Ok(name_token) = name_token.value_token() else {
            return;
        };
        let name_token = name_token.token_text_trimmed();
        let range = node.syntax().text_range_with_trivia();
        scope
            .implicit_variables_references
            .entry(name_token.clone())
            .or_default()
            .insert(VariableReferenceInfo {
                range,
                name: name_token,
            });
    }

    fn push_operation_scope(&mut self, node: &GraphqlOperationDefinition) {
        let range = node.syntax().text_range_with_trivia();
        self.current_scope = Some(Scope {
            scope_id: self.scopes.len(),
            range,
            references: Default::default(),
            implicit_variables_references: Default::default(),
            variables_definitions: Some(Default::default()),
        });
    }

    fn push_fragment_scope(&mut self, node: &GraphqlFragmentDefinition) {
        let range = node.syntax().text_range_with_trivia();
        self.current_scope = Some(Scope {
            scope_id: self.scopes.len(),
            range,
            references: Default::default(),
            implicit_variables_references: Default::default(),
            variables_definitions: None,
        });
    }

    fn leave_scope(&mut self) {
        if let Some(scope) = self.current_scope.take() {
            self.scopes.push(scope);
        }
    }

    fn push_binding(&mut self, name: BindingName, info: BindingInfo) {
        self.bindings.insert(name, info);
    }

    fn push_reference(&mut self, name: BindingName, info: ReferenceInfo) {
        self.references.entry(name).or_default().push(info);
    }

    fn resolve_references(&mut self) {
        for (name, references) in self.references.clone() {
            if let Some(&BindingInfo {
                range: declared_at, ..
            }) = self.bindings.get(&name)
            {
                // We know the declaration of these reference.
                for reference in references {
                    let event = SemanticEvent::Reference {
                        range: reference.range,
                        declared_at,
                    };
                    self.stash.push_back(event);
                }
            } else {
                for reference in references {
                    self.stash.push_back(SemanticEvent::UnresolvedReference {
                        range: reference.range,
                    });
                }
            }
        }
    }

    fn resolve_variables_references(&mut self) {
        let mut processed_scopes = HashSet::new();
        for scope_id in 0..self.scopes.len() {
            self.resolve_scope_implicit_variables_references(scope_id, &mut processed_scopes);
        }

        // Track processed variables to avoid emitting duplicate events
        let mut processed_variables = HashSet::new();

        // Bind variables to its declaration in operation definitions
        for scope in &self.scopes {
            // We are only interested in operation scopes
            let Some(variables_definitions) = &scope.variables_definitions else {
                continue;
            };

            for (name, variables) in &scope.implicit_variables_references {
                if let Some(&VariableBindingInfo { range: declared_at }) =
                    variables_definitions.get(name)
                {
                    // Bind those variables to its declaration
                    for variable in variables {
                        self.stash.push_back(SemanticEvent::Reference {
                            range: variable.range,
                            declared_at,
                        });
                        processed_variables.insert(variable.range);
                    }
                } else {
                    for variable in variables {
                        self.stash
                            .push_back(SemanticEvent::UnresolvedVariableReference {
                                range: variable.range,
                                referenced_operation: Some(scope.range),
                            });
                        processed_variables.insert(variable.range);
                    }
                }
            }
        }

        // Resolve remaining undefined variables references in fragments
        for scope in &self.scopes {
            let None = &scope.variables_definitions else {
                continue;
            };

            for references in scope.implicit_variables_references.values() {
                for reference in references {
                    if processed_variables.contains(&reference.range) {
                        continue;
                    }
                    self.stash
                        .push_back(SemanticEvent::UnresolvedVariableReference {
                            range: reference.range,
                            referenced_operation: None,
                        });
                }
            }
        }
    }

    /// Recursively resolve implicit variables references from referenced fragments
    fn resolve_scope_implicit_variables_references(
        &mut self,
        current_scope_id: usize,
        processed_scopes: &mut HashSet<usize>,
    ) {
        // Prevent cycles
        if processed_scopes.contains(&current_scope_id) {
            return;
        }
        processed_scopes.insert(current_scope_id);
        let references = self.scopes[current_scope_id].references.clone();
        for (name, _) in references {
            let Some(&BindingInfo {
                scope_id: Some(binding_scope_id),
                ..
            }) = self.bindings.get(&name)
            else {
                continue;
            };

            // Only interested in fragment scopes
            if self.scopes[binding_scope_id]
                .variables_definitions
                .is_some()
            {
                continue;
            }

            // Bring every implicit variable reference from fragment definition's scope to the current scope
            // Cycle detection is handled by a lint rule
            self.resolve_scope_implicit_variables_references(binding_scope_id, processed_scopes);

            let binding_scope_variables = self.scopes[binding_scope_id]
                .implicit_variables_references
                .clone();

            for (name, variables) in binding_scope_variables {
                self.scopes[current_scope_id]
                    .implicit_variables_references
                    .entry(name)
                    .or_default()
                    .extend(variables);
            }
        }
    }

    fn get_current_scope_id(&self) -> Option<usize> {
        self.current_scope.as_ref().map(|s| s.scope_id)
    }
}

use super::*;
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode, TextRange, TsConditionalType};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry;

/// Builds the [SemanticModel] consuming [SemanticEvent] and [JsSyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvent] and build all the
/// data necessary to build a semantic model, that is allocated with an
/// [std::sync::Arc] and stored inside the [SemanticModel].
pub struct SemanticModelBuilder {
    root: AnyJsRoot,
    /// Binding and reference nodes indexed by their range start
    binding_node_by_start: FxHashMap<TextSize, JsSyntaxNode>,
    scope_node_by_range: FxHashMap<TextRange, JsSyntaxNode>,
    globals: Vec<SemanticModelGlobalBindingData>,
    globals_by_name: FxHashMap<String, Option<u32>>,
    scopes: Vec<SemanticModelScopeData>,
    scope_range_by_start: FxHashMap<TextSize, BTreeSet<Interval<u32, ScopeId>>>,
    scope_hoisted_to_by_range: FxHashMap<TextSize, ScopeId>,
    bindings: Vec<SemanticModelBindingData>,
    /// maps a binding range start to its index inside [SemanticModelBuilder::bindings] vec
    bindings_by_start: FxHashMap<TextSize, BindingId>,
    /// maps a reference range start to its binding index inside [SemanticModelBuilder::bindings] vec
    declared_at_by_start: FxHashMap<TextSize, BindingId>,
    exported: FxHashSet<TextSize>,
    unresolved_references: Vec<SemanticModelUnresolvedReference>,
}

impl SemanticModelBuilder {
    pub fn new(root: AnyJsRoot) -> Self {
        Self {
            root,
            binding_node_by_start: FxHashMap::default(),
            scope_node_by_range: FxHashMap::default(),
            globals: vec![],
            globals_by_name: FxHashMap::default(),
            scopes: vec![],
            scope_range_by_start: FxHashMap::default(),
            scope_hoisted_to_by_range: FxHashMap::default(),
            bindings: vec![],
            bindings_by_start: FxHashMap::default(),
            declared_at_by_start: FxHashMap::default(),
            exported: FxHashSet::default(),
            unresolved_references: Vec::new(),
        }
    }

    #[inline]
    pub fn push_node(&mut self, node: &JsSyntaxNode) {
        use JsSyntaxKind::*;
        match node.kind() {
            // Accessible from bindings and references
            JS_IDENTIFIER_BINDING
            | TS_IDENTIFIER_BINDING
            | JS_REFERENCE_IDENTIFIER
            | JSX_REFERENCE_IDENTIFIER
            | TS_TYPE_PARAMETER_NAME
            | TS_LITERAL_ENUM_MEMBER_NAME
            | JS_IDENTIFIER_ASSIGNMENT => {
                self.binding_node_by_start
                    .insert(node.text_trimmed_range().start(), node.clone());
            }

            // Accessible from scopes, closures
            JS_MODULE
            | JS_SCRIPT
            | TS_DECLARATION_MODULE
            | JS_FUNCTION_DECLARATION
            | JS_FUNCTION_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_METHOD_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_METHOD_OBJECT_MEMBER
            | JS_GETTER_OBJECT_MEMBER
            | JS_SETTER_OBJECT_MEMBER
            | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | JS_CLASS_DECLARATION
            | JS_CLASS_EXPORT_DEFAULT_DECLARATION
            | JS_CLASS_EXPRESSION
            | JS_FUNCTION_BODY
            | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
            | TS_MODULE_DECLARATION
            | TS_EXTERNAL_MODULE_DECLARATION
            | TS_INTERFACE_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION
            | TS_DECLARE_FUNCTION_DECLARATION
            | TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | TS_CALL_SIGNATURE_TYPE_MEMBER
            | TS_METHOD_SIGNATURE_CLASS_MEMBER
            | TS_METHOD_SIGNATURE_TYPE_MEMBER
            | TS_INDEX_SIGNATURE_CLASS_MEMBER
            | TS_INDEX_SIGNATURE_TYPE_MEMBER
            | JS_BLOCK_STATEMENT
            | JS_FOR_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_SWITCH_STATEMENT
            | JS_CATCH_CLAUSE
            | TS_CONSTRUCTOR_TYPE
            | TS_FUNCTION_TYPE
            | TS_MAPPED_TYPE => {
                self.scope_node_by_range
                    .insert(node.text_trimmed_range(), node.clone());
            }
            _ => {
                if let Some(conditional_type) = TsConditionalType::cast_ref(node) {
                    if let Ok(conditional_true_type) = conditional_type.true_type() {
                        let syntax = conditional_true_type.into_syntax();
                        self.scope_node_by_range
                            .insert(syntax.text_trimmed_range(), syntax);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn push_global(&mut self, name: impl Into<String>) {
        self.globals_by_name.insert(name.into(), None);
    }

    #[inline]
    pub fn push_event(&mut self, e: SemanticEvent) {
        use SemanticEvent::*;
        match e {
            ScopeStarted {
                range,
                parent_scope_id,
                is_closure,
            } => {
                // Scopes will be raised in order
                let scope_id = ScopeId::new(self.scopes.len());

                self.scopes.push(SemanticModelScopeData {
                    range,
                    parent: parent_scope_id,
                    children: vec![],
                    bindings: vec![],
                    bindings_by_name: FxHashMap::default(),
                    read_references: vec![],
                    write_references: vec![],
                    is_closure,
                });

                if let Some(parent_scope_id) = parent_scope_id {
                    self.scopes[parent_scope_id.index()].children.push(scope_id);
                }

                let start = range.start();
                self.scope_range_by_start
                    .entry(start)
                    .or_default()
                    .insert(Interval {
                        start: start.into(),
                        stop: range.end().into(),
                        val: scope_id,
                    });
            }
            ScopeEnded { .. } => {}
            DeclarationFound {
                range,
                scope_id,
                hoisted_scope_id,
            } => {
                let binding_scope_id = hoisted_scope_id.unwrap_or(scope_id);

                // SAFETY: this scope id is guaranteed to exist because they were generated by the
                // event extractor
                debug_assert!((binding_scope_id.index()) < self.scopes.len());

                let binding_id = BindingId::new(self.bindings.len());
                self.bindings.push(SemanticModelBindingData {
                    range,
                    references: Vec::new(),
                    export_by_start: smallvec::SmallVec::new(),
                });
                self.bindings_by_start.insert(range.start(), binding_id);

                let scope = &mut self.scopes[binding_scope_id.index()];

                scope.bindings.push(binding_id);
                // Handle bindings with a bogus name
                if let Some(node) = self.binding_node_by_start.get(&range.start()) {
                    if let Some(node) = JsIdentifierBinding::cast_ref(node) {
                        if let Ok(name_token) = node.name_token() {
                            let name = name_token.token_text_trimmed();
                            scope.bindings_by_name.insert(name, binding_id);
                        }
                    }
                }

                if let Some(hoisted_scope_id) = hoisted_scope_id {
                    self.scope_hoisted_to_by_range
                        .insert(range.start(), hoisted_scope_id);
                }
            }
            Read {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(SemanticModelReference {
                    range_start: range.start(),
                    ty: SemanticModelReferenceType::Read { hoisted: false },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);

                self.declared_at_by_start.insert(range.start(), binding_id);
            }
            HoistedRead {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(SemanticModelReference {
                    range_start: range.start(),
                    ty: SemanticModelReferenceType::Read { hoisted: true },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);

                self.declared_at_by_start.insert(range.start(), binding_id);
            }
            Write {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(SemanticModelReference {
                    range_start: range.start(),
                    ty: SemanticModelReferenceType::Write { hoisted: false },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);

                self.declared_at_by_start.insert(range.start(), binding_id);
            }
            HoistedWrite {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(SemanticModelReference {
                    range_start: range.start(),
                    ty: SemanticModelReferenceType::Write { hoisted: true },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);

                self.declared_at_by_start.insert(range.start(), binding_id);
            }
            UnresolvedReference { is_read, range } => {
                let ty = if is_read {
                    SemanticModelReferenceType::Read { hoisted: false }
                } else {
                    SemanticModelReferenceType::Write { hoisted: false }
                };

                let node = &self.binding_node_by_start[&range.start()];
                let name = node.text_trimmed().to_string();

                match self.globals_by_name.entry(name) {
                    Entry::Occupied(mut entry) => {
                        let entry = entry.get_mut();
                        match entry {
                            Some(index) => {
                                self.globals[(*index) as usize].references.push(
                                    SemanticModelGlobalReferenceData {
                                        range_start: range.start(),
                                        ty,
                                    },
                                );
                            }
                            None => {
                                let id = self.globals.len() as u32;
                                self.globals.push(SemanticModelGlobalBindingData {
                                    references: vec![SemanticModelGlobalReferenceData {
                                        range_start: range.start(),
                                        ty,
                                    }],
                                });
                                *entry = Some(id);
                            }
                        }
                    }
                    Entry::Vacant(_) => self
                        .unresolved_references
                        .push(SemanticModelUnresolvedReference { range }),
                }
            }
            Export {
                declaration_at,
                range,
            } => {
                self.exported.insert(declaration_at);

                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                binding.export_by_start.push(range.start());
            }
        }
    }

    #[inline]
    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            scopes: self.scopes,
            scope_by_range: Lapper::new(
                self.scope_range_by_start
                    .iter()
                    .flat_map(|(_, scopes)| scopes.iter())
                    .cloned()
                    .collect(),
            ),
            scope_hoisted_to_by_range: self.scope_hoisted_to_by_range,
            binding_node_by_start: self.binding_node_by_start,
            scope_node_by_range: self.scope_node_by_range,
            bindings: self.bindings,
            bindings_by_start: self.bindings_by_start,
            declared_at_by_start: self.declared_at_by_start,
            exported: self.exported,
            unresolved_references: self.unresolved_references,
            globals: self.globals,
        };
        SemanticModel::new(data)
    }
}

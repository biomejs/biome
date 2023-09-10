use super::*;
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode, TextRange};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry;

/// Builds the [SemanticModel] consuming [SemanticEvent] and [SyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvents] and build all the
/// data necessary to build a [SemanticModelData], that is allocated with an [Arc]
/// and stored inside the [SemanticModel].
pub struct SemanticModelBuilder {
    root: AnyJsRoot,
    node_by_range: FxHashMap<TextRange, JsSyntaxNode>,
    globals: Vec<SemanticModelGlobalBindingData>,
    globals_by_name: FxHashMap<String, Option<usize>>,
    scopes: Vec<SemanticModelScopeData>,
    scope_range_by_start: FxHashMap<TextSize, BTreeSet<Interval<usize, usize>>>,
    scope_hoisted_to_by_range: FxHashMap<TextSize, usize>,
    bindings: Vec<SemanticModelBindingData>,
    /// maps a binding range to its index inside SemanticModelBuilder::bindings vec
    bindings_by_range: FxHashMap<TextRange, usize>,
    /// maps a reference range to its bindings. usize points to SemanticModelBuilder::bindings vec
    declared_at_by_range: FxHashMap<TextRange, usize>,
    exported: FxHashSet<TextRange>,
    unresolved_references: Vec<SemanticModelUnresolvedReference>,
}

impl SemanticModelBuilder {
    pub fn new(root: AnyJsRoot) -> Self {
        Self {
            root,
            node_by_range: FxHashMap::default(),
            globals: vec![],
            globals_by_name: FxHashMap::default(),
            scopes: vec![],
            scope_range_by_start: FxHashMap::default(),
            scope_hoisted_to_by_range: FxHashMap::default(),
            bindings: vec![],
            bindings_by_range: FxHashMap::default(),
            declared_at_by_range: FxHashMap::default(),
            exported: FxHashSet::default(),
            unresolved_references: Vec::new(),
        }
    }

    #[inline]
    pub fn push_node(&mut self, node: &JsSyntaxNode) {
        use JsSyntaxKind::*;
        match node.kind() {
            // Acessible from bindings and references
            JS_IDENTIFIER_BINDING
            | TS_IDENTIFIER_BINDING
            | JS_REFERENCE_IDENTIFIER
            | JSX_REFERENCE_IDENTIFIER
            | TS_TYPE_PARAMETER_NAME
            | JS_IDENTIFIER_ASSIGNMENT => {
                self.node_by_range.insert(node.text_range(), node.clone());
            }

            // Acessible from scopes, closures
            JS_MODULE
            | JS_SCRIPT
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
            | TS_INTERFACE_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION
            | TS_FUNCTION_TYPE
            | JS_BLOCK_STATEMENT
            | JS_FOR_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_SWITCH_STATEMENT
            | JS_CATCH_CLAUSE => {
                self.node_by_range.insert(node.text_range(), node.clone());
            }
            _ => {}
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
                scope_id,
                is_closure,
            } => {
                // Scopes will be raised in order
                debug_assert!(scope_id == self.scopes.len());

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
                    self.scopes[parent_scope_id].children.push(scope_id);
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
                name,
                range,
                scope_id,
                hoisted_scope_id,
                ..
            } => {
                let binding_scope_id = hoisted_scope_id.unwrap_or(scope_id);

                // SAFETY: this scope id is guaranteed to exist because they were generated by the
                // event extractor
                debug_assert!(binding_scope_id < self.scopes.len());

                let binding_id = self.bindings.len();
                self.bindings.push(SemanticModelBindingData {
                    id: binding_id.into(),
                    range,
                    references: vec![],
                });
                self.bindings_by_range.insert(range, binding_id);

                let scope = self.scopes.get_mut(binding_scope_id).unwrap();

                scope.bindings.push(binding_id);
                scope.bindings_by_name.insert(name, binding_id);

                if let Some(hoisted_scope_id) = hoisted_scope_id {
                    self.scope_hoisted_to_by_range
                        .insert(range.start(), hoisted_scope_id);
                }
            }
            Read {
                range,
                declared_at: declaration_at, //TODO change to binding_id like we do with scope_id
                scope_id,
            } => {
                let binding_id = match self.bindings_by_range.entry(declaration_at) {
                    Entry::Occupied(entry) => *entry.get(),
                    Entry::Vacant(entry) => {
                        let id = self.bindings.len();
                        self.bindings.push(SemanticModelBindingData {
                            id: id.into(),
                            range,
                            references: vec![],
                        });
                        *entry.insert(id)
                    }
                };
                let binding = &mut self.bindings[binding_id];
                let reference_index = binding.references.len();

                binding.references.push(SemanticModelReference {
                    index: (binding.id, reference_index).into(),
                    range,
                    ty: SemanticModelReferenceType::Read { hoisted: false },
                });

                let scope = &mut self.scopes[scope_id];
                scope.read_references.push(SemanticModelScopeReference {
                    binding_id,
                    reference_id: reference_index,
                });

                self.declared_at_by_range.insert(range, binding_id);
            }
            HoistedRead {
                range,
                declared_at: declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_range[&declaration_at];
                let binding = &mut self.bindings[binding_id];

                let reference_index = binding.references.len();
                binding.references.push(SemanticModelReference {
                    index: (binding.id, reference_index).into(),
                    range,
                    ty: SemanticModelReferenceType::Read { hoisted: true },
                });

                let scope = &mut self.scopes[scope_id];
                scope.read_references.push(SemanticModelScopeReference {
                    binding_id,
                    reference_id: reference_index,
                });

                self.declared_at_by_range.insert(range, binding_id);
            }
            Write {
                range,
                declared_at: declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_range[&declaration_at];
                let binding = &mut self.bindings[binding_id];

                let reference_index = binding.references.len();
                binding.references.push(SemanticModelReference {
                    index: (binding.id, reference_index).into(),
                    range,
                    ty: SemanticModelReferenceType::Write { hoisted: false },
                });

                let scope = &mut self.scopes[scope_id];
                scope.read_references.push(SemanticModelScopeReference {
                    binding_id,
                    reference_id: reference_index,
                });

                self.declared_at_by_range.insert(range, binding_id);
            }
            HoistedWrite {
                range,
                declared_at: declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_range[&declaration_at];
                let binding = &mut self.bindings[binding_id];

                let reference_index = binding.references.len();
                binding.references.push(SemanticModelReference {
                    index: (binding.id, reference_index).into(),
                    range,
                    ty: SemanticModelReferenceType::Write { hoisted: true },
                });

                let scope = &mut self.scopes[scope_id];
                scope.read_references.push(SemanticModelScopeReference {
                    binding_id,
                    reference_id: reference_index,
                });

                self.declared_at_by_range.insert(range, binding_id);
            }
            UnresolvedReference { is_read, range } => {
                let ty = if is_read {
                    SemanticModelReferenceType::Read { hoisted: false }
                } else {
                    SemanticModelReferenceType::Write { hoisted: false }
                };

                let node = &self.node_by_range[&range];
                let name = node.text_trimmed().to_string();

                match self.globals_by_name.entry(name) {
                    Entry::Occupied(mut entry) => {
                        let entry = entry.get_mut();
                        match entry {
                            Some(index) => {
                                self.globals[*index]
                                    .references
                                    .push(SemanticModelGlobalReferenceData { range, ty });
                            }
                            None => {
                                let id = self.globals.len();
                                self.globals.push(SemanticModelGlobalBindingData {
                                    references: vec![SemanticModelGlobalReferenceData {
                                        range,
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
            Exported { range } => {
                self.exported.insert(range);
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
            node_by_range: self.node_by_range,
            bindings: self.bindings,
            bindings_by_range: self.bindings_by_range,
            declared_at_by_range: self.declared_at_by_range,
            exported: self.exported,
            unresolved_references: self.unresolved_references,
            globals: self.globals,
        };
        SemanticModel::new(data)
    }
}

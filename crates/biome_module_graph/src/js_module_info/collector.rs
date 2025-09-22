use std::{borrow::Cow, collections::BTreeSet, sync::Arc};

use biome_js_semantic::{SemanticEvent, SemanticEventExtractor};
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsExportDefaultDeclaration, AnyJsExpression,
    AnyJsImportClause, JsForVariableDeclaration, JsFormalParameter, JsIdentifierBinding,
    JsRestParameter, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsVariableDeclaration,
    TsIdentifierBinding, TsTypeParameter, TsTypeParameterName, inner_string_text,
};
use biome_js_type_info::{
    BindingId, FunctionParameter, GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, GenericTypeParameter,
    MAX_FLATTEN_DEPTH, Module, Namespace, Resolvable, ResolvedTypeData, ResolvedTypeId, ScopeId,
    TypeData, TypeId, TypeImportQualifier, TypeMember, TypeMemberKind, TypeReference,
    TypeReferenceQualifier, TypeResolver, TypeResolverLevel, TypeStore,
};
use biome_jsdoc_comment::JsdocComment;
use biome_rowan::{AstNode, Text, TextRange, TextSize, TokenText};
use indexmap::IndexMap;
use rust_lapper::{Interval, Lapper};
use rustc_hash::FxHashMap;

use super::{
    Exports, ImportSymbol, Imports, JsExport, JsImport, JsModuleInfo, JsModuleInfoDiagnostic,
    JsModuleInfoInner, JsOwnExport, JsReexport, ResolvedPath, binding::JsBindingData,
    scope::JsScopeData,
};
use crate::js_module_info::{
    binding::{JsBindingReference, JsBindingReferenceKind, JsDeclarationKind},
    scope::TsBindingReference,
    scope_id_for_range,
    utils::reached_too_many_types,
};
use crate::{JsImportPath, JsImportPhase};

/// Responsible for collecting all the information from which to build the
/// [`JsModuleInfo`].
///
/// This collects a lot of fields with raw information, which then goes through
/// another round of processing as we create the intermediate
/// [`JsModuleInfoBag`], and finally the [`JsModuleInfo`] itself.
#[derive(Default)]
pub(super) struct JsModuleInfoCollector {
    pub(super) bindings: Vec<JsBindingData>,

    /// Maps a binding range start to its index inside the [Self::bindings]
    /// vector.
    bindings_by_start: FxHashMap<TextSize, BindingId>,

    /// Binding and reference nodes indexed by their range start
    binding_node_by_start: FxHashMap<TextSize, JsSyntaxNode>,

    /// Creates semantic events from the full node traversal.
    ///
    /// Re-used from the semantic model in `biome_js_semantic`.
    extractor: SemanticEventExtractor,

    /// Function parameters, both formal parameters as well as rest parameters.
    function_parameters: FxHashMap<JsSyntaxNode, FunctionParameter>,

    /// Variable declarations.
    variable_declarations: FxHashMap<JsSyntaxNode, Box<[(Text, TypeReference)]>>,

    /// Map of parsed declarations, for caching purposes.
    parsed_expressions: FxHashMap<TextRange, ResolvedTypeId>,

    /// Collection of all the scopes within the module.
    ///
    /// The first entry is always the module's global scope.
    pub(super) scopes: Vec<JsScopeData>,

    /// Used to build the Lapper lookup tree for finding scopes by text range.
    scope_range_by_start: FxHashMap<TextSize, BTreeSet<Interval<u32, ScopeId>>>,

    /// Used for tracking the scope we are currently in.
    scope_stack: Vec<ScopeId>,

    /// Map with all static import paths, from the source specifier to the
    /// resolved path.
    static_import_paths: IndexMap<Text, JsImportPath>,

    /// Map with all dynamic import paths, from the import source to the
    /// resolved path.
    dynamic_import_paths: IndexMap<Text, JsImportPath>,

    /// All collected exports.
    ///
    /// When we've completed a pass over the module, we will attempt to resolve
    /// the references of these exports and construct the final exports.
    exports: Vec<JsCollectedExport>,

    /// List of all blanket re-exports.
    blanket_reexports: Vec<JsReexport>,

    /// Types collected in the module.
    types: TypeStore,

    /// Static imports mapped from the local name of the binding being imported.
    static_imports: IndexMap<Text, JsImport>,

    /// Diagnostics emitted during the collection of module graph information
    diagnostics: Vec<JsModuleInfoDiagnostic>,
}

/// Intermediary representation for an exported symbol.
pub(super) enum JsCollectedExport {
    ExportNamedSymbol {
        /// Name under which the symbol will be exported.
        export_name: Text,

        /// Local name of the symbol in the global scope.
        local_name: TokenText,
    },
    ExportDefault {
        /// Reference to the type being exported.
        ty: TypeReference,
    },
    ExportNamedDefault {
        /// Local name of the symbol in the global scope.
        local_name: TokenText,
    },
    ExportDefaultAssignment {
        /// Reference to the type assigned to the export.
        ty: TypeReference,
    },
    Reexport {
        /// Name under which the import will be re-exported.
        export_name: Text,

        /// Re-export definition.
        reexport: JsReexport,
    },
}

impl JsModuleInfoCollector {
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

            _ => {}
        }

        self.extractor.enter(node);
    }

    pub fn leave_node(&mut self, node: &JsSyntaxNode) {
        self.extractor.leave(node);

        while let Some(event) = self.extractor.pop() {
            self.push_event(event);
        }

        if let Some(expr) = AnyJsExpression::cast_ref(node) {
            let range = expr.range();
            let scope_id = *self.scope_stack.last().expect("there must be a scope");
            let ty = TypeData::from_any_js_expression(self, scope_id, &expr);
            let resolved_id = match GLOBAL_RESOLVER.find_type(&ty) {
                Some(id) => ResolvedTypeId::new(TypeResolverLevel::Global, id),
                None => {
                    let id = self.register_type(Cow::Owned(ty));
                    ResolvedTypeId::new(TypeResolverLevel::Thin, id)
                }
            };

            self.parsed_expressions.insert(range, resolved_id);
        } else if let Some(decl) = JsForVariableDeclaration::cast_ref(node) {
            let scope_id = *self.scope_stack.last().expect("there must be a scope");
            let type_bindings =
                TypeData::typed_bindings_from_js_for_statement(self, scope_id, &decl)
                    .unwrap_or_default();
            self.variable_declarations
                .insert(decl.syntax().clone(), type_bindings);
        } else if let Some(param) = JsFormalParameter::cast_ref(node) {
            let scope_id = *self.scope_stack.last().expect("there must be a scope");
            let parsed_param = FunctionParameter::from_js_formal_parameter(self, scope_id, &param);
            self.function_parameters
                .insert(param.syntax().clone(), parsed_param);
        } else if let Some(param) = JsRestParameter::cast_ref(node) {
            let scope_id = *self.scope_stack.last().expect("there must be a scope");
            let parsed_param = FunctionParameter::from_js_rest_parameter(self, scope_id, &param);
            self.function_parameters
                .insert(param.syntax().clone(), parsed_param);
        } else if let Some(decl) = JsVariableDeclaration::cast_ref(node) {
            let scope_id = *self.scope_stack.last().expect("there must be a scope");
            let type_bindings =
                TypeData::typed_bindings_from_js_variable_declaration(self, scope_id, &decl);
            self.variable_declarations
                .insert(decl.syntax().clone(), type_bindings);
        } else if let Some(import) = biome_js_syntax::JsImport::cast_ref(node) {
            self.push_static_import(import);
        }
    }

    fn push_static_import(&mut self, node: biome_js_syntax::JsImport) -> Option<()> {
        match node.import_clause().ok()? {
            AnyJsImportClause::JsImportBareClause(_node) => {}
            AnyJsImportClause::JsImportCombinedClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let JsImportPath { resolved_path, .. } =
                    self.static_import_paths.get(source.text())?;

                let default_specifier = node.default_specifier().ok()?;
                let local_name = default_specifier.local_name().ok()?;
                let local_name = local_name.as_js_identifier_binding()?;
                let local_name_token = local_name.name_token().ok()?;
                self.static_imports.insert(
                    local_name_token.token_text_trimmed().into(),
                    JsImport {
                        specifier: source.clone().into(),
                        resolved_path: resolved_path.clone(),
                        symbol: ImportSymbol::Default,
                    },
                );

                match node.specifier().ok()? {
                    AnyJsCombinedSpecifier::JsNamedImportSpecifiers(specifiers) => {
                        for specifier in specifiers.specifiers() {
                            let specifier = specifier.ok()?;
                            let local_name = specifier.local_name()?;
                            let local_name = local_name.as_js_identifier_binding()?;
                            let local_name_token = local_name.name_token().ok()?;
                            let symbol_name = specifier
                                .imported_name()
                                .unwrap_or_else(|| local_name_token.clone())
                                .token_text_trimmed();
                            self.static_imports.insert(
                                local_name_token.token_text_trimmed().into(),
                                JsImport {
                                    specifier: source.clone().into(),
                                    resolved_path: resolved_path.clone(),
                                    symbol: ImportSymbol::Named(symbol_name.into()),
                                },
                            );
                        }
                    }
                    AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(specifier) => {
                        let local_name = specifier.local_name().ok()?;
                        let local_name = local_name.as_js_identifier_binding()?;
                        let local_name_token = local_name.name_token().ok()?;
                        self.static_imports.insert(
                            local_name_token.token_text_trimmed().into(),
                            JsImport {
                                specifier: source.into(),
                                resolved_path: resolved_path.clone(),
                                symbol: ImportSymbol::All,
                            },
                        );
                    }
                }
            }
            AnyJsImportClause::JsImportDefaultClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let JsImportPath { resolved_path, .. } =
                    self.static_import_paths.get(source.text())?;

                let local_name = node.default_specifier().ok()?.local_name().ok()?;
                let local_name = local_name.as_js_identifier_binding()?;
                let local_name_token = local_name.name_token().ok()?;
                self.static_imports.insert(
                    local_name_token.token_text_trimmed().into(),
                    JsImport {
                        specifier: source.into(),
                        resolved_path: resolved_path.clone(),
                        symbol: ImportSymbol::Default,
                    },
                );
            }
            AnyJsImportClause::JsImportNamedClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let JsImportPath { resolved_path, .. } =
                    self.static_import_paths.get(source.text())?;

                for specifier in node.named_specifiers().ok()?.specifiers() {
                    let specifier = specifier.ok()?;
                    let local_name = specifier.local_name()?;
                    let local_name = local_name.as_js_identifier_binding()?;
                    let local_name_token = local_name.name_token().ok()?;
                    let symbol_name = specifier
                        .imported_name()
                        .unwrap_or_else(|| local_name_token.clone())
                        .token_text_trimmed();
                    self.static_imports.insert(
                        local_name_token.token_text_trimmed().into(),
                        JsImport {
                            specifier: source.clone().into(),
                            resolved_path: resolved_path.clone(),
                            symbol: ImportSymbol::Named(symbol_name.into()),
                        },
                    );
                }
            }
            AnyJsImportClause::JsImportNamespaceClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let JsImportPath { resolved_path, .. } =
                    self.static_import_paths.get(source.text())?;

                let specifier = node.namespace_specifier().ok()?;
                let local_name = specifier.local_name().ok()?;
                let local_name = local_name.as_js_identifier_binding()?;
                let local_name_token = local_name.name_token().ok()?;
                self.static_imports.insert(
                    local_name_token.token_text_trimmed().into(),
                    JsImport {
                        specifier: source.into(),
                        resolved_path: resolved_path.clone(),
                        symbol: ImportSymbol::All,
                    },
                );
            }
        }

        Some(())
    }

    pub fn register_export(&mut self, export: JsCollectedExport) {
        self.exports.push(export)
    }

    pub fn register_export_with_name(
        &mut self,
        export_name: impl Into<Text>,
        local_name: TokenText,
    ) {
        self.register_export(JsCollectedExport::ExportNamedSymbol {
            export_name: export_name.into(),
            local_name,
        })
    }

    pub fn register_blanket_reexport(&mut self, reexport: JsReexport) {
        self.blanket_reexports.push(reexport);
    }

    pub fn register_static_import_path(
        &mut self,
        specifier: TokenText,
        resolved_path: ResolvedPath,
        phase: JsImportPhase,
    ) {
        self.static_import_paths.insert(
            specifier.into(),
            JsImportPath {
                resolved_path,
                phase,
            },
        );
    }

    pub fn register_dynamic_import_path(
        &mut self,
        specifier: TokenText,
        resolved_path: ResolvedPath,
        phase: JsImportPhase,
    ) {
        self.dynamic_import_paths.insert(
            specifier.into(),
            JsImportPath {
                resolved_path,
                phase,
            },
        );
    }

    fn push_event(&mut self, event: SemanticEvent) {
        use SemanticEvent::*;
        match event {
            ScopeStarted {
                range,
                parent_scope_id,
                ..
            } => {
                // Scopes will be raised in order
                let scope_id = ScopeId::new(self.scopes.len());

                self.scopes.push(JsScopeData {
                    range,
                    parent: parent_scope_id.map(|id| ScopeId::new(id.index())),
                    children: Vec::new(),
                    bindings: Vec::new(),
                    bindings_by_name: FxHashMap::default(),
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

                self.scope_stack.push(scope_id);
            }
            ScopeEnded { .. } => {
                self.scope_stack.pop();
            }
            DeclarationFound {
                range,
                scope_id,
                hoisted_scope_id,
            } => {
                let binding_scope_id = hoisted_scope_id.unwrap_or(scope_id);

                // SAFETY: this scope id is guaranteed to exist because they
                //         were generated by the event extractor
                debug_assert!((binding_scope_id.index()) < self.scopes.len());

                let binding_id = BindingId::new(self.bindings.len());

                // We must proceed and register the binding, even if the node
                // cannot be found. Otherwise, later lookups for the binding
                // may fail.
                let node = self.binding_node_by_start.get(&range.start()).cloned();
                let name_token = node.as_ref().and_then(|node| {
                    if let Some(node) = JsIdentifierBinding::cast_ref(node) {
                        node.name_token().ok()
                    } else if let Some(node) = TsIdentifierBinding::cast_ref(node) {
                        node.name_token().ok()
                    } else if let Some(node) = TsTypeParameterName::cast_ref(node) {
                        node.ident_token().ok()
                    } else {
                        None
                    }
                });

                let name = name_token.as_ref().map(JsSyntaxToken::token_text_trimmed);
                let declaration_kind = node
                    .as_ref()
                    .map(JsDeclarationKind::from_node)
                    .unwrap_or_default();
                let scope_id = *self.scope_stack.last().expect("scope must be present");

                self.bindings.push(JsBindingData {
                    name: name
                        .as_ref()
                        .map(|name| name.clone().into())
                        .unwrap_or_default(),
                    references: Vec::new(),
                    scope_id,
                    declaration_kind,
                    ty: TypeReference::unknown(),
                    jsdoc: node.as_ref().and_then(find_jsdoc),
                    export_ranges: Vec::new(),
                    range,
                });
                self.bindings_by_start.insert(range.start(), binding_id);

                let scope = &mut self.scopes[binding_scope_id.index()];

                scope.bindings.push(binding_id);
                if let Some(name) = name {
                    let binding_reference = TsBindingReference::from_binding_and_declaration_kind(
                        binding_id,
                        declaration_kind,
                    );

                    scope
                        .bindings_by_name
                        .entry(name)
                        .and_modify(|binding| {
                            *binding = binding.union_with(binding_reference);
                        })
                        .or_insert(binding_reference);
                }
            }
            Read {
                range,
                declaration_at,
                ..
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Read { _hoisted: false },
                });
            }
            HoistedRead {
                range,
                declaration_at,
                ..
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Read { _hoisted: true },
                });
            }
            Write {
                range,
                declaration_at,
                ..
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Write { _hoisted: false },
                });
            }
            HoistedWrite {
                range,
                declaration_at,
                ..
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Write { _hoisted: true },
                });
            }
            Export {
                declaration_at,
                range,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                binding.export_ranges.push(range);
            }
            UnresolvedReference { .. } => {}
        }
    }

    fn finalise(&mut self) -> (IndexMap<Text, JsExport>, Lapper<u32, ScopeId>) {
        let scope_by_range = Lapper::new(
            self.scope_range_by_start
                .iter()
                .flat_map(|(_, scopes)| scopes.iter())
                .cloned()
                .collect(),
        );

        self.infer_all_types(&scope_by_range);
        self.resolve_all_and_downgrade_project_references();

        // Purging before flattening will save us from duplicate work during
        // flattening. We'll purge again after for a final cleanup.
        self.purge_redundant_types();
        self.flatten_all();
        self.purge_redundant_types();

        let exports = self.collect_exports();

        (exports, scope_by_range)
    }

    fn infer_all_types(&mut self, scope_by_range: &Lapper<u32, ScopeId>) {
        for index in 0..self.bindings.len() {
            let binding = &self.bindings[index];
            if let Some(node) = self.binding_node_by_start.get(&binding.range.start()) {
                let name = binding.name.clone();
                let scope_id = scope_id_for_range(scope_by_range, binding.range);
                let ty = self.infer_type(&node.clone(), &name, scope_id);
                self.bindings[index].ty = ty;
            }
        }
    }

    fn infer_type(
        &mut self,
        node: &JsSyntaxNode,
        binding_name: &Text,
        scope_id: ScopeId,
    ) -> TypeReference {
        for ancestor in node.ancestors() {
            if let Some(decl) = AnyJsDeclaration::cast_ref(&ancestor) {
                return if let Some(typed_bindings) = decl
                    .as_js_variable_declaration()
                    .and_then(|decl| self.variable_declarations.get(decl.syntax()))
                {
                    typed_bindings
                        .iter()
                        .find_map(|(name, ty)| (name == binding_name).then(|| ty.clone()))
                        .unwrap_or_default()
                } else {
                    let data = TypeData::from_any_js_declaration(self, scope_id, &decl);
                    self.reference_to_owned_data(data)
                };
            } else if let Some(declaration) = AnyJsExportDefaultDeclaration::cast_ref(&ancestor) {
                let data =
                    TypeData::from_any_js_export_default_declaration(self, scope_id, &declaration);
                return self.reference_to_owned_data(data);
            } else if let Some(typed_bindings) = JsForVariableDeclaration::cast_ref(&ancestor)
                .and_then(|decl| self.variable_declarations.get(decl.syntax()))
            {
                return typed_bindings
                    .iter()
                    .find_map(|(name, ty)| (name == binding_name).then(|| ty.clone()))
                    .unwrap_or_default();
            } else if let Some(param) = JsFormalParameter::cast_ref(&ancestor)
                .and_then(|param| self.function_parameters.get(param.syntax()))
                .or_else(|| {
                    JsRestParameter::cast_ref(&ancestor)
                        .and_then(|param| self.function_parameters.get(param.syntax()))
                })
            {
                return match param {
                    FunctionParameter::Named(named) => named.ty.clone(),
                    FunctionParameter::Pattern(pattern) => pattern
                        .bindings
                        .iter()
                        .find_map(|binding| {
                            (binding.name == *binding_name).then(|| binding.ty.clone())
                        })
                        .unwrap_or_default(),
                };
            } else if let Some(param) = TsTypeParameter::cast_ref(&ancestor) {
                return match GenericTypeParameter::from_ts_type_parameter(self, scope_id, &param) {
                    Some(generic) => self.reference_to_owned_data(TypeData::from(generic)),
                    None => TypeReference::unknown(),
                };
            }
        }

        TypeReference::unknown()
    }

    /// After the first pass of the collector, import references have been
    /// resolved to an import binding. But we can't store the information of the
    /// import target inside the `ResolvedTypeId`, because it resides in the
    /// module's semantic data, and `ResolvedTypeId` is only 8 bytes. So during
    /// resolving, we "downgrade" the import references from
    /// [`TypeReference::Resolved`] to [`TypeReference::Import`].
    fn resolve_all_and_downgrade_project_references(&mut self) {
        // First do a pass in which we populate module and namespace members:
        let mut i = 0;
        while i < self.types.len() {
            match self.types.get(i).as_ref() {
                TypeData::Module(module) => {
                    if let Some(module_binding) = self.find_binding_for_type_index(i) {
                        let ty = TypeData::from(Module {
                            name: module.name.clone(),
                            // Populate module members:
                            members: self.find_type_members_in_scope(module_binding.scope_id),
                        });
                        self.types.replace(i, ty);
                    }
                }
                TypeData::Namespace(namespace) => {
                    if let Some(namespace_binding) = self.find_binding_for_type_index(i) {
                        let ty = TypeData::from(Namespace {
                            path: namespace.path.clone(),
                            // Populate namespace members:
                            members: self.find_type_members_in_scope(namespace_binding.scope_id),
                        });
                        self.types.replace(i, ty);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Now perform a pass for the actual resolving:
        let mut i = 0;
        while i < self.types.len() {
            let ty = self.types.get(i);
            let mut ty = match ty.resolved(self) {
                Some(ty) => ty,
                None => ty.as_ref().clone(),
            };
            ty.update_all_references(|reference| match reference {
                TypeReference::Resolved(resolved)
                    if resolved.level() == TypeResolverLevel::Import =>
                {
                    let binding = &self.bindings[resolved.index()];
                    *reference = self.static_imports.get(&binding.name).map_or(
                        TypeReference::unknown(),
                        |import| {
                            TypeReference::from(TypeImportQualifier {
                                symbol: import.symbol.clone(),
                                resolved_path: import.resolved_path.clone(),
                                type_only: binding.declaration_kind.is_import_type_declaration(),
                            })
                        },
                    );
                }
                TypeReference::Qualifier(_) => {
                    // Qualifiers that haven't been resolved yet will never
                    // be resolved.
                    *reference = TypeReference::unknown();
                }
                _ => {}
            });
            self.types.replace(i, ty);
            i += 1;
        }
    }

    fn find_binding_for_type_index(&self, type_index: usize) -> Option<&JsBindingData> {
        self.bindings.iter().find(|binding| match binding.ty {
            TypeReference::Resolved(resolved) => {
                resolved.level() == TypeResolverLevel::Thin && resolved.id().index() == type_index
            }
            _ => false,
        })
    }

    fn find_binding_in_scope(&self, name: &str, scope_id: ScopeId) -> Option<TsBindingReference> {
        let mut scope = &self.scopes[scope_id.index()];
        loop {
            if let Some(binding_ref) = scope.bindings_by_name.get(name) {
                return Some(*binding_ref);
            }

            match &scope.parent {
                Some(parent_id) => scope = &self.scopes[parent_id.index()],
                None => break,
            }
        }

        None
    }

    fn find_type_members_in_scope(&self, scope_id: ScopeId) -> Box<[TypeMember]> {
        self.bindings
            .iter()
            .filter(|binding| {
                let scope = &self.scopes[binding.scope_id.index()];
                scope
                    .parent
                    .is_some_and(|parent_scope_id| parent_scope_id == scope_id)
            })
            .map(|binding| TypeMember {
                kind: TypeMemberKind::NamedStatic(binding.name.clone()),
                ty: binding.ty.clone(),
            })
            .collect()
    }

    fn flatten_all(&mut self) {
        for _ in 0..MAX_FLATTEN_DEPTH {
            let mut did_flatten = false;

            let mut i = 0;
            while i < self.types.len() {
                if let Err(diagnostic) = reached_too_many_types(i) {
                    self.diagnostics.push(diagnostic);
                    return;
                }

                if let Some(ty) = self.types.get(i).flattened(self) {
                    self.types.replace(i, ty);
                    did_flatten = true;
                }
                i += 1;
            }

            if !did_flatten {
                break;
            }
        }
    }

    fn purge_redundant_types(&mut self) {
        let Some(update_resolved_id) = self.types.deduplicate(TypeResolverLevel::Thin) else {
            return;
        };

        for binding in &mut self.bindings {
            if let TypeReference::Resolved(resolved_id) = &mut binding.ty {
                update_resolved_id(resolved_id);
            }
        }

        for collected_export in &mut self.exports {
            match collected_export {
                JsCollectedExport::ExportDefault { ty }
                | JsCollectedExport::ExportDefaultAssignment { ty } => {
                    if let TypeReference::Resolved(resolved_id) = ty {
                        update_resolved_id(resolved_id);
                    }
                }
                JsCollectedExport::ExportNamedSymbol { .. }
                | JsCollectedExport::ExportNamedDefault { .. }
                | JsCollectedExport::Reexport { .. } => {}
            }
        }

        for resolved_id in self.parsed_expressions.values_mut() {
            update_resolved_id(resolved_id);
        }
    }

    fn collect_exports(&mut self) -> IndexMap<Text, JsExport> {
        let mut finalised_exports = IndexMap::new();

        let exports = std::mem::take(&mut self.exports);
        for export in exports {
            match export {
                JsCollectedExport::ExportNamedSymbol {
                    export_name,
                    local_name,
                } => {
                    if let Some(export) = self.get_export_for_local_name(local_name) {
                        finalised_exports.insert(export_name, JsExport::Own(export));
                    }
                }
                JsCollectedExport::ExportNamedDefault { local_name } => {
                    if let Some(export) = self.get_export_for_local_name(local_name) {
                        finalised_exports
                            .insert(Text::new_static("default"), JsExport::Own(export));
                    }
                }
                JsCollectedExport::ExportDefault { ty } => {
                    let resolved = self.resolve_reference(&ty).unwrap_or(GLOBAL_UNKNOWN_ID);

                    let export = JsExport::Own(JsOwnExport::Type(resolved));
                    finalised_exports.insert(Text::new_static("default"), export);
                }
                JsCollectedExport::ExportDefaultAssignment { ty } => {
                    let resolved = self.resolve_reference(&ty).unwrap_or(GLOBAL_UNKNOWN_ID);

                    if let Some(data) = self.get_by_resolved_id(resolved) {
                        for member in data.as_raw_data().own_members() {
                            let Some(name) = member.name() else {
                                continue;
                            };

                            // DANGER: Normally, when resolving a type reference
                            //         retrieved through `as_raw_data()`, we
                            //         should call
                            //         `apply_module_id_to_reference()` on the
                            //         reference first. But because we know we
                            //         are resolving inside the collector,
                            //         before any module IDs _could_ be applied,
                            //         we can omit this here.
                            if let Some(resolved_member) = self.resolve_reference(&member.ty) {
                                let export = JsExport::Own(JsOwnExport::Type(resolved_member));
                                finalised_exports.insert(name, export);
                            }
                        }
                    }

                    let export = JsExport::Own(JsOwnExport::Type(resolved));
                    finalised_exports.insert(Text::new_static("default"), export);
                }
                JsCollectedExport::Reexport {
                    export_name,
                    reexport,
                } => {
                    finalised_exports.insert(export_name, JsExport::Reexport(reexport));
                }
            }
        }

        finalised_exports
    }

    fn get_export_for_local_name(&mut self, local_name: TokenText) -> Option<JsOwnExport> {
        let binding_ref = self.scopes[0].bindings_by_name.get(&local_name)?;

        let export = match binding_ref {
            TsBindingReference::Merged {
                ty,
                value_ty,
                namespace_ty,
            } => {
                let ty = ty.map(|ty| &self.bindings[ty.index()].ty);
                let value_ty = value_ty.map(|ty| &self.bindings[ty.index()].ty);
                let namespace_ty = namespace_ty.map(|ty| &self.bindings[ty.index()].ty);
                match (ty, value_ty, namespace_ty) {
                    (Some(ty1), Some(ty2), None)
                    | (Some(ty1), None, Some(ty2))
                    | (None, Some(ty1), Some(ty2))
                        if ty1 == ty2 =>
                    {
                        let ty = self.register_and_resolve(TypeData::reference(ty1.clone()));
                        JsOwnExport::Type(ty)
                    }
                    (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty2 == ty3 => {
                        let ty = self.register_and_resolve(TypeData::reference(ty1.clone()));
                        JsOwnExport::Type(ty)
                    }
                    _ => {
                        let ty = self.register_and_resolve(TypeData::merged_reference(
                            ty.cloned(),
                            value_ty.cloned(),
                            namespace_ty.cloned(),
                        ));
                        JsOwnExport::Type(ty)
                    }
                }
            }
            TsBindingReference::Type(binding_id)
            | TsBindingReference::ValueType(binding_id)
            | TsBindingReference::TypeAndValueType(binding_id)
            | TsBindingReference::NamespaceAndValueType(binding_id) => {
                JsOwnExport::Binding(*binding_id)
            }
        };

        Some(export)
    }
}

impl TypeResolver for JsModuleInfoCollector {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Thin
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
        let mut id = id;
        loop {
            let resolved_data: ResolvedTypeData = match id.level() {
                TypeResolverLevel::Thin => (id, self.get_by_id(id.id())).into(),
                TypeResolverLevel::Global => (id, GLOBAL_RESOLVER.get_by_id(id.id())).into(),
                TypeResolverLevel::Full | TypeResolverLevel::Import => break None,
            };

            match resolved_data.as_raw_data() {
                TypeData::Reference(TypeReference::Resolved(resolved_id)) if id != *resolved_id => {
                    id = *resolved_id;
                }
                _ => break Some(resolved_data),
            }
        }
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        match GLOBAL_RESOLVER.find_type(&type_data) {
            Some(id) => {
                let reference =
                    TypeData::reference(ResolvedTypeId::new(TypeResolverLevel::Global, id));
                self.types.insert_cow(Cow::Owned(reference))
            }
            None => self.types.insert_cow(type_data),
        }
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => Some(*resolved_id),
            TypeReference::Import(_) => None,
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        let mut path_parts = qualifier.path.iter();
        let identifier = path_parts.next()?;
        let Some(binding_ref) = self.find_binding_in_scope(identifier, qualifier.scope_id) else {
            return GLOBAL_RESOLVER.resolve_qualifier(qualifier);
        };

        let binding_id = binding_ref.get_binding_id_for_qualifier(qualifier)?;

        let binding = &self.bindings[binding_id.index()];
        if binding.declaration_kind.is_import_declaration() {
            return Some(ResolvedTypeId::new(
                TypeResolverLevel::Import,
                binding_id.into(),
            ));
        }

        let mut ty = Cow::Borrowed(&binding.ty);
        for identifier in path_parts {
            let resolved = self.resolve_and_get(&ty)?;
            let member = resolved
                .all_members(self)
                .with_excluded_binding_id(binding_id)
                .find(|member| member.is_static() && member.has_name(identifier))?;
            ty = Cow::Owned(member.ty().into_owned());
        }

        self.resolve_reference(&ty)
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        if let Some(binding_id) = self
            .find_binding_in_scope(identifier, scope_id)
            .map(|binding_ref| binding_ref.value_ty_or_ty())
        {
            let binding = &self.bindings[binding_id.index()];
            return if binding.declaration_kind.is_import_declaration() {
                Some(ResolvedTypeId::new(
                    TypeResolverLevel::Import,
                    binding_id.into(),
                ))
            } else {
                self.resolve_reference(&binding.ty)
            };
        }

        GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id)
    }

    fn resolve_expression(
        &mut self,
        _scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Cow<'_, TypeData> {
        match self.parsed_expressions.get(&expr.range()) {
            Some(resolved_id) => match resolved_id.level() {
                TypeResolverLevel::Thin => Cow::Borrowed(self.get_by_id(resolved_id.id())),
                TypeResolverLevel::Global => {
                    Cow::Borrowed(GLOBAL_RESOLVER.get_by_id(resolved_id.id()))
                }
                TypeResolverLevel::Full | TypeResolverLevel::Import => {
                    Cow::Owned(TypeData::unknown())
                }
            },
            None => Cow::Owned(TypeData::unknown()),
        }
    }

    fn reference_to_resolved_expression(
        &mut self,
        _scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> TypeReference {
        self.parsed_expressions
            .get(&expression.range())
            .map(|resolved_id| TypeReference::Resolved(*resolved_id))
            .unwrap_or_default()
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(GLOBAL_RESOLVER.as_ref())
    }

    fn registered_types(&self) -> Vec<&TypeData> {
        self.types.as_references()
    }
}

impl JsModuleInfo {
    pub(super) fn new(mut collector: JsModuleInfoCollector) -> Self {
        let (exports, scope_by_range) = collector.finalise();

        Self(Arc::new(JsModuleInfoInner {
            static_imports: Imports(collector.static_imports),
            static_import_paths: collector.static_import_paths,
            dynamic_import_paths: collector.dynamic_import_paths,
            exports: Exports(exports),
            blanket_reexports: collector.blanket_reexports,
            bindings: collector.bindings,
            expressions: collector.parsed_expressions,
            scopes: collector.scopes,
            scope_by_range,
            types: collector.types.into(),
            diagnostics: collector.diagnostics.into_iter().map(Into::into).collect(),
        }))
    }
}

fn find_jsdoc(node: &JsSyntaxNode) -> Option<JsdocComment> {
    node.ancestors().find_map(|ancestor| {
        if let Some(export) = biome_js_syntax::JsExport::cast_ref(&ancestor) {
            JsdocComment::try_from(export.syntax()).ok()
        } else if let Some(decl) = AnyJsDeclaration::cast(ancestor) {
            JsdocComment::try_from(decl.syntax()).ok()
        } else {
            None
        }
    })
}

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use biome_js_semantic::{
    BindingId, ReferenceId, ScopeId, SemanticEvent, SemanticEventExtractor, find_import_node,
};
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsExportDefaultDeclaration, AnyJsImportClause,
    JsFormalParameter, JsIdentifierBinding, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    TsIdentifierBinding, inner_string_text,
};
use biome_js_type_info::{FunctionParameter, Type};
use biome_rowan::{AstNode, Text, TextSize, TokenText};
use rust_lapper::{Interval, Lapper};
use rustc_hash::FxHashMap;

use crate::{
    js_module_info::binding::{JsBindingReference, JsBindingReferenceKind, JsDeclarationKind},
    jsdoc_comment::JsdocComment,
};

use super::{
    Exports, Imports, JsExport, JsImport, JsImportSymbol, JsModuleInfo, JsModuleInfoInner,
    JsOwnExport, JsReexport, JsResolvedPath, binding::JsBindingData,
    global_scope_resolver::GlobalScopeResolver, scope::JsScopeData,
};

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

    /// Collection of all the scopes within the module.
    ///
    /// The first entry is always the module's global scope.
    pub(super) scopes: Vec<JsScopeData>,

    /// Used to build the Lapper lookup tree for finding scopes by text range.
    scope_range_by_start: FxHashMap<TextSize, BTreeSet<Interval<u32, ScopeId>>>,

    /// Used for tracking the scope we are currently in.
    scope_stack: Vec<ScopeId>,

    /// Map with all static import paths, from the source specifier to the resolved path.
    static_import_paths: BTreeMap<Text, JsResolvedPath>,

    /// Map with all dynamic import paths, from the import source to the resolved path.
    dynamic_import_paths: BTreeMap<Text, JsResolvedPath>,

    /// Map with exports, from the exported symbol name to a [JsExport] definition.
    exports: BTreeMap<Text, JsExport>,

    /// List of all blanket re-exports.
    blanket_reexports: Vec<JsReexport>,

    /// Map of parsed declarations, for caching purposes.
    parsed_declarations: FxHashMap<JsSyntaxNode, Box<[(Text, Type)]>>,

    /// Map of parsed function parameters, for caching purposes.
    parsed_parameters: FxHashMap<JsSyntaxNode, FunctionParameter>,
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
    }

    pub fn register_export(&mut self, name: impl Into<Text>, export: JsExport) -> Option<()> {
        self.exports.insert(name.into(), export);

        Some(())
    }

    pub fn register_export_with_name(
        &mut self,
        export_name: impl Into<Text>,
        local_name: Option<TokenText>,
    ) -> Option<()> {
        self.register_export(
            export_name,
            JsExport::Own(JsOwnExport {
                jsdoc_comment: None,
                local_name,
                ty: Type::unknown(),
            }),
        )
    }

    pub fn register_blanket_reexport(&mut self, reexport: JsReexport) {
        self.blanket_reexports.push(reexport);
    }

    pub fn register_static_import_path(
        &mut self,
        specifier: TokenText,
        resolved_path: JsResolvedPath,
    ) {
        self.static_import_paths
            .insert(specifier.into(), resolved_path);
    }

    pub fn register_dynamic_import_path(
        &mut self,
        specifier: TokenText,
        resolved_path: JsResolvedPath,
    ) {
        self.dynamic_import_paths
            .insert(specifier.into(), resolved_path);
    }

    pub fn finalise(&mut self) {
        while let Some(event) = self.extractor.pop() {
            self.push_event(event);
        }
    }

    fn push_event(&mut self, event: SemanticEvent) {
        use SemanticEvent::*;
        match event {
            ScopeStarted {
                range,
                parent_scope_id,
                is_closure,
            } => {
                // Scopes will be raised in order
                let scope_id = ScopeId::new(self.scopes.len());

                self.scopes.push(JsScopeData {
                    range,
                    parent: parent_scope_id,
                    children: Vec::new(),
                    bindings: Vec::new(),
                    bindings_by_name: FxHashMap::default(),
                    read_references: Vec::new(),
                    _write_references: Vec::new(),
                    _is_closure: is_closure,
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
                let node = self.binding_node_by_start.get(&range.start());
                let name_token = node.and_then(|node| {
                    if let Some(node) = JsIdentifierBinding::cast_ref(node) {
                        node.name_token().ok()
                    } else if let Some(node) = TsIdentifierBinding::cast_ref(node) {
                        node.name_token().ok()
                    } else {
                        None
                    }
                });

                let name = name_token.as_ref().map(JsSyntaxToken::token_text_trimmed);
                let ty = match (node, &name) {
                    (Some(node), Some(name)) => infer_type(
                        node,
                        name,
                        &mut self.parsed_declarations,
                        &mut self.parsed_parameters,
                    ),
                    _ => Type::unknown(),
                };

                self.bindings.push(JsBindingData {
                    name: name
                        .as_ref()
                        .map(|name| name.clone().into())
                        .unwrap_or_default(),
                    range,
                    references: Vec::new(),
                    scope_id: *self.scope_stack.last().expect("scope must be present"),
                    declaration_kind: node.map(JsDeclarationKind::from_node).unwrap_or_default(),
                    ty,
                    jsdoc: node.and_then(find_jsdoc),
                    export_ranges: Vec::new(),
                });
                self.bindings_by_start.insert(range.start(), binding_id);

                let scope = &mut self.scopes[binding_scope_id.index()];

                scope.bindings.push(binding_id);
                if let Some(name) = name {
                    scope.bindings_by_name.insert(name, binding_id);
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
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Read { _hoisted: false },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);
            }
            HoistedRead {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Read { _hoisted: true },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);
            }
            Write {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Write { _hoisted: false },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);
            }
            HoistedWrite {
                range,
                declaration_at,
                scope_id,
            } => {
                let binding_id = self.bindings_by_start[&declaration_at];
                let binding = &mut self.bindings[binding_id.index()];
                let reference_id = ReferenceId::new(binding_id, binding.references.len());
                binding.references.push(JsBindingReference {
                    range_start: range.start(),
                    kind: JsBindingReferenceKind::Write { _hoisted: true },
                });

                let scope = &mut self.scopes[scope_id.index()];
                scope.read_references.push(reference_id);
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
}

/// Used for collecting information to store in the [JsModuleInfo].
///
/// The fields stored on this bag are constructed from the raw fields in the
/// [JsModuleInfoCollector] and combined into their final shape as they
#[derive(Clone, Debug, Default)]
pub(super) struct JsModuleInfoBag {
    static_imports: BTreeMap<Text, JsImport>,
    exports: BTreeMap<Text, JsExport>,
    blanket_reexports: Vec<JsReexport>,
}

impl JsModuleInfoBag {
    pub(super) fn from_collector(collector: &JsModuleInfoCollector) -> Self {
        let mut info = Self::default();
        info.collect_imports(collector);
        info.collect_exports(collector);
        info
    }

    fn collect_imports(&mut self, collector: &JsModuleInfoCollector) {
        // Iterate over all bindings in the global scope, and extract imports
        // and exports.
        for binding_id in &collector.scopes[0].bindings {
            let binding = &collector.bindings[binding_id.index()];
            let node = collector
                .binding_node_by_start
                .get(&binding.range.start())
                .expect("missing node");

            let import = find_import_node(node).and_then(|import_node| {
                import_node
                    .ancestors()
                    .find_map(biome_js_syntax::JsImport::cast)
            });
            if let Some(import) = import {
                self.push_static_import(import, collector);
            }
        }
    }

    fn push_static_import(
        &mut self,
        node: biome_js_syntax::JsImport,
        collector: &JsModuleInfoCollector,
    ) -> Option<()> {
        match node.import_clause().ok()? {
            AnyJsImportClause::JsImportBareClause(_node) => {}
            AnyJsImportClause::JsImportCombinedClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let resolved_path = collector.static_import_paths.get(source.text())?;

                let default_specifier = node.default_specifier().ok()?;
                let local_name = default_specifier.local_name().ok()?;
                let local_name = local_name.as_js_identifier_binding()?;
                let local_name_token = local_name.name_token().ok()?;
                self.static_imports.insert(
                    local_name_token.token_text_trimmed().into(),
                    JsImport {
                        specifier: source.clone().into(),
                        resolved_path: resolved_path.clone(),
                        symbol: JsImportSymbol::Default,
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
                                    symbol: JsImportSymbol::Named(symbol_name.into()),
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
                                symbol: JsImportSymbol::All,
                            },
                        );
                    }
                }
            }
            AnyJsImportClause::JsImportDefaultClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let resolved_path = collector.static_import_paths.get(source.text())?;

                let local_name = node.default_specifier().ok()?.local_name().ok()?;
                let local_name = local_name.as_js_identifier_binding()?;
                let local_name_token = local_name.name_token().ok()?;
                self.static_imports.insert(
                    local_name_token.token_text_trimmed().into(),
                    JsImport {
                        specifier: source.into(),
                        resolved_path: resolved_path.clone(),
                        symbol: JsImportSymbol::Default,
                    },
                );
            }
            AnyJsImportClause::JsImportNamedClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let resolved_path = collector.static_import_paths.get(source.text())?;

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
                            symbol: JsImportSymbol::Named(symbol_name.into()),
                        },
                    );
                }
            }
            AnyJsImportClause::JsImportNamespaceClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let source = inner_string_text(&source_token);
                let resolved_path = collector.static_import_paths.get(source.text())?;

                let specifier = node.namespace_specifier().ok()?;
                let local_name = specifier.local_name().ok()?;
                let local_name = local_name.as_js_identifier_binding()?;
                let local_name_token = local_name.name_token().ok()?;
                self.static_imports.insert(
                    local_name_token.token_text_trimmed().into(),
                    JsImport {
                        specifier: source.into(),
                        resolved_path: resolved_path.clone(),
                        symbol: JsImportSymbol::All,
                    },
                );
            }
        }

        Some(())
    }

    fn collect_exports(&mut self, collector: &JsModuleInfoCollector) {
        self.exports.clone_from(&collector.exports);
        self.blanket_reexports
            .clone_from(&collector.blanket_reexports);

        // Lookup types from the bindings in the global scope.
        let global_scope = &collector.scopes[0];
        for export in self.exports.values_mut() {
            let Some(export) = export.as_own_export_mut() else {
                continue;
            };

            let Some(local_name) = &export.local_name else {
                continue;
            };

            if let Some(binding_id) = global_scope.bindings_by_name.get(local_name) {
                let binding = &collector.bindings[binding_id.index()];
                export.jsdoc_comment.clone_from(&binding.jsdoc);
                export.ty.clone_from(&binding.ty);
            }
        }
    }

    /// Performs "thin" or module-level type resolution.
    ///
    /// Iterates over all exported symbols in the module and resolves them if
    /// they refer to any other symbols in scope of the module.
    fn resolve_module_types(&mut self, collector: &JsModuleInfoCollector) {
        let resolver = GlobalScopeResolver::from_collector(collector);

        for export in self.exports.values_mut() {
            if let Some(export) = export.as_own_export_mut() {
                export.ty.resolve(&resolver);
            }
        }
    }
}

impl JsModuleInfo {
    pub(super) fn new(collector: JsModuleInfoCollector) -> Self {
        let mut bag = JsModuleInfoBag::from_collector(&collector);
        bag.resolve_module_types(&collector);

        Self(Arc::new(JsModuleInfoInner {
            static_imports: Imports(bag.static_imports),
            static_import_paths: collector.static_import_paths,
            dynamic_import_paths: collector.dynamic_import_paths,
            exports: Exports(bag.exports),
            blanket_reexports: bag.blanket_reexports.into(),
            bindings: collector.bindings.into(),
            scopes: collector.scopes.into(),
            scope_by_range: Lapper::new(
                collector
                    .scope_range_by_start
                    .iter()
                    .flat_map(|(_, scopes)| scopes.iter())
                    .cloned()
                    .collect(),
            ),
        }))
    }
}

fn find_jsdoc(node: &JsSyntaxNode) -> Option<JsdocComment> {
    match node.ancestors().find_map(biome_js_syntax::JsExport::cast) {
        Some(export) => JsdocComment::try_from(export.syntax()).ok(),
        None => match node.ancestors().find_map(AnyJsDeclaration::cast) {
            Some(decl) => JsdocComment::try_from(decl.syntax()).ok(),
            None => JsdocComment::try_from(node).ok(),
        },
    }
}

fn infer_type(
    node: &JsSyntaxNode,
    binding_name: &TokenText,
    parsed_declarations: &mut FxHashMap<JsSyntaxNode, Box<[(Text, Type)]>>,
    parsed_parameters: &mut FxHashMap<JsSyntaxNode, FunctionParameter>,
) -> Type {
    for ancestor in node.ancestors() {
        if let Some(decl) = AnyJsDeclaration::cast_ref(&ancestor) {
            return if let Some(var_decl) = decl.as_js_variable_declaration() {
                let typed_bindings = parsed_declarations
                    .entry(var_decl.syntax().clone())
                    .or_insert_with(|| Type::typed_bindings_from_js_variable_declaration(var_decl));
                typed_bindings
                    .iter()
                    .find_map(|(name, ty)| (*name == binding_name.text()).then(|| ty.clone()))
                    .unwrap_or_default()
            } else {
                Type::from_any_js_declaration(&decl)
            };
        } else if let Some(declaration) = AnyJsExportDefaultDeclaration::cast_ref(&ancestor) {
            return Type::from_any_js_export_default_declaration(&declaration);
        } else if let Some(param) = JsFormalParameter::cast_ref(&ancestor) {
            let param = parsed_parameters
                .entry(ancestor.clone())
                .or_insert_with(|| FunctionParameter::from_js_formal_parameter(&param));
            return param
                .bindings
                .iter()
                .find_map(|binding| {
                    (binding.name == binding_name.text()).then(|| binding.ty.clone())
                })
                .unwrap_or_default();
        }
    }

    Type::unknown()
}

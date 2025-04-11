use std::{collections::BTreeMap, sync::Arc};

use biome_js_semantic::{
    BindingId, ReferenceId, ScopeId, SemanticEvent, SemanticEventExtractor, find_import_node,
};
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsExportDefaultDeclaration, AnyJsImportClause,
    JsIdentifierBinding, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, inner_string_text,
};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, Text, TextSize, TokenText};
use rustc_hash::FxHashMap;

use crate::{
    js_module_info::binding::{JsBindingReference, JsBindingReferenceKind, JsDeclarationKind},
    jsdoc_comment::JsdocComment,
};

use super::{
    JsExport, JsImport, JsImportSymbol, JsModuleInfo, JsModuleInfoInner, JsOwnExport, JsReexport,
    JsResolvedPath, binding::JsBindingData, scope::JsScopeData,
};

/// Responsible for collecting all the information from which to build the
/// [`JsModuleInfo`].
///
/// This collects a lot of fields with raw information, which then goes through
/// another round of processing as we create the intermediate
/// [`JsModuleInfoBag`], and finally the [`JsModuleInfo`] itself.
#[derive(Default)]
pub(super) struct JsModuleInfoCollector {
    bindings: Vec<JsBindingData>,

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
    scopes: Vec<JsScopeData>,

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
                ty: Type::Unknown,
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
                    JsIdentifierBinding::cast_ref(node).and_then(|node| node.name_token().ok())
                });

                let name = name_token.as_ref().map(JsSyntaxToken::token_text_trimmed);

                self.bindings.push(JsBindingData {
                    range,
                    references: Vec::new(),
                    scope_id: *self.scope_stack.last().expect("scope must be present"),
                    declaration_kind: node.map(JsDeclarationKind::from_node).unwrap_or_default(),
                    ty: match (node, &name) {
                        (Some(node), Some(name)) => infer_type(node, name),
                        _ => Type::Unknown,
                    },
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
    /// Creates the
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
            let export = match export {
                JsExport::Own(export) | JsExport::OwnType(export) => export,
                JsExport::Reexport(_) | JsExport::ReexportType(_) => continue,
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
}

impl JsModuleInfo {
    pub(super) fn new(collector: JsModuleInfoCollector) -> Self {
        let bag = JsModuleInfoBag::from_collector(&collector);

        Self(Arc::new(JsModuleInfoInner {
            static_imports: bag.static_imports,
            static_import_paths: collector.static_import_paths,
            dynamic_import_paths: collector.dynamic_import_paths,
            exports: bag.exports,
            blanket_reexports: bag.blanket_reexports.into(),
            bindings: collector.bindings.into(),
            scopes: collector.scopes.into(),
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

fn infer_type(node: &JsSyntaxNode, binding_name: &TokenText) -> Type {
    let Some(declaration) = node.ancestors().find_map(AnyJsDeclaration::cast) else {
        let Some(declaration) = node
            .ancestors()
            .find_map(AnyJsExportDefaultDeclaration::cast)
        else {
            return Type::Unknown;
        };

        return Type::from_any_js_export_default_declaration(&declaration);
    };

    if let AnyJsDeclaration::JsVariableDeclaration(decl) = declaration {
        decl.declarators()
            .into_iter()
            .filter_map(|decl| decl.ok())
            .find_map(|decl| {
                let binding = decl.id().ok()?;
                // TODO: Handle object and array patterns
                let binding = binding.as_any_js_binding()?.as_js_identifier_binding()?;
                let name_token = binding.name_token().ok()?;
                (*binding_name == name_token.text_trimmed()).then_some(decl)
            })
            .and_then(|declarator| Type::from_js_variable_declarator(&declarator))
            .unwrap_or_default()
    } else {
        Type::from_any_js_declaration(&declaration)
    }
}

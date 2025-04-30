use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use biome_js_semantic::{ScopeId, SemanticEvent, SemanticEventExtractor};
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsExportDefaultDeclaration, AnyJsImportClause,
    JsFormalParameter, JsIdentifierBinding, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    TsIdentifierBinding, inner_string_text,
};
use biome_js_type_info::{
    FunctionParameter, GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, Resolvable, ResolvedTypeId, TypeData,
    TypeId, TypeImportQualifier, TypeReference, TypeReferenceQualifier, TypeResolver,
    TypeResolverLevel,
};
use biome_rowan::{AstNode, Text, TextSize, TokenText};
use rust_lapper::{Interval, Lapper};
use rustc_hash::FxHashMap;

use crate::{
    js_module_info::binding::{JsBindingReference, JsBindingReferenceKind, JsDeclarationKind},
    jsdoc_comment::JsdocComment,
};

use super::{
    Exports, ImportSymbol, Imports, JsExport, JsImport, JsModuleInfo, JsModuleInfoInner,
    JsOwnExport, JsReexport, ResolvedPath,
    binding::{BindingId, JsBindingData},
    scope::JsScopeData,
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
    static_import_paths: BTreeMap<Text, ResolvedPath>,

    /// Map with all dynamic import paths, from the import source to the resolved path.
    dynamic_import_paths: BTreeMap<Text, ResolvedPath>,

    /// Map with exports, from the exported symbol name to a [JsExport] definition.
    exports: BTreeMap<Text, JsExport>,

    /// All imports nodes.
    imports: Vec<biome_js_syntax::JsImport>,

    /// List of all blanket re-exports.
    blanket_reexports: Vec<JsReexport>,

    /// Types collected in the module.
    types: Vec<TypeData>,
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
            JS_IMPORT => self
                .imports
                .push(biome_js_syntax::JsImport::cast_ref(node).unwrap()),
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
                ty: TypeReference::Unknown,
            }),
        )
    }

    pub fn register_blanket_reexport(&mut self, reexport: JsReexport) {
        self.blanket_reexports.push(reexport);
    }

    pub fn register_static_import_path(
        &mut self,
        specifier: TokenText,
        resolved_path: ResolvedPath,
    ) {
        self.static_import_paths
            .insert(specifier.into(), resolved_path);
    }

    pub fn register_dynamic_import_path(
        &mut self,
        specifier: TokenText,
        resolved_path: ResolvedPath,
    ) {
        self.dynamic_import_paths
            .insert(specifier.into(), resolved_path);
    }

    pub fn finalise(&mut self) {
        let mut finaliser = JsModuleInfoCollectorFinaliser::default();
        while let Some(event) = self.extractor.pop() {
            self.push_event(&mut finaliser, event);
        }
    }

    fn push_event(&mut self, finaliser: &mut JsModuleInfoCollectorFinaliser, event: SemanticEvent) {
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
                    parent: parent_scope_id,
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
                    } else {
                        None
                    }
                });

                let name = name_token.as_ref().map(JsSyntaxToken::token_text_trimmed);
                let ty = match (&node, &name) {
                    (Some(node), Some(name)) => self.infer_type(finaliser, node, name),
                    _ => TypeReference::Unknown,
                };

                self.bindings.push(JsBindingData {
                    name: name
                        .as_ref()
                        .map(|name| name.clone().into())
                        .unwrap_or_default(),
                    references: Vec::new(),
                    scope_id: *self.scope_stack.last().expect("scope must be present"),
                    declaration_kind: node
                        .as_ref()
                        .map(JsDeclarationKind::from_node)
                        .unwrap_or_default(),
                    ty,
                    jsdoc: node.as_ref().and_then(find_jsdoc),
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

    fn infer_type(
        &mut self,
        finaliser: &mut JsModuleInfoCollectorFinaliser,
        node: &JsSyntaxNode,
        binding_name: &TokenText,
    ) -> TypeReference {
        let mut infer_type = || {
            for ancestor in node.ancestors() {
                if let Some(decl) = AnyJsDeclaration::cast_ref(&ancestor) {
                    return if let Some(var_decl) = decl.as_js_variable_declaration() {
                        let typed_bindings = finaliser
                            .parsed_declarations
                            .entry(var_decl.syntax().clone())
                            .or_insert_with(|| {
                                TypeData::typed_bindings_from_js_variable_declaration(
                                    self, var_decl,
                                )
                            });
                        typed_bindings
                            .iter()
                            .find_map(|(name, ty)| {
                                (*name == binding_name.text()).then(|| ty.clone())
                            })
                            .unwrap_or_default()
                    } else {
                        TypeData::from_any_js_declaration(self, &decl)
                    };
                } else if let Some(declaration) = AnyJsExportDefaultDeclaration::cast_ref(&ancestor)
                {
                    return TypeData::from_any_js_export_default_declaration(self, &declaration);
                } else if let Some(param) = JsFormalParameter::cast_ref(&ancestor) {
                    let param = finaliser
                        .parsed_parameters
                        .entry(ancestor.clone())
                        .or_insert_with(|| {
                            FunctionParameter::from_js_formal_parameter(self, &param)
                        });
                    return param
                        .bindings
                        .iter()
                        .find_map(|binding| {
                            (binding.name == binding_name.text()).then(|| binding.ty.clone())
                        })
                        .unwrap_or_default();
                }
            }

            TypeData::unknown()
        };

        let type_data = infer_type();
        self.register_and_resolve(type_data).into()
    }

    /// After the first pass of the collector, import references have been
    /// resolved to an import binding. But we can't store the information of the
    /// import target inside the `ResolvedTypeId`, because it resides in the
    /// module's semantic data, and `ResolvedTypeId` is only 8 bytes. So during
    /// resolving, we "downgrade" the import references from
    /// [`TypeReference::Resolved`] to [`TypeReference::Import`].
    fn resolve_all_and_downgrade_project_references(&mut self, bag: &JsModuleInfoBag) {
        let bindings = self.bindings.clone(); // TODO: Can we omit the clone?
        let downgrade_import_reference = |id: BindingId| {
            let binding = &bindings[id.index()];
            bag.static_imports
                .get(&binding.name)
                .map_or(TypeReference::Unknown, |import| {
                    TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                    }
                    .into()
                })
        };

        let mut i = 0;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.resolved_with_mapped_references(
                |reference, _| match reference {
                    TypeReference::Resolved(resolved)
                        if resolved.level() == TypeResolverLevel::Import =>
                    {
                        downgrade_import_reference(resolved.id().into())
                    }
                    other => other,
                },
                self,
            );
            i += 1;
        }
    }

    fn flatten_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.flattened(self);
            i += 1;
        }
    }
}

impl TypeResolver for JsModuleInfoCollector {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Module
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types
            .iter()
            .position(|data| data == type_data)
            .map(TypeId::new)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        &self.types[id.index()]
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData> {
        match id.level() {
            TypeResolverLevel::Module => Some(self.get_by_id(id.id())),
            TypeResolverLevel::Global => Some(GLOBAL_RESOLVER.get_by_id(id.id())),
            TypeResolverLevel::AdHoc | TypeResolverLevel::Import => None,
        }
    }

    fn register_type(&mut self, type_data: TypeData) -> TypeId {
        // Searching linearly may potentially become quite expensive, but it
        // should be outweighed by index lookups quite heavily.
        match self.types.iter().position(|data| data == &type_data) {
            Some(index) => TypeId::new(index),
            None => {
                let id = TypeId::new(self.types.len());
                self.types.push(type_data);
                id
            }
        }
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => Some(*resolved_id),
            TypeReference::Import(_) => None,
            TypeReference::Unknown => Some(GLOBAL_UNKNOWN_ID),
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        if qualifier.path.len() == 1 {
            self.resolve_type_of(&qualifier.path[0])
                .or_else(|| GLOBAL_RESOLVER.resolve_qualifier(qualifier))
        } else {
            // TODO: Resolve nested qualifiers
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<ResolvedTypeId> {
        // We only care about the global scope, since that's where all exported
        // symbols reside.
        if let Some(binding_id) = self.scopes[0].bindings_by_name.get(identifier.text()) {
            let binding = &self.bindings[binding_id.index()];
            return if binding.declaration_kind.is_import_declaration() {
                Some(ResolvedTypeId::new(
                    TypeResolverLevel::Import,
                    (*binding_id).into(),
                ))
            } else {
                self.resolve_reference(&binding.ty)
            };
        }

        GLOBAL_RESOLVER.resolve_type_of(identifier)
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&*GLOBAL_RESOLVER)
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types
    }
}

#[derive(Default)]
struct JsModuleInfoCollectorFinaliser {
    /// Map of parsed declarations, for caching purposes.
    parsed_declarations: FxHashMap<JsSyntaxNode, Box<[(Text, TypeData)]>>,

    /// Map of parsed function parameters, for caching purposes.
    parsed_parameters: FxHashMap<JsSyntaxNode, FunctionParameter>,
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
        // Extract imports from collected import nodes.
        for import in &collector.imports {
            self.push_static_import(import.clone(), collector);
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
                let resolved_path = collector.static_import_paths.get(source.text())?;

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
                            symbol: ImportSymbol::Named(symbol_name.into()),
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
                        symbol: ImportSymbol::All,
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
}

impl JsModuleInfo {
    pub(super) fn new(mut collector: JsModuleInfoCollector) -> Self {
        let bag = JsModuleInfoBag::from_collector(&collector);

        collector.resolve_all_and_downgrade_project_references(&bag);
        collector.flatten_all();

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
            types: collector.types.into(),
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

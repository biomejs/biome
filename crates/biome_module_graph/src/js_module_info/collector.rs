use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use biome_js_semantic::{SemanticEvent, SemanticEventExtractor};
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsExportDefaultDeclaration, AnyJsExpression,
    AnyJsImportClause, JsFormalParameter, JsIdentifierBinding, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken, JsVariableDeclaration, TsIdentifierBinding, TsTypeParameter,
    TsTypeParameterName, inner_string_text,
};
use biome_js_type_info::{
    BindingId, FunctionParameter, GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, GenericTypeParameter, Module,
    Namespace, Resolvable, ResolvedTypeData, ResolvedTypeId, ScopeId, TypeData, TypeId,
    TypeImportQualifier, TypeMember, TypeMemberKind, TypeReference, TypeReferenceQualifier,
    TypeResolver, TypeResolverLevel, TypeStore,
};
use biome_jsdoc_comment::JsdocComment;
use biome_rowan::{AstNode, Text, TextRange, TextSize, TokenText};
use rust_lapper::{Interval, Lapper};
use rustc_hash::FxHashMap;

use crate::js_module_info::{
    binding::{JsBindingReference, JsBindingReferenceKind, JsDeclarationKind},
    scope::TsBindingReference,
    scope_id_for_range,
};

use super::{
    Exports, ImportSymbol, Imports, JsExport, JsImport, JsModuleInfo, JsModuleInfoInner,
    JsOwnExport, JsReexport, ResolvedPath, binding::JsBindingData, scope::JsScopeData,
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

    /// Expression nodes.
    ///
    /// They will get parsed when bindings from their scope is known.
    expressions: Vec<AnyJsExpression>,

    /// Formal parameters.
    ///
    /// They will get parsed when bindings from their scope is known.
    formal_parameters: Vec<JsFormalParameter>,

    /// Variable declarations.
    ///
    /// They will get parsed when bindings from their scope is known.
    variable_declarations: Vec<JsVariableDeclaration>,

    /// Map of parsed declarations, for caching purposes.
    parsed_declarations: FxHashMap<JsSyntaxNode, Box<[(Text, TypeReference)]>>,

    /// Map of parsed declarations, for caching purposes.
    parsed_expressions: FxHashMap<TextRange, TypeId>,

    /// Map of parsed function parameters, for caching purposes.
    parsed_parameters: FxHashMap<JsSyntaxNode, FunctionParameter>,

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

    /// All collected exports.
    ///
    /// When we've completed a pass over the module, we will attempt to resolve
    /// the references of these exports and construct the final exports.
    exports: Vec<JsCollectedExport>,

    /// All imports nodes.
    imports: Vec<biome_js_syntax::JsImport>,

    /// List of all blanket re-exports.
    blanket_reexports: Vec<JsReexport>,

    /// Types collected in the module.
    types: TypeStore,
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

            _ => {
                if let Some(expr) = AnyJsExpression::cast_ref(node) {
                    self.expressions.push(expr);
                } else if let Some(param) = JsFormalParameter::cast_ref(node) {
                    self.formal_parameters.push(param);
                } else if let Some(decl) = JsVariableDeclaration::cast_ref(node) {
                    self.variable_declarations.push(decl);
                } else if let Some(import) = biome_js_syntax::JsImport::cast_ref(node) {
                    self.imports.push(import)
                }
            }
        }

        self.extractor.enter(node);
    }

    pub fn leave_node(&mut self, node: &JsSyntaxNode) {
        self.extractor.leave(node);
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

    pub fn finalise(&mut self) -> Lapper<u32, ScopeId> {
        while let Some(event) = self.extractor.pop() {
            self.push_event(event);
        }

        let scope_by_range = Lapper::new(
            self.scope_range_by_start
                .iter()
                .flat_map(|(_, scopes)| scopes.iter())
                .cloned()
                .collect(),
        );

        self.infer_all_types(&scope_by_range);

        scope_by_range
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
                    ty: TypeReference::Unknown,
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

    fn infer_all_types(&mut self, scope_by_range: &Lapper<u32, ScopeId>) {
        let expressions = std::mem::take(&mut self.expressions);
        for expr in expressions {
            let range = expr.range();
            let scope_id = scope_id_for_range(scope_by_range, range);
            let ty = TypeData::from_any_js_expression(self, scope_id, &expr);
            let id = self.register_type(Cow::Owned(ty));

            self.parsed_expressions.insert(range, id);
        }

        let formal_parameters = std::mem::take(&mut self.formal_parameters);
        for param in formal_parameters {
            let range = param.range();
            let scope_id = scope_id_for_range(scope_by_range, range);
            let parsed_param = FunctionParameter::from_js_formal_parameter(self, scope_id, &param);
            self.parsed_parameters
                .insert(param.syntax().clone(), parsed_param);
        }

        let variable_declarations = std::mem::take(&mut self.variable_declarations);
        for decl in variable_declarations {
            let range = decl.range();
            let scope_id = scope_id_for_range(scope_by_range, range);
            let type_bindings =
                TypeData::typed_bindings_from_js_variable_declaration(self, scope_id, &decl);
            self.parsed_declarations
                .insert(decl.syntax().clone(), type_bindings);
        }

        for index in 0..self.bindings.len() {
            let binding_id = BindingId::new(index);
            let binding = &self.bindings[binding_id.index()];
            if let Some(node) = self.binding_node_by_start.get(&binding.range.start()) {
                let name = binding.name.clone();
                let scope_id = scope_id_for_range(scope_by_range, binding.range);
                let ty = self.infer_type(&node.clone(), &name, scope_id);
                self.bindings[binding_id.index()].ty = ty;
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
                    .and_then(|decl| self.parsed_declarations.get(decl.syntax()))
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
            } else if let Some(param) = JsFormalParameter::cast_ref(&ancestor)
                .and_then(|param| self.parsed_parameters.get(param.syntax()))
            {
                return param
                    .bindings
                    .iter()
                    .find_map(|binding| (binding.name == *binding_name).then(|| binding.ty.clone()))
                    .unwrap_or_default();
            } else if let Some(param) = TsTypeParameter::cast_ref(&ancestor) {
                return match GenericTypeParameter::from_ts_type_parameter(self, scope_id, &param) {
                    Some(generic) => self.reference_to_owned_data(TypeData::from(generic)),
                    None => TypeReference::Unknown,
                };
            }
        }

        TypeReference::Unknown
    }

    /// After the first pass of the collector, import references have been
    /// resolved to an import binding. But we can't store the information of the
    /// import target inside the `ResolvedTypeId`, because it resides in the
    /// module's semantic data, and `ResolvedTypeId` is only 8 bytes. So during
    /// resolving, we "downgrade" the import references from
    /// [`TypeReference::Resolved`] to [`TypeReference::Import`].
    fn resolve_all_and_downgrade_project_references(
        &mut self,
        static_imports: &BTreeMap<Text, JsImport>,
    ) {
        let bindings = self.bindings.clone(); // TODO: Can we omit the clone?
        let downgrade_import_reference = |id: BindingId| {
            let binding = &bindings[id.index()];
            static_imports
                .get(&binding.name)
                .map_or(TypeReference::Unknown, |import| {
                    TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: binding.declaration_kind.is_import_type_declaration(),
                    }
                    .into()
                })
        };

        // First do a pass in which we populate module and namespace members:
        let mut i = 0;
        while i < self.types.len() {
            // SAFETY: We immediately reinsert after taking.
            let ty = unsafe { self.types.take_from_index_temporarily(i) };
            let ty = match ty {
                TypeData::Module(module) => match self.find_binding_for_type_index(i) {
                    Some(module_binding) => TypeData::from(Module {
                        name: module.name,
                        // Populate module members:
                        members: self.find_type_members_in_scope(module_binding.scope_id),
                    }),
                    None => TypeData::Module(module),
                },
                TypeData::Namespace(namespace) => match self.find_binding_for_type_index(i) {
                    Some(namespace_binding) => TypeData::from(Namespace {
                        path: namespace.path,
                        // Populate namespace members:
                        members: self.find_type_members_in_scope(namespace_binding.scope_id),
                    }),
                    None => TypeData::Namespace(namespace),
                },
                ty => ty,
            };
            // SAFETY: We reinsert before anyone got a chance to do lookups.
            unsafe { self.types.reinsert_temporarily_taken_data(i, ty) };
            i += 1;
        }

        // Now perform a pass for the actual resolving:
        let mut i = 0;
        while i < self.types.len() {
            // SAFETY: We immediately reinsert after taking.
            let ty = unsafe { self.types.take_from_index_temporarily(i) };
            let ty = ty.resolved_with_mapped_references(
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
            // SAFETY: We reinsert before anyone got a chance to do lookups.
            unsafe { self.types.reinsert_temporarily_taken_data(i, ty) };
            i += 1;
        }
    }

    fn find_binding_for_type_index(&self, type_index: usize) -> Option<&JsBindingData> {
        self.bindings.iter().find(|binding| match binding.ty {
            TypeReference::Resolved(resolved) => {
                resolved.level() == TypeResolverLevel::Module && resolved.id().index() == type_index
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
                kind: TypeMemberKind::Named(binding.name.clone()),
                is_static: true,
                ty: binding.ty.clone(),
            })
            .collect()
    }

    fn flatten_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // SAFETY: We reinsert before anyone got a chance to do lookups.
            unsafe {
                let ty = self.types.take_from_index_temporarily(i);
                let ty = ty.flattened(self);
                self.types.reinsert_temporarily_taken_data(i, ty);
            }
            i += 1;
        }
    }
}

impl TypeResolver for JsModuleInfoCollector {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Module
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find_type(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData> {
        match id.level() {
            TypeResolverLevel::Module => Some((id, self.get_by_id(id.id())).into()),
            TypeResolverLevel::Global => Some((id, GLOBAL_RESOLVER.get_by_id(id.id())).into()),
            TypeResolverLevel::Scope | TypeResolverLevel::Import => None,
        }
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.register_type(type_data)
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
        let identifier = qualifier.path.first()?;
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
        for identifier in &qualifier.path[1..] {
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

    fn resolve_expression(&mut self, _scope_id: ScopeId, expr: &AnyJsExpression) -> Cow<TypeData> {
        match self.parsed_expressions.get(&expr.range()) {
            Some(id) => Cow::Borrowed(self.get_by_id(*id)),
            None => Cow::Owned(TypeData::unknown()),
        }
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&*GLOBAL_RESOLVER)
    }

    fn registered_types(&self) -> &[TypeData] {
        self.types.as_slice()
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
    pub(super) fn from_collector(collector: &mut JsModuleInfoCollector) -> Self {
        let mut info = Self::default();
        info.collect_imports(collector);

        collector.resolve_all_and_downgrade_project_references(&info.static_imports);
        collector.flatten_all();

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

    fn collect_exports(&mut self, collector: &mut JsModuleInfoCollector) {
        self.blanket_reexports = std::mem::take(&mut collector.blanket_reexports);

        let exports = std::mem::take(&mut collector.exports);
        for export in exports {
            match export {
                JsCollectedExport::ExportNamedSymbol {
                    export_name,
                    local_name,
                } => {
                    let Some(binding_ref) = collector.scopes[0].bindings_by_name.get(&local_name)
                    else {
                        continue;
                    };

                    let export = match binding_ref {
                        TsBindingReference::Merged {
                            ty,
                            value_ty,
                            namespace_ty,
                        } => {
                            let ty = ty.map(|ty| &collector.bindings[ty.index()].ty);
                            let value_ty = value_ty.map(|ty| &collector.bindings[ty.index()].ty);
                            let namespace_ty =
                                namespace_ty.map(|ty| &collector.bindings[ty.index()].ty);
                            match (ty, value_ty, namespace_ty) {
                                (Some(ty1), Some(ty2), None)
                                | (Some(ty1), None, Some(ty2))
                                | (None, Some(ty1), Some(ty2))
                                    if ty1 == ty2 =>
                                {
                                    let ty = collector
                                        .register_and_resolve(TypeData::reference(ty1.clone()));
                                    JsOwnExport::Type(ty)
                                }
                                (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty2 == ty3 => {
                                    let ty = collector
                                        .register_and_resolve(TypeData::reference(ty1.clone()));
                                    JsOwnExport::Type(ty)
                                }
                                _ => {
                                    let ty =
                                        collector.register_and_resolve(TypeData::merged_reference(
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

                    self.exports.insert(export_name, JsExport::Own(export));
                }
                JsCollectedExport::ExportDefault { ty } => {
                    let resolved = collector
                        .resolve_reference(&ty)
                        .unwrap_or(GLOBAL_UNKNOWN_ID);

                    let export = JsExport::Own(JsOwnExport::Type(resolved));
                    self.exports.insert(Text::Static("default"), export);
                }
                JsCollectedExport::ExportDefaultAssignment { ty } => {
                    let resolved = collector
                        .resolve_reference(&ty)
                        .unwrap_or(GLOBAL_UNKNOWN_ID);

                    if let Some(data) = collector.get_by_resolved_id(resolved) {
                        for member in data.as_raw_data().own_members() {
                            let Some(name) = member.name() else {
                                continue;
                            };

                            // DANGER: Normally, when resolving a type reference retrieved through
                            //         `as_raw_data()`, we should call
                            //         `apply_module_id_to_reference()` on the reference first. But
                            //         because we know we are resolving inside the collector, before
                            //         any module IDs _could_ be applied, we can omit this here.
                            if let Some(resolved_member) = collector.resolve_reference(&member.ty) {
                                let export = JsExport::Own(JsOwnExport::Type(resolved_member));
                                self.exports.insert(name, export);
                            }
                        }
                    }

                    let export = JsExport::Own(JsOwnExport::Type(resolved));
                    self.exports.insert(Text::Static("default"), export);
                }
                JsCollectedExport::Reexport {
                    export_name,
                    reexport,
                } => {
                    self.exports
                        .insert(export_name, JsExport::Reexport(reexport));
                }
            }
        }
    }
}

impl JsModuleInfo {
    pub(super) fn new(
        mut collector: JsModuleInfoCollector,
        scope_by_range: Lapper<u32, ScopeId>,
    ) -> Self {
        let bag = JsModuleInfoBag::from_collector(&mut collector);

        Self(Arc::new(JsModuleInfoInner {
            static_imports: Imports(bag.static_imports),
            static_import_paths: collector.static_import_paths,
            dynamic_import_paths: collector.dynamic_import_paths,
            exports: Exports(bag.exports),
            blanket_reexports: bag.blanket_reexports.into(),
            bindings: collector.bindings.into(),
            expressions: collector.parsed_expressions,
            scopes: collector.scopes.into(),
            scope_by_range,
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

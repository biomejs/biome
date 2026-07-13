use crate::css_module_info::CssClassReference;
use std::{borrow::Cow, sync::Arc};

use biome_js_semantic::{Reference, ScopeId, SemanticModel, TsBindingReference};
use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsDeclaration, AnyJsExportDefaultDeclaration, AnyJsExpression,
    AnyJsImportClause, JsAssignmentExpression, JsForVariableDeclaration, JsFormalParameter,
    JsRestParameter, JsSyntaxNode, JsVariableDeclaration, TsTypeParameter, inner_string_text,
};
use biome_js_type_info::{
    FunctionParameter, GenericTypeParameter, RawTypeCollector, RawTypeData, RawTypeId, TypeData,
    TypeId, TypeImportQualifier, TypeMember, TypeMemberKind, TypeReference, TypeStore,
    UnionCollector,
};
use biome_rowan::{AstNode, Text, TextRange, TokenText};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use super::{
    Exports, ImportSymbol, Imports, JsExport, JsImport, JsModuleInfo, JsModuleInfoDiagnostic,
    JsModuleInfoInner, JsOwnExport, JsReexport, ResolvedPath, binding::JsBindingData,
};
use crate::{JsImportPath, JsImportPhase};

/// Responsible for collecting all the information from which to build the
/// [`JsModuleInfo`].
///
/// This collects a lot of fields with raw information, which then goes through
/// another round of processing as we create the intermediate
/// [`JsModuleInfoBag`], and finally the [`JsModuleInfo`] itself.
pub(super) struct JsModuleInfoCollector {
    semantic_model: Arc<SemanticModel>,

    pub(super) bindings: Vec<JsBindingData>,

    /// Function parameters, both formal parameters as well as rest parameters.
    function_parameters: FxHashMap<JsSyntaxNode, FunctionParameter>,

    /// Variable declarations.
    variable_declarations: FxHashMap<JsSyntaxNode, Box<[(Text, TypeReference)]>>,

    /// Map of parsed declarations, for caching purposes.
    parsed_expressions: FxHashMap<TextRange, TypeId>,

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

    /// How much type inference work to perform when finalizing the module info
    inference_mode: TypeInferenceMode,

    /// CSS class references from JSX `className` or `class` attributes
    /// (static string literals only).
    pub(super) referenced_classes: Vec<CssClassReference>,
}

/// Intermediary representation for an exported symbol.
#[derive(Clone)]
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
    pub(super) fn new(semantic_model: Arc<SemanticModel>) -> Self {
        let mut bindings = Vec::new();

        for binding in semantic_model.all_bindings() {
            let name = binding
                .tree()
                .name_token()
                .ok()
                .map(|t| t.token_text_trimmed().into())
                .unwrap_or_default();
            let range = binding.syntax().text_trimmed_range();

            bindings.push(JsBindingData {
                name,
                scope_id: binding.scope().id(),
                declaration_kind: binding.declaration_kind(),
                ty: TypeReference::unknown(),
                range,
            });
        }

        Self {
            semantic_model,
            bindings,
            function_parameters: FxHashMap::default(),
            variable_declarations: FxHashMap::default(),
            parsed_expressions: FxHashMap::default(),
            static_import_paths: IndexMap::new(),
            dynamic_import_paths: IndexMap::new(),
            exports: Vec::new(),
            blanket_reexports: Vec::new(),
            types: TypeStore::default(),
            static_imports: IndexMap::new(),
            diagnostics: Vec::new(),
            inference_mode: TypeInferenceMode::Disabled,
            referenced_classes: Vec::new(),
        }
    }

    pub fn leave_node(&mut self, node: &JsSyntaxNode) {
        if let Some(expr) = AnyJsExpression::cast_ref(node) {
            let range = expr.range();
            let scope_id = self.semantic_model.scope(node).id();
            let ty = TypeData::from_any_js_expression(self, scope_id, &expr);
            let id = self.register_type(Cow::Owned(ty));
            self.parsed_expressions.insert(range, id);
        } else if let Some(decl) = JsForVariableDeclaration::cast_ref(node) {
            let scope_id = self.semantic_model.scope(node).id();
            let type_bindings =
                TypeData::typed_bindings_from_js_for_statement(self, scope_id, &decl)
                    .unwrap_or_default();
            self.variable_declarations
                .insert(decl.syntax().clone(), type_bindings);
        } else if let Some(param) = JsFormalParameter::cast_ref(node) {
            let scope_id = self.semantic_model.scope(node).id();
            let parsed_param = FunctionParameter::from_js_formal_parameter(self, scope_id, &param);
            self.function_parameters
                .insert(param.syntax().clone(), parsed_param);
        } else if let Some(param) = JsRestParameter::cast_ref(node) {
            let scope_id = self.semantic_model.scope(node).id();
            let parsed_param = FunctionParameter::from_js_rest_parameter(self, scope_id, &param);
            self.function_parameters
                .insert(param.syntax().clone(), parsed_param);
        } else if let Some(decl) = JsVariableDeclaration::cast_ref(node) {
            let scope_id = self.semantic_model.scope(node).id();
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

    pub fn register_default_export_declaration(
        &mut self,
        declaration: &AnyJsExportDefaultDeclaration,
    ) {
        let type_data =
            TypeData::from_any_js_export_default_declaration(self, ScopeId::GLOBAL, declaration);
        let ty = TypeReference::from(self.register_and_resolve(type_data));
        self.register_export(JsCollectedExport::ExportDefault { ty });
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

    fn finalise(&mut self, semantic_model: &SemanticModel) -> FinalisedModuleTypes {
        if self.inference_mode != TypeInferenceMode::Disabled {
            self.infer_all_types(semantic_model);
        }

        let exports = self.collect_exports_from(self.exports.clone());

        let (raw_types, raw_expressions, raw_binding_types) =
            if self.inference_mode == TypeInferenceMode::Disabled {
                (Vec::new(), FxHashMap::default(), FxHashMap::default())
            } else {
                (
                    self.take_raw_types(),
                    self.raw_expressions(),
                    self.raw_binding_types(),
                )
            };

        FinalisedModuleTypes {
            exports,
            raw_types,
            raw_expressions,
            raw_binding_types,
        }
    }

    fn take_raw_types(&mut self) -> Vec<RawTypeData> {
        std::mem::take(&mut self.types).into()
    }

    fn raw_expressions(&self) -> FxHashMap<TextRange, TypeReference> {
        self.parsed_expressions
            .iter()
            .map(|(range, id)| (*range, TypeReference::Resolved(RawTypeId::Local(*id))))
            .collect()
    }

    fn raw_binding_types(&self) -> FxHashMap<TextRange, TypeReference> {
        self.bindings
            .iter()
            .map(|binding| (binding.range, binding.ty.clone()))
            .collect()
    }

    fn infer_all_types(&mut self, semantic_model: &biome_js_semantic::SemanticModel) {
        for index in 0..self.bindings.len() {
            let binding = &self.bindings[index];
            if let Some(node) = semantic_model
                .as_binding_by_range(binding.range)
                .map(|b| b.syntax().clone())
            {
                let scope_id = semantic_model.scope_for_range(binding.range).id();
                let ty = self.infer_type(&node, binding.clone(), scope_id, semantic_model);
                self.bindings[index].ty = ty;
            }
        }

        // A set of same-name function overloads becomes an object with one call
        // signature per declaration, placed on the binding that name resolution
        // returns for the set (its last one) so a call site can select among them.
        let carriers: Vec<(usize, Vec<TypeMember>)> = semantic_model
            .scopes()
            .flat_map(|scope| scope.overload_sets())
            .map(|set| {
                let signatures = set
                    .iter()
                    .map(|id| TypeMember {
                        kind: TypeMemberKind::CallSignature,
                        ty: self.bindings[id.index()].ty.clone(),
                    })
                    .collect();
                let representative = set.last().expect("overload set has 2+ entries").index();
                (representative, signatures)
            })
            .collect();
        for (representative, signatures) in carriers {
            let ty = self.reference_to_owned_data(TypeData::object_with_members(signatures.into()));
            self.bindings[representative].ty = ty;
        }
    }

    fn has_writable_reference(&self, semantic_model: &SemanticModel, range: TextRange) -> bool {
        semantic_model
            .as_binding_by_range(range)
            .is_some_and(|binding| binding.all_writes().next().is_some())
    }

    fn get_writable_references(
        &self,
        semantic_model: &SemanticModel,
        range: TextRange,
    ) -> Vec<Reference> {
        semantic_model
            .as_binding_by_range(range)
            .map(|binding| binding.all_writes().collect())
            .unwrap_or_default()
    }

    fn infer_type(
        &mut self,
        node: &JsSyntaxNode,
        binding: JsBindingData,
        scope_id: ScopeId,
        semantic_model: &SemanticModel,
    ) -> TypeReference {
        let binding_name = &binding.name.clone();

        // If this binding is an import, create a TypeReference::Import directly
        if binding.declaration_kind.is_import_declaration()
            && let Some(import) = self.static_imports.get(binding_name)
        {
            return TypeReference::from(TypeImportQualifier {
                symbol: import.symbol.clone(),
                resolved_path: import.resolved_path.clone(),
                type_only: binding.declaration_kind.is_import_type_declaration(),
            });
        }

        for ancestor in node.ancestors() {
            if let Some(decl) = AnyJsDeclaration::cast_ref(&ancestor) {
                let ty = if let Some(typed_bindings) = decl
                    .as_js_variable_declaration()
                    .and_then(|decl| self.variable_declarations.get(decl.syntax()))
                {
                    let ty = typed_bindings
                        .iter()
                        .find_map(|(name, ty)| (name == binding_name).then(|| ty.clone()))
                        .unwrap_or_default();

                    if self.has_writable_reference(semantic_model, binding.range) {
                        self.widen_binding_from_writable_references(
                            scope_id,
                            &binding,
                            &ty,
                            semantic_model,
                        )
                    } else {
                        ty
                    }
                } else {
                    let data = TypeData::from_any_js_declaration(self, scope_id, &decl);
                    self.reference_to_owned_data(data)
                };

                return ty;
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

    /// Widen the type of binding from its writable references.
    fn widen_binding_from_writable_references(
        &mut self,
        scope_id: ScopeId,
        binding: &JsBindingData,
        ty: &TypeReference,
        semantic_model: &SemanticModel,
    ) -> TypeReference {
        let references = self.get_writable_references(semantic_model, binding.range);
        let mut union_collector = UnionCollector::new();
        union_collector.add(ty.clone());
        let mut saw_widening_reference = false;
        for reference in references {
            let node = reference.syntax();
            let reference_scope = reference.scope().id();

            // We don't want to widen types inside the same scope
            if binding.scope_id == reference_scope {
                continue;
            }
            let assignment = node
                .ancestors()
                .skip(1)
                .find_map(|ancestor| JsAssignmentExpression::cast_ref(&ancestor))
                .and_then(|assignment| assignment.right().ok());
            if let Some(right) = assignment {
                let data = TypeData::from_any_js_expression(self, scope_id, &right);
                let assigned_type = self.reference_to_owned_data(data);
                union_collector.add(assigned_type);
                saw_widening_reference = true;
            }
        }

        if !saw_widening_reference {
            return ty.clone();
        }

        let id = self.register_type(union_collector.finish());
        RawTypeId::Local(id).into()
    }

    fn find_binding_in_scope(&self, name: &str, scope_id: ScopeId) -> Option<TsBindingReference> {
        let mut scope = self.semantic_model.scope_from_id(scope_id);
        loop {
            if let Some(binding_ref) = scope.get_binding_reference(name) {
                return Some(binding_ref);
            }
            match scope.parent() {
                Some(parent) => scope = parent,
                None => break,
            }
        }
        None
    }

    /// Given a binding name and scope, looks up the binding and, if it is a
    /// namespace or module declaration, inserts all direct child-scope bindings
    /// into `exports`.
    fn collect_namespace_exports_for_binding(
        &self,
        name: &str,
        scope_id: ScopeId,
        exports: &mut IndexMap<Text, JsExport>,
    ) {
        let Some(binding_ref) = self.find_binding_in_scope(name, scope_id) else {
            return;
        };

        let binding_id = binding_ref.value_ty_or_ty();
        let binding = &self.bindings[binding_id.index()];

        if !binding.declaration_kind.declares_namespace() {
            return;
        }

        // Collect bindings from immediate child scopes of the namespace
        // binding's scope.
        for child_binding in &self.bindings {
            if child_binding.name.is_empty() {
                continue;
            }

            let child_scope = &self.semantic_model.scope_from_id(child_binding.scope_id);
            if child_scope
                .parent()
                .is_some_and(|parent| parent.id() == binding.scope_id)
            {
                exports
                    .entry(child_binding.name.clone())
                    .or_insert_with(|| JsExport::Own(JsOwnExport::Binding(child_binding.range)));
            }
        }
    }

    fn collect_exports_from(
        &mut self,
        exports: Vec<JsCollectedExport>,
    ) -> IndexMap<Text, JsExport> {
        let mut finalised_exports = IndexMap::new();

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
                    let resolved = self.raw_resolved_id(&ty);

                    let export = JsExport::Own(JsOwnExport::Type(resolved));
                    finalised_exports.insert(Text::new_static("default"), export);
                }
                JsCollectedExport::ExportDefaultAssignment { ty } => {
                    let resolved = self.raw_resolved_id(&ty);
                    let mut found_members = false;

                    if let Some(data) = self.raw_type_by_resolved_id(resolved) {
                        for member in data.own_members() {
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
                            if let TypeReference::Resolved(RawTypeId::Local(resolved_member)) =
                                member.ty
                            {
                                let export = JsExport::Own(JsOwnExport::Type(resolved_member));
                                finalised_exports.insert(name, export);
                                found_members = true;
                            }
                        }
                    }

                    // If the resolved type has no members (e.g., the
                    // reference is still an unresolved qualifier), fall back
                    // to the scope tree to collect namespace bindings directly.
                    if !found_members
                        && let Some(data) = self.raw_type_by_resolved_id(resolved)
                        && let TypeData::Reference(TypeReference::Qualifier(qualifier)) = data
                        && let Some(first_part) = qualifier.path.iter().next()
                    {
                        self.collect_namespace_exports_for_binding(
                            first_part,
                            qualifier.scope_id,
                            &mut finalised_exports,
                        );
                    }

                    let export = JsExport::Own(JsOwnExport::Type(resolved));
                    finalised_exports.insert(Text::new_static("default"), export);
                }
                JsCollectedExport::Reexport {
                    export_name,
                    reexport,
                } => {
                    // `export * as Foo from "..."` creates a namespace export:
                    // `Foo` is an own symbol of this module (a namespace object),
                    // not a forwarded individual symbol from the target.
                    // We store the full `JsReexport` so that the JSDoc comment
                    // and resolved target path are preserved for documentation
                    // tooling and type inference.
                    if reexport.import.symbol == ImportSymbol::All {
                        finalised_exports
                            .insert(export_name, JsExport::Own(JsOwnExport::Namespace(reexport)));
                    } else {
                        finalised_exports.insert(export_name, JsExport::Reexport(reexport));
                    }
                }
            }
        }

        finalised_exports
    }

    fn get_export_for_local_name(&mut self, local_name: TokenText) -> Option<JsOwnExport> {
        let binding_ref = self
            .semantic_model
            .global_scope()
            .get_binding_reference(&local_name)?;

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
                        JsOwnExport::Type(self.raw_id_to_local(ty))
                    }
                    (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty2 == ty3 => {
                        let ty = self.register_and_resolve(TypeData::reference(ty1.clone()));
                        JsOwnExport::Type(self.raw_id_to_local(ty))
                    }
                    _ => {
                        let ty = self.register_and_resolve(TypeData::merged_reference(
                            ty.cloned(),
                            value_ty.cloned(),
                            namespace_ty.cloned(),
                        ));
                        JsOwnExport::Type(self.raw_id_to_local(ty))
                    }
                }
            }
            TsBindingReference::Type(binding_id)
            | TsBindingReference::ValueType(binding_id)
            | TsBindingReference::TypeAndValueType(binding_id)
            | TsBindingReference::NamespaceAndValueType(binding_id) => {
                // Get the binding range instead of storing the BindingId
                let binding_range = self.bindings[binding_id.index()].range;
                JsOwnExport::Binding(binding_range)
            }
        };

        Some(export)
    }

    fn raw_resolved_id(&mut self, ty: &TypeReference) -> TypeId {
        match ty {
            TypeReference::Resolved(RawTypeId::Local(id)) => *id,
            _ => self.register_type(Cow::Owned(TypeData::reference(ty.clone()))),
        }
    }

    fn raw_id_to_local(&mut self, id: RawTypeId) -> TypeId {
        match id {
            RawTypeId::Local(id) => id,
            RawTypeId::Global(id) => self.register_type(Cow::Owned(TypeData::reference(
                TypeReference::Resolved(RawTypeId::Global(id)),
            ))),
        }
    }

    fn raw_type_by_resolved_id(&self, id: TypeId) -> Option<&TypeData> {
        self.types.as_slice().get(id.index())
    }
}

impl RawTypeCollector for JsModuleInfoCollector {
    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.insert_cow(type_data)
    }

    fn resolve_expression(
        &mut self,
        _scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Cow<'_, TypeData> {
        match self.parsed_expressions.get(&expr.range()) {
            Some(id) => Cow::Borrowed(self.get_by_id(*id)),
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
            .map(|id| TypeReference::Resolved(RawTypeId::Local(*id)))
            .unwrap_or_default()
    }
}

/// Selects how much type inference work the collector performs while
/// finalizing a module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeInferenceMode {
    /// No type inference at all.
    Disabled,
    /// Collects the raw type table for the salsa-backed inference engine,
    /// without running the legacy resolution and flattening passes.
    RawTypesOnly,
}

impl JsModuleInfo {
    pub(super) fn new(
        mut collector: JsModuleInfoCollector,
        semantic_model: std::sync::Arc<biome_js_semantic::SemanticModel>,
        inference_mode: TypeInferenceMode,
    ) -> Self {
        collector.inference_mode = inference_mode;
        let finalised = collector.finalise(&semantic_model);

        Self(Arc::new(JsModuleInfoInner {
            static_imports: Imports(collector.static_imports),
            static_import_paths: collector.static_import_paths,
            dynamic_import_paths: collector.dynamic_import_paths,
            exports: Exports(finalised.exports),
            blanket_reexports: collector.blanket_reexports,
            semantic_model,
            raw_types: finalised.raw_types,
            raw_expressions: finalised.raw_expressions,
            raw_binding_types: finalised.raw_binding_types,
            diagnostics: collector.diagnostics.into_iter().map(Into::into).collect(),
            infer_types: collector.inference_mode != TypeInferenceMode::Disabled,
            referenced_classes: collector.referenced_classes,
        }))
    }
}

struct FinalisedModuleTypes {
    exports: IndexMap<Text, JsExport>,
    raw_types: Vec<RawTypeData>,
    raw_expressions: FxHashMap<TextRange, TypeReference>,
    raw_binding_types: FxHashMap<TextRange, TypeReference>,
}

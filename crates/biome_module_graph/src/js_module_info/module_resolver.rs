use std::{borrow::Cow, collections::hash_map::Entry, ops::Deref, sync::Arc};

use biome_js_syntax::AnyJsExpression;
use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, MAX_FLATTEN_DEPTH, ModuleId, Resolvable,
    ResolvedTypeData, ResolvedTypeId, ResolverId, ScopeId, Type, TypeData, TypeId,
    TypeImportQualifier, TypeReference, TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
    TypeStore,
};
use biome_resolver::ResolvedPath;
use biome_rowan::{AstNode, RawSyntaxKind, Text, TextRange, TokenText};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    JsExport, JsImportPath, JsOwnExport, ModuleGraph,
    js_module_info::{JsModuleInfoInner, scope::TsBindingReference, utils::reached_too_many_types},
};

use super::{JsModuleInfo, JsModuleInfoDiagnostic};

const MAX_IMPORT_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

// Any references that resolve to "module 0" will be rewritten to reference the
// module resolver. The reason we do this is that any references to other
// modules can be resolved within the module resolver, whereas references that
// still point to the module's own types would remain unresolved.
const MODULE_0_ID: ResolverId = ResolverId::from_level(TypeResolverLevel::Thin);

/// Type resolver that is able to resolve types _across_ modules.
///
/// This resolver is instantiated for a given module using
/// [`Self::for_module()`]. Besides the module info, it also receives a
/// reference to the [`ModuleGraph`] so that other modules can be resolved.
///
/// The module for which this resolver is created is referred to as "module 0",
/// because it is stored at the first index of the resolver's [`Self::modules`]
/// vector. In other words, "module 0" is the module from which all inference
/// in this resolver starts.
///
/// When [`Self::run_inference()`] is called, the modules imported by module 0
/// are retrieved from the module graph, and types are inferred across `import`
/// statements.
///
/// The module resolver is typically consumed through the `Typed` service.
pub struct ModuleResolver {
    module_graph: Arc<ModuleGraph>,

    /// Modules from which this resolver is using types.
    ///
    /// "Module 0" derives its name from being stored at the first index of this
    /// vector. Other modules are those imported by module 0, either directly
    /// or transitively.
    modules: Vec<JsModuleInfo>,

    /// Map of module IDs keyed by their resolved paths.
    ///
    /// Module IDs are used to index the [`Self::modules`] vector.
    pub modules_by_path: FxHashMap<ResolvedPath, ModuleId>,

    /// Parsed expressions, mapped by their starting index to the ID of their
    /// type.
    expressions: FxHashMap<TextRange, ResolvedTypeId>,

    /// Types registered within this resolver.
    types: TypeStore,

    /// Maps from `TypeId` indices in module 0 to indices in our own `types`
    /// store.
    type_id_map: Vec<TypeId>,

    /// Diagnostics emitted during the resolution of types
    diagnostics: Vec<JsModuleInfoDiagnostic>,
}

impl ModuleResolver {
    pub fn for_module(module_info: JsModuleInfo, module_graph: Arc<ModuleGraph>) -> Self {
        let num_initial_types = module_info.types.len();

        let mut resolver = Self {
            module_graph,
            modules: vec![module_info],
            modules_by_path: Default::default(),
            expressions: Default::default(),
            types: TypeStore::with_capacity(num_initial_types),
            type_id_map: Default::default(),
            diagnostics: Default::default(),
        };

        resolver.run_inference();
        resolver
    }

    /// Runs the resolver's inference.
    ///
    /// This gets called once on construction of the resolver.
    fn run_inference(&mut self) {
        self.resolve_imports_in_modules();
        self.resolve_all();
        self.flatten_all();
    }

    /// Creates a [`Type`] instance for the given `resolved_id`.
    ///
    /// If `resolved_id` points to a reference, this automatically dereferences
    /// the type.
    #[inline]
    pub fn resolved_type_for_id(self: &Arc<Self>, resolved_id: ResolvedTypeId) -> Type {
        let ty = Type::from_id(self.clone(), resolved_id);
        match ty.deref() {
            TypeData::Reference(reference) => self.resolved_type_for_reference(reference),
            _ => ty,
        }
    }

    /// Returns the resolved type for the given `reference`.
    #[inline]
    pub fn resolved_type_for_reference(self: &Arc<Self>, reference: &TypeReference) -> Type {
        match reference {
            TypeReference::Resolved(resolved_id) => self.resolved_type_for_id(*resolved_id),
            _ => Type::default(),
        }
    }

    /// Returns the type of "module 0"'s default export, if any.
    pub fn resolved_type_of_default_export(self: &Arc<Self>) -> Option<Type> {
        let module = &self.modules[0];
        module
            .exports
            .get("default")
            .and_then(JsExport::as_own_export)
            .map(|own_export| match own_export {
                JsOwnExport::Binding(binding_id) => {
                    self.resolved_type_for_reference(&module.bindings[binding_id.index()].ty)
                }
                JsOwnExport::Type(resolved_id) => self.resolved_type_for_id(*resolved_id),
            })
    }

    /// Returns the resolved type of the given expression within this module.
    pub fn resolved_type_of_expression(self: &Arc<Self>, expr: &AnyJsExpression) -> Type {
        self.resolved_type_for_id(self.resolved_id_for_expression(expr))
    }

    /// Returns the resolved type of the value with the given `name`, as
    /// defined in the scope of the given `range`.
    pub fn resolved_type_of_named_value(self: &Arc<Self>, range: TextRange, name: &str) -> Type {
        let module = &self.modules[0];
        let scope = module.scope_for_range(range);
        let Some(resolved_id) = module
            .find_binding_in_scope(name, scope.id)
            .and_then(TsBindingReference::value_ty)
            .and_then(|binding_id| match &module.binding(binding_id).ty {
                TypeReference::Resolved(resolved_id) => Some(*resolved_id),
                _ => None,
            })
        else {
            return self.resolved_type_for_id(GLOBAL_UNKNOWN_ID);
        };

        self.resolved_type_for_id(self.mapped_resolved_id(resolved_id))
    }

    fn find_module(&self, path: &ResolvedPath) -> Option<ModuleId> {
        self.modules_by_path.get(path).copied()
    }

    fn register_module(&mut self, path: ResolvedPath) -> Option<ModuleId> {
        match self.modules_by_path.entry(path) {
            Entry::Occupied(entry) => Some(*entry.get()),
            Entry::Vacant(entry) => {
                let path = entry.key().as_path()?;
                let module_info = self.module_graph.module_info_for_path(path)?;
                let module_id = ModuleId::new(self.modules.len());
                self.modules.push(module_info);
                self.types
                    .insert_cow(Cow::Owned(TypeData::ImportNamespace(module_id)));
                Some(*entry.insert(module_id))
            }
        }
    }

    /// Actively resolves imports in the modules we are aware of.
    ///
    /// This process starts from "module 0" and works recursively until all the
    /// paths from static `import` statements are resolved.
    fn resolve_imports_in_modules(&mut self) {
        let mut i = 0;
        while i < self.modules.len() {
            let module = self.modules[i].clone();
            for JsImportPath { resolved_path, .. } in module.static_import_paths.values() {
                self.register_module(resolved_path.clone());
            }

            i += 1;
        }
    }

    fn resolve_all(&mut self) {
        let module = self.modules[0].clone();
        self.type_id_map = module
            .types
            .iter()
            .map(|ty| match ty.resolved(self) {
                Some(ty) => self.types.insert_cow(Cow::Owned(ty)),
                None => self.types.insert_arc(ty),
            })
            .collect();

        for (range, resolved_id) in &module.expressions {
            self.expressions
                .insert(*range, self.mapped_resolved_id(*resolved_id));
        }
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

    fn resolved_id_for_expression(&self, expr: &AnyJsExpression) -> ResolvedTypeId {
        self.expressions
            .get(&expr.range())
            .copied()
            .unwrap_or(GLOBAL_UNKNOWN_ID)
    }

    fn resolve_import_internal<'a>(
        &'a self,
        mut module_id: ModuleId,
        mut symbol: Cow<'a, ImportSymbol>,
        mut type_only: bool,
        mut seen_modules: FxHashSet<ModuleId>,
    ) -> Option<ResolvedTypeId> {
        while seen_modules.len() < MAX_IMPORT_DEPTH {
            if !seen_modules.insert(module_id) {
                return None;
            }

            let module = &self.modules[module_id.index()];

            let name = match symbol.as_ref() {
                ImportSymbol::Default => "default",
                ImportSymbol::Named(name) => name.text(),
                ImportSymbol::All => {
                    // Create a `ResolvedTypeId` that resolves to a
                    // `TypeData::ImportNamespace` variant.
                    return Some(ResolvedTypeId::new(
                        TypeResolverLevel::Import,
                        TypeId::new(module_id.index()),
                    ));
                }
            };

            let export = match module.exports.get(name) {
                Some(JsExport::Own(export) | JsExport::OwnType(export)) => export,
                Some(JsExport::Reexport(reexport)) => {
                    module_id = self.find_module(&reexport.import.resolved_path)?;
                    symbol = Cow::Borrowed(&reexport.import.symbol);
                    continue;
                }
                Some(JsExport::ReexportType(reexport)) => {
                    module_id = self.find_module(&reexport.import.resolved_path)?;
                    symbol = Cow::Borrowed(&reexport.import.symbol);
                    type_only = true;
                    continue;
                }
                None => {
                    return module
                        .blanket_reexports
                        .iter()
                        .filter_map(|reexport| self.find_module(&reexport.import.resolved_path))
                        .find_map(|module_id| {
                            self.resolve_import_internal(
                                module_id,
                                symbol.clone(),
                                type_only,
                                seen_modules.clone(),
                            )
                        });
                }
            };

            match resolve_from_export(module_id, module, export) {
                ResolveFromExportResult::Resolved(resolved) => return resolved,
                ResolveFromExportResult::FollowImport(import) => {
                    module_id = self.find_module(&import.resolved_path)?;
                    symbol = Cow::Owned(import.symbol.clone());
                    type_only = if import.type_only { true } else { type_only };
                }
            }
        }

        None
    }
}

impl TypeResolver for ModuleResolver {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Full
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
        match id.level() {
            TypeResolverLevel::Full => Some(ResolvedTypeData::from((id, self.get_by_id(id.id())))),
            TypeResolverLevel::Thin => {
                let module_id = id.module_id();
                let module = &self.modules[module_id.index()];
                if let Some(ty) = module.types.get(id.index()) {
                    Some(ResolvedTypeData::from((id, ty.as_ref())))
                } else {
                    debug_assert!(false, "Invalid type reference: {id:?}");
                    None
                }
            }
            TypeResolverLevel::Import => self
                .find_type(&TypeData::ImportNamespace(ModuleId::new(id.index())))
                .map(|type_id| ResolvedTypeData::from((id, self.get_by_id(type_id)))),
            TypeResolverLevel::Global => Some((id, GLOBAL_RESOLVER.get_by_id(id.id())).into()),
        }
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.insert_cow(type_data)
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(_qualifier) => None,
            TypeReference::Resolved(resolved_id) => Some(self.mapped_resolved_id(*resolved_id)),
            TypeReference::Import(import) => self.resolve_import(import),
        }
    }

    fn resolve_import(&self, qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        let module_id = self.find_module(&qualifier.resolved_path)?;
        self.resolve_import_internal(
            module_id,
            Cow::Borrowed(&qualifier.symbol),
            qualifier.type_only,
            FxHashSet::default(),
        )
    }

    fn resolve_import_namespace_member(
        &self,
        module_id: ModuleId,
        name: &str,
    ) -> Option<ResolvedTypeId> {
        self.resolve_import_internal(
            module_id,
            Cow::Owned(ImportSymbol::Named(
                TokenText::new_raw(RawSyntaxKind(0), name).into(),
            )),
            false,
            FxHashSet::default(),
        )
    }

    fn resolve_qualifier(&self, _qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        // We rely on qualifiers to have been resolved during the construction
        // of the module graph.
        None
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        let module = &self.modules[0];
        let Some(binding_ref) = module.find_binding_in_scope(identifier, scope_id) else {
            return GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id);
        };

        let binding = module.binding(binding_ref.value_ty_or_ty());
        if binding.declaration_kind.is_import_declaration() {
            module.static_imports.get(&binding.name).and_then(|import| {
                self.resolve_import(&TypeImportQualifier {
                    symbol: import.symbol.clone(),
                    resolved_path: import.resolved_path.clone(),
                    type_only: binding.declaration_kind.is_import_type_declaration(),
                })
            })
        } else {
            self.resolve_reference(&binding.ty)
        }
    }

    fn resolve_expression(
        &mut self,
        _scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Cow<'_, TypeData> {
        let id = self.resolved_id_for_expression(expr);
        match self.get_by_resolved_id(id) {
            Some(resolved) => Cow::Owned(resolved.to_data()),
            None => Cow::Owned(TypeData::unknown()),
        }
    }

    fn reference_to_resolved_expression(
        &mut self,
        _scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> TypeReference {
        self.resolved_id_for_expression(expression).into()
    }

    /// Maps the given `resolved_id` such that if it references a type belonging
    /// to "module 0" (the module for which this resolver was created) it will
    /// be mapped to the resolver's own types instead.
    fn mapped_resolved_id(&self, resolved_id: ResolvedTypeId) -> ResolvedTypeId {
        if resolved_id.resolver_id() == MODULE_0_ID {
            ResolvedTypeId::new(
                TypeResolverLevel::Full,
                self.type_id_map[resolved_id.id().index()],
            )
        } else {
            resolved_id
        }
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(GLOBAL_RESOLVER.as_ref())
    }

    fn registered_types(&self) -> Vec<&TypeData> {
        self.types.as_references()
    }
}

enum ResolveFromExportResult<'a> {
    Resolved(Option<ResolvedTypeId>),
    FollowImport(&'a TypeImportQualifier),
}

/// Resolves an export from the given `module` with the given `module_id`.
///
/// If the export can be resolved within the given module, an optional
/// [`ResolvedTypeId`] is returned. If the export references another module, it
/// returns the [`TypeImportQualifier`] to follow.
#[inline]
fn resolve_from_export<'a>(
    module_id: ModuleId,
    module: &'a JsModuleInfoInner,
    export: &JsOwnExport,
) -> ResolveFromExportResult<'a> {
    let resolved = match export {
        JsOwnExport::Binding(binding_id) => match &module.bindings[binding_id.index()].ty {
            TypeReference::Qualifier(_qualifier) => {
                // If it wasn't resolved before exporting, we can't help it
                // anymore.
                None
            }
            TypeReference::Resolved(resolved_id) => Some(resolved_id.with_module_id(module_id)),
            TypeReference::Import(import) => {
                return ResolveFromExportResult::FollowImport(import);
            }
        },
        JsOwnExport::Type(resolved_id) => Some(resolved_id.with_module_id(module_id)),
    };

    ResolveFromExportResult::Resolved(resolved)
}

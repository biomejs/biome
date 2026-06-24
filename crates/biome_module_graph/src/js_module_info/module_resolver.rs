use std::{
    borrow::Cow,
    cell::RefCell,
    collections::hash_map::Entry,
    ops::Deref,
    sync::{Arc, Mutex},
};

use biome_js_syntax::AnyJsExpression;
use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, InstantiationKey, MAX_ALIAS_CHAIN_DEPTH,
    MAX_FLATTEN_DEPTH, ModuleId, Resolvable, ResolvedTypeData, ResolvedTypeId, ResolverId, ScopeId,
    Type, TypeData, TypeId, TypeImportQualifier, TypeReference, TypeReferenceQualifier,
    TypeResolver, TypeResolverLevel, TypeStore, instantiated_generic_alias,
};
use biome_resolver::ResolvedPath;
use biome_rowan::{AstNode, RawSyntaxKind, Text, TextRange, TokenText};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    JsExport, JsImportPath, JsOwnExport, ModuleDb, ProjectDatabase,
    js_module_info::{JsModuleInfoInner, utils::reached_too_many_types},
};

use super::{JsModuleInfo, JsModuleInfoDiagnostic};

const MAX_IMPORT_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

// Any references that resolve to "module 0" will be rewritten to reference the
// module resolver. The reason we do this is that any references to other
// modules can be resolved within the module resolver, whereas references that
// still point to the module's own types would remain unresolved.
const MODULE_0_ID: ResolverId = ResolverId::from_level(TypeResolverLevel::Thin);

/// Append-only store for alias instantiations minted after the primary store is frozen. Lazy ids
/// are `primary_type_count() + local_index`, valid only while that base stays fixed.
#[derive(Default)]
struct LazyInstantiationStore {
    types: boxcar::Vec<Arc<TypeData>>,
    state: Mutex<LazyInstantiationState>,
}

#[derive(Default)]
struct LazyInstantiationState {
    /// Memoizes each `(alias, arguments)` instantiation to its lazy id.
    cache: FxHashMap<InstantiationKey, ResolvedTypeId>,
    /// Reverse map from stored data to its lazy `TypeId`, for content dedup.
    type_ids_by_data: FxHashMap<Arc<TypeData>, TypeId>,
    /// Primary-store length at the first lazy insertion. Lazy ids offset by it; it must not change.
    recorded_base: Option<usize>,
}

impl LazyInstantiationStore {
    fn cached(&self, key: &InstantiationKey) -> Option<ResolvedTypeId> {
        self.lock().cache.get(key).copied()
    }

    /// Returns `true` while the primary base used to offset lazy ids has not
    /// shifted since the first lazy insertion. Used only for debug assertions.
    fn base_is_stable(&self, current_base: usize) -> bool {
        self.lock()
            .recorded_base
            .is_none_or(|recorded| recorded == current_base)
    }

    /// Finds content-identical data already interned here, rebased to its global `base_len + local` id.
    fn find_type(&self, base_len: usize, type_data: &TypeData) -> Option<TypeId> {
        self.lock()
            .type_ids_by_data
            .get(type_data)
            .map(|id| TypeId::new(base_len + id.index()))
    }

    /// Reads data for a global id, or `None` if the id sits below the lazy range (`base_len`).
    fn get(&self, base_len: usize, id: TypeId) -> Option<&TypeData> {
        let local_index = id.index().checked_sub(base_len)?;
        self.types.get(local_index).map(Arc::as_ref)
    }

    /// Interns `type_data`, returning the lazy `TypeId` `base_len + local_index`. `base_len` must be
    /// identical on every call; the `debug_assert` pins it on first insertion.
    fn insert_type(&self, base_len: usize, type_data: Cow<TypeData>) -> TypeId {
        let mut state = self.lock();
        match state.recorded_base {
            None => state.recorded_base = Some(base_len),
            Some(recorded) => debug_assert_eq!(
                recorded, base_len,
                "lazy instantiation base shifted ({recorded} -> {base_len}); the primary type store must stay frozen once lazy types are minted",
            ),
        }
        if let Some(id) = state.type_ids_by_data.get(type_data.as_ref()) {
            return TypeId::new(base_len + id.index());
        }

        let type_data = Arc::new(type_data.into_owned());
        let id = TypeId::new(self.types.push(type_data.clone()));
        state.type_ids_by_data.insert(type_data, id);
        TypeId::new(base_len + id.index())
    }

    fn insert_cache(&self, key: InstantiationKey, resolved_id: ResolvedTypeId) {
        self.lock().cache.insert(key, resolved_id);
    }

    /// All lazy types in insertion order, for the debug/snapshot dump. The lock only fences
    /// concurrent insertions; the returned references outlive it since boxcar slots never move.
    fn registered_types(&self) -> Vec<&TypeData> {
        let _state = self.lock();
        self.types.iter().map(|(_, ty)| Arc::as_ref(ty)).collect()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, LazyInstantiationState> {
        self.state
            .lock()
            .expect("lazy instantiation store lock must not be poisoned")
    }
}

/// Type resolver that is able to resolve types _across_ modules.
///
/// This resolver is instantiated for a given module using
/// [`Self::for_module()`]. Besides the module info, it also receives a
/// reference to a [`ModuleDb`] so that other modules can be resolved.
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
    module_db: ProjectDatabase,

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

    /// Alias instantiations materialized after construction, when the primary store is immutable.
    lazy_instantiations: LazyInstantiationStore,

    /// Primary-store length, frozen when inference ends (`None` until then). The fixed offset base
    /// for lazy instantiation ids, so they never collide with a still-growing primary store.
    frozen_primary_count: Option<usize>,
}

impl ModuleResolver {
    pub fn for_module(module_info: JsModuleInfo, module_db: ProjectDatabase) -> Self {
        let infer_types = module_info.infer_types;

        let types = if infer_types {
            TypeStore::with_capacity(module_info.types.len())
        } else {
            TypeStore::default()
        };
        let mut resolver = Self {
            module_db,
            modules: vec![module_info],
            modules_by_path: Default::default(),
            expressions: Default::default(),
            types,
            type_id_map: Default::default(),
            diagnostics: Default::default(),
            lazy_instantiations: Default::default(),
            frozen_primary_count: None,
        };

        if infer_types {
            resolver.run_inference();
        }
        // Freeze the primary store: its length is now the fixed base for lazy ids.
        resolver.frozen_primary_count = Some(resolver.types.len());
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
                JsOwnExport::Binding(binding_range) => {
                    // Check if this binding is an import and resolve it
                    if let Some(import_qualifier) =
                        resolve_binding_as_import(module, *binding_range)
                    {
                        return self.resolve_import(&import_qualifier).map_or_else(
                            || self.resolved_type_for_id(GLOBAL_UNKNOWN_ID),
                            |resolved_id| self.resolved_type_for_id(resolved_id),
                        );
                    }

                    // Not an import, look up the binding's type from augmentation data
                    module
                        .binding_type_data(*binding_range)
                        .and_then(|data| self.resolve_reference(&data.ty))
                        .map_or_else(
                            || self.resolved_type_for_id(GLOBAL_UNKNOWN_ID),
                            |resolved_id| self.resolved_type_for_id(resolved_id),
                        )
                }
                JsOwnExport::Type(resolved_id) => self.resolved_type_for_id(*resolved_id),
                JsOwnExport::Namespace(reexport) => self
                    .resolve_import(&TypeImportQualifier {
                        resolved_path: reexport.import.resolved_path.clone(),
                        symbol: reexport.import.symbol.clone(),
                        type_only: false,
                    })
                    .map_or_else(
                        || self.resolved_type_for_id(GLOBAL_UNKNOWN_ID),
                        |resolved_id| self.resolved_type_for_id(resolved_id),
                    ),
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

        // Get the scope at the specified range for correct lexical scoping
        let scope = module.scope_for_range(range);
        let scope_id = scope.scope.id();

        let Some(resolved_id) =
            module
                .find_binding_in_scope(name, scope_id)
                .and_then(|binding_range| {
                    // Look up the binding's type data by its range
                    module
                        .binding_type_data(binding_range)
                        // Resolve the type reference, handling Resolved, Import, and Qualifier cases
                        .and_then(|data| self.resolve_reference(&data.ty))
                })
        else {
            return self.resolved_type_for_id(GLOBAL_UNKNOWN_ID);
        };

        self.resolved_type_for_id(resolved_id)
    }

    fn find_module(&self, path: &ResolvedPath) -> Option<ModuleId> {
        self.modules_by_path.get(path).copied()
    }

    fn register_module(&mut self, path: ResolvedPath) -> Option<ModuleId> {
        match self.modules_by_path.entry(path) {
            Entry::Occupied(entry) => Some(*entry.get()),
            Entry::Vacant(entry) => {
                let path = entry.key().as_path()?;
                let module_info = self.module_db.js_module_info_for_path(path)?;
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
                    symbol = Cow::Owned(import.symbol);
                    type_only = if import.type_only { true } else { type_only };
                }
            }
        }

        None
    }

    /// Size of the primary store: the frozen count once set, else the live length. The offset
    /// base that separates primary ids from lazy instantiation ids.
    fn primary_type_count(&self) -> usize {
        self.frozen_primary_count
            .unwrap_or_else(|| self.types.len())
    }

    /// Routes a `TypeId` below [`Self::primary_type_count`] to the primary store, otherwise to the
    /// lazy instantiation store.
    fn get_by_id_raw(&self, id: TypeId) -> Option<&TypeData> {
        // The base is frozen at finalization, so the lazy store's recorded base must still match it.
        debug_assert!(
            self.lazy_instantiations
                .base_is_stable(self.primary_type_count()),
            "lazy instantiation base disagrees with the frozen primary length; id routing is corrupted",
        );
        if id.index() < self.primary_type_count() {
            Some(self.types.get_by_id(id))
        } else {
            self.lazy_instantiations.get(self.primary_type_count(), id)
        }
    }

    /// Resolves an id to its stored data without materializing deferred alias
    /// applications; `Full` ids route through [`Self::get_by_id_raw`].
    fn get_by_resolved_id_raw(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
        match id.level() {
            TypeResolverLevel::Full => self
                .get_by_id_raw(id.id())
                .map(|data| ResolvedTypeData::from((id, data))),
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

    /// Resolves `id`, materializing a deferred generic-alias application. `stack` guards
    /// alias-application cycles. Returns the raw resolution until the store is frozen (no lazy minting).
    fn get_by_resolved_id_with_stack(
        &self,
        id: ResolvedTypeId,
        stack: &mut Vec<InstantiationKey>,
    ) -> Option<ResolvedTypeData<'_>> {
        let resolved = self.get_by_resolved_id_raw(id)?;
        if self.frozen_primary_count.is_none() {
            return Some(resolved);
        }
        // A non-expandable application yields `None`; fall back to the opaque `resolved`, which
        // the rule treats as indeterminate.
        self.instantiated_alias_application(resolved, stack)
            .or(Some(resolved))
    }

    /// For an `InstanceOf` of a generic alias (e.g. `Maybe<string>`), substitutes the body with its
    /// arguments and interns it. `None` for nominal instances (`Promise<T>`) and `stack` cycles.
    fn instantiated_alias_application(
        &self,
        resolved: ResolvedTypeData<'_>,
        stack: &mut Vec<InstantiationKey>,
    ) -> Option<ResolvedTypeData<'_>> {
        let (key, declared, body_ref) = self.decompose_alias_application(&resolved)?;
        if let Some(cached) = self.lazy_instantiations.cached(&key) {
            return self.get_by_resolved_id_raw(cached);
        }
        if stack.contains(&key) {
            return None;
        }
        // A parameterized chain (`type W1<T> = W0<T>; ...`) re-enters here per level via
        // `instantiated_generic_alias`, so cap the re-entry depth and leave deeper aliases opaque
        // (indeterminate to the rule) rather than overflowing the stack.
        if stack.len() >= MAX_ALIAS_CHAIN_DEPTH {
            return None;
        }

        // Walk the chain of identity-alias handoffs (`type A = Id<B>`) iteratively, interning one
        // level per hop. The walk stops and leaves the alias opaque when a hop fails to materialize
        // or closes a cycle (`type Loop = Id<Loop>`, mutual aliases). It terminates because every
        // hop pushes a fresh key and a repeated key ends it.
        let base = stack.len();
        let mut chain: Vec<(InstantiationKey, TypeData)> = Vec::new();
        let mut current = Some((key, declared, body_ref));
        let mut leave_opaque = false;
        while let Some((hop_key, hop_declared, hop_body)) = current.take() {
            stack.push(hop_key.clone());
            let Some(materialized) =
                self.materialize_alias_level(&hop_key, &hop_declared, &hop_body, stack)
            else {
                // The hop cannot materialize: a cycle, or a malformed/unresolvable alias.
                leave_opaque = true;
                break;
            };

            // Does the materialized body hand off to another alias application?
            let next = if let TypeData::Reference(reference) = &materialized
                && let Some(ref_id) = self.resolve_reference(reference)
                && let Some(ref_resolved) = self.get_by_resolved_id_raw(ref_id)
            {
                self.decompose_alias_application(&ref_resolved)
            } else {
                None
            };
            chain.push((hop_key, materialized));

            let Some((next_key, next_declared, next_body)) = next else {
                break; // chain ends in a concrete body
            };
            if self.lazy_instantiations.cached(&next_key).is_some() {
                break; // handoff target already interned
            }
            if stack.contains(&next_key) {
                // Handoff target is still on the stack: a cycle.
                leave_opaque = true;
                break;
            }
            current = Some((next_key, next_declared, next_body));
        }
        while stack.len() > base {
            stack.pop();
        }
        if leave_opaque {
            return None;
        }

        // Intern deepest-first so type IDs match the order a depth-first walk produces.
        let mut entry_id = None;
        for (hop_key, data) in chain.into_iter().rev() {
            let instantiated_id = self.register_lazy_type(Cow::Owned(data));
            let resolved_id = ResolvedTypeId::new(TypeResolverLevel::Full, instantiated_id);
            self.lazy_instantiations.insert_cache(hop_key, resolved_id);
            entry_id = Some(resolved_id);
        }
        self.get_by_resolved_id_raw(entry_id?)
    }

    /// Substitutes a single alias level via [`instantiated_generic_alias`], keeping the cycle
    /// `stack` visible to the nested resolution so the walk above can intern each level on its own.
    fn materialize_alias_level(
        &self,
        key: &InstantiationKey,
        declared: &[TypeReference],
        body_ref: &TypeReference,
        stack: &mut Vec<InstantiationKey>,
    ) -> Option<TypeData> {
        let mut lazy_resolver = LazyInstantiationResolver {
            resolver: self,
            stack: RefCell::new(std::mem::take(stack)),
        };
        let instantiated =
            instantiated_generic_alias(&key.arguments, declared, body_ref, &mut lazy_resolver);
        *stack = lazy_resolver.stack.into_inner();
        instantiated
    }

    /// Decomposes a generic-alias application into its [`InstantiationKey`], declared parameters, and
    /// body reference. `None` for non-applications such as nominal instances (`Promise<T>`, `Array<T>`).
    fn decompose_alias_application(
        &self,
        resolved: &ResolvedTypeData,
    ) -> Option<(InstantiationKey, Vec<TypeReference>, TypeReference)> {
        let TypeData::InstanceOf(instance) = resolved.as_raw_data() else {
            return None;
        };
        if instance.type_parameters.is_empty() {
            return None;
        }

        let target_ref = resolved.apply_module_id_to_reference(&instance.ty);
        let target_id = self.resolve_reference(target_ref.as_ref())?;
        let target = self.get_by_resolved_id_raw(target_id)?;
        let TypeData::InstanceOf(alias) = target.as_raw_data() else {
            return None;
        };
        if alias.type_parameters.is_empty() {
            return None;
        }

        let arguments = instance
            .type_parameters
            .iter()
            .map(|param| resolved.apply_module_id_to_reference(param).into_owned())
            .collect::<Box<[_]>>();
        let declared = alias
            .type_parameters
            .iter()
            .map(|param| target.apply_module_id_to_reference(param).into_owned())
            .collect::<Vec<_>>();
        let body_ref = target.apply_module_id_to_reference(&alias.ty).into_owned();
        Some((
            InstantiationKey::new(target_id, arguments),
            declared,
            body_ref,
        ))
    }

    /// Interns a materialized type, reusing the primary-store id when the data already lives there
    /// so lazy ids are minted only for types the primary store lacks.
    fn register_lazy_type(&self, type_data: Cow<TypeData>) -> TypeId {
        if let Some(type_id) = self.types.find(type_data.as_ref()) {
            return type_id;
        }
        self.lazy_instantiations
            .insert_type(self.primary_type_count(), type_data)
    }
}

/// [`TypeResolver`] view for alias materialization: reads through the parent [`ModuleResolver`],
/// registers new types into its lazy store, and threads `stack` to break instantiation cycles.
struct LazyInstantiationResolver<'a> {
    resolver: &'a ModuleResolver,
    stack: RefCell<Vec<InstantiationKey>>,
}

impl TypeResolver for LazyInstantiationResolver<'_> {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Full
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.resolver.types.find(type_data).or_else(|| {
            self.resolver
                .lazy_instantiations
                .find_type(self.resolver.primary_type_count(), type_data)
        })
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.resolver
            .get_by_id_raw(id)
            .unwrap_or_else(|| GLOBAL_RESOLVER.get_by_id(GLOBAL_UNKNOWN_ID.id()))
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
        let mut id = id;
        let mut hops = 0;
        loop {
            let resolved = {
                let mut stack = self.stack.borrow_mut();
                self.resolver
                    .get_by_resolved_id_with_stack(id, &mut stack)?
            };
            match resolved.as_raw_data() {
                // Follow `Reference -> Reference` chains, bounded against cycles of distinct ids;
                // on exhaustion the reference is returned un-followed.
                TypeData::Reference(TypeReference::Resolved(next))
                    if id != *next && hops < MAX_FLATTEN_DEPTH =>
                {
                    id = resolved.apply_module_id(*next);
                    hops += 1;
                }
                _ => return Some(resolved),
            }
        }
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.resolver.register_lazy_type(type_data)
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        self.resolver.resolve_reference(ty)
    }

    fn resolve_import(&self, qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        self.resolver.resolve_import(qualifier)
    }

    fn resolve_import_namespace_member(
        &self,
        module_id: ModuleId,
        name: &str,
    ) -> Option<ResolvedTypeId> {
        self.resolver
            .resolve_import_namespace_member(module_id, name)
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        self.resolver.resolve_qualifier(qualifier)
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        self.resolver.resolve_type_of(identifier, scope_id)
    }

    fn resolve_expression(
        &mut self,
        _scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Cow<'_, TypeData> {
        let id = self.resolver.resolved_id_for_expression(expr);
        let mut stack = self.stack.borrow_mut();
        match self.resolver.get_by_resolved_id_with_stack(id, &mut stack) {
            Some(resolved) => Cow::Owned(resolved.to_data()),
            None => Cow::Owned(TypeData::unknown()),
        }
    }

    fn reference_to_resolved_expression(
        &mut self,
        _scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> TypeReference {
        self.resolver.resolved_id_for_expression(expression).into()
    }

    fn mapped_resolved_id(&self, resolved_id: ResolvedTypeId) -> ResolvedTypeId {
        self.resolver.mapped_resolved_id(resolved_id)
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        self.resolver.fallback_resolver()
    }

    fn registered_types(&self) -> Vec<&TypeData> {
        self.resolver.registered_types()
    }
}

impl TypeResolver for ModuleResolver {
    /// The module resolver is the full-resolution stage.
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Full
    }

    /// Searches the primary store for content-identical data, then the lazy instantiation store.
    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data).or_else(|| {
            self.lazy_instantiations
                .find_type(self.primary_type_count(), type_data)
        })
    }

    /// Resolves a local id, falling back to the global `unknown` type when it is out of range.
    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.get_by_id_raw(id)
            .unwrap_or_else(|| GLOBAL_RESOLVER.get_by_id(GLOBAL_UNKNOWN_ID.id()))
    }

    /// Entry point for id resolution; seeds the empty cycle stack threaded through materialization.
    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
        let mut stack = Vec::new();
        self.get_by_resolved_id_with_stack(id, &mut stack)
    }

    // Before freeze, grow the primary store. After freeze its length is the fixed lazy-id base, so
    // late registrations go to the lazy store; growing the primary store then would corrupt id routing.
    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        if self.frozen_primary_count.is_some() {
            return self.register_lazy_type(type_data);
        }
        self.types.insert_cow(type_data)
    }

    fn should_instantiate_generic_qualifiers(&self) -> bool {
        false
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

        // Convert scope_id to semantic ScopeId
        let semantic_scope_id = biome_js_semantic::ScopeId::new(scope_id.index());

        let Some(binding_range) = module.find_binding_in_scope(identifier, semantic_scope_id)
        else {
            return GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id);
        };

        // Check if the binding is actually an import declaration
        // This prevents incorrectly treating local bindings that shadow imported names as imports
        if module.is_binding_imported(identifier, semantic_scope_id) {
            module
                .static_imports
                .get(identifier.text())
                .and_then(|import| {
                    // TODO: Determine if this is a type-only import
                    // JsImport doesn't store phase information directly
                    // We may need to look it up from static_import_paths
                    let type_only = module
                        .static_import_paths
                        .get(import.specifier.text())
                        .is_some_and(|path| path.phase == crate::JsImportPhase::Type);

                    self.resolve_import(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only,
                    })
                })
        } else {
            // Look up the binding's type from augmentation data
            module
                .binding_type_data(binding_range)
                .and_then(|data| self.resolve_reference(&data.ty))
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
        let mut types = self.types.as_references();
        types.extend(self.lazy_instantiations.registered_types());
        types
    }
}

/// Attempts to resolve a binding to an import qualifier if the binding is imported.
///
/// Returns `Some(TypeImportQualifier)` if the binding at `binding_range` is an imported
/// binding, `None` otherwise.
fn resolve_binding_as_import(
    module: &JsModuleInfoInner,
    binding_range: TextRange,
) -> Option<TypeImportQualifier> {
    // Find the binding in the semantic model to get its name
    let binding = module
        .semantic_model
        .scope_from_id(ScopeId::GLOBAL)
        .bindings()
        .find(|b| b.syntax().text_trimmed_range() == binding_range)?;

    let name = binding.syntax().text_trimmed().into_text();

    // Check if this binding is an import
    if !module.is_binding_imported(name.text(), ScopeId::GLOBAL) {
        return None;
    }

    // Resolve the import directly from static_imports.
    // Note: We don't need to check dynamic_import_paths because:
    // - Dynamic imports (import('./foo')) return Promises and don't create direct bindings
    // - CommonJS require() calls are expressions that return values, not import bindings
    // - Only static imports (import { foo } from '...') create bindings that
    //   is_binding_imported() recognizes as imported
    let import = module.static_imports.get(name.text())?;
    let type_only = module
        .static_import_paths
        .get(import.specifier.text())
        .is_some_and(|path| path.phase == crate::JsImportPhase::Type);

    Some(TypeImportQualifier {
        symbol: import.symbol.clone(),
        resolved_path: import.resolved_path.clone(),
        type_only,
    })
}

enum ResolveFromExportResult {
    Resolved(Option<ResolvedTypeId>),
    FollowImport(TypeImportQualifier),
}

/// Resolves an export from the given `module` with the given `module_id`.
///
/// If the export can be resolved within the given module, an optional
/// [`ResolvedTypeId`] is returned. If the export references another module, it
/// returns the [`TypeImportQualifier`] to follow.
#[inline]
fn resolve_from_export(
    module_id: ModuleId,
    module: &JsModuleInfoInner,
    export: &JsOwnExport,
) -> ResolveFromExportResult {
    let resolved = match export {
        JsOwnExport::Binding(binding_range) => {
            // Look up the binding's type data by its range
            match module.binding_type_data(*binding_range) {
                Some(data) => match &data.ty {
                    TypeReference::Qualifier(_qualifier) => {
                        // If it wasn't resolved before exporting, we can't help it
                        // anymore.
                        None
                    }
                    TypeReference::Resolved(resolved_id) => {
                        Some(resolved_id.with_module_id(module_id))
                    }
                    TypeReference::Import(import) => {
                        return ResolveFromExportResult::FollowImport(*import.clone());
                    }
                },
                None => {
                    // If there's no binding_type_data, check if this binding is an import.
                    // Imported bindings don't have local type data - their types come from
                    // the imported module.
                    if let Some(import_qualifier) =
                        resolve_binding_as_import(module, *binding_range)
                    {
                        return ResolveFromExportResult::FollowImport(import_qualifier);
                    }
                    None
                }
            }
        }
        JsOwnExport::Type(resolved_id) => Some(resolved_id.with_module_id(module_id)),
        JsOwnExport::Namespace(reexport) => {
            // `export * as Ns from "./mod"` makes `Ns` a namespace object whose
            // type is the full import namespace of the target module. We tell
            // the caller to follow the import with `ImportSymbol::All`, which
            // will produce a `TypeData::ImportNamespace` pointing at the right
            // module.
            return ResolveFromExportResult::FollowImport(TypeImportQualifier {
                symbol: reexport.import.symbol.clone(),
                resolved_path: reexport.import.resolved_path.clone(),
                type_only: false,
            });
        }
    };

    ResolveFromExportResult::Resolved(resolved)
}

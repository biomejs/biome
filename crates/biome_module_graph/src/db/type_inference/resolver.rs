//! Raw type resolution and declaration-level cycle recovery.
//!
//! A strongly connected component in the import graph does not imply that
//! every declaration in those modules depends on a cycle. When an on-demand
//! lookup crosses such a component, the root lookup retries with a shared
//! declaration evaluator. The evaluator records the exact bindings, local
//! types, and namespace exports requested by that lookup. Acyclic declarations
//! can then settle independently. Cycle breaking marks only declarations in a
//! dependency cycle as `Unknown`; exhausting the evaluator's work limit marks
//! every unfinished declaration as `Unknown`.
//!
//! Dependency discovery is iterative. An unfinished dependency supplies a
//! provisional `Unknown`, and the declaration using it waits instead of
//! keeping that provisional result. The evaluator retries waiting declarations
//! after their dependencies settle. Imported resolution contexts share the
//! evaluator, which keeps the declaration graph intact across module
//! boundaries without recursing through it on the Rust stack.

use super::{BindingTypeData, InferredModuleTypes, globals::global_type};
use crate::db::queries::{
    LocalTypeInput, infer_local_type, infer_module_types, infer_module_types_from_tables,
    inference_module_sccs,
};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsModuleInfo, ModuleDb, ModuleGraphGeneration, module_for_key};
use biome_js_syntax::TsModuleDeclaration;
use biome_js_type_info::{
    GlobalTypeId, RawTypeData, ResolvedTypeId, ScopeId, TypeId, TypeReference,
    TypeReferenceQualifier, TypeResolverLevel,
    interned_types::{
        InternedModule as InferredModule, InternedNamespace as InferredNamespace,
        InternedTypeofValue, LocalTypeHandle, LocalTypeId, ModuleKey, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::{AstNode, Text, TextRange};
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::AsId;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
};

// This limit guards Rust stack recursion. Each `ResolutionCtx::resolve` frame
// clones a raw type and runs a conversion walk. Named declarations become local
// handles before recursion, so the remaining depth comes from structural types
// nested within one declaration.
pub(super) const MAX_RAW_TYPE_RESOLUTION_DEPTH: usize = 64;
pub(super) const MAX_ON_DEMAND_IMPORT_DEPTH: u8 = 128;
const MAX_INFERRED_EXPRESSION_WRAPPER_STEPS: usize = 64;
const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;
// Bounds retries when resolving a dynamically discovered declaration graph.
// The graph itself is stored on the heap, so this is a work limit rather than
// a recursion limit.
const MAX_ON_DEMAND_DECLARATION_ATTEMPTS: usize = 16_384;

/// Selects how a resolution context follows references into imported modules.
///
/// Regular queries resolve only the requested imported symbol. Salsa cycle
/// recovery cannot recursively request members of the strongly connected
/// component that caused the cycle. Cycle fallback therefore reads complete
/// inferred tables for dependencies outside that component and treats imports
/// into the component as unavailable.
#[derive(Clone, Copy)]
pub(in crate::db) enum ImportResolution<'a> {
    /// Resolves imported symbols through export and lookup queries.
    OnDemand { remaining: u8 },
    /// Reads imports from complete module tables while blocking cyclic edges.
    FromTables { root: ModuleInfo },
    /// Reads imports from complete module tables while blocking the active cycle.
    CycleFallback(&'a FxHashSet<ModuleInfo>),
}

impl ImportResolution<'_> {
    pub(in crate::db) fn on_demand() -> Self {
        Self::OnDemand {
            remaining: MAX_ON_DEMAND_IMPORT_DEPTH,
        }
    }
}

/// Identifies one declaration in the dependency graph for a selected lookup.
///
/// Module identity is part of the key because one evaluator follows
/// declarations across every module in the active import component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum OnDemandDeclaration {
    Binding {
        module: ModuleInfo,
        range: TextRange,
    },
    Type {
        module: ModuleInfo,
        type_id: TypeId,
    },
    Namespace {
        module: ModuleInfo,
        name: Text,
    },
}

/// Progress of one declaration in the on-demand evaluation state machine.
///
/// A queued declaration becomes evaluating. The attempt either resolves it or
/// leaves it waiting on discovered dependencies. Settling a dependency queues
/// its waiting dependents for another attempt. `Resolved` is terminal.
#[derive(Clone, Copy)]
enum OnDemandDeclarationState<'db> {
    Queued,
    Evaluating,
    Waiting,
    Resolved(InferredTypeData<'db>),
}

/// Shared worklist and dependency graph for one root declaration lookup.
///
/// `dependencies` supports cycle detection. The reverse `dependents` edges
/// identify which waiting declarations become eligible for another attempt
/// when a dependency settles.
#[derive(Default)]
struct OnDemandDeclarationEvaluator<'db> {
    states: FxHashMap<OnDemandDeclaration, OnDemandDeclarationState<'db>>,
    queue: VecDeque<OnDemandDeclaration>,
    current: Option<OnDemandDeclaration>,
    current_has_unresolved_dependencies: bool,
    dependencies: FxHashMap<OnDemandDeclaration, FxHashSet<OnDemandDeclaration>>,
    dependents: FxHashMap<OnDemandDeclaration, FxHashSet<OnDemandDeclaration>>,
    attempts: usize,
}

type SharedOnDemandDeclarationEvaluator<'db> = Rc<RefCell<OnDemandDeclarationEvaluator<'db>>>;

/// Returns every node that belongs to a multi-node component or has a self-edge.
///
/// Both passes are iterative because declaration depth comes from source code
/// and must not consume the Rust stack.
fn cyclic_dependency_ids(edges: &[Vec<usize>]) -> FxHashSet<usize> {
    let mut visited = vec![false; edges.len()];
    let mut next_edge = vec![0; edges.len()];
    let mut finish_order = Vec::with_capacity(edges.len());
    let mut stack = Vec::new();

    for start in 0..edges.len() {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        stack.push(start);
        while let Some(&node) = stack.last() {
            if let Some(&next) = edges[node].get(next_edge[node]) {
                next_edge[node] += 1;
                if !visited[next] {
                    visited[next] = true;
                    stack.push(next);
                }
            } else {
                finish_order.push(node);
                stack.pop();
            }
        }
    }

    let mut reverse_edges = vec![Vec::new(); edges.len()];
    for (from, targets) in edges.iter().enumerate() {
        for &to in targets {
            reverse_edges[to].push(from);
        }
    }

    let mut component_by_id = vec![usize::MAX; edges.len()];
    let mut component_sizes = Vec::new();
    for start in finish_order.into_iter().rev() {
        if component_by_id[start] != usize::MAX {
            continue;
        }
        let component = component_sizes.len();
        component_sizes.push(0);
        stack.push(start);
        while let Some(node) = stack.pop() {
            if component_by_id[node] != usize::MAX {
                continue;
            }
            component_by_id[node] = component;
            component_sizes[component] += 1;
            stack.extend(
                reverse_edges[node]
                    .iter()
                    .copied()
                    .filter(|&predecessor| component_by_id[predecessor] == usize::MAX),
            );
        }
    }

    edges
        .iter()
        .enumerate()
        .filter_map(|(id, targets)| {
            let component = component_by_id[id];
            (component_sizes[component] > 1 || targets.contains(&id)).then_some(id)
        })
        .collect()
}

pub(in crate::db) struct ResolutionCtx<'db, 'a> {
    pub(in crate::db::type_inference) db: &'db dyn ModuleDb,
    pub(in crate::db::type_inference) module: ModuleInfo,
    pub(in crate::db::type_inference) module_key: ModuleKey,
    pub(in crate::db::type_inference) js_info: &'a JsModuleInfo,
    pub(in crate::db::type_inference) import_resolution: ImportResolution<'a>,
    pub(in crate::db::type_inference) resolved: FxHashMap<TypeId, InferredTypeData<'db>>,
    pub(in crate::db::type_inference) in_progress: FxHashSet<TypeId>,
    pub(in crate::db::type_inference) resolution_depth: Cell<usize>,
    encountered_inference_cycle: Cell<bool>,
    on_demand_declarations: Option<SharedOnDemandDeclarationEvaluator<'db>>,
}

pub(in crate::db) fn resolve_raw_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    import_resolution: ImportResolution<'_>,
) -> InferredModuleTypes<'db> {
    let mut ctx = ResolutionCtx::new(db, module, js_info, import_resolution);

    let types = (0..js_info.raw_types.len())
        .map(|index| ctx.resolve_raw_type_id(TypeId::new(index)))
        .collect();

    let expressions = js_info
        .raw_expressions
        .iter()
        .map(|(range, reference)| (*range, ctx.resolve(reference)))
        .collect();

    let binding_type_data = js_info
        .raw_binding_types
        .iter()
        .map(|(range, reference)| {
            (
                *range,
                BindingTypeData {
                    ty: ctx.resolve(reference),
                },
            )
        })
        .collect();

    InferredModuleTypes {
        module_key: ctx.module_key,
        named_type_ids: js_info.named_type_ids.clone(),
        types,
        expressions,
        binding_type_data,
    }
}

impl<'db, 'a> ResolutionCtx<'db, 'a> {
    pub(in crate::db) fn new(
        db: &'db dyn ModuleDb,
        module: ModuleInfo,
        js_info: &'a JsModuleInfo,
        import_resolution: ImportResolution<'a>,
    ) -> Self {
        Self::new_with_declarations(db, module, js_info, import_resolution, None)
    }

    fn new_with_declarations(
        db: &'db dyn ModuleDb,
        module: ModuleInfo,
        js_info: &'a JsModuleInfo,
        import_resolution: ImportResolution<'a>,
        on_demand_declarations: Option<SharedOnDemandDeclarationEvaluator<'db>>,
    ) -> Self {
        Self {
            db,
            module,
            module_key: ModuleKey::new(module.as_id()),
            js_info,
            import_resolution,
            resolved: FxHashMap::default(),
            in_progress: FxHashSet::default(),
            resolution_depth: Cell::new(0),
            encountered_inference_cycle: Cell::new(false),
            on_demand_declarations,
        }
    }

    pub(super) fn for_on_demand_import<'info>(
        &self,
        module: ModuleInfo,
        js_info: &'info JsModuleInfo,
        remaining: u8,
        resolve_declarations_directly: bool,
    ) -> ResolutionCtx<'db, 'info> {
        // Contexts inside the active import component must contribute to the
        // same declaration graph. An ordinary imported lookup keeps its own
        // tracked-query boundary and does not need this shared state.
        let on_demand_declarations = resolve_declarations_directly.then(|| {
            self.on_demand_declarations
                .as_ref()
                .map_or_else(Rc::default, Rc::clone)
        });
        ResolutionCtx::new_with_declarations(
            self.db,
            module,
            js_info,
            ImportResolution::OnDemand { remaining },
            on_demand_declarations,
        )
    }

    pub(in crate::db) fn resolve_local_binding(
        &mut self,
        range: TextRange,
    ) -> InferredTypeData<'db> {
        let Some(reference) = self.js_info.raw_binding_types.get(&range).cloned() else {
            return InferredTypeData::Unknown;
        };
        if !matches!(self.import_resolution, ImportResolution::OnDemand { .. })
            || self.on_demand_declarations.is_none()
        {
            return self.resolve(&reference);
        }

        let declaration = OnDemandDeclaration::Binding {
            module: self.module,
            range,
        };
        self.resolve_on_demand_declaration(declaration)
    }

    pub(in crate::db) fn resolve_root_binding(
        &mut self,
        range: TextRange,
    ) -> InferredTypeData<'db> {
        self.on_demand_declarations.get_or_insert_with(Rc::default);
        self.resolve_on_demand_declaration(OnDemandDeclaration::Binding {
            module: self.module,
            range,
        })
    }

    pub(in crate::db) fn resolve_local_declaration(
        &mut self,
        type_id: TypeId,
    ) -> InferredTypeData<'db> {
        if !matches!(self.import_resolution, ImportResolution::OnDemand { .. })
            || self.on_demand_declarations.is_none()
        {
            return self.resolve_raw_type_id(type_id);
        }

        let declaration = OnDemandDeclaration::Type {
            module: self.module,
            type_id,
        };
        self.resolve_on_demand_declaration(declaration)
    }

    pub(in crate::db) fn resolve_root_declaration(
        &mut self,
        type_id: TypeId,
    ) -> InferredTypeData<'db> {
        self.on_demand_declarations.get_or_insert_with(Rc::default);
        self.resolve_on_demand_declaration(OnDemandDeclaration::Type {
            module: self.module,
            type_id,
        })
    }

    /// Resolves the selected declaration and every declaration it reaches.
    ///
    /// Nested requests register dependency edges and return provisional
    /// `Unknown` values. Results containing those placeholders are discarded
    /// until their dependencies settle. Exhausting the attempt budget makes
    /// every unfinished declaration `Unknown`; a dependency cycle affects only
    /// the declarations in that cycle.
    pub(super) fn resolve_on_demand_declaration(
        &self,
        declaration: OnDemandDeclaration,
    ) -> InferredTypeData<'db> {
        if self.on_demand_declarations().borrow().current.is_some() {
            return self.request_on_demand_declaration(declaration);
        }

        self.on_demand_declarations().borrow_mut().attempts = 0;
        self.schedule_on_demand_declaration(declaration.clone());
        loop {
            if let Some(OnDemandDeclarationState::Resolved(ty)) = self
                .on_demand_declarations()
                .borrow()
                .states
                .get(&declaration)
                .copied()
            {
                return ty;
            }

            let next = self.on_demand_declarations().borrow_mut().queue.pop_front();
            if let Some(next) = next {
                if !self.begin_on_demand_declaration_attempt(&next) {
                    self.abort_on_demand_declarations();
                    return InferredTypeData::Unknown;
                }
                let ty = self.evaluate_on_demand_declaration(&next);
                self.finish_on_demand_declaration_attempt(next, ty);
            } else if !self.resolve_on_demand_declaration_cycles() {
                self.abort_on_demand_declarations();
                return InferredTypeData::Unknown;
            }
        }
    }

    fn abort_on_demand_declarations(&self) {
        let mut evaluator = self.on_demand_declarations().borrow_mut();
        evaluator.queue.clear();
        evaluator.current = None;
        evaluator.current_has_unresolved_dependencies = false;
        for state in evaluator.states.values_mut() {
            if !matches!(state, OnDemandDeclarationState::Resolved(_)) {
                *state = OnDemandDeclarationState::Resolved(InferredTypeData::Unknown);
            }
        }
    }

    fn request_on_demand_declaration(
        &self,
        declaration: OnDemandDeclaration,
    ) -> InferredTypeData<'db> {
        let mut evaluator = self.on_demand_declarations().borrow_mut();
        if let Some(OnDemandDeclarationState::Resolved(ty)) =
            evaluator.states.get(&declaration).copied()
        {
            return ty;
        }

        let Some(current) = evaluator.current.clone() else {
            return InferredTypeData::Unknown;
        };
        evaluator.current_has_unresolved_dependencies = true;
        evaluator
            .dependencies
            .entry(current.clone())
            .or_default()
            .insert(declaration.clone());
        evaluator
            .dependents
            .entry(declaration.clone())
            .or_default()
            .insert(current);
        if !evaluator.states.contains_key(&declaration) {
            evaluator
                .states
                .insert(declaration.clone(), OnDemandDeclarationState::Queued);
            evaluator.queue.push_back(declaration);
        }
        InferredTypeData::Unknown
    }

    fn schedule_on_demand_declaration(&self, declaration: OnDemandDeclaration) {
        let mut evaluator = self.on_demand_declarations().borrow_mut();
        if !evaluator.states.contains_key(&declaration) {
            evaluator
                .states
                .insert(declaration.clone(), OnDemandDeclarationState::Queued);
            evaluator.queue.push_back(declaration);
        }
    }

    fn begin_on_demand_declaration_attempt(&self, declaration: &OnDemandDeclaration) -> bool {
        let mut evaluator = self.on_demand_declarations().borrow_mut();
        if evaluator.attempts >= MAX_ON_DEMAND_DECLARATION_ATTEMPTS {
            return false;
        }
        evaluator.attempts += 1;

        // A retry can discover fewer dependencies after provisional values
        // have settled. Removing the old edges prevents those stale
        // dependencies from creating a false cycle.
        if let Some(previous) = evaluator.dependencies.remove(declaration) {
            for dependency in previous {
                if let Some(dependents) = evaluator.dependents.get_mut(&dependency) {
                    dependents.remove(declaration);
                }
            }
        }
        evaluator.current = Some(declaration.clone());
        evaluator.current_has_unresolved_dependencies = false;
        evaluator
            .states
            .insert(declaration.clone(), OnDemandDeclarationState::Evaluating);
        true
    }

    fn finish_on_demand_declaration_attempt(
        &self,
        declaration: OnDemandDeclaration,
        ty: InferredTypeData<'db>,
    ) {
        let mut evaluator = self.on_demand_declarations().borrow_mut();
        evaluator.current = None;
        if evaluator.current_has_unresolved_dependencies {
            // `ty` contains at least one provisional `Unknown`. Discard it so
            // the declaration is evaluated again with settled dependencies.
            evaluator
                .states
                .insert(declaration, OnDemandDeclarationState::Waiting);
        } else {
            evaluator
                .states
                .insert(declaration.clone(), OnDemandDeclarationState::Resolved(ty));
            Self::schedule_waiting_dependents(&mut evaluator, &declaration);
        }
    }

    fn schedule_waiting_dependents(
        evaluator: &mut OnDemandDeclarationEvaluator<'db>,
        declaration: &OnDemandDeclaration,
    ) {
        let dependents = evaluator
            .dependents
            .get(declaration)
            .cloned()
            .unwrap_or_default();
        for dependent in dependents {
            if matches!(
                evaluator.states.get(&dependent),
                Some(OnDemandDeclarationState::Waiting)
            ) {
                evaluator
                    .states
                    .insert(dependent.clone(), OnDemandDeclarationState::Queued);
                evaluator.queue.push_back(dependent);
            }
        }
    }

    fn evaluate_on_demand_declaration(
        &self,
        declaration: &OnDemandDeclaration,
    ) -> InferredTypeData<'db> {
        let module = match declaration {
            OnDemandDeclaration::Binding { module, .. }
            | OnDemandDeclaration::Type { module, .. }
            | OnDemandDeclaration::Namespace { module, .. } => *module,
        };
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return InferredTypeData::Unknown;
        };
        if !js_info.infer_types {
            return InferredTypeData::Unknown;
        }

        let ImportResolution::OnDemand { remaining } = self.import_resolution else {
            return InferredTypeData::Unknown;
        };
        let mut ctx = self.for_on_demand_import(module, &js_info, remaining, true);
        match declaration {
            OnDemandDeclaration::Binding { range, .. } => js_info
                .raw_binding_types
                .get(range)
                .cloned()
                .map_or(InferredTypeData::Unknown, |reference| {
                    ctx.resolve(&reference)
                }),
            OnDemandDeclaration::Type { type_id, .. } => ctx.resolve_raw_type_id(*type_id),
            OnDemandDeclaration::Namespace { name, .. } => {
                ctx.resolve_own_namespace_export(name.text())
            }
        }
    }

    fn resolve_on_demand_declaration_cycles(&self) -> bool {
        let mut evaluator = self.on_demand_declarations().borrow_mut();
        let waiting = evaluator
            .states
            .iter()
            .filter_map(|(declaration, state)| {
                matches!(state, OnDemandDeclarationState::Waiting).then_some(declaration.clone())
            })
            .collect::<Vec<_>>();
        if waiting.is_empty() {
            return false;
        }

        let id_by_declaration = waiting
            .iter()
            .cloned()
            .enumerate()
            .map(|(id, declaration)| (declaration, id))
            .collect::<FxHashMap<_, _>>();
        let edges = waiting
            .iter()
            .map(|declaration| {
                evaluator
                    .dependencies
                    .get(declaration)
                    .into_iter()
                    .flatten()
                    .filter_map(|dependency| id_by_declaration.get(dependency).copied())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let cyclic = cyclic_dependency_ids(&edges);
        if cyclic.is_empty() {
            return false;
        }

        // Only declarations in dependency components become `Unknown`.
        // Treating them as settled lets acyclic declarations that refer to a
        // cyclic result finish with as much surrounding structure as possible.
        let cyclic_declarations = cyclic
            .into_iter()
            .map(|id| waiting[id].clone())
            .collect::<Vec<_>>();
        for declaration in &cyclic_declarations {
            evaluator.states.insert(
                declaration.clone(),
                OnDemandDeclarationState::Resolved(InferredTypeData::Unknown),
            );
        }
        for declaration in cyclic_declarations {
            Self::schedule_waiting_dependents(&mut evaluator, &declaration);
        }
        true
    }

    fn on_demand_declarations(&self) -> &SharedOnDemandDeclarationEvaluator<'db> {
        self.on_demand_declarations
            .as_ref()
            .expect("on-demand declaration evaluation must be initialized")
    }

    pub(super) fn resolves_declarations_directly(&self) -> bool {
        self.on_demand_declarations.is_some()
    }

    pub(in crate::db) fn encountered_inference_cycle(&self) -> bool {
        self.encountered_inference_cycle.get()
    }

    pub(super) fn mark_inference_cycle(&self) {
        self.encountered_inference_cycle.set(true);
    }

    /// Infers `module` as a dependency of the module currently being resolved.
    ///
    /// The tracked query records the imported result as a dependency of the
    /// current inference. Returns `None` for unsupported modules, disabled type
    /// inference, or an import cycle.
    pub(in crate::db::type_inference) fn infer_imported_module(
        &self,
        module: ModuleInfo,
    ) -> Option<&'db InferredModuleTypes<'db>> {
        match self.import_resolution {
            ImportResolution::OnDemand { .. } => infer_module_types(self.db, module),
            ImportResolution::FromTables { root } => {
                let sccs = inference_module_sccs(self.db, ModuleGraphGeneration::get(self.db));
                if module == self.module || sccs.contains_cycle_between(self.module, module) {
                    None
                } else {
                    infer_module_types_from_tables(self.db, root, module)
                }
            }
            ImportResolution::CycleFallback(blocked) => {
                if blocked.contains(&module) {
                    None
                } else {
                    infer_module_types(self.db, module)
                }
            }
        }
    }

    /// Converts a raw type reference to an inferred type.
    ///
    /// Recursive conversion is limited to 64 active calls shared by resolved
    /// IDs, qualifiers, and imports in this context. A reference that would
    /// exceed the limit resolves to `Unknown`; its caller may still construct a
    /// surrounding type containing that `Unknown`. Named declarations usually
    /// avoid this recursion by resolving to symbolic local handles.
    pub(in crate::db) fn resolve(&mut self, reference: &TypeReference) -> InferredTypeData<'db> {
        let resolution_depth = self.resolution_depth.get();
        if resolution_depth >= MAX_RAW_TYPE_RESOLUTION_DEPTH {
            return InferredTypeData::Unknown;
        }

        self.resolution_depth.set(resolution_depth + 1);
        let resolved = match reference {
            TypeReference::Resolved(resolved_id) => self.resolve_resolved_id(*resolved_id),
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Import(import) => self.resolve_import(import),
        };
        self.resolution_depth.set(resolution_depth);
        resolved
    }

    pub(in crate::db::type_inference) fn resolve_resolved_id(
        &mut self,
        resolved_id: ResolvedTypeId,
    ) -> InferredTypeData<'db> {
        match resolved_id.level() {
            TypeResolverLevel::Thin => self.resolve_raw_type_reference(resolved_id.id()),
            TypeResolverLevel::Global => GlobalTypeId::try_from_type_id(resolved_id.id())
                .map_or(InferredTypeData::Unknown, |id| global_type(self.db, id)),
            TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
        }
    }

    fn resolve_raw_type_reference(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if self.js_info.is_named_type(type_id) {
            return self.local_type(type_id);
        }

        self.resolve_raw_type_id(type_id)
    }

    fn local_type(&self, type_id: TypeId) -> InferredTypeData<'db> {
        InferredTypeData::Local(LocalTypeHandle::new(
            self.db,
            self.module_key,
            LocalTypeId::new(type_id.index()),
        ))
    }

    pub(in crate::db) fn resolve_raw_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if let Some(ty) = self.resolved.get(&type_id) {
            return *ty;
        }

        if !self.in_progress.insert(type_id) {
            // A resolution cycle keeps the reference symbolic: lookups that
            // are aware of in-progress types can still read the raw
            // declaration behind the handle, which `Unknown` would rule out.
            return self.local_type(type_id);
        }

        let js_info = self.js_info;
        let ty = js_info
            .raw_types
            .get(type_id.index())
            .map_or(InferredTypeData::Unknown, |raw| self.resolve_raw_type(raw));
        let ty = self.with_namespace_members(type_id, ty);

        self.in_progress.remove(&type_id);
        self.resolved.insert(type_id, ty);
        ty
    }

    fn with_namespace_members(
        &mut self,
        type_id: TypeId,
        fallback: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        enum NamespaceMetadata {
            Module(biome_rowan::Text),
            Namespace(biome_js_type_info::Path),
        }

        let metadata = match self.js_info.raw_types.get(type_id.index()) {
            Some(RawTypeData::Module(module)) => NamespaceMetadata::Module(module.name.clone()),
            Some(RawTypeData::Namespace(namespace)) => {
                NamespaceMetadata::Namespace(namespace.path.clone())
            }
            _ => return fallback,
        };
        let resolves_declarations_directly = self.resolves_declarations_directly();
        let members = self
            .js_info
            .semantic_model
            .all_bindings()
            .filter_map(|binding| {
                let reference = self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())?;
                if !matches!(
                    reference,
                    TypeReference::Resolved(id)
                        if id.level() == TypeResolverLevel::Thin && id.id() == type_id
                ) {
                    return None;
                }
                let declaration = binding.tree().declaration()?;
                TsModuleDeclaration::cast(declaration.syntax().clone())
            })
            .flat_map(|declaration| {
                self.js_info
                    .semantic_model
                    .scope(declaration.syntax())
                    .bindings()
            })
            .filter_map(|binding| {
                let name = binding
                    .tree()
                    .name_token()
                    .ok()?
                    .token_text_trimmed()
                    .into();
                let range = binding.syntax().text_trimmed_range();
                let reference = self.js_info.raw_binding_types.get(&range)?.clone();
                Some((name, range, reference))
            })
            .map(|(name, range, reference)| InferredTypeMember {
                kind: InferredTypeMemberKind::NamedStatic(name),
                ty: if resolves_declarations_directly {
                    self.resolve_local_binding(range)
                } else {
                    self.resolve(&reference)
                },
            })
            .collect::<Box<[_]>>();

        match metadata {
            NamespaceMetadata::Module(name) => {
                InferredTypeData::Module(InferredModule::new(self.db, members, name))
            }
            NamespaceMetadata::Namespace(path) => {
                InferredTypeData::Namespace(InferredNamespace::new(self.db, members, path))
            }
        }
    }

    fn resolve_raw_type(&mut self, raw: &RawTypeData) -> InferredTypeData<'db> {
        if let RawTypeData::TypeofExpression(expression) = raw
            && let Some(ty) = self.resolve_typeof_expression(expression)
        {
            return ty;
        }

        if let RawTypeData::TypeofValue(value) = raw {
            let ty = if value.ty.is_unknown() {
                self.resolve_qualifier(&TypeReferenceQualifier::from_path(
                    value.scope_id.unwrap_or(ScopeId::GLOBAL),
                    value.identifier.clone(),
                ))
            } else {
                self.resolve(&value.ty)
            };
            return InferredTypeData::TypeofValue(InternedTypeofValue::new(
                self.db,
                ty,
                value.identifier.clone(),
                value.scope_id,
            ));
        }

        let db = self.db;
        let ty = InferredTypeData::from_raw_with_resolver(db, raw, false, &mut |reference| {
            self.resolve(reference)
        });
        self.resolve_inferred_expression_wrappers(ty)
    }

    fn resolve_inferred_expression_wrappers(
        &mut self,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        for _ in 0..MAX_INFERRED_EXPRESSION_WRAPPER_STEPS {
            match ty {
                InferredTypeData::TypeofExpression(expression) => {
                    ty = self
                        .resolve_inferred_typeof_expression(expression.expression(self.db))
                        .unwrap_or(InferredTypeData::Unknown);
                }
                InferredTypeData::InstanceOf(instance) => {
                    let target = instance.ty(self.db);
                    let InferredTypeData::TypeofExpression(expression) = target else {
                        return ty;
                    };
                    let target = self
                        .resolve_inferred_typeof_expression(expression.expression(self.db))
                        .unwrap_or(InferredTypeData::Unknown);
                    if target.should_flatten_instance(instance.type_parameters(self.db)) {
                        ty = target;
                    } else {
                        return InferredTypeData::instance_of(
                            self.db,
                            target,
                            instance
                                .type_parameters(self.db)
                                .to_vec()
                                .into_boxed_slice(),
                        );
                    }
                }
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => return ty,
            }
        }

        ty
    }

    pub(in crate::db::type_inference) fn resolve_inferred_type(
        &mut self,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(self.db);
            let local_type_id = local.type_id(self.db);
            if !seen.insert((module_key, local_type_id)) {
                return ty;
            }

            ty = if module_key == self.module_key {
                self.resolve_raw_type_id(TypeId::new(local_type_id.index()))
            } else {
                module_for_key(self.db, module_key)
                    .and_then(|module| {
                        let input = LocalTypeInput::new(self.db, module, local_type_id);
                        infer_local_type(self.db, input)
                    })
                    .unwrap_or(InferredTypeData::Unknown)
            };
        }

        ty
    }
}

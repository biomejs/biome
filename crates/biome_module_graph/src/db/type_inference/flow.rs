//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Collection records the incoming flow of each refinable read, the guards
//! that the conditions of its execution root apply, and its writes (see
//! [`crate::js_module_info::flow`]). This module resolves the type that the
//! read's subject starts from, the types that its writes assign, what the
//! call points that may narrow it do, and which statement calls before it
//! never return. It applies the guards and the writes along every path that
//! reaches the read. The solve's key carries all of these, so a solve
//! resolves no declared type, assigned value, or callee.
//! A variable starts from its declared type. In a closure that continues the
//! flow where it is created, the variable starts instead from its type at
//! that point, its capture point. The solve of the enclosing root gives that
//! type, and the solve of the closure's root starts from it. The declared
//! type of a property path at a read is the type of the member access there,
//! given the narrowed type of its object, as TypeScript types property reads.
//! Likewise, the declared type of an element of a destructuring pattern at a
//! read is its field on the pattern narrowed there.

use super::flow_expressions::collected_member_name;
use super::imports::{ExportOriginResult, find_export_origin};
use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, CallableFunctionInput, FlowAssignedTypes, FlowCallEffect, FlowCallEffects,
    FlowCallInput, FlowCallParameter, FlowComparedTypes, FlowPredicateInput, FlowReachability,
    FlowReachabilityInput, FlowSubjectRootInput, FlowSubjectTypeInput, FlowSubjectWritesInput,
    FlowSwitchInput, LocalTypeInput, global_contributions, infer_flow_assigned_types,
    infer_flow_binding_baseline, infer_flow_call_effect, infer_flow_compared_types,
    infer_flow_inferred_predicate, infer_flow_reachability, infer_flow_subject_states,
    infer_flow_subject_type, infer_flow_switch_exhaustive, infer_local_type,
    resolve_callable_function,
};
use crate::js_module_info::flow::{
    AnyMemberDeclaration, CalleeReturn, DeclaredReturn, FlowAssigned, FlowAssignedValue, FlowCall,
    FlowCallKind, FlowCallRoot, FlowCallee, FlowCapture, FlowComparedValue, FlowEvolving,
    FlowGuard, FlowNode, FlowNodeId, FlowPredicate, FlowRead, FlowSubject, FlowSubjectId,
    FlowSwitch, FlowTest, FlowWriteEffect, PropertyKey, ReachabilityGraph, RootFlow, UnitValue,
    binding_unit_value, build_reachability, declaration_return, declared_member_returns,
    declared_return, enum_member_name_range, enum_member_unit_value, guard_signature,
    member_explicitness, member_key, member_return,
};
use crate::module_graph::ModuleInfoKind;
use crate::type_inference::TypeInferenceClassification;
use crate::{
    JsExport, JsModuleInfo, JsOwnExport, ModuleDb, ModuleInfo, ResolutionMode, module_for_key,
    resolve_module_import,
};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::ScopeId;
use biome_js_syntax::{
    AnyJsExpression, JsIdentifierExpression, JsReferenceIdentifier, TsDeclareStatement,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_js_type_info::interned_types::{
    ClassParents, FunctionParameter, InternedFunction, Literal, ReturnType, TypeData, TypeMember,
};
use biome_js_type_info::{
    ImportSymbol, Literal as RawLiteral, NarrowingPredicate, ObjectKind, PropertyTest, RawTypeData,
    RawTypeId, TypeId, TypeReference, TypeofExpression, base_literal_type, evolving_array_element,
    evolving_assigned_type, has_readonly_property, instantiate_intersection_alias,
    is_narrowing_invariant, is_type_parameter_or_intersection, may_narrow_by_instanceof,
    may_narrow_by_property, narrow_by_assignment, narrow_by_constructor, narrow_by_in_operator,
    narrow_by_instanceof, narrow_by_private_name, narrow_by_property, narrow_by_type_predicate,
    narrow_type, type_parameter_constraint,
};
use biome_rowan::{AstNode, NodeOrToken, Text, TextRange, TextSize, TokenText};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

const MAX_FLOW_STEPS: usize = 16_384;
/// Limits the nodes that [`ResolutionCtx::reachability`] visits in one root.
const MAX_REACHABILITY_STEPS: usize = 1_048_576;
/// Limits the handles to local types of a module that
/// [`ResolutionCtx::resolve_module_handle`] follows.
const MAX_MODULE_HANDLE_STEPS: usize = 16;
/// Limits the types visited while flattening a flow baseline or checking the
/// members of a refined union.
pub(super) const MAX_FLOW_TYPE_STEPS: usize = 1024;

pub(in crate::db) fn flow_expression_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    range: TextRange,
) -> Option<TypeData<'db>> {
    // A candidate made only by reads of a binding that no test of a property
    // can narrow keeps its type, as in `local_flow_expression_type`.
    if let Some(binding) = info.flow.property_only_candidate(range)
        && infer_flow_binding_baseline(db, BindingTypeInput::new(db, module, binding)).is_none()
    {
        return None;
    }
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    // A candidate made only by reads that only call points reach keeps its
    // type when none of those calls narrows.
    if let Some((root, subject)) = info.flow.call_only_candidate(range)
        && ctx.call_effects(root, subject).is_none()
    {
        return None;
    }
    let tree = info.semantic_model.root();
    let expression = info.flow.candidate(range)?.try_to_node(tree.syntax())?;
    ctx.resolve_flow_expression(&expression)
}

pub(in crate::db) fn flow_binding_baseline<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    range: TextRange,
) -> Option<TypeData<'db>> {
    // The declared type is resolved from the raw type of the binding, or of
    // the value that a pattern destructures, not through a declaration query,
    // because declaration queries can erase a cyclic object's shape. The raw
    // type still lets flow detect an impossible branch and return an unknown
    // override.
    let reference = info
        .raw_binding_types
        .get(&range)
        .or_else(|| info.flow.pattern_type(range))?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    let declared = ctx.resolve(reference);
    ctx.binding_baseline(range, declared)
}

pub(in crate::db) fn flow_subject_states<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    input: FlowSubjectRootInput<'db>,
) -> Option<Box<[(FlowNodeId, TypeData<'db>)]>> {
    let root = info.flow.root(input.root(db))?;
    let subject = input.subject(db);
    let points = root.read_points(subject);
    if points.is_empty() {
        return None;
    }
    let baseline = input.baseline(db)?;
    if baseline == TypeData::Unknown {
        return None;
    }
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    let states = ctx.solve_subject(root, points, input, baseline)?;
    Some(points.iter().copied().zip(states).collect())
}

pub(in crate::db) fn flow_subject_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    input: FlowSubjectTypeInput<'db>,
) -> Option<TypeData<'db>> {
    let states = input.states(db);
    let root = info.flow.root(states.root(db))?;
    let point = input.point(db);
    if point as usize >= root.nodes.len() {
        return None;
    }
    let baseline = states.baseline(db)?;
    if baseline == TypeData::Unknown {
        return Some(TypeData::Unknown);
    }
    let shared = infer_flow_subject_states(db, states).and_then(|states| {
        let index = states
            .binary_search_by_key(&point, |(point, _)| *point)
            .ok()?;
        Some(states[index].1)
    });
    let narrowed = match shared {
        Some(narrowed) => narrowed,
        None => {
            let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
            match ctx.solve_subject(root, &[point], states, baseline) {
                Some(states) => states[0],
                None => return Some(TypeData::Unknown),
            }
        }
    };
    if narrowed == TypeData::NeverKeyword {
        return Some(TypeData::Unknown);
    }
    match states.evolving(db) {
        FlowEvolving::None => {}
        FlowEvolving::Variable => return Some(narrowed),
        FlowEvolving::Array => return evolving_array_read(db, narrowed),
    }
    // TypeScript reports a read that may come before the variable or the
    // field is assigned (TS2454, TS2565) and gives it the declared type.
    if root.is_uninitialized(states.subject(db))
        && contains_undefined(db, narrowed)
        && !contains_undefined(db, baseline)
    {
        return None;
    }
    (narrowed != baseline).then_some(narrowed)
}

pub(in crate::db) fn flow_assigned_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    root: usize,
    subject: FlowSubjectId,
) -> Option<FlowAssignedTypes<'db>> {
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.assigned_types(root, subject)
}

/// Returns the type of a read of an array whose type evolves with its writes,
/// given the array's type `state` at the read. The result is an array of the
/// element type of `state`.
///
/// The read is `Unknown` when `state` is not an array, or has a member that
/// isn't one: a write that reaches the read may assign a value other than an
/// array, or a path reaches it without passing the declarator of a `var`.
/// The declared type, `unknown[]`, lacks such a value.
///
/// Returns `None`, so that the read keeps its declared type, in two cases,
/// where `unknown[]` is wider than TypeScript's type. When nothing has been
/// added to the array, TypeScript gives the read an implicit `any[]`
/// (TS7005). When the element type is `Unknown`, a write that reaches the
/// read adds a value that Biome can't type as TypeScript does.
fn evolving_array_read<'db>(db: &'db dyn ModuleDb, state: TypeData<'db>) -> Option<TypeData<'db>> {
    let Some(element) = evolving_array_element(db, state) else {
        return Some(TypeData::Unknown);
    };
    (!matches!(element, TypeData::NeverKeyword | TypeData::Unknown)).then(|| array_of(db, element))
}

/// Returns `Array<element>`.
fn array_of<'db>(db: &'db dyn ModuleDb, element: TypeData<'db>) -> TypeData<'db> {
    TypeData::instance_of(db, TypeData::array_class(), Box::new([element]))
}

/// Returns whether `ty` is `undefined` or a union with an `undefined` member.
fn contains_undefined(db: &dyn ModuleDb, ty: TypeData) -> bool {
    if let TypeData::Union(union) = ty {
        union.types(db).contains(&TypeData::Undefined)
    } else {
        ty == TypeData::Undefined
    }
}

/// Returns whether a write can change a subject whose declared type is
/// `baseline`. An assignment narrows only a union, and a compound assignment
/// changes only a literal type.
fn may_narrow_by_assignment(baseline: TypeData) -> bool {
    matches!(
        baseline,
        TypeData::Union(_) | TypeData::Literal(_) | TypeData::Unknown
    )
}

pub(in crate::db) fn flow_compared_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    root: usize,
    subject: FlowSubjectId,
) -> Option<FlowComparedTypes<'db>> {
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.resolve_compared_types(root, subject)
}

pub(in crate::db) fn flow_call_effect<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    call: TextRange,
) -> FlowCallEffect<'db> {
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.call_effect(call)
}

pub(in crate::db) fn flow_reachability(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    root: TextRange,
) -> Option<FlowReachability> {
    let tree = info.semantic_model.root();
    let start = match tree.syntax().covering_element(root) {
        NodeOrToken::Node(node) => node,
        NodeOrToken::Token(token) => token.parent()?,
    };
    let root = start
        .ancestors()
        .filter_map(AnyJsControlFlowRoot::cast)
        .find(|candidate| candidate.syntax().text_trimmed_range() == root)?;
    let graph = build_reachability(
        &root,
        &info.semantic_model,
        &info.raw_types,
        &info.raw_binding_types,
    )?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.reachability(&graph)
}

pub(in crate::db) fn flow_inferred_predicate<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    function: TextRange,
) -> Option<TypeData<'db>> {
    let predicate = info.flow.predicate(function)?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.inferred_predicate(predicate)
}

pub(in crate::db) fn flow_switch_exhaustive(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    root: usize,
    switch: u32,
) -> bool {
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.switch_is_exhaustive(root, switch)
}

/// Why [`ResolutionCtx::flatten_baseline`] gives no type.
enum FlatteningError {
    /// Flattening exceeded its work limit.
    Exceeded,
    /// Handles to local types of the module form a cycle or a chain longer
    /// than [`MAX_MODULE_HANDLE_STEPS`].
    Cycle,
}

/// The state of one solve that its guards share: the subject it narrows, the
/// subject's declared type at the root's start, whether the subject is a
/// readonly path, as [`FlowGuard::ReadonlyPath`] requires, the `switch`
/// statements found exhaustive, as [`FlowGuard::Bypass`] requires, one bit
/// each, the types that the subject's writes assign, how its type evolves
/// with its writes ([`FlowEvolving`]), the effects of the call
/// points that narrow it, as [`FlowGuard::Call`] requires, the statement
/// calls found to end the path, one bit each, the results of the
/// property guards, `in` tests, `instanceof` tests, and calls to type-guard
/// functions applied so far, keyed by guard index and incoming type, the
/// values of the imported variables and enum members that
/// [`FlowTest::Imported`] compares with, found so far, keyed by declaration
/// or by the range of the member's name, and the type that each write of the
/// subject that assigns a value gives it.
struct GuardSolve<'db> {
    /// The subject that the solve narrows.
    subject: FlowSubjectId,
    /// The subject's declared type at the root's start.
    declared: TypeData<'db>,
    /// Whether the subject is a readonly path, as [`FlowGuard::ReadonlyPath`]
    /// requires.
    readonly_path: bool,
    /// The `switch` statements found exhaustive, one bit each, as
    /// [`FlowGuard::Bypass`] requires.
    exhaustive: u64,
    /// The types that the subject's writes assign.
    assigned: Option<FlowAssignedTypes<'db>>,
    evolving: FlowEvolving,
    calls: Option<FlowCallEffects<'db>>,
    ends: u64,
    /// The declared types of the values that comparisons narrow the subject
    /// by, as [`FlowTest::Compared`] requires.
    compared: Option<FlowComparedTypes<'db>>,
    properties: FxHashMap<(usize, TypeData<'db>), TypeData<'db>>,
    /// The values of the imported variables and enum members that
    /// [`FlowTest::Imported`] compares with, found so far, keyed by the
    /// declaration of the variable, or by the range of the member's name for
    /// a member.
    imported: FxHashMap<TextRange, Option<TypeData<'db>>>,
    /// The type that each write of the subject that assigns a value gives it,
    /// keyed by write index.
    writes: FxHashMap<u32, TypeData<'db>>,
}

impl<'db> GuardSolve<'db> {
    #[expect(
        clippy::too_many_arguments,
        reason = "Each argument is a part of the solve's key that its guards read."
    )]
    fn new(
        subject: FlowSubjectId,
        declared: TypeData<'db>,
        readonly_path: bool,
        exhaustive: u64,
        assigned: Option<FlowAssignedTypes<'db>>,
        evolving: FlowEvolving,
        calls: Option<FlowCallEffects<'db>>,
        ends: u64,
        compared: Option<FlowComparedTypes<'db>>,
    ) -> Self {
        Self {
            subject,
            declared,
            readonly_path,
            exhaustive,
            assigned,
            evolving,
            calls,
            ends,
            compared,
            properties: FxHashMap::default(),
            imported: FxHashMap::default(),
            writes: FxHashMap::default(),
        }
    }
}

impl<'db> GuardSolve<'db> {
    /// Returns the declared type of the compared value identified by `range`
    /// in the solve's key, or `None` when the key has none.
    fn compared_type(&self, db: &'db dyn ModuleDb, range: TextRange) -> Option<TypeData<'db>> {
        let types = self.compared?.types(db);
        let key = (u32::from(range.start()), u32::from(range.end()));
        let index = types
            .binary_search_by_key(&key, |(start, end, _)| (*start, *end))
            .ok()?;
        Some(types[index].2)
    }
}

/// Returns the predicate of a strict or loose comparison with `value`, the
/// type of a unit value, as a literal written in the comparison gives it.
fn unit_predicate(value: TypeData, strict: bool) -> NarrowingPredicate {
    if strict {
        NarrowingPredicate::Literal(value)
    } else {
        NarrowingPredicate::LooseLiteral(value)
    }
}

/// Marks a root node that the walk in [`ReachingFlow::new`] has not reached.
const UNREACHED: u32 = u32::MAX;

/// The nodes of a root's flow graph that lie on some path from its start to
/// at least one of several points.
///
/// Reached nodes are numbered from zero in the order a backward walk from the
/// points first visits them. The solver keeps its per-node state over these
/// numbers, so that state grows with the reached nodes rather than with the
/// whole root.
struct ReachingFlow {
    /// The root's id of each reached node.
    nodes: Vec<FlowNodeId>,
    /// The reached nodes that flow directly into each reached node.
    predecessors: NodeLists,
    /// The reached nodes that each reached node flows directly into, in
    /// ascending order.
    successors: NodeLists,
    /// The number of each point the walk started from, in the order given.
    points: Vec<u32>,
}

impl ReachingFlow {
    /// Walks backward from `points` through the predecessors in `root`.
    ///
    /// Returns `None` when the walk exceeds its work limit or meets a node
    /// that `root` does not contain.
    fn new(root: &RootFlow, points: &[FlowNodeId]) -> Option<Self> {
        let mut numbering = vec![UNREACHED; root.nodes.len()];
        let mut nodes = Vec::new();
        let mut pending = points.to_vec();
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(node) = pending.pop() {
            remaining = remaining.checked_sub(1)?;
            let number = numbering.get_mut(node as usize)?;
            if *number != UNREACHED {
                continue;
            }
            *number = u32::try_from(nodes.len()).ok()?;
            nodes.push(node);
            pending.extend(predecessor_ids(&root.nodes[node as usize]));
        }
        let mut predecessors = NodeLists {
            numbers: Vec::new(),
            ends: Vec::with_capacity(nodes.len()),
        };
        for node in &nodes {
            let ids = predecessor_ids(&root.nodes[*node as usize]);
            predecessors
                .numbers
                .extend(ids.iter().map(|id| numbering[*id as usize]));
            predecessors
                .ends
                .push(u32::try_from(predecessors.numbers.len()).ok()?);
        }
        let successors = predecessors.inverted();
        let points = points
            .iter()
            .map(|point| numbering[*point as usize])
            .collect();
        Some(Self {
            nodes,
            predecessors,
            successors,
            points,
        })
    }
}

/// Lists of reached node numbers stored back to back, one list per reached
/// node.
struct NodeLists {
    numbers: Vec<u32>,
    /// The end of each node's list in `numbers`; a list starts where the
    /// previous node's list ends.
    ends: Vec<u32>,
}

impl NodeLists {
    /// Returns the list of the reached node `number`.
    fn get(&self, number: u32) -> &[u32] {
        let number = number as usize;
        let start = number
            .checked_sub(1)
            .map_or(0, |previous| self.ends[previous]);
        &self.numbers[start as usize..self.ends[number] as usize]
    }

    /// Returns, for each node, the nodes whose lists contain it, in ascending
    /// order.
    fn inverted(&self) -> Self {
        // Each counter starts at its list's start and advances past every
        // number written into the list, so it finishes at the list's end.
        let mut ends = vec![0; self.ends.len()];
        for &number in &self.numbers {
            ends[number as usize] += 1;
        }
        let mut start = 0;
        for end in &mut ends {
            let size = *end;
            *end = start;
            start += size;
        }
        let mut numbers = vec![0; self.numbers.len()];
        for (node, _) in (0..).zip(&self.ends) {
            for &other in self.get(node) {
                let end = &mut ends[other as usize];
                numbers[*end as usize] = node;
                *end += 1;
            }
        }
        Self { numbers, ends }
    }
}

/// Returns the ids of the nodes that flow directly into `node`.
fn predecessor_ids(node: &FlowNode) -> &[FlowNodeId] {
    match node {
        FlowNode::Start => &[],
        FlowNode::Join(predecessors) => predecessors,
        FlowNode::Condition { antecedent, .. } | FlowNode::Write { antecedent, .. } => {
            std::slice::from_ref(antecedent)
        }
    }
}

/// Flow results that a [`ResolutionCtx`] evaluates with its own resolved
/// types instead of through the flow expression and baseline queries.
///
/// Complete module inference uses it: by the time it reaches expressions, its
/// context has resolved every raw type of the module, so a fresh context in a
/// query would only resolve those types again.
#[derive(Default)]
pub(in crate::db) struct LocalFlow<'db> {
    /// The flow-sensitive type of each candidate expression evaluated so far.
    expressions: FxHashMap<TextRange, Option<TypeData<'db>>>,
    /// The type that narrowing starts from for each binding read so far.
    baselines: FxHashMap<TextRange, Option<TypeData<'db>>>,
    /// Caches for [`ResolutionCtx::is_readonly_path`] whether each property
    /// path read so far is readonly, keyed by the index of its root and its
    /// subject.
    readonly_paths: FxHashMap<(usize, FlowSubjectId), bool>,
    /// Whether each `switch` statement asked about so far is exhaustive,
    /// keyed by the index of its root and its own index there.
    exhaustive_switches: FxHashMap<(usize, u32), bool>,
    /// The types that the writes of each subject read so far assign, keyed
    /// by the index of its root and its subject, as
    /// [`ResolutionCtx::assigned_types`] resolves them.
    assigned: FxHashMap<(usize, FlowSubjectId), Option<FlowAssignedTypes<'db>>>,
    /// The declared types of the values that the comparisons of each subject
    /// read so far narrow it by, keyed by the index of its root and its
    /// subject, as [`ResolutionCtx::resolve_compared_types`] resolves them.
    compared: FxHashMap<(usize, FlowSubjectId), Option<FlowComparedTypes<'db>>>,
    /// The type of each variable at each capture point resolved so far,
    /// keyed by the index of the enclosing root, the variable there, and the
    /// point, as [`ResolutionCtx::captured_type`] resolves it.
    captures: FxHashMap<(usize, FlowSubjectId, FlowNodeId), Option<(TypeData<'db>, bool)>>,
    /// The effect of each call point resolved so far, keyed by the range of
    /// its call, as [`ResolutionCtx::call_effect`] resolves it.
    call_effects: FxHashMap<TextRange, FlowCallEffect<'db>>,
    /// The effects of the call points that narrow each subject read so far,
    /// keyed by the index of its root and its subject, as
    /// [`ResolutionCtx::call_effects`] gives them.
    subject_call_effects: FxHashMap<(usize, FlowSubjectId), Option<FlowCallEffects<'db>>>,
    /// Whether a path reaches each read point asked about so far without
    /// crossing a call that never returns, keyed by the index of its root, the
    /// point, and the mask of the calls found to never return. Filled by
    /// [`ResolutionCtx::ending_calls`].
    reached: FxHashMap<(usize, FlowNodeId, u64), bool>,
}

impl<'db> ResolutionCtx<'db, '_> {
    /// Makes this context evaluate flow candidates with its own resolved
    /// types, as [`Self::local_flow_expression_type`] describes.
    pub(in crate::db) fn evaluate_flow_locally(&mut self) {
        self.local_flow = Some(Box::default());
    }

    /// Returns the flow-sensitive type of the expression at `range`, as
    /// [`crate::db::queries::infer_flow_expression_type`] returns it, but
    /// evaluated in this context.
    ///
    /// Bindings start narrowing from their collected types resolved in this
    /// context, and the solves of their roots are shared with lookups that
    /// start from the same types. Returns `None` when this context does not
    /// evaluate flow locally, when the expression is not a collected flow
    /// candidate, or when no runtime test changes its type.
    pub(in crate::db) fn local_flow_expression_type(
        &mut self,
        range: TextRange,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        if !info.flow.is_candidate(range) || !info.raw_expressions.contains_key(&range) {
            return None;
        }
        if let Some(ty) = self.local_flow.as_ref()?.expressions.get(&range) {
            return *ty;
        }
        // A candidate made only by reads of a binding that conditions narrow
        // only through tests of its properties keeps its type when no such
        // test can narrow the binding. The binding's baseline shows that
        // without the syntax node.
        if let Some(binding) = info.flow.property_only_candidate(range)
            && self.local_binding_baseline(binding).is_none()
        {
            self.local_flow.as_mut()?.expressions.insert(range, None);
            return None;
        }
        // Likewise, a candidate made only by reads that only call points
        // reach keeps its type when none of those calls narrows.
        if let Some((root, subject)) = info.flow.call_only_candidate(range)
            && self.call_effects(root, subject).is_none()
        {
            self.local_flow.as_mut()?.expressions.insert(range, None);
            return None;
        }
        let tree = info.semantic_model.root();
        let ty = info
            .flow
            .candidate(range)?
            .try_to_node(tree.syntax())
            .and_then(|expression| self.resolve_flow_expression(&expression));
        self.local_flow.as_mut()?.expressions.insert(range, ty);
        ty
    }

    /// Returns whether this context evaluates flow candidates itself.
    pub(super) fn evaluates_flow_locally(&self) -> bool {
        self.local_flow.is_some()
    }

    /// Returns the flow-sensitive type of the expression at `range`, as
    /// [`Self::local_flow_expression_type`] does, evaluated in this context
    /// whether or not it evaluates flow locally, and without reading or
    /// filling a cache. [`Self::flow_operand_at`] uses it while the
    /// arguments of a call point are read, whose types differ from those of
    /// ordinary reads (see [`ResolutionCtx::calls_before`]).
    pub(super) fn uncached_flow_expression_type(
        &mut self,
        range: TextRange,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        if !info.flow.is_candidate(range) || !info.raw_expressions.contains_key(&range) {
            return None;
        }
        if let Some(binding) = info.flow.property_only_candidate(range)
            && self.flow_binding_baseline(binding).is_none()
        {
            return None;
        }
        if let Some((root, subject)) = info.flow.call_only_candidate(range)
            && self.call_effects(root, subject).is_none()
        {
            return None;
        }
        let tree = info.semantic_model.root();
        let expression = info.flow.candidate(range)?.try_to_node(tree.syntax())?;
        self.resolve_flow_expression(&expression)
    }

    /// Returns the narrowed type of the variable read at `identifier`.
    ///
    /// When the variable is an element of a destructuring pattern that a test
    /// narrows at the read, as `v` in `const { kind, v } = s` after
    /// `kind === "c"`, its declared type there is its property on the
    /// narrowed pattern. Tests of the variable itself then narrow it from that
    /// type, as tests of a property path narrow it from its type on the
    /// narrowed object.
    ///
    /// In a closure that continues the flow of an enclosing root, the
    /// variable starts from its type at the capture point, where the
    /// enclosing root creates the closure (see [`Self::narrow_captured`]).
    ///
    /// A read of a variable whose type evolves with its writes, in code that
    /// the root declaring it doesn't run in its own flow, is Unknown when
    /// collection recorded it so (see
    /// [`ModuleFlow::is_outer_evolving_read`]).
    ///
    /// Returns `None` when collection kept no flow for the read, when no
    /// runtime test can change the variable's type, or when narrowing leaves
    /// it unchanged.
    ///
    /// [`ModuleFlow::is_outer_evolving_read`]: crate::js_module_info::flow::ModuleFlow::is_outer_evolving_read
    pub(super) fn narrow_reference(
        &mut self,
        identifier: &JsIdentifierExpression,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        if info.flow.is_outer_evolving_read(identifier.range()) {
            return Some(TypeData::Unknown);
        }
        let read = info.flow.read(identifier.range())?;
        let root = info.flow.root(read.root)?;
        let &FlowSubject::Binding(binding) = root.subject(read.subject)? else {
            return None;
        };
        let start = identifier.range().start();
        let evolving = root.evolving(read.subject);
        if evolving != FlowEvolving::None {
            return self.narrow_evolving(read, start, evolving);
        }
        let declared = root
            .element_pattern(read.subject)
            .and_then(|pattern| self.narrowed_element(read, start, pattern, binding));
        // The read is a candidate only through its pattern when no test of
        // the variable itself reaches it.
        if !root.reads_at(read.subject, read.point) {
            return declared;
        }
        if declared.is_none() {
            if let Some(capture) = root.capture(read.subject) {
                return self.narrow_captured(read, start, capture, binding);
            }
            // A read in a closure that the root creates assumes the variable
            // assigned where it is declared when some code assigns it.
            if read.assumes_initialized && root.is_uninitialized(read.subject) {
                let capture = FlowCapture {
                    root: read.root,
                    point: read.point,
                    subject: read.subject,
                    start,
                    assumes_initialized: true,
                };
                let (captured, _) = self.captured_type(capture, binding)?;
                let declared = self.flow_binding_baseline(binding)?;
                let ty = self.assumed_initialized(captured, declared);
                return (ty != declared).then_some(ty);
            }
        }
        let calls = self.call_effects(read.root, read.subject);
        let baseline = match declared {
            Some(declared) => {
                let baseline = if root.is_property_only(read.subject) && calls.is_none() {
                    self.property_baseline(declared, root.is_instance_tested(read.subject))
                } else {
                    self.narrowing_baseline(declared)
                };
                match baseline {
                    Some(baseline) => baseline,
                    None => return Some(declared),
                }
            }
            None if calls.is_some() => self.called_binding_baseline(binding)?,
            None => self.flow_binding_baseline(binding)?,
        };
        if !self.may_change_baseline(read.root, read.subject, baseline) {
            return declared;
        }
        let exhaustive = self.exhaustive_switches(read, start);
        let assigned = self.flow_assigned_types(read.root, read.subject);
        let ends = self.ending_calls(read)?;
        infer_flow_subject_type(
            self.db,
            FlowSubjectTypeInput::new(
                self.db,
                FlowSubjectRootInput::new(
                    self.db,
                    self.module,
                    read.root,
                    read.subject,
                    Some(baseline),
                    false,
                    exhaustive,
                    assigned,
                    FlowEvolving::None,
                    calls,
                    ends,
                    self.compared_types(read.root, read.subject),
                ),
                read.point,
            ),
        )
        .or(declared)
    }

    /// Returns the type of the variable read at `read`, whose expression
    /// starts at `start`, when its type evolves with its writes as `evolving`
    /// describes. The type comes from the writes that reach the read, starting
    /// from `undefined` where the variable is declared, and ignores the
    /// declared type.
    ///
    /// A variable is Unknown where a write that reaches the read assigns a
    /// value that Biome can't type as TypeScript does, unless the variable's
    /// declared type covers the value, as [`FlowAssigned::covered`]
    /// describes. An array is Unknown where a write that reaches the read
    /// assigns such a value or one that isn't an array, and where a path
    /// reaches the read without passing the declarator of a `var`, because
    /// its declared type, `unknown[]`, lacks those values. An array keeps
    /// that declared type where a write adds a value that Biome can't type,
    /// or where nothing has been added since an empty array literal, because
    /// `unknown[]` is wider than TypeScript's type there.
    ///
    /// Two cases need no solve:
    ///
    /// - When the root adds to an array only values that Biome can't type,
    ///   or nothing, and the array is an array at every read, as
    ///   [`RootFlow::is_always_array`] describes, this returns `None`, so
    ///   that the read keeps its declared type, `unknown[]`.
    /// - When every write of a variable gives `null`, `undefined`, or a value
    ///   that Biome can't type, and at least one gives such a value, the read
    ///   is Unknown. A solve could give `null` or `undefined` at a read that
    ///   only such values reach, but the untyped value reaches most reads, as
    ///   for `ctx` after `ctx = get(ctx)` in `let ctx = undefined`.
    fn narrow_evolving(
        &mut self,
        read: FlowRead,
        start: TextSize,
        evolving: FlowEvolving,
    ) -> Option<TypeData<'db>> {
        let assigned = self.flow_assigned_types(read.root, read.subject);
        let types = assigned.map_or(&[][..], |assigned| assigned.types(self.db).as_ref());
        match evolving {
            FlowEvolving::Variable => {
                if types.iter().all(|ty| {
                    matches!(ty, TypeData::Null | TypeData::Undefined | TypeData::Unknown)
                }) && (types.contains(&TypeData::Unknown)
                    || self
                        .js_info
                        .flow
                        .root(read.root)?
                        .has_valueless_write(read.subject))
                {
                    return Some(TypeData::Unknown);
                }
            }
            FlowEvolving::Array => {
                if types.iter().all(|ty| *ty == TypeData::Unknown)
                    && self
                        .js_info
                        .flow
                        .root(read.root)?
                        .is_always_array(read.subject)
                {
                    return None;
                }
            }
            FlowEvolving::None => return None,
        }
        let exhaustive = self.exhaustive_switches(read, start);
        // Runtime tests don't narrow an array whose type evolves with its
        // writes.
        let calls = if evolving == FlowEvolving::Variable {
            self.call_effects(read.root, read.subject)
        } else {
            None
        };
        let ends = self.ending_calls(read)?;
        infer_flow_subject_type(
            self.db,
            FlowSubjectTypeInput::new(
                self.db,
                FlowSubjectRootInput::new(
                    self.db,
                    self.module,
                    read.root,
                    read.subject,
                    Some(TypeData::Undefined),
                    false,
                    exhaustive,
                    assigned,
                    evolving,
                    calls,
                    ends,
                    self.compared_types(read.root, read.subject),
                ),
                read.point,
            ),
        )
    }

    /// Returns the narrowed type of the variable declared at `binding` at
    /// `read` when the read's root continues the variable from `capture`.
    /// Narrowing starts from the variable's type at the capture point and
    /// applies the tests and writes of the read's root. `start` is where the
    /// read's expression starts.
    ///
    /// A variable that may be read before it is assigned starts at the
    /// capture point without `undefined` when some code assigns it, as
    /// [`FlowCapture::assumes_initialized`] records. Otherwise, when
    /// `undefined` remains at the read although the declared type lacks it,
    /// the read keeps its declared type, as a read in the root that declares
    /// the variable does.
    ///
    /// Returns `None` when the variable can't narrow, or when the narrowed
    /// type equals the declared type.
    fn narrow_captured(
        &mut self,
        read: FlowRead,
        start: TextSize,
        capture: FlowCapture,
        binding: TextRange,
    ) -> Option<TypeData<'db>> {
        let (mut captured, mut uninitialized) = self.captured_type(capture, binding)?;
        let declared = self.flow_binding_baseline(binding)?;
        if uninitialized && capture.assumes_initialized {
            captured = self.assumed_initialized(captured, declared);
            uninitialized = false;
        }
        let narrowed = self.narrow_from(read, start, captured)?;
        if uninitialized
            && contains_undefined(self.db, narrowed)
            && !contains_undefined(self.db, declared)
        {
            return None;
        }
        (narrowed != declared).then_some(narrowed)
    }

    /// Removes `undefined` from `ty` when `declared` lacks it. `ty` is a type
    /// that a solve gave from `declared` with `undefined` added, and the
    /// result stands for the type that the solve gives from `declared` alone.
    /// Returns `Unknown` when nothing but `undefined` remains in `ty`,
    /// because no value from `declared` reaches it, as for a read on a path
    /// that no value takes.
    ///
    /// A guard narrows each member of a union on its own, and a write reduces
    /// `declared`, so the start's `undefined` reaches `ty` only as a member of
    /// its own. A guard that can't decide keeps more members, so the result is
    /// never narrower than the type from `declared`.
    fn assumed_initialized(&self, ty: TypeData<'db>, declared: TypeData<'db>) -> TypeData<'db> {
        if contains_undefined(self.db, declared) {
            return ty;
        }
        let ty = if let TypeData::Union(union) = ty {
            TypeData::union_from_types(
                self.db,
                union
                    .types(self.db)
                    .iter()
                    .copied()
                    .filter(|member| *member != TypeData::Undefined)
                    .collect(),
            )
        } else if ty == TypeData::Undefined {
            TypeData::NeverKeyword
        } else {
            ty
        };
        if ty == TypeData::NeverKeyword {
            TypeData::Unknown
        } else {
            ty
        }
    }

    /// Returns the type of the variable `read.subject` at `read`, narrowed by
    /// the tests and writes of its root from `ty`, its type where the root
    /// starts. `start` is where the read's expression starts. Returns `ty`
    /// itself when no test of the root can change it.
    ///
    /// Returns `None` when every path to the read crosses a call that never
    /// returns, so the read keeps its declared type, as in TypeScript.
    fn narrow_from(
        &mut self,
        read: FlowRead,
        start: TextSize,
        ty: TypeData<'db>,
    ) -> Option<TypeData<'db>> {
        let Some(baseline) = self.subject_baseline(read, ty) else {
            return Some(ty);
        };
        let exhaustive = self.exhaustive_switches(read, start);
        let assigned = self.flow_assigned_types(read.root, read.subject);
        let calls = self.call_effects(read.root, read.subject);
        let ends = self.ending_calls(read)?;
        let narrowed = infer_flow_subject_type(
            self.db,
            FlowSubjectTypeInput::new(
                self.db,
                FlowSubjectRootInput::new(
                    self.db,
                    self.module,
                    read.root,
                    read.subject,
                    Some(baseline),
                    false,
                    exhaustive,
                    assigned,
                    FlowEvolving::None,
                    calls,
                    ends,
                    self.compared_types(read.root, read.subject),
                ),
                read.point,
            ),
        );
        Some(narrowed.unwrap_or(baseline))
    }

    /// Returns the type that narrowing of `read.subject` in the root of
    /// `read` starts from when the subject's type at the root's start is
    /// `ty`: [`Self::property_baseline`] when the root narrows it only
    /// through tests of its shape and none of its call points narrows, and
    /// [`Self::narrowing_baseline`] otherwise. Returns `None` when no test of
    /// the root can change `ty`.
    fn subject_baseline(&mut self, read: FlowRead, ty: TypeData<'db>) -> Option<TypeData<'db>> {
        let root = self.js_info.flow.root(read.root)?;
        let baseline = if root.is_property_only(read.subject)
            && self.call_effects(read.root, read.subject).is_none()
        {
            self.property_baseline(ty, root.is_instance_tested(read.subject))
        } else {
            self.narrowing_baseline(ty)
        }?;
        self.may_change_baseline(read.root, read.subject, baseline)
            .then_some(baseline)
    }

    /// Returns whether the solve of `subject` in the root at index `root` can
    /// change `baseline`, the type that its narrowing starts from. It can't
    /// when `baseline` is `any` and the subject is neither in
    /// [`RootFlow::any_tested`] nor narrowed by one of its call points,
    /// because every other runtime test and every write keeps `any`.
    fn may_change_baseline(
        &mut self,
        root: usize,
        subject: FlowSubjectId,
        baseline: TypeData<'db>,
    ) -> bool {
        baseline != TypeData::AnyKeyword
            || self
                .js_info
                .flow
                .root(root)
                .is_some_and(|flow| flow.is_any_tested(subject))
            || self.call_effects(root, subject).is_some()
    }

    /// Returns the type of the variable declared at `binding` at the capture
    /// point of `capture`, and whether the variable starts with `undefined`
    /// in the outermost root of the capture chain, because it may be read
    /// there before it is assigned (see [`RootFlow::uninitialized`]).
    ///
    /// The enclosing root narrows the variable from its declared type. When
    /// that root itself continues the variable from a root around it,
    /// narrowing starts instead from the type at that outer capture point, so
    /// the chain resolves outward. An element of a destructuring pattern that
    /// the enclosing root narrows at the capture point starts from its
    /// property on the narrowed pattern, as in [`Self::narrow_reference`].
    /// The returned type keeps the `undefined` of an uninitialized start, so
    /// the caller decides whether the read keeps it.
    ///
    /// Returns `None` when the variable can't narrow at all. Returns
    /// `Unknown` when the solve of the enclosing root exceeds its work limit
    /// or meets a cycle, or when no value reaches the capture point.
    fn captured_type(
        &mut self,
        capture: FlowCapture,
        binding: TextRange,
    ) -> Option<(TypeData<'db>, bool)> {
        if self.calls_before.is_some() {
            return self.resolve_captured_type(capture, binding);
        }
        let key = (capture.root, capture.subject, capture.point);
        if let Some(cached) = self
            .local_flow
            .as_ref()
            .and_then(|local| local.captures.get(&key))
        {
            return *cached;
        }
        let captured = self.resolve_captured_type(capture, binding);
        if let Some(local) = self.local_flow.as_mut() {
            local.captures.insert(key, captured);
        }
        captured
    }

    /// Computes the type that [`Self::captured_type`] returns, without its
    /// cache.
    fn resolve_captured_type(
        &mut self,
        capture: FlowCapture,
        binding: TextRange,
    ) -> Option<(TypeData<'db>, bool)> {
        let info = self.js_info;
        let root = info.flow.root(capture.root)?;
        let read = FlowRead {
            root: capture.root,
            point: capture.point,
            subject: capture.subject,
            assumes_initialized: false,
        };
        let element = root
            .element_pattern(capture.subject)
            .and_then(|pattern| self.narrowed_element(read, capture.start, pattern, binding));
        // The type where the enclosing root starts the variable, as for its
        // own reads.
        let outer = match root.capture(capture.subject) {
            Some(outer) => Some(self.captured_type(outer, binding)?),
            None => None,
        };
        if !root.reads_at(capture.subject, capture.point) {
            // Only a test of the pattern reaches the capture point.
            return match (element, outer) {
                (Some(element), _) => Some((element, false)),
                (None, Some(outer)) => Some(outer),
                (None, None) => Some((self.flow_binding_baseline(binding)?, false)),
            };
        }
        let (baseline, uninitialized) = match (element, outer) {
            (Some(start), _) => match self.subject_baseline(read, start) {
                Some(baseline) => (baseline, false),
                None => return Some((start, false)),
            },
            (None, Some((start, uninitialized))) => match self.subject_baseline(read, start) {
                Some(baseline) => (baseline, uninitialized),
                None => return Some((start, uninitialized)),
            },
            // The solve keeps the `undefined` of an uninitialized start. A
            // read in the enclosing root itself gets its declared type when
            // that `undefined` remains, so the caller decides for the
            // closure's read.
            (None, None) => {
                let baseline = if self.call_effects(capture.root, capture.subject).is_some() {
                    self.called_binding_baseline(binding)?
                } else {
                    self.flow_binding_baseline(binding)?
                };
                let uninitialized = root.is_uninitialized(capture.subject);
                if !self.may_change_baseline(capture.root, capture.subject, baseline) {
                    return Some((baseline, uninitialized));
                }
                (baseline, uninitialized)
            }
        };
        if baseline == TypeData::Unknown {
            return Some((TypeData::Unknown, uninitialized));
        }
        let exhaustive = self.exhaustive_switches(read, capture.start);
        let assigned = self.flow_assigned_types(capture.root, capture.subject);
        let calls = self.call_effects(capture.root, capture.subject);
        // TypeScript gives the reads of a closure their declared types when
        // every path to the point that creates it crosses a call that never
        // returns.
        let ends = self.ending_calls(read)?;
        let states = infer_flow_subject_states(
            self.db,
            FlowSubjectRootInput::new(
                self.db,
                self.module,
                capture.root,
                capture.subject,
                Some(baseline),
                false,
                exhaustive,
                assigned,
                FlowEvolving::None,
                calls,
                ends,
                self.compared_types(capture.root, capture.subject),
            ),
        );
        let ty = states
            .and_then(|states| {
                let index = states
                    .binary_search_by_key(&capture.point, |(point, _)| *point)
                    .ok()?;
                Some(states[index].1)
            })
            .filter(|ty| *ty != TypeData::NeverKeyword)
            .unwrap_or(TypeData::Unknown);
        Some((ty, uninitialized))
    }

    /// Returns the type of the element declared at `element` at `read`,
    /// given the `pattern` that holds it: the element's property on the type
    /// of the pattern narrowed at the read, destructured as the element's
    /// collected type destructures the pattern's declared type. `start` is
    /// where the expression of `read` starts.
    ///
    /// Returns `None` when no test can narrow the pattern, when the pattern
    /// keeps its declared type at the read, and when the property can't be
    /// read, so the element keeps its declared type. Returns `Unknown` when
    /// the narrowing of the pattern exceeds a work limit or meets a cycle,
    /// and when no value of the pattern reaches the read.
    fn narrowed_element(
        &mut self,
        read: FlowRead,
        start: TextSize,
        pattern: FlowSubjectId,
        element: TextRange,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        let &FlowSubject::Pattern(range) = info.flow.root(read.root)?.subject(pattern)? else {
            return None;
        };
        let baseline = self.flow_binding_baseline(range)?;
        let exhaustive = self.exhaustive_switches(read, start);
        let calls = self.call_effects(read.root, pattern);
        let ends = self.ending_calls(read)?;
        // No write changes the value that a pattern destructures.
        let narrowed = infer_flow_subject_type(
            self.db,
            FlowSubjectTypeInput::new(
                self.db,
                FlowSubjectRootInput::new(
                    self.db,
                    self.module,
                    read.root,
                    pattern,
                    Some(baseline),
                    false,
                    exhaustive,
                    None,
                    FlowEvolving::None,
                    calls,
                    ends,
                    self.compared_types(read.root, pattern),
                ),
                read.point,
            ),
        )?;
        if narrowed == TypeData::Unknown {
            return Some(TypeData::Unknown);
        }
        let TypeReference::Resolved(RawTypeId::Local(id)) = info.raw_binding_types.get(&element)?
        else {
            return None;
        };
        let RawTypeData::TypeofExpression(expression) = info.raw_types.get(id.index())? else {
            return None;
        };
        let TypeofExpression::Destructure(destructure) = expression.as_ref() else {
            return None;
        };
        self.resolve_destructure_field(narrowed, &destructure.destructure_field)
    }

    /// Returns the type that narrowing of the binding or the pattern at
    /// `range` starts from, through
    /// [`crate::db::queries::infer_flow_binding_baseline`], or resolved in
    /// this context when it evaluates flow locally.
    fn flow_binding_baseline(&mut self, range: TextRange) -> Option<TypeData<'db>> {
        if self.evaluates_flow_locally() {
            self.local_binding_baseline(range)
        } else {
            infer_flow_binding_baseline(self.db, BindingTypeInput::new(self.db, self.module, range))
        }
    }

    /// Returns the type that narrowing of the binding declared at `binding`
    /// starts from in a root where one of its call points narrows: its
    /// declared type, flattened for narrowing by [`Self::narrowing_baseline`].
    ///
    /// [`Self::flow_binding_baseline`] gives that type unless the roots that
    /// read the binding narrow it only through tests of its shape or only
    /// through writes. It then gives a type that only those can narrow, so
    /// this function resolves the declared type itself, in the reading
    /// context. Only a call point that narrows needs this.
    fn called_binding_baseline(&mut self, binding: TextRange) -> Option<TypeData<'db>> {
        let info = self.js_info;
        if !info.flow.is_property_only_binding(binding)
            && !info.flow.is_assignment_only_binding(binding)
        {
            return self.flow_binding_baseline(binding);
        }
        let declared = self.resolve(info.raw_binding_types.get(&binding)?);
        self.narrowing_baseline(declared)
    }

    /// Returns the type that narrowing of the binding declared at `binding`,
    /// or of the pattern at that range, starts from, as
    /// [`crate::db::queries::infer_flow_binding_baseline`] returns it, but
    /// resolved in this context.
    fn local_binding_baseline(&mut self, binding: TextRange) -> Option<TypeData<'db>> {
        if let Some(baseline) = self.local_flow.as_ref()?.baselines.get(&binding) {
            return *baseline;
        }
        // The declared type is resolved from the raw type of the binding or
        // the pattern, not through a declaration query, so a cyclic object
        // keeps its shape, as in `flow_binding_baseline`.
        let info = self.js_info;
        let baseline = info
            .raw_binding_types
            .get(&binding)
            .or_else(|| info.flow.pattern_type(binding))
            .and_then(|reference| {
                let declared = self.resolve(reference);
                self.binding_baseline(binding, declared)
            });
        self.local_flow
            .as_mut()?
            .baselines
            .insert(binding, baseline);
        baseline
    }

    /// Returns the narrowed type of the property path read by the member
    /// expression `member`, given its `declared` type there: the type of the
    /// member access with its object's narrowed type.
    ///
    /// The runtime tests that aliased conditions apply to the path narrow it
    /// only when every property along the path is readonly, which
    /// [`Self::is_readonly_path`] checks.
    ///
    /// Returns `None` when collection kept no flow for the read, when no
    /// runtime test can change `declared`, or when narrowing leaves it
    /// unchanged. Reads of the same path with equal declared types share one
    /// solve.
    pub(super) fn narrow_path(
        &mut self,
        member: &AnyJsExpression,
        declared: TypeData<'db>,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        let read = info.flow.read(member.range())?;
        let root = info.flow.root(read.root)?;
        let calls = self.call_effects(read.root, read.subject);
        let baseline = if root.is_property_only(read.subject) && calls.is_none() {
            self.property_baseline(declared, root.is_instance_tested(read.subject))
        } else {
            self.narrowing_baseline(declared)
        }?;
        if calls.is_none()
            && root.is_assignment_only(read.subject)
            && !may_narrow_by_assignment(baseline)
        {
            return None;
        }
        if !self.may_change_baseline(read.root, read.subject, baseline) {
            return None;
        }
        let readonly_path =
            root.is_readonly_tested(read.subject) && self.is_readonly_path(read, member);
        if !readonly_path && root.is_only_readonly_tested(read.subject) {
            return None;
        }
        let exhaustive = self.exhaustive_switches(read, member.range().start());
        let assigned = self.flow_assigned_types(read.root, read.subject);
        let ends = self.ending_calls(read)?;
        infer_flow_subject_type(
            self.db,
            FlowSubjectTypeInput::new(
                self.db,
                FlowSubjectRootInput::new(
                    self.db,
                    self.module,
                    read.root,
                    read.subject,
                    Some(baseline),
                    readonly_path,
                    exhaustive,
                    assigned,
                    FlowEvolving::None,
                    calls,
                    ends,
                    self.compared_types(read.root, read.subject),
                ),
                read.point,
            ),
        )
    }

    /// Returns the types that the writes of `subject` in the root at index
    /// `root` assign. When this context evaluates flow locally, it resolves
    /// them itself and keeps them per root and subject. Otherwise it queries
    /// [`crate::db::queries::infer_flow_assigned_types`].
    ///
    /// Returns `None` without resolving anything when no write with a value
    /// reduces the subject.
    fn flow_assigned_types(
        &mut self,
        root: usize,
        subject: FlowSubjectId,
    ) -> Option<FlowAssignedTypes<'db>> {
        if self
            .js_info
            .flow
            .root(root)?
            .subject_writes(subject)
            .is_empty()
        {
            return None;
        }
        if !self.evaluates_flow_locally() {
            return infer_flow_assigned_types(
                self.db,
                FlowSubjectWritesInput::new(self.db, self.module, root, subject),
            );
        }
        if let Some(cached) = self.local_flow.as_ref()?.assigned.get(&(root, subject)) {
            return *cached;
        }
        let assigned = self.assigned_types(root, subject);
        self.local_flow
            .as_mut()?
            .assigned
            .insert((root, subject), assigned);
        assigned
    }

    /// Resolves the types of the values that the writes of `subject` in the
    /// root at index `root` assign, in the order of
    /// [`RootFlow::subject_writes`].
    ///
    /// Each value is typed without narrowing: the expression's type as its
    /// raw type resolves, an element of an iterable for the head of a
    /// `for...of` loop, or `string` for the head of a `for...in` loop. A
    /// destructured field is read from it as a destructured variable's
    /// declared type is, and a default replaces `undefined` with the
    /// default's type, as in TypeScript. A value that can't be resolved, or
    /// whose type may come from a return type that Biome infers, is `Unknown`,
    /// which reduces nothing.
    ///
    /// For a subject whose type evolves with its writes, each value is the
    /// subject's type after its write, so its literal types are widened, and
    /// it is `Unknown` where Biome may type it differently from TypeScript.
    /// [`evolving_assigned_type`] computes these types.
    pub(super) fn assigned_types(
        &mut self,
        root: usize,
        subject: FlowSubjectId,
    ) -> Option<FlowAssignedTypes<'db>> {
        let info = self.js_info;
        let root = info.flow.root(root)?;
        let values = root.subject_writes(subject);
        if values.is_empty() {
            return None;
        }
        let evolving = root.evolving(subject) != FlowEvolving::None;
        let types: Box<[_]> = values
            .iter()
            .map(|value| {
                root.assigned
                    .get(*value as usize)
                    .and_then(|assigned| self.assigned_type(assigned, evolving))
                    .unwrap_or(TypeData::Unknown)
            })
            .collect();
        Some(FlowAssignedTypes::new(self.db, types))
    }

    /// Returns the type of the value that `assigned` describes, typed without
    /// narrowing, or `None` when it can't be resolved or when a call that it
    /// can give may return a type that Biome infers instead of an annotated
    /// one. [`Self::assigned_types`] lists how each kind of value is typed.
    ///
    /// When `evolving` is true, the subject's type evolves with its writes,
    /// so the result has its literal types widened and is `Unknown` where
    /// Biome may type the value differently from TypeScript. A value that
    /// the subject's declared type covers, as [`FlowAssigned::covered`]
    /// describes, gives the type that Biome gives it instead of `Unknown`,
    /// with its literal types widened.
    fn assigned_type(&mut self, assigned: &FlowAssigned, evolving: bool) -> Option<TypeData<'db>> {
        // Only the solve of a variable whose type evolves with its writes
        // falls back to the type that Biome gives a covered value.
        if assigned.covered && !evolving {
            return None;
        }
        if !assigned
            .callees
            .iter()
            .all(|callee| self.has_annotated_return(*callee))
        {
            return None;
        }
        let mut ty = match assigned.value {
            // TypeScript types an object or array literal by the declared type
            // of its target, so it keeps the literal types of its properties
            // where the declared type has them. Keeping all of them gives a
            // subtype of TypeScript's type. A subtype is assignable to every
            // member that TypeScript's type is assignable to, so the reduction
            // never gives a narrower type than TypeScript's.
            FlowAssignedValue::Expression(range) | FlowAssignedValue::BooleanLiteral(range) => {
                let reference = self.js_info.raw_expressions.get(&range)?.clone();
                let ty = self.resolve(&reference);
                // TypeScript widens the literal types of the properties of an
                // object literal that it assigns to a variable whose type
                // evolves with its writes, because such a variable has no
                // declared type to keep them.
                if evolving {
                    ty
                } else {
                    self.literal_argument(&reference, ty).unwrap_or(ty)
                }
            }
            FlowAssignedValue::IterableValue(range) => {
                let iterable = self.raw_operand_at(range)?;
                self.resolve_iterable_value_type(iterable)?
            }
            FlowAssignedValue::Key => TypeData::String,
        };
        for step in &assigned.steps {
            ty = self.resolve_destructure_field(ty, &step.field)?;
            if let Some(default) = step.default {
                let default = self.raw_operand_at(default)?;
                let field = self.flow_baseline(ty)?;
                let defined = narrow_type(
                    self.db,
                    field,
                    NarrowingPredicate::Literal(TypeData::Undefined),
                    false,
                    self.narrowing_resolver(),
                );
                ty = TypeData::union_from_types(self.db, vec![defined, default]);
            }
        }
        if evolving {
            let db = self.db;
            let evolved = evolving_assigned_type(db, ty, self.narrowing_resolver());
            if evolved == TypeData::Unknown && assigned.covered {
                return self
                    .flow_baseline(ty)
                    .map(|ty| base_literal_type(self.db, ty));
            }
            return Some(evolved);
        }
        Some(ty)
    }

    /// Returns the declared types of the values that the comparisons of
    /// `subject` in the root at index `root` narrow it by, for
    /// [`FlowSubjectRootInput::compared`]. When this context evaluates flow
    /// locally, it resolves them itself and keeps them per root and subject.
    /// Otherwise it queries [`crate::db::queries::infer_flow_compared_types`].
    ///
    /// Returns `None` without resolving anything when no comparison of the
    /// subject narrows it by a declared type.
    pub(super) fn compared_types(
        &mut self,
        root: usize,
        subject: FlowSubjectId,
    ) -> Option<FlowComparedTypes<'db>> {
        if self
            .js_info
            .flow
            .root(root)?
            .subject_compared(subject)
            .is_empty()
        {
            return None;
        }
        if !self.evaluates_flow_locally() {
            return infer_flow_compared_types(
                self.db,
                FlowSubjectWritesInput::new(self.db, self.module, root, subject),
            );
        }
        if let Some(cached) = self.local_flow.as_ref()?.compared.get(&(root, subject)) {
            return *cached;
        }
        let compared = self.resolve_compared_types(root, subject);
        self.local_flow
            .as_mut()?
            .compared
            .insert((root, subject), compared);
        compared
    }

    /// Resolves the declared types of the values that
    /// [`RootFlow::subject_compared`] lists for `subject` in the root at index
    /// `root`, flattened as a declared type is for narrowing, each with the
    /// range that identifies its value.
    ///
    /// An expression is typed without narrowing, so the type of the other
    /// operand of a comparison never depends on a solve. An imported
    /// variable counts only when it holds no unit value, which the solve
    /// reads from the module that declares it, and when the declaration that
    /// it names has an explicit type, as a variable with a type annotation
    /// has. A value whose type is Unknown, or whose flattening exceeds its
    /// work limit, is left out. Returns `None` when no value is left.
    pub(super) fn resolve_compared_types(
        &mut self,
        root: usize,
        subject: FlowSubjectId,
    ) -> Option<FlowComparedTypes<'db>> {
        let values = self.js_info.flow.root(root)?.subject_compared(subject);
        self.resolve_compared_values(values)
    }

    /// Resolves the declared types of `values`, as
    /// [`Self::resolve_compared_types`] does for the values of a subject.
    fn resolve_compared_values(
        &mut self,
        values: &[FlowComparedValue],
    ) -> Option<FlowComparedTypes<'db>> {
        let info = self.js_info;
        let mut types = Vec::with_capacity(values.len());
        for value in values {
            let ty = match *value {
                FlowComparedValue::Expression(range) => self.raw_operand_at(range),
                FlowComparedValue::Import(binding) => {
                    if imported_unit_value(self.db, self.module, info, binding, None).is_some()
                        || !imported_declaration_is_explicit(
                            self.db,
                            self.module,
                            info,
                            binding,
                            None,
                        )
                    {
                        continue;
                    }
                    info.raw_binding_types
                        .get(&binding)
                        .map(|reference| self.resolve(reference))
                }
            };
            let Some(ty) = ty.and_then(|ty| self.flow_baseline(ty)) else {
                continue;
            };
            if ty != TypeData::Unknown {
                let range = value.range();
                types.push((u32::from(range.start()), u32::from(range.end()), ty));
            }
        }
        (!types.is_empty()).then(|| FlowComparedTypes::new(self.db, types.into_boxed_slice()))
    }

    /// Returns whether the return type of `callee` comes from an annotation.
    /// An import is followed to the declaration that it names, and a member
    /// must come from an annotation on every type that its object can have;
    /// see [`CalleeReturn::Resolved`].
    fn has_annotated_return(&mut self, callee: FlowCallee) -> bool {
        let info = self.js_info;
        match callee {
            FlowCallee::Import(binding) => {
                imported_return_is_annotated(self.db, self.module, info, binding, None)
            }
            FlowCallee::Member(member) => {
                let Some(TypeReference::Resolved(RawTypeId::Local(id))) =
                    info.raw_expressions.get(&member)
                else {
                    return false;
                };
                let Some(RawTypeData::TypeofExpression(raw)) = info.raw_types.get(id.index())
                else {
                    return false;
                };
                let (TypeofExpression::StaticMember(expression)
                | TypeofExpression::OptionalChainStaticMember(expression)) = raw.as_ref()
                else {
                    return false;
                };
                let object = self.resolve(&expression.object);
                has_annotated_member(self.db, object, expression.member.text())
            }
        }
    }

    /// Returns the effects of the call points of the root at index `root`
    /// that narrow `subject`, for [`FlowSubjectRootInput::calls`], or `None`
    /// when none of them narrows. The call points checked are those that
    /// [`RootFlow::subject_calls`] lists for the subject. The answer is
    /// cached per root and subject when this context evaluates flow locally.
    ///
    /// While the arguments of a call point are read, only the call points
    /// that end before that call starts count, and nothing is cached (see
    /// [`ResolutionCtx::calls_before`]).
    pub(super) fn call_effects(
        &mut self,
        root: usize,
        subject: FlowSubjectId,
    ) -> Option<FlowCallEffects<'db>> {
        let info = self.js_info;
        let flow = info.flow.root(root)?;
        let calls = flow.subject_calls(subject);
        if calls.is_empty() {
            return None;
        }
        let bound = self.calls_before;
        let key = (root, subject);
        if bound.is_none()
            && let Some(cached) = self
                .local_flow
                .as_ref()
                .and_then(|local| local.subject_call_effects.get(&key))
        {
            return *cached;
        }
        let mut effects = Vec::new();
        for call in calls {
            let Some(flow_call) = flow.calls.get(*call as usize) else {
                continue;
            };
            if bound.is_some_and(|bound| flow_call.range.end() > bound) {
                continue;
            }
            let effect = self.flow_call_effect(flow_call.range);
            if effect.narrows() {
                effects.push((*call, effect));
            }
        }
        let effects = (!effects.is_empty())
            .then(|| FlowCallEffects::new(self.db, effects.into_boxed_slice()));
        if bound.is_none()
            && let Some(local) = self.local_flow.as_mut()
        {
            local.subject_call_effects.insert(key, effects);
        }
        effects
    }

    /// Returns the effect of the call point at `range`. A context that
    /// evaluates flow locally resolves it itself and caches it per call.
    /// Other contexts read it from
    /// [`crate::db::queries::infer_flow_call_effect`].
    fn flow_call_effect(&mut self, range: TextRange) -> FlowCallEffect<'db> {
        if !self.evaluates_flow_locally() {
            return infer_flow_call_effect(
                self.db,
                FlowCallInput::new(self.db, self.module, range),
            );
        }
        if let Some(effect) = self
            .local_flow
            .as_ref()
            .and_then(|local| local.call_effects.get(&range))
        {
            return *effect;
        }
        let effect = self.call_effect(range);
        if let Some(local) = self.local_flow.as_mut() {
            local.call_effects.insert(range, effect);
        }
        effect
    }

    /// Resolves the effect of the call point at `range` in this context, or
    /// [`FlowCallEffect::None`] when the call narrows nothing.
    pub(super) fn call_effect(&mut self, range: TextRange) -> FlowCallEffect<'db> {
        match self.js_info.flow.call(range) {
            Some(call) => self.resolve_call_effect(call).unwrap_or_default(),
            None => FlowCallEffect::None,
        }
    }

    /// Resolves the effect that [`Self::call_effect`] returns, or `None` when
    /// the call narrows nothing.
    ///
    /// The callee's type is its ordinary type, without narrowing, as its raw
    /// type resolves. For a call in a condition, a union callee loses its
    /// `null` and `undefined` members, as in TypeScript, and must keep exactly
    /// one. A statement call resolves only when every name along its callee
    /// has an explicit type, as [`Self::has_explicit_callee`] checks, so its
    /// ordinary type is that explicit type, and its callee can't be a union.
    /// A callee with one signature without type parameters takes that
    /// signature, and so does one with explicit type arguments, which replace
    /// the type parameters of its only signature. Otherwise the signature
    /// depends on the arguments, as [`Self::select_effect_signature`]
    /// decides, and a call that it lets narrow only where it is truthy
    /// removes nothing where it is falsy.
    ///
    /// A call in a condition narrows only through a type predicate, and a
    /// statement call only through an assertion. The target is flattened as a
    /// declared type is, and a target of `any` or `unknown` narrows nothing.
    ///
    /// The effect of a statement call is [`FlowCallEffect::Ends`] when the
    /// return type annotation of its signature's declaration is `never`,
    /// written in place or through type aliases (see
    /// [`Self::has_annotated_callee`]), or when its signature is
    /// `asserts value` and the asserted argument is always false.
    fn resolve_call_effect(&mut self, call: &FlowCall) -> Option<FlowCallEffect<'db>> {
        let info = self.js_info;
        let range = call.range;
        let TypeReference::Resolved(RawTypeId::Local(id)) = info.raw_expressions.get(&range)?
        else {
            return None;
        };
        let RawTypeData::TypeofExpression(raw) = info.raw_types.get(id.index())? else {
            return None;
        };
        let TypeofExpression::Call(raw) = raw.as_ref() else {
            return None;
        };
        let callee_steps = match &call.kind {
            FlowCallKind::Condition => None,
            FlowCallKind::Statement {
                root,
                steps,
                annotated,
            } => {
                if !self.has_explicit_callee(*root, steps) {
                    return None;
                }
                Some((*root, &**steps, *annotated))
            }
        };
        let statement = callee_steps.is_some();
        let explicit_type_arguments = has_type_arguments(info, &raw.callee);
        let callee = self.resolve(&raw.callee);
        let mut callee = self.flow_baseline(callee)?;
        if let TypeData::Union(union) = callee {
            if statement {
                return None;
            }
            let mut members = union
                .types(self.db)
                .iter()
                .copied()
                .filter(|member| !matches!(member, TypeData::Null | TypeData::Undefined));
            callee = members.next()?;
            if members.next().is_some() {
                return None;
            }
        }
        let (function, narrows_falsy) =
            match resolve_callable_function(self.db, CallableFunctionInput::new(self.db, callee)) {
                Some(function)
                    if explicit_type_arguments || function.type_parameters(self.db).is_empty() =>
                {
                    (function, true)
                }
                _ if explicit_type_arguments => return None,
                _ => {
                    let selected = self.select_effect_signature(call, raw, callee)?;
                    (selected.function, selected.narrows_falsy)
                }
            };
        let (name, target, asserts) = match function.return_type(self.db) {
            ReturnType::Predicate(predicate) => {
                (&predicate.parameter_name, Some(predicate.ty), false)
            }
            ReturnType::Asserts(assertion) => {
                // Collection records a bare `asserts value` as `Conditional`,
                // which no type annotation names.
                let target = resolve_local_type_on_demand(self.db, assertion.ty);
                (
                    &assertion.parameter_name,
                    (target != TypeData::Conditional).then_some(target),
                    true,
                )
            }
            ReturnType::Type(ty) => {
                // TypeScript ends the path at a statement call only when the
                // return type annotation of the signature's declaration is
                // `never`, in place or through type aliases. Flattening
                // follows the aliases and drops `never` from a union, so an
                // alias of a union, such as `never | string`, isn't `never`.
                let (root, steps, annotated) = callee_steps?;
                let returns_never = self.flow_baseline(*ty) == Some(TypeData::NeverKeyword)
                    && self.has_annotated_callee(root, steps, annotated);
                return returns_never.then_some(FlowCallEffect::Ends);
            }
        };
        if asserts != statement {
            return None;
        }
        let parameter = predicate_parameter(self.db, function, name, call)?;
        // `asserts value` never returns for an argument that is always false.
        if asserts
            && target.is_none()
            && let FlowCallParameter::Argument(index) = parameter
            && call
                .arguments
                .get(index as usize)
                .is_some_and(|argument| argument.is_false)
        {
            return Some(FlowCallEffect::Ends);
        }
        let target = match target {
            Some(target) => {
                // A target such as `NonNullable<T>` is an alias whose type
                // argument the call's type arguments replace.
                let target = instantiate_intersection_alias(self.db, target, |ty| {
                    resolve_local_type_on_demand(self.db, ty).expand_structural_global(self.db)
                })
                .unwrap_or(target);
                let target = self.flow_baseline(target)?;
                if matches!(
                    target,
                    TypeData::AnyKeyword | TypeData::UnknownKeyword | TypeData::Unknown
                ) {
                    return None;
                }
                Some(target)
            }
            None => None,
        };
        Some(match target {
            Some(target) if !statement => FlowCallEffect::Predicate {
                parameter,
                target,
                narrows_falsy,
            },
            None if !statement => return None,
            target => FlowCallEffect::Assertion { parameter, target },
        })
    }

    /// Returns whether every name along the callee of a statement call has
    /// an explicit type, which TypeScript requires before it applies an
    /// assertion. `root` is how collection classified the name that the
    /// callee starts at, and `steps` are the ranges of its member accesses,
    /// as [`FlowCallKind::Statement`] records them.
    ///
    /// An import has an explicit type when the declaration that it names has
    /// one, as [`imported_declaration_is_explicit`] checks. A member access
    /// has one when every type that its object can have declares the member
    /// with one, as [`has_explicit_member`] checks. The object's type is its
    /// declared type, as its raw type resolves, because TypeScript reads the
    /// explicit types along the callee.
    fn has_explicit_callee(&mut self, root: FlowCallRoot, steps: &[TextRange]) -> bool {
        let info = self.js_info;
        let mut steps = steps.iter();
        match root {
            FlowCallRoot::Explicit => {}
            FlowCallRoot::Namespace => {
                steps.next();
            }
            FlowCallRoot::Import(binding) => {
                let member = if is_namespace_import(info, binding) {
                    match steps.next().and_then(|step| raw_member(info, *step)) {
                        Some((_, member)) => Some(member),
                        None => return false,
                    }
                } else {
                    None
                };
                if !imported_declaration_is_explicit(self.db, self.module, info, binding, member) {
                    return false;
                }
            }
        }
        for step in steps {
            let Some((object, member)) = raw_member(info, *step) else {
                return false;
            };
            let object = self.resolve(object);
            if !has_explicit_member(self.db, object, member.text()) {
                return false;
            }
        }
        true
    }

    /// Resolves the `undefined` that the function whose trimmed range is
    /// `function` returns when a path reaches the end of its body (see
    /// [`biome_js_type_info::TypeofImplicitReturnExpression`]): `undefined`
    /// when a path reaches the end, and `never`, which a union drops, when
    /// every path ends before it, as at a call that never returns or after
    /// an exhaustive `switch`.
    ///
    /// The answer comes from [`infer_flow_reachability`], so every context
    /// shares it. When that query gives no answer, because of a work limit
    /// or a query cycle, as when deciding whether a call ends the path needs
    /// this return type, the end counts as reachable, which only widens.
    pub(super) fn resolve_implicit_return(&self, function: TextRange) -> TypeData<'db> {
        let input = FlowReachabilityInput::new(self.db, self.module, function);
        match infer_flow_reachability(self.db, input) {
            Some(reachability) if !reachability.end => TypeData::NeverKeyword,
            Some(_) | None => TypeData::Undefined,
        }
    }

    /// Resolves the target of the type predicate that TypeScript infers from
    /// the body of the function whose trimmed range is `function` (see
    /// [`biome_js_type_info::TypeofInferredPredicateExpression`]), or `Unknown` when it infers none.
    ///
    /// Lookups read the answer from [`infer_flow_inferred_predicate`], so
    /// they share it. Complete module inference infers it in its own context
    /// instead, which resolves the types that it needs anyway, and keeps it
    /// per function. A work limit or a query cycle gives no type predicate,
    /// which only widens.
    pub(super) fn resolve_inferred_predicate(&mut self, function: TextRange) -> TypeData<'db> {
        let Some(predicates) = self.local_predicates.as_ref() else {
            let input = FlowPredicateInput::new(self.db, self.module, function);
            return infer_flow_inferred_predicate(self.db, input).unwrap_or(TypeData::Unknown);
        };
        if let Some(target) = predicates.get(&function) {
            return target.unwrap_or(TypeData::Unknown);
        }
        let js_info = self.js_info;
        let target = js_info
            .flow
            .predicate(function)
            .and_then(|predicate| self.inferred_predicate(predicate));
        if let Some(predicates) = self.local_predicates.as_mut() {
            predicates.insert(function, target);
        }
        target.unwrap_or(TypeData::Unknown)
    }

    /// Returns the target of the type predicate that the returned value
    /// that `predicate` decomposes gives its function, as TypeScript infers
    /// it, or `None` when it gives none. See
    /// [`crate::db::queries::infer_flow_inferred_predicate`].
    ///
    /// The parameter's declared type is resolved in this context and
    /// flattened as narrowing flattens it. TypeScript ignores a parameter
    /// whose declared type is `boolean`, because a type predicate can only
    /// tell `true` from `false` there. The call points of the value and its
    /// comparisons by declared types narrow by what this context resolves.
    fn inferred_predicate(&mut self, predicate: &FlowPredicate) -> Option<TypeData<'db>> {
        let db = self.db;
        let flow = &predicate.flow;
        let parameter = predicate.parameter;
        let reference = self.js_info.raw_binding_types.get(&predicate.binding)?;
        let declared = self.resolve(reference);
        // Handles to local types of this module resolve through this
        // context, as for a subject that only tests of its shape narrow.
        let declared = self.flatten_baseline(declared, true).ok()?;
        if is_narrowing_invariant(declared) || is_boolean(db, declared) {
            return None;
        }
        let mut effects = Vec::new();
        for call in flow.subject_calls(parameter) {
            let Some(flow_call) = flow.calls.get(*call as usize) else {
                continue;
            };
            if let Some(effect) = self.resolve_call_effect(flow_call)
                && effect.narrows()
            {
                effects.push((*call, effect));
            }
        }
        let calls =
            (!effects.is_empty()).then(|| FlowCallEffects::new(db, effects.into_boxed_slice()));
        let compared = self.resolve_compared_values(flow.subject_compared(parameter));
        let mut solve = GuardSolve::new(
            parameter,
            declared,
            false,
            0,
            None,
            FlowEvolving::None,
            calls,
            0,
            compared,
        );
        let mut remaining = MAX_FLOW_STEPS;
        let truthy =
            self.apply_guard(flow, predicate.truthy, &mut solve, declared, &mut remaining)?;
        if truthy == declared || truthy == TypeData::Unknown {
            return None;
        }
        let falsy = self.apply_guard(flow, predicate.falsy, &mut solve, truthy, &mut remaining)?;
        (falsy == TypeData::NeverKeyword).then_some(truthy)
    }

    /// Returns which statements of the root that `graph` holds some path
    /// reaches, and whether a path reaches the root's end. See
    /// [`crate::db::queries::infer_flow_reachability`].
    ///
    /// A path ends where the graph has no flow, at a statement call that
    /// never returns, and on the path that matches no case of an exhaustive
    /// `switch` statement, as in TypeScript. A call point that collection
    /// kept shares its effect with narrowing; another one is resolved here.
    ///
    /// Returns `None` when the walk exceeds its work limit.
    fn reachability(&mut self, graph: &ReachabilityGraph) -> Option<FlowReachability> {
        let flow = &graph.flow;
        let mut ends = vec![false; flow.calls.len()];
        for (call, ended) in flow.calls.iter().zip(&mut ends) {
            if !matches!(call.kind, FlowCallKind::Statement { .. }) {
                continue;
            }
            let effect = if self.js_info.flow.call(call.range).is_some() {
                self.flow_call_effect(call.range)
            } else {
                self.resolve_call_effect(call).unwrap_or_default()
            };
            *ended = effect == FlowCallEffect::Ends;
        }
        let mut exhaustive = vec![false; flow.switches.len()];
        for (switch, exhaustive) in flow.switches.iter().zip(&mut exhaustive) {
            *exhaustive = self.is_exhaustive_in(flow, switch);
        }
        let ends_at = |node: &FlowNode| -> bool {
            let FlowNode::Condition { guard, .. } = node else {
                return false;
            };
            match flow.guards.get(*guard) {
                Some(FlowGuard::Call { call, .. }) => {
                    ends.get(*call as usize).copied().unwrap_or(false)
                }
                Some(FlowGuard::Bypass { switch, .. }) => {
                    exhaustive.get(*switch as usize).copied().unwrap_or(false)
                }
                _ => false,
            }
        };
        let mut successors: Vec<Vec<FlowNodeId>> = vec![Vec::new(); flow.nodes.len()];
        for (id, node) in (0..).zip(flow.nodes.iter()) {
            for predecessor in predecessor_ids(node) {
                successors.get_mut(*predecessor as usize)?.push(id);
            }
        }
        let mut reached = vec![false; flow.nodes.len()];
        let mut pending = vec![0];
        let mut remaining = MAX_REACHABILITY_STEPS;
        while let Some(node) = pending.pop() {
            remaining = remaining.checked_sub(1)?;
            let seen = reached.get_mut(node as usize)?;
            if *seen || ends_at(&flow.nodes[node as usize]) {
                continue;
            }
            *seen = true;
            pending.extend(&successors[node as usize]);
        }
        let reaches = |point: Option<FlowNodeId>| {
            point.is_some_and(|point| reached.get(point as usize).copied().unwrap_or(false))
        };
        let mut statements: Vec<(TextRange, bool)> = Vec::with_capacity(graph.statements.len());
        for (range, entry) in &graph.statements {
            statements.push((*range, reaches(*entry)));
        }
        // A finally block built twice lists its statements twice, and a
        // statement that either build reaches is reachable.
        statements.sort_unstable_by(|(first, reached_first), (second, reached_second)| {
            first
                .start()
                .cmp(&second.start())
                .then(second.end().cmp(&first.end()))
                .then(reached_second.cmp(reached_first))
        });
        statements.dedup_by_key(|(range, _)| *range);
        Some(FlowReachability {
            statements: statements.into_boxed_slice(),
            end: reaches(graph.end),
        })
    }

    /// Returns whether the return type of the callee of a statement call
    /// comes from an annotation, given the callee's `root`, `steps`, and
    /// `annotated`, as [`FlowCallKind::Statement`] records them. TypeScript
    /// ends the path at a call that returns `never` only when that return
    /// type comes from an annotation.
    ///
    /// A callee without member accesses takes `annotated`, which collection
    /// read from the syntax, unless it is an import. An imported callee, or
    /// a member of a namespace import, has one when the declaration of the
    /// export that it names does. A member of a namespace that `declare`
    /// declares always has one, because it has no body to infer from. Any
    /// other callee reads a member, which every type of its object must
    /// declare with an annotated return type (see
    /// [`has_annotated_return_member`]).
    fn has_annotated_callee(
        &mut self,
        root: FlowCallRoot,
        steps: &[TextRange],
        annotated: bool,
    ) -> bool {
        let info = self.js_info;
        match (root, steps) {
            (FlowCallRoot::Import(binding), []) => {
                imported_return_is_annotated(self.db, self.module, info, binding, None)
            }
            (FlowCallRoot::Explicit | FlowCallRoot::Namespace, []) => annotated,
            (FlowCallRoot::Namespace, [_]) => true,
            (FlowCallRoot::Import(binding), [step]) if is_namespace_import(info, binding) => {
                raw_member(info, *step).is_some_and(|(_, member)| {
                    imported_return_is_annotated(self.db, self.module, info, binding, Some(member))
                })
            }
            (_, [.., last]) => match raw_member(info, *last) {
                Some((object, member)) => {
                    let object = self.resolve(object);
                    has_annotated_return_member(self.db, object, member.text())
                }
                None => false,
            },
        }
    }

    /// Returns the statement calls of the root of `read` that never return
    /// and whose node reaches the read, one bit each, in the mask that
    /// [`FlowSubjectRootInput::ends`] takes. Returns `None` when every path
    /// to the read crosses one of them.
    ///
    /// A call never returns when inference finds its effect to be
    /// [`FlowCallEffect::Ends`]. TypeScript gives a read that only such paths
    /// reach its declared type, so the caller then keeps the declared type
    /// without a solve. Otherwise the paths through such a call add nothing
    /// to the read's type, as in TypeScript. When the walk that checks for a
    /// path around those calls exceeds its work limit, the read is solved
    /// with the mask, which gives `Unknown` where no path remains.
    fn ending_calls(&mut self, read: FlowRead) -> Option<u64> {
        let Some(root) = self.js_info.flow.root(read.root) else {
            return Some(0);
        };
        let mut pending = root.called_calls(read.point);
        let mut ends = 0;
        let bound = self.calls_before;
        while pending != 0 {
            let bit = pending.trailing_zeros();
            pending &= pending - 1;
            let Some(call) = root
                .ending_calls
                .get(bit as usize)
                .and_then(|call| root.calls.get(*call as usize))
            else {
                continue;
            };
            // While the arguments of a call point are read, only the calls
            // that end before it count (see `ResolutionCtx::calls_before`).
            if bound.is_some_and(|bound| call.range.end() > bound) {
                continue;
            }
            if self.flow_call_effect(call.range) == FlowCallEffect::Ends {
                ends |= 1 << bit;
            }
        }
        if ends == 0 {
            return Some(0);
        }
        let key = (read.root, read.point, ends);
        let reached = match self
            .local_flow
            .as_ref()
            .and_then(|local| local.reached.get(&key))
        {
            Some(reached) => *reached,
            None => {
                let reached = root.reaches_past(read.point, ends).unwrap_or(true);
                if let Some(local) = self.local_flow.as_mut() {
                    local.reached.insert(key, reached);
                }
                reached
            }
        };
        reached.then_some(ends)
    }

    /// Returns the exhaustive `switch` statements of the root of `read` whose
    /// bypass reaches the read, one bit each, as the mask in
    /// [`FlowSubjectRootInput::exhaustive`]. `start` is where the expression
    /// of `read` starts.
    ///
    /// Only the statements whose cases start before `start` count. The read
    /// of a discriminant lies before its own statement's cases, so deciding
    /// whether a statement is exhaustive never asks about itself. A
    /// statement that a loop's back edge brings before the read doesn't
    /// count, so its bypass keeps its guard.
    fn exhaustive_switches(&mut self, read: FlowRead, start: TextSize) -> u64 {
        let Some(root) = self.js_info.flow.root(read.root) else {
            return 0;
        };
        let mut pending = root.bypassed_switches(read.point);
        let mut exhaustive = 0;
        while pending != 0 {
            let bit = pending.trailing_zeros();
            pending &= pending - 1;
            if root
                .switches
                .get(bit as usize)
                .is_some_and(|switch| switch.start <= start)
                && self.is_exhaustive_switch(read.root, bit)
            {
                exhaustive |= 1 << bit;
            }
        }
        exhaustive
    }

    /// Returns whether the `switch` statement at index `switch` of the root
    /// at index `root` is exhaustive. When the context evaluates flow
    /// locally, it decides the statement itself and caches the answer per
    /// statement. Otherwise it asks
    /// [`crate::db::queries::infer_flow_switch_exhaustive`].
    fn is_exhaustive_switch(&mut self, root: usize, switch: u32) -> bool {
        // While the arguments of a call point are read, the discriminant's
        // type depends on which call points count, so the answer is neither
        // shared nor cached (see `ResolutionCtx::calls_before`).
        if self.calls_before.is_some() {
            return self.switch_is_exhaustive(root, switch);
        }
        if !self.evaluates_flow_locally() {
            return infer_flow_switch_exhaustive(
                self.db,
                FlowSwitchInput::new(self.db, self.module, root, switch),
            );
        }
        if let Some(&cached) = self
            .local_flow
            .as_ref()
            .and_then(|local| local.exhaustive_switches.get(&(root, switch)))
        {
            return cached;
        }
        let exhaustive = self.switch_is_exhaustive(root, switch);
        if let Some(local) = self.local_flow.as_mut() {
            local.exhaustive_switches.insert((root, switch), exhaustive);
        }
        exhaustive
    }

    /// Returns whether the `switch` statement at index `switch` of the root
    /// at index `root` is exhaustive, which holds when every value that the
    /// type of its discriminant at the statement allows matches a case, as
    /// in TypeScript.
    ///
    /// That type is the narrowed type of the discriminant, or of the operand
    /// of `typeof`, at the statement, so a `string` that a test narrowed to
    /// `"a" | "b"` before the statement counts as `"a" | "b"`. Without
    /// narrowing, it is the ordinary type. A type parameter counts as its
    /// constraint, so `T extends "a" | "b"` needs a case for `"a"` and one
    /// for `"b"`, and one without a constraint counts as itself. The
    /// statement is exhaustive when the guard of its bypass, which tests that
    /// no case matched, leaves that type `never`. For `switch (typeof x)`
    /// with `x: unknown` or `x: any`, the cases must test every kind of
    /// value, as in TypeScript.
    ///
    /// Returns false when the type is `Unknown`, `never`, or `any` outside
    /// `switch (typeof x)`. TypeScript gives false for `never` too, because
    /// it requires a union of unit types, such as literals.
    pub(super) fn switch_is_exhaustive(&mut self, root: usize, switch: u32) -> bool {
        let info = self.js_info;
        let Some(root) = info.flow.root(root) else {
            return false;
        };
        let Some(switch) = root.switches.get(switch as usize) else {
            return false;
        };
        self.is_exhaustive_in(root, switch)
    }

    /// Returns whether every value that the type of the discriminant of
    /// `switch`, a statement of `root`, allows matches a case. See
    /// [`Self::switch_is_exhaustive`].
    fn is_exhaustive_in(&mut self, root: &RootFlow, switch: &FlowSwitch) -> bool {
        let Some(discriminant) = self
            .flow_operand_at(switch.discriminant)
            .or_else(|| self.raw_operand_at(switch.discriminant))
        else {
            return false;
        };
        let Some(ty) = self.flow_baseline(discriminant) else {
            return false;
        };
        // As in TypeScript, a type parameter's constraint decides it.
        let ty = type_parameter_constraint(self.db, ty, self.narrowing_resolver()).unwrap_or(ty);
        if let (TypeData::AnyKeyword | TypeData::UnknownKeyword, Some(kinds)) =
            (ty, switch.typeof_kinds)
        {
            return kinds == u8::MAX;
        }
        if matches!(
            ty,
            TypeData::Unknown | TypeData::NeverKeyword | TypeData::AnyKeyword
        ) {
            return false;
        }
        let mut solve = GuardSolve::new(
            switch.subject,
            ty,
            false,
            0,
            None,
            FlowEvolving::None,
            None,
            0,
            None,
        );
        if !self.compares_with_unit_values(root, switch.guard, &mut solve) {
            return false;
        }
        let mut remaining = MAX_FLOW_STEPS;
        self.apply_guard(root, switch.guard, &mut solve, ty, &mut remaining)
            == Some(TypeData::NeverKeyword)
    }

    /// Returns whether every imported variable and every enum member that the
    /// guard at `guard` compares the subject of `solve` with holds a unit
    /// value.
    ///
    /// TypeScript finds a `switch` statement exhaustive only when the type of
    /// every case is a unit type. A case that compares with a value that
    /// Biome can't find a unit value for, as an imported variable whose
    /// initializer isn't a literal or an enum member whose value collection
    /// doesn't compute, narrows nothing, so without this check the remaining
    /// cases alone would make the statement exhaustive.
    fn compares_with_unit_values(
        &mut self,
        root: &RootFlow,
        guard: usize,
        solve: &mut GuardSolve<'db>,
    ) -> bool {
        let mut pending = vec![guard];
        while let Some(index) = pending.pop() {
            match root.guards.get(index) {
                Some(FlowGuard::Both { left, right, .. }) => pending.extend([*left, *right]),
                Some(
                    FlowGuard::ReadonlyPath(guard)
                    | FlowGuard::Bypass { guard, .. }
                    | FlowGuard::Unmatched(guard),
                ) => {
                    pending.push(*guard);
                }
                Some(FlowGuard::Test {
                    subject,
                    test:
                        FlowTest::Imported {
                            binding, member, ..
                        },
                    ..
                }) if *subject == solve.subject => {
                    if self.imported_value(*binding, *member, solve).is_none() {
                        return false;
                    }
                }
                Some(FlowGuard::Test {
                    subject,
                    test: FlowTest::EnumMember { member, .. },
                    ..
                }) if *subject == solve.subject => {
                    if self.enum_member_value(*member).is_none() {
                        return false;
                    }
                }
                Some(
                    FlowGuard::Keep
                    | FlowGuard::Incomplete
                    | FlowGuard::Test { .. }
                    | FlowGuard::Property { .. }
                    | FlowGuard::Call { .. },
                )
                | None => {}
            }
        }
        true
    }

    /// Returns whether the property path that `member` reads at `read` is
    /// readonly, as [`FlowGuard::ReadonlyPath`] requires. A path is readonly
    /// when, at every step, the declared type of the object declares the
    /// property readonly in every member other than `null` and `undefined`.
    /// The declared type of an object is the type of its expression without
    /// narrowing. [`has_readonly_property`] lists the properties that count as
    /// readonly.
    ///
    /// Every read of a path gives the same answer, so this context caches the
    /// answer per root and subject when it evaluates flow locally.
    ///
    /// Returns false when the syntax of `member` doesn't match the path of
    /// the subject, and when the declared type of an object is unknown.
    fn is_readonly_path(&mut self, read: FlowRead, member: &AnyJsExpression) -> bool {
        let key = (read.root, read.subject);
        if let Some(&cached) = self
            .local_flow
            .as_ref()
            .and_then(|local| local.readonly_paths.get(&key))
        {
            return cached;
        }
        let is_readonly = self.readonly_path_steps(read, member).unwrap_or(false);
        if let Some(local) = self.local_flow.as_mut() {
            local.readonly_paths.insert(key, is_readonly);
        }
        is_readonly
    }

    /// Returns whether the object at every step of the path that `member`
    /// reads declares the property readonly, checking from the last property
    /// to the first. Returns `None` when the syntax of `member` doesn't match
    /// the path, or when no type was collected for an object expression.
    fn readonly_path_steps(&mut self, read: FlowRead, member: &AnyJsExpression) -> Option<bool> {
        let info = self.js_info;
        let root = info.flow.root(read.root)?;
        let mut subject = read.subject;
        let mut expression = member.clone();
        while let FlowSubject::Member { object, key } = root.subject(subject)? {
            let (object_expression, name) = match (&expression, key) {
                (AnyJsExpression::JsStaticMemberExpression(member), PropertyKey::Name(name)) => {
                    (member.object().ok()?, name.text())
                }
                // Class members store a private name with its `#`, which the
                // subject's key lacks.
                (AnyJsExpression::JsStaticMemberExpression(member), PropertyKey::Private(_)) => {
                    (member.object().ok()?, collected_member_name(info, member)?)
                }
                (AnyJsExpression::JsComputedMemberExpression(member), PropertyKey::Name(name)) => {
                    (member.object().ok()?, name.text())
                }
                _ => return None,
            };
            let mut object_expression = object_expression;
            while let AnyJsExpression::JsParenthesizedExpression(parenthesized) = object_expression
            {
                object_expression = parenthesized.expression().ok()?;
            }
            let declared = self.raw_operand(&object_expression)?;
            if !has_readonly_property(self.db, declared, name, self.narrowing_resolver()) {
                return Some(false);
            }
            subject = *object;
            expression = object_expression;
        }
        Some(true)
    }

    /// Returns the type that narrowing of a subject whose declared type is
    /// `declared` starts from: `declared` with its unions, aliases, and
    /// `typeof` types flattened.
    ///
    /// Returns `None` when no runtime test can change the type, because it
    /// contains an undetermined member, and `Unknown` when flattening exceeds
    /// its work limit. `any` is a baseline, because a primitive `typeof`
    /// test, `instanceof`, and a type predicate narrow it.
    fn narrowing_baseline(&mut self, declared: TypeData<'db>) -> Option<TypeData<'db>> {
        match self.flow_baseline(declared) {
            Some(baseline) if is_narrowing_invariant(baseline) => None,
            Some(baseline) => Some(baseline),
            None => Some(TypeData::Unknown),
        }
    }

    /// Returns the type that narrowing of the binding declared at `binding`,
    /// or of the pattern at that range, starts from, given its `declared`
    /// type: [`Self::pattern_baseline`] for a pattern,
    /// [`Self::property_baseline`] when conditions narrow the binding only
    /// through tests of its shape, and [`Self::narrowing_baseline`]
    /// otherwise.
    fn binding_baseline(
        &mut self,
        binding: TextRange,
        declared: TypeData<'db>,
    ) -> Option<TypeData<'db>> {
        let flow = &self.js_info.flow;
        if flow.pattern_type(binding).is_some() {
            self.pattern_baseline(declared)
        } else if flow.is_property_only_binding(binding) {
            let instance_tested = flow.is_instance_tested_binding(binding);
            self.property_baseline(declared, instance_tested)
        } else if flow.is_assignment_only_binding(binding) {
            self.narrowing_baseline(declared)
                .filter(|baseline| may_narrow_by_assignment(*baseline))
        } else {
            self.narrowing_baseline(declared)
        }
    }

    /// Returns the type that narrowing of a destructuring pattern starts
    /// from, given its `declared` type, which is the type of the value it
    /// destructures.
    ///
    /// The result is the [`Self::property_baseline`] of `declared` without
    /// `null` and `undefined`, because destructuring throws on both. So in
    /// `if (!s) return; const { kind, v } = s;`, the pattern narrows even
    /// when the collected type of `s` there still includes `undefined`.
    fn pattern_baseline(&mut self, declared: TypeData<'db>) -> Option<TypeData<'db>> {
        let flattened = match self.flatten_baseline(declared, true) {
            Ok(flattened) => flattened,
            Err(FlatteningError::Exceeded) => return Some(TypeData::Unknown),
            Err(FlatteningError::Cycle) => return None,
        };
        let baseline = if let TypeData::Union(union) = flattened {
            TypeData::union_from_types(
                self.db,
                union
                    .types(self.db)
                    .iter()
                    .copied()
                    .filter(|member| !matches!(member, TypeData::Null | TypeData::Undefined))
                    .collect(),
            )
        } else {
            flattened
        };
        (!is_narrowing_invariant(baseline)
            && baseline != TypeData::AnyKeyword
            && may_narrow_by_property(self.db, baseline))
        .then_some(baseline)
    }

    /// Returns the type that narrowing of a subject in
    /// [`RootFlow::property_only`] starts from, given its `declared` type, or
    /// `None` when no test of its shape can narrow it.
    ///
    /// A test of a property or `in` narrows only a union with two object
    /// types, as [`may_narrow_by_property`] describes, so this is
    /// [`Self::narrowing_baseline`] when the flattened type is one. When
    /// `instance_tested` is true, `instanceof` or a type-guard function whose
    /// target has only object types narrows the subject too. These tests also
    /// narrow `unknown`, `any`, and a type with one object type, which
    /// [`may_narrow_by_instanceof`] checks. No test of a property or `in`
    /// narrows `any`, so the result is `any` only when `instance_tested` is
    /// true.
    ///
    /// Unlike [`Self::narrowing_baseline`], flattening resolves handles to
    /// local types of this context's module through this context instead of
    /// lookups. Complete module inference has resolved all of those types by
    /// the time it evaluates flow, so there, finding out that a subject can't
    /// narrow costs only cache reads. A lookup would resolve each type again
    /// in a fresh context.
    fn property_baseline(
        &mut self,
        declared: TypeData<'db>,
        instance_tested: bool,
    ) -> Option<TypeData<'db>> {
        match self.flatten_baseline(declared, true) {
            Ok(baseline) => (!is_narrowing_invariant(baseline)
                && if instance_tested {
                    may_narrow_by_instanceof(self.db, baseline)
                } else {
                    may_narrow_by_property(self.db, baseline)
                })
            .then_some(baseline),
            Err(FlatteningError::Exceeded) => Some(TypeData::Unknown),
            Err(FlatteningError::Cycle) => None,
        }
    }

    /// Returns `ty`, resolved through this context while it is a handle to a
    /// local type of this context's module, or `None` when such handles lead
    /// back to one already seen or exceed [`MAX_MODULE_HANDLE_STEPS`].
    fn resolve_module_handle(&mut self, mut ty: TypeData<'db>) -> Option<TypeData<'db>> {
        let mut seen = Vec::new();
        while let TypeData::Local(local) = ty
            && local.module(self.db) == self.module_key
        {
            if seen.contains(&local) || seen.len() == MAX_MODULE_HANDLE_STEPS {
                return None;
            }
            seen.push(local);
            ty = self.resolve_raw_type_id(TypeId::new(local.type_id(self.db).index()));
        }
        Some(ty)
    }

    /// Returns the type of the subject of `input` at each of `points`, given
    /// its `baseline` type and the rest of the solve's key, or `None` when
    /// the walk or the solve exceeds the flow work limit.
    fn solve_subject(
        &mut self,
        root: &RootFlow,
        points: &[FlowNodeId],
        input: FlowSubjectRootInput<'db>,
        baseline: TypeData<'db>,
    ) -> Option<Vec<TypeData<'db>>> {
        let db = self.db;
        let reaching = ReachingFlow::new(root, points)?;
        let states = self.solve_flow(
            root,
            &reaching,
            GuardSolve::new(
                input.subject(db),
                baseline,
                input.readonly_path(db),
                input.exhaustive(db),
                input.assigned(db),
                input.evolving(db),
                input.calls(db),
                input.ends(db),
                input.compared(db),
            ),
        )?;
        Some(
            reaching
                .points
                .iter()
                .map(|number| states[*number as usize])
                .collect(),
        )
    }

    /// Returns `ty` with its unions, aliases, and `typeof` types flattened, or
    /// `None` when flattening exceeds its work limit.
    pub(super) fn flow_baseline(&mut self, ty: TypeData<'db>) -> Option<TypeData<'db>> {
        self.flatten_baseline(ty, false).ok()
    }

    /// Returns `ty` with its unions, aliases, and `typeof` types flattened, or
    /// [`FlatteningError::Exceeded`] when flattening exceeds its work limit.
    ///
    /// When `in_context` is true, handles to local types of this context's
    /// module are resolved through this context instead of lookups, as
    /// [`Self::resolve_module_handle`] resolves them. A cycle of such handles,
    /// or a chain longer than [`MAX_MODULE_HANDLE_STEPS`], stops the
    /// flattening with [`FlatteningError::Cycle`].
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Other type shapes remain unchanged."
    )]
    fn flatten_baseline(
        &mut self,
        ty: TypeData<'db>,
        in_context: bool,
    ) -> Result<TypeData<'db>, FlatteningError> {
        let mut pending = vec![ty];
        let mut types = Vec::new();
        for step in 0..MAX_FLOW_TYPE_STEPS {
            let Some(ty) = pending.pop() else {
                return Ok(TypeData::union_from_types(self.db, types));
            };
            let ty = if in_context {
                self.resolve_module_handle(ty)
                    .ok_or(FlatteningError::Cycle)?
            } else {
                ty
            };
            let ty = resolve_local_type_on_demand(self.db, ty).expand_global_local(self.db);
            match ty {
                TypeData::Union(union) => {
                    let children = union.types(self.db);
                    if children.len()
                        > (MAX_FLOW_TYPE_STEPS - step - 1).saturating_sub(pending.len())
                    {
                        return Err(FlatteningError::Exceeded);
                    }
                    pending.extend(children.iter().rev().copied());
                }
                TypeData::TypeofType(wrapper) => pending.push(wrapper.ty(self.db)),
                TypeData::TypeofValue(wrapper) => pending.push(wrapper.ty(self.db)),
                TypeData::InstanceOf(instance) => {
                    let target = if in_context {
                        self.resolve_module_handle(instance.ty(self.db))
                            .ok_or(FlatteningError::Cycle)?
                    } else {
                        instance.ty(self.db)
                    };
                    let target = resolve_local_type_on_demand(self.db, target);
                    if target.should_flatten_instance(instance.type_parameters(self.db)) {
                        pending.push(target);
                    } else {
                        types.push(ty);
                    }
                }
                _ => types.push(ty),
            }
        }
        Err(FlatteningError::Exceeded)
    }

    /// Returns the type of the subject that `guards` narrows at every node of
    /// `reaching`, starting from its declared type at the root's start, or
    /// `None` when the solve exceeds the flow work limit.
    fn solve_flow(
        &mut self,
        root: &RootFlow,
        reaching: &ReachingFlow,
        mut guards: GuardSolve<'db>,
    ) -> Option<Vec<TypeData<'db>>> {
        // A variable or a field that may be read before it is assigned holds
        // `undefined` at the start, as in TypeScript.
        let start = if root.is_uninitialized(guards.subject) {
            TypeData::union_from_types(self.db, vec![guards.declared, TypeData::Undefined])
        } else {
            guards.declared
        };
        let count = reaching.nodes.len();
        let mut queue = VecDeque::new();
        for (number, node) in (0..).zip(&reaching.nodes) {
            if matches!(root.nodes[*node as usize], FlowNode::Start) {
                queue.push_back(number);
            }
        }
        let mut states = vec![TypeData::NeverKeyword; count];
        let mut queued = vec![false; count];
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(number) = queue.pop_front() {
            let index = number as usize;
            queued[index] = false;
            remaining = remaining.checked_sub(1)?;
            let predecessors = reaching.predecessors.get(number);
            let ty = match &root.nodes[reaching.nodes[index] as usize] {
                FlowNode::Start => start,
                FlowNode::Join(_) => {
                    // `never` adds nothing to a union, and a union of equal
                    // types is that type, so the union builder runs only when
                    // the remaining states differ.
                    let mut incoming = predecessors
                        .iter()
                        .map(|predecessor| states[*predecessor as usize])
                        .filter(|ty| *ty != TypeData::NeverKeyword);
                    match incoming.next() {
                        None => TypeData::NeverKeyword,
                        Some(first) if incoming.clone().all(|ty| ty == first) => first,
                        Some(first) => TypeData::union_from_types(
                            self.db,
                            std::iter::once(first).chain(incoming).collect(),
                        ),
                    }
                }
                FlowNode::Condition { guard, .. } => {
                    let incoming = states[predecessors[0] as usize];
                    if incoming == TypeData::NeverKeyword {
                        incoming
                    } else {
                        self.apply_guard(root, *guard, &mut guards, incoming, &mut remaining)?
                    }
                }
                FlowNode::Write { write, .. } => {
                    let incoming = states[predecessors[0] as usize];
                    if incoming == TypeData::NeverKeyword {
                        incoming
                    } else {
                        self.apply_write(root, *write, &mut guards, incoming)
                    }
                }
            };
            if states[index] != ty {
                states[index] = ty;
                for &successor in reaching.successors.get(number) {
                    if !queued[successor as usize] {
                        queued[successor as usize] = true;
                        queue.push_back(successor);
                    }
                }
            }
        }
        Some(states)
    }

    /// Returns the type of the subject that `solve` narrows after the guard
    /// at `index` holds, given its incoming type `ty`, or `None` when the flow
    /// work limit runs out.
    fn apply_guard(
        &mut self,
        root: &RootFlow,
        index: usize,
        solve: &mut GuardSolve<'db>,
        ty: TypeData<'db>,
        remaining: &mut usize,
    ) -> Option<TypeData<'db>> {
        *remaining = remaining.checked_sub(1)?;
        let Some(guard) = root.guards.get(index) else {
            return Some(TypeData::Unknown);
        };
        let subject = solve.subject;
        Some(match guard {
            FlowGuard::Keep => ty,
            FlowGuard::Incomplete => TypeData::Unknown,
            FlowGuard::Test {
                subject: tested,
                test,
                positive,
            } => {
                // Runtime tests don't narrow an array whose type evolves with
                // its writes.
                if *tested != subject || solve.evolving == FlowEvolving::Array {
                    return Some(ty);
                }
                // A variable whose type evolves with its writes has no
                // declared type, so the tests that look at one look at the
                // incoming type.
                let declared = if solve.evolving == FlowEvolving::Variable {
                    ty
                } else {
                    solve.declared
                };
                if let FlowTest::In(_)
                | FlowTest::Instanceof(_)
                | FlowTest::PrivateName { .. }
                | FlowTest::Constructor(_)
                | FlowTest::Predicate { .. } = test
                {
                    // Loops revisit conditions, and each application compares
                    // every member of the union.
                    if let Some(narrowed) = solve.properties.get(&(index, ty)) {
                        return Some(*narrowed);
                    }
                    let narrowed = match *test {
                        FlowTest::In(key) => self.narrow_by_in(ty, declared, key, *positive),
                        FlowTest::Predicate {
                            function,
                            object_target,
                        } => self.narrow_by_predicate(
                            ty,
                            declared,
                            function,
                            object_target,
                            *positive,
                        ),
                        test @ FlowTest::Instanceof(_) => {
                            self.narrow_by_instance(ty, declared, test, *positive)
                        }
                        FlowTest::PrivateName { class, is_static } => {
                            self.narrow_by_private_name(ty, declared, class, is_static, *positive)
                        }
                        FlowTest::Constructor(constructor) if *positive => {
                            self.narrow_by_constructor(ty, declared, constructor)
                        }
                        FlowTest::Constructor(_) => ty,
                        FlowTest::Truthy
                        | FlowTest::Nullish
                        | FlowTest::Typeof(_)
                        | FlowTest::Null
                        | FlowTest::Literal { .. }
                        | FlowTest::Undefined { .. }
                        | FlowTest::EnumMember { .. }
                        | FlowTest::Imported { .. }
                        | FlowTest::Compared { .. }
                        | FlowTest::ComparedReceiver { .. } => ty,
                    };
                    solve.properties.insert((index, ty), narrowed);
                    return Some(narrowed);
                }
                match self.test_predicate(*test, solve) {
                    Some(predicate) => {
                        narrow_type(self.db, ty, predicate, *positive, self.narrowing_resolver())
                    }
                    None => ty,
                }
            }
            FlowGuard::Property {
                object,
                key: PropertyKey::Name(name),
                test,
                positive,
                optional_chain,
                ..
            } => {
                if *object != subject || solve.evolving == FlowEvolving::Array {
                    return Some(ty);
                }
                let declared = if solve.evolving == FlowEvolving::Variable {
                    ty
                } else {
                    solve.declared
                };
                // Loops revisit conditions, and each application looks up the
                // property in every member of the union.
                if let Some(narrowed) = solve.properties.get(&(index, ty)) {
                    return Some(*narrowed);
                }
                let narrowed = match self.test_predicate(*test, solve) {
                    Some(predicate) => {
                        let test = PropertyTest {
                            name: name.text(),
                            predicate,
                            positive: *positive,
                            optional_chain: *optional_chain,
                        };
                        narrow_by_property(self.db, ty, declared, test, self.narrowing_resolver())
                    }
                    None => ty,
                };
                solve.properties.insert((index, ty), narrowed);
                narrowed
            }
            // Collection emits property guards only for named properties.
            FlowGuard::Property { .. } => ty,
            FlowGuard::ReadonlyPath(inner) => {
                if !solve.readonly_path {
                    return Some(ty);
                }
                return self.apply_guard(root, *inner, solve, ty, remaining);
            }
            FlowGuard::Bypass { switch, guard } => {
                if solve.exhaustive & (1u64 << *switch) != 0 {
                    TypeData::NeverKeyword
                } else {
                    return self.apply_guard(root, *guard, solve, ty, remaining);
                }
            }
            FlowGuard::Unmatched(guard) => {
                let members = if let TypeData::Union(union) = ty {
                    union.types(self.db)
                } else {
                    std::slice::from_ref(&ty)
                };
                if members.contains(&TypeData::UnknownKeyword) {
                    return Some(ty);
                }
                if !members
                    .iter()
                    .any(|member| is_type_parameter_or_intersection(self.db, *member))
                {
                    return self.apply_guard(root, *guard, solve, ty, remaining);
                }
                // Where no case matches, TypeScript removes the members whose
                // every value a case covers and keeps the others whole. So a
                // type parameter, or an intersection with one, stays as it is
                // unless the guard leaves it, or the constraint of a type
                // parameter, no value, instead of becoming the intersection
                // that the guard gives.
                let mut kept = Vec::with_capacity(members.len());
                let mut others = Vec::new();
                for &member in members {
                    if !is_type_parameter_or_intersection(self.db, member) {
                        others.push(member);
                    } else if self.apply_guard(root, *guard, solve, member, remaining)?
                        != TypeData::NeverKeyword
                        && !self.unmatched_constraint(root, *guard, solve, member, remaining)?
                    {
                        kept.push(member);
                    }
                }
                let removed_type_parameter = kept.len() + others.len() < members.len();
                if !others.is_empty() {
                    let others = TypeData::union_from_types(self.db, others);
                    let narrowed = self.apply_guard(root, *guard, solve, others, remaining)?;
                    if !removed_type_parameter && narrowed == others {
                        return Some(ty);
                    }
                    kept.push(narrowed);
                } else if !removed_type_parameter {
                    return Some(ty);
                }
                TypeData::union_from_types(self.db, kept)
            }
            FlowGuard::Call { call, positive } => {
                return self.apply_call(root, index, *call, *positive, solve, ty, remaining);
            }
            FlowGuard::Both {
                left,
                right,
                sequential,
            } => {
                let left = self.apply_guard(root, *left, solve, ty, remaining)?;
                let right = self.apply_guard(
                    root,
                    *right,
                    solve,
                    if *sequential { left } else { ty },
                    remaining,
                )?;
                if *sequential {
                    right
                } else {
                    TypeData::union_from_types(self.db, vec![left, right])
                }
            }
        })
    }

    /// Returns whether the guard at `guard`, which holds where no case of a
    /// `switch` matches, leaves no value of the constraint of `member`, a
    /// type parameter, so that TypeScript removes `member` there, as `T`
    /// with `T extends null` after `case null:`. Returns `None` when the flow
    /// work limit runs out.
    ///
    /// TypeScript removes a type parameter only when its constraint is one
    /// type, as `null`, `"a"`, or `string` for `case "string":` on `typeof`.
    /// It keeps `T extends "a" | "b"` whole after `case "a": case "b":`, so
    /// a union keeps `member`. So do `boolean`, which TypeScript treats as
    /// `true | false`, and `void` and `never`, which aren't unit types.
    fn unmatched_constraint(
        &mut self,
        root: &RootFlow,
        guard: usize,
        solve: &mut GuardSolve<'db>,
        member: TypeData<'db>,
        remaining: &mut usize,
    ) -> Option<bool> {
        let Some(constraint) =
            type_parameter_constraint(self.db, member, self.narrowing_resolver())
        else {
            return Some(false);
        };
        if matches!(
            constraint,
            TypeData::Union(_) | TypeData::Boolean | TypeData::VoidKeyword | TypeData::NeverKeyword
        ) {
            return Some(false);
        }
        Some(self.apply_guard(root, guard, solve, constraint, remaining)? == TypeData::NeverKeyword)
    }

    /// Returns the type of the subject that `solve` narrows after the guard
    /// at `index` holds, given its incoming type `ty`, or `None` when the
    /// flow work limit runs out. The guard is the [`FlowGuard::Call`] of the
    /// call point at index `call`, whose result is truthy when `positive` is
    /// true.
    ///
    /// The effect comes from the solve's key, where the reading context put
    /// it, so the solve resolves no callee. A type predicate narrows the
    /// operand that it names to its target where the call is truthy, and
    /// removes the target where the call is falsy, unless its target may be
    /// wider than TypeScript's (see [`FlowCallEffect::Predicate`]). An
    /// assertion narrows the operand as a truthy type predicate does.
    /// `asserts value` instead applies the guard of the argument decomposed
    /// as a condition that holds. A call whose effect the key lacks keeps
    /// `ty`.
    #[expect(
        clippy::too_many_arguments,
        reason = "The arguments mirror `apply_guard`, with the call's guard."
    )]
    fn apply_call(
        &mut self,
        root: &RootFlow,
        index: usize,
        call: u32,
        positive: bool,
        solve: &mut GuardSolve<'db>,
        ty: TypeData<'db>,
        remaining: &mut usize,
    ) -> Option<TypeData<'db>> {
        // A call that never returns adds nothing to any subject after it.
        if solve.ends != 0
            && root
                .ending_bit(call)
                .is_some_and(|bit| solve.ends & (1 << bit) != 0)
        {
            return Some(TypeData::NeverKeyword);
        }
        let effect = solve.calls.and_then(|calls| {
            let effects = calls.effects(self.db);
            effects
                .binary_search_by_key(&call, |(index, _)| *index)
                .ok()
                .map(|position| effects[position].1)
        });
        let (parameter, target, positive) = match effect {
            Some(FlowCallEffect::Predicate {
                parameter,
                target,
                narrows_falsy,
            }) => {
                if !positive && !narrows_falsy {
                    return Some(ty);
                }
                (parameter, Some(target), positive)
            }
            Some(FlowCallEffect::Assertion { parameter, target }) => (parameter, target, true),
            Some(FlowCallEffect::None | FlowCallEffect::Ends) | None => return Some(ty),
        };
        let Some(flow_call) = root.calls.get(call as usize) else {
            return Some(ty);
        };
        let operand = match parameter {
            FlowCallParameter::Argument(argument) => flow_call.arguments.get(argument as usize),
            FlowCallParameter::Receiver => None,
        };
        let Some(target) = target else {
            // `asserts value` holds the condition that its argument is
            // truthy, and `asserts this` narrows nothing.
            return match operand.and_then(|operand| operand.condition) {
                Some(guard) => self.apply_guard(root, guard, solve, ty, remaining),
                None => Some(ty),
            };
        };
        let tested = match parameter {
            FlowCallParameter::Argument(_) => operand.and_then(|operand| operand.subject),
            FlowCallParameter::Receiver => flow_call.receiver.map(|receiver| receiver.subject),
        };
        if tested != Some(solve.subject) || solve.evolving == FlowEvolving::Array {
            return Some(ty);
        }
        // Loops revisit conditions, and each application compares every
        // member of the union.
        if let Some(narrowed) = solve.properties.get(&(index, ty)) {
            return Some(*narrowed);
        }
        let narrowed =
            narrow_by_type_predicate(self.db, ty, target, positive, self.narrowing_resolver());
        solve.properties.insert((index, ty), narrowed);
        Some(narrowed)
    }

    /// Returns the type of the subject that `solve` narrows right after the
    /// write at index `index`, given its incoming type `ty`. TypeScript types
    /// a reference after an assignment the same way.
    ///
    /// A write of the subject itself gives the declared type reduced by the
    /// assigned type; [`FlowWriteEffect`] lists the exceptions. The result
    /// depends only on the declared type and the value, except for a compound
    /// assignment, so it is computed once per write. A write of a proper
    /// prefix of the subject's path, or of any property of one, gives the
    /// declared type, as in TypeScript. Any other write leaves the incoming
    /// type.
    fn apply_write(
        &mut self,
        root: &RootFlow,
        index: u32,
        solve: &mut GuardSolve<'db>,
        ty: TypeData<'db>,
    ) -> TypeData<'db> {
        let Some(write) = root.writes.get(index as usize) else {
            return TypeData::Unknown;
        };
        if write.subject != solve.subject {
            return if root.is_proper_prefix(write.subject, solve.subject) {
                solve.declared
            } else {
                ty
            };
        }
        if solve.evolving != FlowEvolving::None {
            return self.apply_evolving_write(root, write.effect, solve, ty);
        }
        let (declared, value) = match write.effect {
            FlowWriteEffect::Wildcard => return ty,
            // Only a subject whose type evolves with its writes has these.
            FlowWriteEffect::Elements(_) | FlowWriteEffect::EmptyArray => return solve.declared,
            FlowWriteEffect::Compound => return base_literal_type(self.db, ty),
            FlowWriteEffect::Reset | FlowWriteEffect::Assign(None) => return solve.declared,
            FlowWriteEffect::CompoundLike(None) => {
                return base_literal_type(self.db, solve.declared);
            }
            FlowWriteEffect::Assign(Some(value)) => (solve.declared, Some(value)),
            FlowWriteEffect::CompoundLike(Some(value)) => {
                (base_literal_type(self.db, solve.declared), Some(value))
            }
            FlowWriteEffect::Undefined => (solve.declared, None),
        };
        if let Some(narrowed) = solve.writes.get(&index) {
            return *narrowed;
        }
        let assigned = match value {
            Some(value) => root
                .subject_writes(solve.subject)
                .binary_search(&value)
                .ok()
                .and_then(|position| {
                    solve
                        .assigned
                        .and_then(|assigned| assigned.types(self.db).get(position).copied())
                }),
            None => Some(TypeData::Undefined),
        };
        let is_boolean_literal = value
            .and_then(|value| root.assigned.get(value as usize))
            .is_some_and(|assigned| {
                matches!(assigned.value, FlowAssignedValue::BooleanLiteral(_))
                    && assigned.steps.is_empty()
            });
        let narrowed = match assigned {
            Some(assigned) => narrow_by_assignment(
                self.db,
                declared,
                assigned,
                is_boolean_literal,
                self.narrowing_resolver(),
            ),
            None => declared,
        };
        solve.writes.insert(index, narrowed);
        narrowed
    }

    /// Returns the type of a subject whose type evolves with its writes right
    /// after its write `effect`, given its incoming type `ty`, as TypeScript
    /// types such a subject after an assignment or an array mutation.
    ///
    /// A variable takes the type of the assigned value. An array starts again
    /// at `Array<never>` for an empty array literal, takes the type of another
    /// assigned array, and adds a value to the element type of `ty`. An
    /// assignment whose value Biome can't type as TypeScript does, or whose
    /// value is not an array for an array, gives `Unknown`, so the subject is
    /// Unknown at the reads that it reaches. A value that Biome can't type
    /// that a write adds to an array gives an array of `Unknown`, so the array
    /// keeps its declared type there.
    fn apply_evolving_write(
        &self,
        root: &RootFlow,
        effect: FlowWriteEffect,
        solve: &GuardSolve<'db>,
        ty: TypeData<'db>,
    ) -> TypeData<'db> {
        let db = self.db;
        let assigned = |value: u32| {
            root.subject_writes(solve.subject)
                .binary_search(&value)
                .ok()
                .and_then(|position| {
                    solve
                        .assigned
                        .and_then(|assigned| assigned.types(db).get(position).copied())
                })
                .unwrap_or(TypeData::Unknown)
        };
        match (solve.evolving, effect) {
            (_, FlowWriteEffect::Wildcard) => ty,
            (FlowEvolving::Variable, FlowWriteEffect::Assign(Some(value))) => assigned(value),
            (FlowEvolving::Array, FlowWriteEffect::EmptyArray) => {
                array_of(db, TypeData::NeverKeyword)
            }
            (FlowEvolving::Array, FlowWriteEffect::Assign(Some(value))) => {
                let value = assigned(value);
                if evolving_array_element(db, value).is_some() {
                    value
                } else {
                    TypeData::Unknown
                }
            }
            // A value that Biome can't type gives `Array<Unknown>`, because
            // `Unknown` absorbs the other members of a union.
            (FlowEvolving::Array, FlowWriteEffect::Elements(value)) => {
                let value = value.map_or(TypeData::Unknown, assigned);
                match evolving_array_element(db, ty) {
                    Some(element) => {
                        array_of(db, TypeData::union_from_types(db, vec![element, value]))
                    }
                    None => TypeData::Unknown,
                }
            }
            (
                FlowEvolving::None | FlowEvolving::Variable,
                FlowWriteEffect::Elements(_) | FlowWriteEffect::EmptyArray,
            )
            | (
                FlowEvolving::None | FlowEvolving::Variable | FlowEvolving::Array,
                FlowWriteEffect::Assign(_)
                | FlowWriteEffect::CompoundLike(_)
                | FlowWriteEffect::Compound
                | FlowWriteEffect::Undefined
                | FlowWriteEffect::Reset,
            ) => TypeData::Unknown,
        }
    }

    /// Narrows `ty` by `name in value`, where `name` is the string or number
    /// literal at `key`. When `positive` is true, the members of `ty` that
    /// lack the property are removed, or, when every member lacks it, `ty`
    /// becomes its intersection with `{ name: unknown }`. When `positive` is
    /// false, the members that declare it as required are removed. See
    /// [`narrow_by_in_operator`].
    ///
    /// Returns `ty` unchanged when the subject's `declared` type has no
    /// object type and is neither `unknown` nor `any`, because the test could
    /// then only make a branch impossible. Also returns `ty` unchanged when
    /// the key doesn't resolve to a string or number literal.
    fn narrow_by_in(
        &mut self,
        ty: TypeData<'db>,
        declared: TypeData<'db>,
        key: TextRange,
        positive: bool,
    ) -> TypeData<'db> {
        if !may_narrow_by_instanceof(self.db, declared) {
            return ty;
        }
        let Some(reference) = self.js_info.raw_expressions.get(&key) else {
            return ty;
        };
        let TypeData::Literal(literal) = self.resolve(reference) else {
            return ty;
        };
        let name = match literal.literal(self.db) {
            Literal::String(string) => string.as_ref(),
            Literal::Number(number) => number.text(),
            Literal::BigInt(_)
            | Literal::Boolean(_)
            | Literal::Object(_)
            | Literal::RegExp(_)
            | Literal::Template(_) => return ty,
        };
        narrow_by_in_operator(self.db, ty, name, positive, self.narrowing_resolver())
    }

    /// Returns `ty` narrowed by `value instanceof C` through
    /// [`narrow_by_instanceof`], where `test` is the [`FlowTest::Instanceof`]
    /// that names `C`.
    ///
    /// Returns `ty` unchanged when the subject's `declared` type has no object
    /// type and is neither `unknown` nor `any`, because the test could then
    /// only make a branch impossible. `C` is resolved only after that check.
    fn narrow_by_instance(
        &mut self,
        ty: TypeData<'db>,
        declared: TypeData<'db>,
        test: FlowTest,
        positive: bool,
    ) -> TypeData<'db> {
        let FlowTest::Instanceof(constructor) = test else {
            return ty;
        };
        if !may_narrow_by_instanceof(self.db, declared) {
            return ty;
        }
        let Some(reference) = self.js_info.raw_expressions.get(&constructor) else {
            return ty;
        };
        let constructor = self.resolve(reference);
        narrow_by_instanceof(
            self.db,
            ty,
            constructor,
            positive,
            self.narrowing_resolver(),
        )
    }

    /// Returns `ty` narrowed by `#name in value` through
    /// [`narrow_by_private_name`], where `class` is the range at which the
    /// class that declares `#name` has its type, and `is_static` tells
    /// whether the name is static.
    ///
    /// Returns `ty` unchanged when the subject's `declared` type has no object
    /// type and is neither `unknown` nor `any`, and when Biome has no type for
    /// the class. The class is resolved only after that check.
    fn narrow_by_private_name(
        &mut self,
        ty: TypeData<'db>,
        declared: TypeData<'db>,
        class: TextRange,
        is_static: bool,
        positive: bool,
    ) -> TypeData<'db> {
        if !may_narrow_by_instanceof(self.db, declared) {
            return ty;
        }
        let js_info = self.js_info;
        let Some(reference) = js_info
            .raw_binding_types
            .get(&class)
            .or_else(|| js_info.raw_expressions.get(&class))
        else {
            return ty;
        };
        let class = self.resolve(reference);
        narrow_by_private_name(
            self.db,
            ty,
            class,
            is_static,
            positive,
            self.narrowing_resolver(),
        )
    }

    /// Returns `ty` narrowed by `value.constructor === C` through
    /// [`narrow_by_constructor`], where `constructor` is the range of `C`, in
    /// the branch where the comparison holds.
    ///
    /// Returns `ty` unchanged when the subject's `declared` type has no object
    /// type and is neither `unknown` nor `any`. `C` is resolved only after
    /// that check.
    fn narrow_by_constructor(
        &mut self,
        ty: TypeData<'db>,
        declared: TypeData<'db>,
        constructor: TextRange,
    ) -> TypeData<'db> {
        if !may_narrow_by_instanceof(self.db, declared) {
            return ty;
        }
        let Some(reference) = self.js_info.raw_expressions.get(&constructor) else {
            return ty;
        };
        let constructor = self.resolve(reference);
        narrow_by_constructor(self.db, ty, constructor, self.narrowing_resolver())
    }

    /// Returns the predicate that `test` applies.
    ///
    /// A comparison with a unit value narrows as by that value, strictly or
    /// loosely ([`NarrowingPredicate::new_equality`]). A comparison by a
    /// declared type narrows by the type in the solve's key, and so does a
    /// comparison with an imported variable that holds no unit value when the
    /// key has its type. The test of a receiver of such a comparison's
    /// optional chain applies only when that type excludes `undefined`, and,
    /// for a loose comparison, `null`.
    ///
    /// Returns `None` for a call to a type-guard function, an `in` test, and an
    /// `instanceof` test, which have no predicate, and for a comparison with a
    /// value that has neither a unit value nor a type in the key, because such
    /// a comparison narrows nothing.
    fn test_predicate(
        &mut self,
        test: FlowTest,
        solve: &mut GuardSolve<'db>,
    ) -> Option<NarrowingPredicate<'db>> {
        let db = self.db;
        Some(match test {
            FlowTest::Truthy => NarrowingPredicate::Truthy,
            FlowTest::Nullish => NarrowingPredicate::Nullish,
            FlowTest::Typeof(kind) => NarrowingPredicate::Typeof(kind),
            FlowTest::Null => NarrowingPredicate::Literal(TypeData::Null),
            FlowTest::Literal { value, strict } => {
                let reference = self.js_info.raw_expressions.get(&value)?.clone();
                unit_predicate(self.resolve(&reference), strict)
            }
            FlowTest::Undefined { strict: true } => {
                NarrowingPredicate::Literal(TypeData::Undefined)
            }
            FlowTest::Undefined { strict: false } => NarrowingPredicate::Nullish,
            FlowTest::EnumMember { member, strict } => {
                unit_predicate(self.enum_member_value(member)?, strict)
            }
            FlowTest::Imported {
                binding,
                member,
                strict,
            } => match self.imported_value(binding, member, solve) {
                Some(value) => unit_predicate(value, strict),
                None if member.is_none() => {
                    NarrowingPredicate::new_equality(db, solve.compared_type(db, binding)?, strict)
                }
                None => return None,
            },
            FlowTest::Compared { value, strict } => {
                NarrowingPredicate::new_equality(db, solve.compared_type(db, value)?, strict)
            }
            FlowTest::ComparedReceiver { value, strict } => {
                let compared = solve.compared_type(db, value)?;
                let nullish = narrow_type(
                    db,
                    compared,
                    NarrowingPredicate::Nullish,
                    true,
                    self.narrowing_resolver(),
                );
                let excluded = if strict {
                    narrow_type(
                        db,
                        nullish,
                        NarrowingPredicate::Literal(TypeData::Undefined),
                        true,
                        self.narrowing_resolver(),
                    )
                } else {
                    nullish
                };
                if excluded != TypeData::NeverKeyword {
                    return None;
                }
                NarrowingPredicate::Nullish
            }
            FlowTest::Predicate { .. }
            | FlowTest::In(_)
            | FlowTest::Instanceof(_)
            | FlowTest::PrivateName { .. }
            | FlowTest::Constructor(_) => {
                return None;
            }
        })
    }

    /// Returns the unit value that the imported variable declared at `binding`
    /// holds in the module that declares it, or, when `member` is the range
    /// of a member's name, the value of that member of the enum that the
    /// variable imports. Returns `None` when there is no such unit value.
    ///
    /// The result is kept in `solve`, so a loop that applies the test again
    /// doesn't follow the import again. [`imported_unit_value`] lists the
    /// imports followed.
    fn imported_value(
        &mut self,
        binding: TextRange,
        member: Option<TextRange>,
        solve: &mut GuardSolve<'db>,
    ) -> Option<TypeData<'db>> {
        let key = member.unwrap_or(binding);
        if let Some(value) = solve.imported.get(&key) {
            return *value;
        }
        let value = imported_unit_value(self.db, self.module, self.js_info, binding, member);
        solve.imported.insert(key, value);
        value
    }

    /// Returns the literal type that collection gives the member of an enum
    /// of this module whose name is at `member`, or `None` when collection
    /// gives it another type, because it doesn't compute the member's value.
    fn enum_member_value(&self, member: TextRange) -> Option<TypeData<'db>> {
        raw_literal_type(
            self.db,
            self.js_info,
            self.js_info.raw_binding_types.get(&member)?,
        )
    }

    /// Returns `ty` narrowed by a call to the type-guard function declared at
    /// `function`, given the subject's `declared` type, with
    /// [`narrow_by_type_predicate`].
    ///
    /// `object_target` is true when collection found that every member of the
    /// target is an object type. Biome narrows by such a target only a type
    /// that has an object type or is `unknown` or `any`, so when `declared` is
    /// not one, this returns `ty` unchanged without resolving the target.
    /// TypeScript also narrows a primitive by such a target, as `string` by
    /// `{ length: number }`. See [`may_narrow_by_instanceof`].
    fn narrow_by_predicate(
        &mut self,
        ty: TypeData<'db>,
        declared: TypeData<'db>,
        function: TextRange,
        object_target: bool,
        positive: bool,
    ) -> TypeData<'db> {
        if object_target && !may_narrow_by_instanceof(self.db, declared) {
            return ty;
        }
        let Some(target) = self.guard_target(function) else {
            return ty;
        };
        narrow_by_type_predicate(self.db, ty, target, positive, self.narrowing_resolver())
    }

    /// Returns the target type of the type predicate of the guard declared at
    /// `function`, flattened as a declared type is, or `None` when the guard
    /// has no target, as `asserts value`, or flattening exceeds its work
    /// limit.
    fn guard_target(&mut self, function: TextRange) -> Option<TypeData<'db>> {
        let info = self.js_info;
        let (_, contract) = guard_signature(&info.raw_types, &info.raw_binding_types, function)?;
        let target = self.resolve(contract.target?);
        // An alias resolves to an instance of its target. The baseline flattens
        // that instance, as it does for a variable declared with the alias.
        self.flow_baseline(target)
    }
}

/// Returns whether `ty` is `boolean`, whether written so or as `true | false`.
fn is_boolean(db: &dyn ModuleDb, ty: TypeData) -> bool {
    if ty == TypeData::Boolean {
        return true;
    }
    let TypeData::Union(union) = ty else {
        return false;
    };
    let types = union.types(db);
    types.len() == 2
        && [true, false].iter().all(|value| {
            types.iter().any(|member| {
                matches!(member, TypeData::Literal(literal)
                    if matches!(literal.literal(db), Literal::Boolean(boolean)
                        if boolean.as_bool() == *value))
            })
        })
}

/// Limits the imports followed to find the value of an imported variable that
/// a comparison names.
const MAX_IMPORT_STEPS: usize = 16;

/// Returns the unit value that the imported variable declared at `binding` in
/// `js_info`, the module `module`, holds: `null`, `undefined`, or a string,
/// number, bigint, or boolean literal type. When `member` is the range of a
/// member's name in `js_info`, as `A` in `E.A`, returns the value of that
/// member of the enum that the variable imports instead: the literal type
/// that collection gives the member in the module that declares the enum.
///
/// The import is followed through re-exports to the module that declares the
/// variable. When that variable is a `const` that names another imported
/// variable or a member of an imported enum, or is itself an import, the
/// search continues with that import, through at most 16 imports in all. The
/// value comes from the syntax of the declarations and the raw type of the
/// literal, without resolving any type, so it is the same in every
/// resolution context.
///
/// Returns `None` for a type-only import, a namespace import, an import that
/// doesn't resolve to a single declaration, an export that isn't a variable
/// or, for a member, an enum, a variable that holds no unit value, a member
/// whose value collection doesn't compute, and more than 16 imports. An
/// export that isn't a variable includes `export default "a"`, which
/// TypeScript narrows by.
fn imported_unit_value<'db>(
    db: &'db dyn ModuleDb,
    mut module: ModuleInfo,
    js_info: &JsModuleInfo,
    mut binding: TextRange,
    member: Option<TextRange>,
) -> Option<TypeData<'db>> {
    let mut info = js_info;
    let mut member = match member {
        Some(member) => Some(member_name_at(js_info, member)?),
        None => None,
    };
    for _ in 0..MAX_IMPORT_STEPS {
        let TypeReference::Import(import) = info.raw_binding_types.get(&binding)? else {
            return None;
        };
        if import.type_only {
            return None;
        }
        let name = match &import.symbol {
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::Default => Text::new_static("default"),
            ImportSymbol::All => return None,
        };
        let resolved =
            resolve_module_import(db, module, &import.specifier, ResolutionMode::JavaScript);
        let target = db.module_for_path(resolved.path().as_path()?)?;
        let ExportOriginResult::Found {
            module: origin,
            name,
        } = find_export_origin(db, target, name)
        else {
            return None;
        };
        let ModuleInfoKind::Js(origin_info) = origin.kind(db) else {
            return None;
        };
        let Some(JsExport::Own(JsOwnExport::Binding(declaration))) =
            origin_info.exports.get(name.text())
        else {
            return None;
        };
        let declared = origin_info
            .semantic_model
            .as_binding_by_range(*declaration)?;
        if let Some(name) = &member {
            if let Some(AnyJsBindingDeclaration::TsEnumDeclaration(enum_declaration)) =
                declared.tree().declaration()
            {
                let range = enum_member_name_range(&enum_declaration, name.text())?;
                return raw_literal_type(
                    db,
                    origin_info,
                    origin_info.raw_binding_types.get(&range)?,
                );
            }
            if !declared.is_imported() {
                return None;
            }
            module = origin;
            info = origin_info;
            binding = *declaration;
            continue;
        }
        match binding_unit_value(&declared, &origin_info.semantic_model)? {
            UnitValue::Null => return Some(TypeData::Null),
            UnitValue::Undefined => return Some(TypeData::Undefined),
            UnitValue::Literal(literal) => {
                return raw_literal_type(
                    db,
                    origin_info,
                    origin_info.raw_expressions.get(&literal)?,
                );
            }
            UnitValue::EnumMember(name) => {
                return raw_literal_type(
                    db,
                    origin_info,
                    origin_info.raw_binding_types.get(&name)?,
                );
            }
            UnitValue::Imported {
                binding: next,
                member: next_member,
            } => {
                if let Some(next_member) = next_member {
                    member = Some(member_name_at(origin_info, next_member)?);
                }
                module = origin;
                info = origin_info;
                binding = next;
            }
        }
    }
    None
}

/// Returns the name of a member that the token at `range` in `js_info` gives,
/// as `A` in `E.A` or in `E["A"]`.
fn member_name_at(js_info: &JsModuleInfo, range: TextRange) -> Option<TokenText> {
    let token = js_info
        .semantic_model
        .root()
        .syntax()
        .covering_element(range)
        .into_token()?;
    Some(member_key(&token))
}

/// Classifies whether the expression at `range` in `js_info`, the module
/// `module`, reads a member of an enum through an identifier that names the
/// enum, as `E.A` and `E["A"]` do, also inside parentheses.
///
/// The identifier names an enum when it binds to an enum declaration of the
/// module, or to an import that names one; see
/// [`classify_imported_enum_member`]. The syntax and the semantic models of
/// the modules decide it, without inferring any type.
///
/// Returns [`TypeInferenceClassification::NoMatch`] for other expressions,
/// including `E?.A`, a member that the enum doesn't declare with an
/// identifier or a string literal as its name, and a member read through a
/// variable that holds the enum. Returns
/// [`TypeInferenceClassification::Indeterminate`] when no expression has
/// exactly that range, or when the import can't be followed.
pub(in crate::db) fn classify_enum_member(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    range: TextRange,
) -> TypeInferenceClassification {
    let start = match js_info
        .semantic_model
        .root()
        .syntax()
        .covering_element(range)
    {
        NodeOrToken::Node(node) => Some(node),
        NodeOrToken::Token(token) => token.parent(),
    };
    let Some(expression) = start
        .into_iter()
        .flat_map(|node| node.ancestors())
        .filter_map(AnyJsExpression::cast)
        .find(|expression| expression.range() == range)
    else {
        return TypeInferenceClassification::Indeterminate;
    };
    match enum_member_unit_value(&expression.omit_parentheses(), &js_info.semantic_model) {
        Some(UnitValue::EnumMember(_)) => TypeInferenceClassification::Match,
        Some(UnitValue::Imported {
            binding,
            member: Some(member),
        }) => match member_name_at(js_info, member) {
            Some(member) => {
                classify_imported_enum_member(db, module, js_info, binding, member.text())
            }
            None => TypeInferenceClassification::Indeterminate,
        },
        Some(
            UnitValue::Null
            | UnitValue::Undefined
            | UnitValue::Literal(_)
            | UnitValue::Imported { member: None, .. },
        )
        | None => TypeInferenceClassification::NoMatch,
    }
}

/// Classifies whether the variable that an import declares at `binding` in
/// `js_info`, the module `module`, names an enum that has a member named
/// `member`.
///
/// The import is followed through re-exports to the module that declares
/// the variable. When an import declares the variable there too, the search
/// continues with that import, through at most 16 imports in all.
///
/// Returns [`TypeInferenceClassification::NoMatch`] when the declaration
/// isn't an enum, as a `const` or a namespace import, or is an enum without
/// a member of that name written as an identifier or a string literal.
/// Returns [`TypeInferenceClassification::Indeterminate`] for an import that
/// doesn't resolve to a single declaration, and after 16 imports.
fn classify_imported_enum_member(
    db: &dyn ModuleDb,
    mut module: ModuleInfo,
    js_info: &JsModuleInfo,
    mut binding: TextRange,
    member: &str,
) -> TypeInferenceClassification {
    let mut info = js_info;
    for _ in 0..MAX_IMPORT_STEPS {
        let Some(TypeReference::Import(import)) = info.raw_binding_types.get(&binding) else {
            return TypeInferenceClassification::Indeterminate;
        };
        let name = match &import.symbol {
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::Default => Text::new_static("default"),
            ImportSymbol::All => return TypeInferenceClassification::NoMatch,
        };
        let resolved =
            resolve_module_import(db, module, &import.specifier, ResolutionMode::JavaScript);
        let Some(target) = resolved
            .path()
            .as_path()
            .and_then(|path| db.module_for_path(path))
        else {
            return TypeInferenceClassification::Indeterminate;
        };
        let ExportOriginResult::Found {
            module: origin,
            name,
        } = find_export_origin(db, target, name)
        else {
            return TypeInferenceClassification::Indeterminate;
        };
        let ModuleInfoKind::Js(origin_info) = origin.kind(db) else {
            return TypeInferenceClassification::Indeterminate;
        };
        let Some(JsExport::Own(JsOwnExport::Binding(declaration))) =
            origin_info.exports.get(name.text())
        else {
            return TypeInferenceClassification::NoMatch;
        };
        let Some(declared) = origin_info.semantic_model.as_binding_by_range(*declaration) else {
            return TypeInferenceClassification::Indeterminate;
        };
        if let Some(AnyJsBindingDeclaration::TsEnumDeclaration(enum_declaration)) =
            declared.tree().declaration()
        {
            return if enum_member_name_range(&enum_declaration, member).is_some() {
                TypeInferenceClassification::Match
            } else {
                TypeInferenceClassification::NoMatch
            };
        }
        if !declared.is_imported() {
            return TypeInferenceClassification::NoMatch;
        }
        module = origin;
        info = origin_info;
        binding = *declaration;
    }
    TypeInferenceClassification::Indeterminate
}

/// Returns whether the return type of a call to the imported binding at
/// `binding` in `js_info`, the module `module`, comes from an annotation.
/// For a namespace import, the callee is the export that `member` names.
/// See [`followed_return_is_annotated`].
fn imported_return_is_annotated(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    binding: TextRange,
    member: Option<&Text>,
) -> bool {
    followed_return_is_annotated(db, module, js_info, FollowedCallee::Import(binding, member))
}

/// A callee that [`followed_return_is_annotated`] follows to the declaration
/// that gives its return type.
enum FollowedCallee<'a> {
    /// The binding at this range, which an import declares, with the name of
    /// the export for a namespace import.
    Import(TextRange, Option<&'a Text>),
    /// The variable that a declaration file types with `typeof`, as `f` in
    /// `typeof f`.
    Queried(JsReferenceIdentifier),
}

/// Returns whether the return type of a call to `callee`, found in
/// `js_info`, the module `module`, comes from an annotation.
///
/// An import is followed through re-exports to the module that declares the
/// export. A binding that an import declares there is followed in turn. In a
/// declaration file, a declaration is annotated, because a declaration file
/// declares only annotated functions, unless its type annotation is `typeof`
/// of a variable: that variable is followed in turn, and a variable that the
/// file doesn't declare is a global of the lib or of a declaration file,
/// which is annotated. At most [`MAX_IMPORT_STEPS`] imports and queries are
/// followed. Elsewhere a declaration is annotated when it has a return type
/// annotation or a function type annotation; see [`declaration_return`].
fn followed_return_is_annotated(
    db: &dyn ModuleDb,
    mut module: ModuleInfo,
    js_info: &JsModuleInfo,
    mut callee: FollowedCallee,
) -> bool {
    let mut info = js_info;
    for _ in 0..MAX_IMPORT_STEPS {
        let declared = match callee {
            FollowedCallee::Import(binding, member) => {
                let Some(TypeReference::Import(import)) = info.raw_binding_types.get(&binding)
                else {
                    return false;
                };
                let name = match &import.symbol {
                    ImportSymbol::Named(name) => name.clone(),
                    ImportSymbol::Default => Text::new_static("default"),
                    ImportSymbol::All => match member {
                        Some(member) => member.clone(),
                        None => return false,
                    },
                };
                let resolved = resolve_module_import(
                    db,
                    module,
                    &import.specifier,
                    ResolutionMode::JavaScript,
                );
                let Some(target) = resolved
                    .path()
                    .as_path()
                    .and_then(|path| db.module_for_path(path))
                else {
                    return false;
                };
                let ExportOriginResult::Found {
                    module: origin,
                    name,
                } = find_export_origin(db, target, name)
                else {
                    return false;
                };
                let ModuleInfoKind::Js(origin_info) = origin.kind(db) else {
                    return false;
                };
                let Some(JsExport::Own(JsOwnExport::Binding(declaration))) =
                    origin_info.exports.get(name.text())
                else {
                    return false;
                };
                let Some(declared) = origin_info.semantic_model.as_binding_by_range(*declaration)
                else {
                    return false;
                };
                module = origin;
                info = origin_info;
                declared
            }
            FollowedCallee::Queried(name) => match info.semantic_model.binding(&name) {
                Some(declared) => declared,
                None => return true,
            },
        };
        callee = match declaration_return(&declared) {
            CalleeReturn::Resolved(FlowCallee::Import(next)) => FollowedCallee::Import(next, None),
            _ if is_declaration_file(db, module) => match declared_return(&declared) {
                DeclaredReturn::Stated => return true,
                DeclaredReturn::Queried(name) => FollowedCallee::Queried(name),
                DeclaredReturn::Unfollowed => return false,
            },
            CalleeReturn::Annotated => return true,
            CalleeReturn::Inferred | CalleeReturn::Resolved(FlowCallee::Member(_)) => return false,
        };
    }
    false
}

/// Returns the scope that `ty` records when it is a class or an interface.
fn member_scope(db: &dyn ModuleDb, ty: TypeData) -> Option<ScopeId> {
    if let TypeData::Class(class) = ty {
        class.scope_id(db)
    } else if let TypeData::Interface(interface) = ty {
        interface.scope_id(db)
    } else {
        None
    }
}

/// Returns whether a call of each property named `name` that the class or
/// the interface whose scope is `scope_id` declares itself, in the
/// declaration file `module`, has a return type from an annotation. A
/// property typed `typeof` of a variable has one only when that variable
/// does (see [`followed_return_is_annotated`]); see [`declared_member_returns`]
/// for the other members.
fn declared_members_are_annotated(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    scope_id: Option<ScopeId>,
    name: &str,
) -> bool {
    let Some(scope_id) = scope_id else {
        return false;
    };
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return false;
    };
    let Some(declaration) =
        AnyMemberDeclaration::cast(info.semantic_model.scope_from_id(scope_id).syntax())
    else {
        return false;
    };
    declared_member_returns(&declaration, name)
        .into_iter()
        .all(|declared| match declared {
            DeclaredReturn::Stated => true,
            DeclaredReturn::Queried(name) => {
                followed_return_is_annotated(db, module, info, FollowedCallee::Queried(name))
            }
            DeclaredReturn::Unfollowed => false,
        })
}

/// Returns the operand of `call` that the type predicate or the assertion of
/// `function` names with `name`: the receiver for `this`, or the argument at
/// the position of the parameter with that name, counted without a `this`
/// parameter.
///
/// Returns `None` when no parameter has the name or more than one has it,
/// when it is a rest or an optional parameter, when the call passes no
/// argument there before a spread, and for a name spelled with an escape. An
/// optional parameter is rejected because a default can replace its
/// argument.
fn predicate_parameter<'db>(
    db: &'db dyn ModuleDb,
    function: InternedFunction<'db>,
    name: &Text,
    call: &FlowCall,
) -> Option<FlowCallParameter> {
    let name = name.text();
    if name.contains('\\') {
        return None;
    }
    if name == "this" {
        return Some(FlowCallParameter::Receiver);
    }
    let mut found = None;
    let parameters = function
        .parameters(db)
        .iter()
        .filter(|parameter| !parameter.is_this());
    for (index, parameter) in parameters.enumerate() {
        let FunctionParameter::Named(named) = parameter else {
            continue;
        };
        if named.name.text() != name {
            continue;
        }
        if found.is_some() || named.is_rest || named.is_optional {
            return None;
        }
        found = Some(index);
    }
    let index = found?;
    if index >= call.arguments.len() {
        return None;
    }
    Some(FlowCallParameter::Argument(u32::try_from(index).ok()?))
}

/// Returns whether `callee`, the collected type of a call's callee, passes
/// explicit type arguments, as `isNonNull<string>` in `isNonNull<string>(x)`.
/// Collection records such a callee as an instance of the callee's type with
/// the type arguments.
fn has_type_arguments(js_info: &JsModuleInfo, callee: &TypeReference) -> bool {
    matches!(
        callee,
        TypeReference::Resolved(RawTypeId::Local(id))
            if matches!(
                js_info.raw_types.get(id.index()),
                Some(RawTypeData::InstanceOf(instance)) if !instance.type_parameters.is_empty()
            )
    )
}

/// Returns whether the binding at `binding` in `js_info` is a namespace
/// import, as `g` in `import * as g from "./g"`.
fn is_namespace_import(js_info: &JsModuleInfo, binding: TextRange) -> bool {
    matches!(
        js_info.raw_binding_types.get(&binding),
        Some(TypeReference::Import(import)) if import.symbol == ImportSymbol::All
    )
}

/// Returns whether every type that `ty` can be declares its member `name`
/// with a return type annotation (see [`has_annotated_member`]), or `ty` is a
/// type literal whose members named `name` are all functions written in
/// place. A method signature and a property annotated with a function type
/// are such functions, and their syntax states their return types.
fn has_annotated_return_member(db: &dyn ModuleDb, ty: TypeData, name: &str) -> bool {
    if has_annotated_member(db, ty, name) {
        return true;
    }
    let mut ty = resolve_local_type_on_demand(db, ty);
    // A variable annotated with a type literal holds an instance of it.
    if let TypeData::InstanceOf(instance) = ty {
        ty = resolve_local_type_on_demand(db, instance.ty(db));
    }
    let TypeData::Object(object) = ty else {
        return false;
    };
    if object.kind(db) != ObjectKind::Shape {
        return false;
    }
    let mut members = object
        .members(db)
        .iter()
        .filter(|member| !member.kind.is_static() && member.kind.has_name(name))
        .peekable();
    members.peek().is_some() && members.all(|member| matches!(member.ty, TypeData::Function(_)))
}

/// Returns the object and the name of the static member access at `range`
/// in `js_info`, as local inference collected them.
fn raw_member(js_info: &JsModuleInfo, range: TextRange) -> Option<(&TypeReference, &Text)> {
    let TypeReference::Resolved(RawTypeId::Local(id)) = js_info.raw_expressions.get(&range)? else {
        return None;
    };
    let RawTypeData::TypeofExpression(raw) = js_info.raw_types.get(id.index())? else {
        return None;
    };
    if let TypeofExpression::StaticMember(expression)
    | TypeofExpression::OptionalChainStaticMember(expression) = raw.as_ref()
    {
        Some((&expression.object, &expression.member))
    } else {
        None
    }
}

/// Returns whether the declaration that an imported binding names has an
/// explicit type, which TypeScript requires of every name along the callee
/// of a statement call. `binding` is the range of the import in `js_info`,
/// the module `module`. For a namespace import, the declaration is the
/// export that `member` names.
///
/// The import is followed through re-exports to the module that declares the
/// export, and a binding that an import declares there is followed in turn,
/// at most [`MAX_IMPORT_STEPS`] of them. A declaration file declares only
/// names with explicit types. Elsewhere, a function, a class, a variable
/// with a type annotation, and a namespace that `declare` declares have one.
fn imported_declaration_is_explicit(
    db: &dyn ModuleDb,
    mut module: ModuleInfo,
    js_info: &JsModuleInfo,
    mut binding: TextRange,
    mut member: Option<&Text>,
) -> bool {
    let mut info = js_info;
    for _ in 0..MAX_IMPORT_STEPS {
        let Some(TypeReference::Import(import)) = info.raw_binding_types.get(&binding) else {
            return false;
        };
        let name = match &import.symbol {
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::Default => Text::new_static("default"),
            ImportSymbol::All => match member.take() {
                Some(member) => member.clone(),
                None => return false,
            },
        };
        let resolved =
            resolve_module_import(db, module, &import.specifier, ResolutionMode::JavaScript);
        let Some(target) = resolved
            .path()
            .as_path()
            .and_then(|path| db.module_for_path(path))
        else {
            return false;
        };
        let ExportOriginResult::Found {
            module: origin,
            name,
        } = find_export_origin(db, target, name)
        else {
            return false;
        };
        let ModuleInfoKind::Js(origin_info) = origin.kind(db) else {
            return false;
        };
        let Some(JsExport::Own(JsOwnExport::Binding(declaration))) =
            origin_info.exports.get(name.text())
        else {
            return false;
        };
        if is_declaration_file(db, origin) {
            return true;
        }
        let Some(declared) = origin_info.semantic_model.as_binding_by_range(*declaration) else {
            return false;
        };
        match declared.tree().declaration() {
            Some(
                AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
                | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
                | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_),
            ) => {
                module = origin;
                info = origin_info;
                binding = declared.range();
            }
            Some(
                AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
                | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
                | AnyJsBindingDeclaration::JsClassDeclaration(_)
                | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_),
            ) => return true,
            Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
                return declarator.variable_annotation().is_some();
            }
            Some(AnyJsBindingDeclaration::TsModuleDeclaration(namespace)) => {
                return namespace.parent::<TsDeclareStatement>().is_some();
            }
            _ => return false,
        }
    }
    false
}

/// Returns whether every type that `ty` can be declares its member `name`
/// with an explicit type, which TypeScript requires of every member along
/// the callee of a statement call.
///
/// These types qualify:
///
/// - a lib global or an instance of one, unless a module other than a
///   declaration file merges an interface into it;
/// - a primitive, a literal that isn't an object, and a tuple, because their
///   members are those of their lib types;
/// - a type literal, because all its members have explicit types;
/// - a type that a declaration file declares, when the classes and
///   interfaces that it extends qualify too;
/// - a class or an interface that a module other than a declaration file
///   declares, when its own members named `name` are methods, or properties
///   with type annotations, or when it inherits the member from a type that
///   qualifies. For the class itself, its `static` members count, as
///   [`own_member`] checks.
///
/// An object literal, a union, `null`, and `undefined` don't qualify, and
/// neither do namespaces and modules, whose members collection checks
/// itself.
fn has_explicit_member(db: &dyn ModuleDb, ty: TypeData, name: &str) -> bool {
    // Each type comes with whether a value of it is an instance, as the
    // object of a call of a class's method is.
    let mut pending = vec![(ty, false)];
    let mut remaining = MAX_FLOW_TYPE_STEPS;
    while let Some((ty, instance)) = pending.pop() {
        let Some(left) = remaining.checked_sub(1) else {
            return false;
        };
        remaining = left;
        match ty {
            TypeData::GlobalType(id) => {
                if !global_contributions(db, id).iter().all(|(module, _)| {
                    module_for_key(db, *module)
                        .is_some_and(|module| is_declaration_file(db, module))
                }) {
                    return false;
                }
            }
            TypeData::Global
            | TypeData::GlobalLocal(_)
            | TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Tuple(_) => {}
            TypeData::Literal(literal) => {
                if matches!(literal.literal(db), Literal::Object(_)) {
                    return false;
                }
            }
            TypeData::Object(object) => {
                if object.kind(db) != ObjectKind::Shape {
                    return false;
                }
            }
            TypeData::InstanceOf(instance_of) => pending.push((instance_of.ty(db), true)),
            TypeData::Local(handle) => {
                let Some(module) = module_for_key(db, handle.module(db)) else {
                    return false;
                };
                let input = LocalTypeInput::new(db, module, handle.type_id(db));
                let Some(resolved) = infer_local_type(db, input) else {
                    return false;
                };
                let in_declaration_file = is_declaration_file(db, module);
                let (parents, own) = if let TypeData::Class(class) = resolved {
                    let ClassParents {
                        extends,
                        interface_bases,
                    } = class.parents(db);
                    let own = if in_declaration_file {
                        OwnMember::Inherited
                    } else {
                        own_member(
                            db,
                            module,
                            class.scope_id(db),
                            class.members(db),
                            class.contributions(db),
                            name,
                            MemberRule::Explicit {
                                is_static: !instance,
                            },
                        )
                    };
                    // The class itself inherits only the static members of
                    // the class it extends, which Biome doesn't follow.
                    let parents: Vec<_> = if instance {
                        extends.iter().chain(interface_bases).copied().collect()
                    } else {
                        Vec::new()
                    };
                    (parents, own)
                } else if let TypeData::Interface(interface) = resolved {
                    let own = if in_declaration_file {
                        OwnMember::Inherited
                    } else {
                        own_member(
                            db,
                            module,
                            interface.scope_id(db),
                            interface.members(db),
                            interface.contributions(db),
                            name,
                            MemberRule::Explicit { is_static: false },
                        )
                    };
                    (interface.extends(db).to_vec(), own)
                } else {
                    // A type literal, a handle in another module, or a type
                    // that an alias names is checked as the type that it
                    // resolves to in its own module.
                    pending.push((resolved, instance));
                    continue;
                };
                match own {
                    OwnMember::Annotated => continue,
                    OwnMember::Inherited if in_declaration_file || !parents.is_empty() => {}
                    OwnMember::Inherited | OwnMember::Other => return false,
                }
                pending.extend(parents.into_iter().map(|parent| (parent, instance)));
            }
            TypeData::Unknown
            | TypeData::Null
            | TypeData::Undefined
            | TypeData::Union(_)
            | TypeData::Conditional
            | TypeData::Class(_)
            | TypeData::Constructor(_)
            | TypeData::Function(_)
            | TypeData::Interface(_)
            | TypeData::Module(_)
            | TypeData::Namespace(_)
            | TypeData::Generic(_)
            | TypeData::Intersection(_)
            | TypeData::TypeOperator(_)
            | TypeData::IndexedAccess(_)
            | TypeData::MappedType(_)
            | TypeData::Extends(_)
            | TypeData::MergedReference(_)
            | TypeData::TypeofExpression(_)
            | TypeData::TypeofType(_)
            | TypeData::TypeofValue(_)
            | TypeData::AnyKeyword
            | TypeData::NeverKeyword
            | TypeData::ObjectKeyword
            | TypeData::ThisKeyword
            | TypeData::UnknownKeyword
            | TypeData::VoidKeyword => return false,
        }
    }
    true
}

/// Returns whether every type that `ty` can be gets its member `name` from
/// the lib, from a declaration file, or from methods with a return type
/// annotation, so that a call of the member has a return type that comes
/// from an annotation.
///
/// That holds for:
///
/// - a lib global or an instance of one, unless a module other than a
///   declaration file merges an interface into it;
/// - a primitive, a literal that isn't an object, and a tuple, whose members
///   are those of their lib types;
/// - a type that a declaration file declares, when the classes and
///   interfaces that it extends are such types too;
/// - an instance of a class or an interface that a module other than a
///   declaration file declares, when it declares the member only as methods
///   with a return type annotation, or inherits it from such types; see
///   [`own_member`].
///
/// A class or an interface of a declaration file, including one that a
/// declaration file merges into a lib global, can declare the member as a
/// property typed `typeof` of a variable. A call of it returns what a call
/// of that variable returns, so the variable must have a return type from an
/// annotation too (see [`declared_members_are_annotated`]). `null` and
/// `undefined` have no members to call.
fn has_annotated_member(db: &dyn ModuleDb, ty: TypeData, name: &str) -> bool {
    // Each type comes with whether a value of it is an instance, as the
    // object of a call of a class's method is.
    let mut pending = vec![(ty, false)];
    let mut remaining = MAX_FLOW_TYPE_STEPS;
    while let Some((ty, instance)) = pending.pop() {
        let Some(left) = remaining.checked_sub(1) else {
            return false;
        };
        remaining = left;
        match ty {
            TypeData::GlobalType(id) => {
                if !global_contributions(db, id)
                    .iter()
                    .all(|(module, type_id)| {
                        module_for_key(db, *module).is_some_and(|module| {
                            is_declaration_file(db, module)
                                && infer_local_type(db, LocalTypeInput::new(db, module, *type_id))
                                    .is_some_and(|contribution| {
                                        declared_members_are_annotated(
                                            db,
                                            module,
                                            member_scope(db, contribution),
                                            name,
                                        )
                                    })
                        })
                    })
                {
                    return false;
                }
            }
            TypeData::Global
            | TypeData::GlobalLocal(_)
            | TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Undefined
            | TypeData::Tuple(_) => {}
            TypeData::Literal(literal) => {
                if matches!(literal.literal(db), Literal::Object(_)) {
                    return false;
                }
            }
            TypeData::InstanceOf(instance_of) => pending.push((instance_of.ty(db), true)),
            TypeData::Local(handle) => {
                let Some(module) = module_for_key(db, handle.module(db)) else {
                    return false;
                };
                let input = LocalTypeInput::new(db, module, handle.type_id(db));
                let Some(resolved) = infer_local_type(db, input) else {
                    return false;
                };
                let in_declaration_file = is_declaration_file(db, module);
                // A class or an interface that doesn't declare the member
                // inherits it from its parents, which other modules can
                // declare.
                let (parents, own) = if let TypeData::Class(class) = resolved {
                    let ClassParents {
                        extends,
                        interface_bases,
                    } = class.parents(db);
                    let parents: Vec<_> = extends.iter().chain(interface_bases).copied().collect();
                    // A call of a class itself reads its static members.
                    let own = if in_declaration_file {
                        if !declared_members_are_annotated(db, module, class.scope_id(db), name) {
                            return false;
                        }
                        OwnMember::Inherited
                    } else if instance {
                        own_member(
                            db,
                            module,
                            class.scope_id(db),
                            class.members(db),
                            class.contributions(db),
                            name,
                            MemberRule::AnnotatedReturn,
                        )
                    } else {
                        OwnMember::Other
                    };
                    (parents, own)
                } else if let TypeData::Interface(interface) = resolved {
                    let own = if in_declaration_file {
                        if !declared_members_are_annotated(db, module, interface.scope_id(db), name)
                        {
                            return false;
                        }
                        OwnMember::Inherited
                    } else {
                        own_member(
                            db,
                            module,
                            interface.scope_id(db),
                            interface.members(db),
                            interface.contributions(db),
                            name,
                            MemberRule::AnnotatedReturn,
                        )
                    };
                    (interface.extends(db).to_vec(), own)
                } else if matches!(resolved, TypeData::Object(_)) {
                    if in_declaration_file {
                        continue;
                    }
                    return false;
                } else {
                    // A handle to a type of another module, or a type that
                    // an alias names, is checked in turn as resolved.
                    pending.push((resolved, instance));
                    continue;
                };
                match own {
                    OwnMember::Annotated => continue,
                    // A declaration file declares only annotated members, so
                    // its types need only parents that do too.
                    OwnMember::Inherited if in_declaration_file || !parents.is_empty() => {}
                    OwnMember::Inherited | OwnMember::Other => return false,
                }
                pending.extend(parents.into_iter().map(|parent| (parent, instance)));
            }
            TypeData::Union(union) => {
                pending.extend(union.types(db).iter().map(|member| (*member, instance)))
            }
            TypeData::Unknown
            | TypeData::Conditional
            | TypeData::Class(_)
            | TypeData::Constructor(_)
            | TypeData::Function(_)
            | TypeData::Interface(_)
            | TypeData::Module(_)
            | TypeData::Namespace(_)
            | TypeData::Object(_)
            | TypeData::Generic(_)
            | TypeData::Intersection(_)
            | TypeData::TypeOperator(_)
            | TypeData::IndexedAccess(_)
            | TypeData::MappedType(_)
            | TypeData::Extends(_)
            | TypeData::MergedReference(_)
            | TypeData::TypeofExpression(_)
            | TypeData::TypeofType(_)
            | TypeData::TypeofValue(_)
            | TypeData::AnyKeyword
            | TypeData::NeverKeyword
            | TypeData::ObjectKeyword
            | TypeData::ThisKeyword
            | TypeData::UnknownKeyword
            | TypeData::VoidKeyword => return false,
        }
    }
    true
}

/// Where a class or an interface gets a member that a call reads.
enum OwnMember {
    /// Its own members, each of them as the [`MemberRule`] requires.
    Annotated,
    /// The classes and interfaces that it extends, because it declares no
    /// member with the name, and no computed key or index signature that
    /// could stand for it.
    Inherited,
    /// Anything else, including a static member, a member without an
    /// annotation, and a member of a merged declaration.
    Other,
}

/// What [`own_member`] requires of each own member that it finds.
#[derive(Clone, Copy)]
enum MemberRule {
    /// A method of an instance with a return type annotation, as
    /// [`member_return`] finds it.
    AnnotatedReturn,
    /// A method, or a property with a type annotation, among the `static`
    /// members when `is_static` is true and among the others otherwise, as
    /// [`member_explicitness`] finds it.
    Explicit { is_static: bool },
}

/// Returns where a class or an interface gets its member `name`, among the
/// members of an instance unless `rule` asks for `static` ones.
///
/// `module` declares the class or interface and is no declaration file.
/// `scope_id` is the scope that the class or interface records, `members`
/// are the members that local inference collected for it, `contributions`
/// counts the declarations that other modules merge into it, and `rule` is
/// what its own members must satisfy.
///
/// Its own members are those that the rule finds in the declaration that
/// opens the scope. Declarations merged into it can add members with the
/// name. Member lookup finds a class's own members before them, but can find
/// those of a later interface declaration first. So the member is its own
/// only when local inference collected no more members with the name than
/// the declaration declares, and inherited only when it collected none.
fn own_member(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    scope_id: Option<ScopeId>,
    members: &[TypeMember],
    contributions: usize,
    name: &str,
    rule: MemberRule,
) -> OwnMember {
    // A module augmentation can declare the member as a property whose type
    // comes from an inferred return type.
    if contributions != 0 {
        return OwnMember::Other;
    }
    let Some(scope_id) = scope_id else {
        return OwnMember::Other;
    };
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return OwnMember::Other;
    };
    let Some(declaration) =
        AnyMemberDeclaration::cast(info.semantic_model.scope_from_id(scope_id).syntax())
    else {
        return OwnMember::Other;
    };
    let (is_static, (declared, own)) = match rule {
        MemberRule::AnnotatedReturn => {
            let (declared, callee_return) = member_return(&declaration, name);
            (false, (declared, callee_return == CalleeReturn::Annotated))
        }
        MemberRule::Explicit { is_static } => (
            is_static,
            member_explicitness(&declaration, name, is_static),
        ),
    };
    let collected = members
        .iter()
        .filter(|member| member.kind.is_static() == is_static && member.kind.has_name(name))
        .count();
    if declared == 0
        && collected == 0
        && members.iter().all(|member| {
            member.kind.computed_value_type().is_none()
                && member.kind.index_signature_type().is_none()
        })
    {
        OwnMember::Inherited
    } else if own && declared != 0 && collected <= declared {
        OwnMember::Annotated
    } else {
        OwnMember::Other
    }
}

/// Returns whether `module` is a declaration file: its name ends with
/// `.d.ts`, `.d.mts`, or `.d.cts`, in any letter case, as `index.d.ts`.
fn is_declaration_file(db: &dyn ModuleDb, module: ModuleInfo) -> bool {
    let Some(name) = module.path(db).file_name() else {
        return false;
    };
    let name = name.as_bytes();
    [".d.ts", ".d.mts", ".d.cts"].iter().any(|suffix| {
        name.len()
            .checked_sub(suffix.len())
            .is_some_and(|start| name[start..].eq_ignore_ascii_case(suffix.as_bytes()))
    })
}

/// Returns the type that `reference`, a raw type reference of `js_info`,
/// refers to when it is a string, number, bigint, or boolean literal,
/// converted without a resolution context, because a literal refers to no
/// other type.
fn raw_literal_type<'db>(
    db: &'db dyn ModuleDb,
    js_info: &JsModuleInfo,
    reference: &TypeReference,
) -> Option<TypeData<'db>> {
    let TypeReference::Resolved(RawTypeId::Local(id)) = reference else {
        return None;
    };
    let raw = js_info.raw_types.get(id.index())?;
    let RawTypeData::Literal(literal) = raw else {
        return None;
    };
    match literal.as_ref() {
        RawLiteral::String(_)
        | RawLiteral::Number(_)
        | RawLiteral::BigInt(_)
        | RawLiteral::Boolean(_) => Some(TypeData::from_raw_with_resolver(
            db,
            raw,
            false,
            &mut |_| TypeData::Unknown,
        )),
        RawLiteral::Object(_) | RawLiteral::RegExp(_) | RawLiteral::Template(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reaching_flow_numbers_only_the_nodes_before_its_points() {
        // 0: start, 1: loop header, 2: loop body, 3: back edge, 4: loop exit.
        let root = RootFlow {
            nodes: Box::from([
                FlowNode::Start,
                FlowNode::Join(vec![0, 3]),
                FlowNode::Condition {
                    antecedent: 1,
                    guard: 0,
                },
                FlowNode::Condition {
                    antecedent: 2,
                    guard: 0,
                },
                FlowNode::Condition {
                    antecedent: 1,
                    guard: 0,
                },
            ]),
            guards: Box::from([FlowGuard::Keep]),
            subjects: Box::default(),
            read_points: Default::default(),
            property_only: Default::default(),
            instance_tested: Default::default(),
            any_tested: Default::default(),
            pattern_elements: Default::default(),
            readonly_tested: Default::default(),
            only_readonly_tested: Default::default(),
            switches: Box::default(),
            bypassed: Box::default(),
            writes: Box::default(),
            assigned: Box::default(),
            subject_writes: Default::default(),
            assignment_only: Default::default(),
            uninitialized: Default::default(),
            evolving: Default::default(),
            hoisted: Default::default(),
            captures: Default::default(),
            calls: Box::default(),
            subject_calls: Default::default(),
            subject_compared: Default::default(),
            ending_calls: Box::default(),
            called: Box::default(),
        };
        let reaching = ReachingFlow::new(&root, &[2]).unwrap();
        assert_eq!(reaching.nodes, [2, 1, 3, 0]);
        assert_eq!(reaching.points, [0]);
        let predecessors: Vec<_> = (0..4)
            .map(|number| reaching.predecessors.get(number))
            .collect();
        assert_eq!(predecessors, [&[1][..], &[3, 2], &[0], &[]]);
        let successors: Vec<_> = (0..4)
            .map(|number| reaching.successors.get(number))
            .collect();
        assert_eq!(successors, [&[2][..], &[0], &[1], &[1]]);
        assert!(ReachingFlow::new(&root, &[5]).is_none());

        // The loop exit and the loop body share the header and the back edge.
        let both = ReachingFlow::new(&root, &[2, 4]).unwrap();
        assert_eq!(both.nodes, [4, 1, 3, 2, 0]);
        assert_eq!(both.points, [3, 0]);
    }
}

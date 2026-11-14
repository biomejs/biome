//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Collection records the incoming flow of each refinable read and the guards
//! that the conditions of its execution root apply (see
//! [`crate::js_module_info::flow`]). This module resolves the type that the
//! read's subject starts from and applies those guards along every path that
//! reaches the read. A variable starts from its declared type. The declared
//! type of a property path at a read is the type of the member access there,
//! given the narrowed type of its object, as TypeScript types property reads.

use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, FlowSubjectRootInput, FlowSubjectTypeInput, infer_flow_binding_baseline,
    infer_flow_subject_states, infer_flow_subject_type,
};
use crate::js_module_info::flow::{
    FlowGuard, FlowNode, FlowNodeId, FlowSubject, FlowSubjectId, FlowTest, PropertyKey, RootFlow,
    guard_signature,
};
use crate::{JsModuleInfo, ModuleDb, ModuleInfo};
use biome_js_syntax::JsIdentifierExpression;
use biome_js_type_info::interned_types::{Literal, TypeData};
use biome_js_type_info::{
    NarrowingPredicate, PropertyTest, TypeId, TypeofKind, is_narrowing_invariant,
    may_narrow_by_property, narrow_by_property, narrow_type,
};
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

const MAX_FLOW_STEPS: usize = 16_384;
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
    // As in `local_flow_expression_type`, a candidate made only by reads of a
    // binding that no test of a property can narrow keeps its type.
    if let Some(binding) = info.flow.property_only_candidate(range)
        && infer_flow_binding_baseline(db, BindingTypeInput::new(db, module, binding)).is_none()
    {
        return None;
    }
    let tree = info.semantic_model.root();
    let expression = info.flow.candidate(range)?.try_to_node(tree.syntax())?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.resolve_flow_expression(&expression)
}

pub(in crate::db) fn flow_binding_baseline<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    range: TextRange,
) -> Option<TypeData<'db>> {
    // Declaration queries can erase a cyclic object's shape. The raw baseline
    // still lets flow detect an impossible branch and return an unknown override.
    let reference = info.raw_binding_types.get(&range)?;
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
    let states = ctx.solve_subject(root, points, subject, baseline)?;
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
            match ctx.solve_subject(root, &[point], states.subject(db), baseline) {
                Some(states) => states[0],
                None => return Some(TypeData::Unknown),
            }
        }
    };
    if narrowed == TypeData::NeverKeyword {
        return Some(TypeData::Unknown);
    }
    (narrowed != baseline).then_some(narrowed)
}

/// Why [`ResolutionCtx::flatten_baseline`] gives no type.
enum FlatteningError {
    /// Flattening exceeded its work limit.
    Exceeded,
    /// Handles to local types of the module form a cycle.
    Cycle,
}

/// The state of one solve that its guards share: the subject it narrows, the
/// subject's declared type at the root's start, and the results of the
/// property guards applied so far, keyed by guard index and incoming type.
struct GuardSolve<'db> {
    subject: FlowSubjectId,
    declared: TypeData<'db>,
    properties: FxHashMap<(usize, TypeData<'db>), TypeData<'db>>,
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
        FlowNode::Condition { antecedent, .. } => std::slice::from_ref(antecedent),
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
        // test can narrow the binding. Its baseline tells that without the
        // syntax node.
        if let Some(binding) = info.flow.property_only_candidate(range)
            && self.local_binding_baseline(binding).is_none()
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

    /// Returns the narrowed type of the variable read at `identifier`.
    ///
    /// Returns `None` when collection kept no flow for the read, when no
    /// runtime test can change the variable's type, or when narrowing leaves
    /// it unchanged.
    pub(super) fn narrow_reference(
        &mut self,
        identifier: &JsIdentifierExpression,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        let read = info.flow.read(identifier.range())?;
        let &FlowSubject::Binding(binding) = info.flow.root(read.root)?.subject(read.subject)?
        else {
            return None;
        };
        let baseline = if self.evaluates_flow_locally() {
            self.local_binding_baseline(binding)
        } else {
            infer_flow_binding_baseline(
                self.db,
                BindingTypeInput::new(self.db, self.module, binding),
            )
        };
        let baseline = baseline?;
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
                ),
                read.point,
            ),
        )
    }

    /// Returns the type that narrowing of the binding declared at `binding`
    /// starts from, as [`crate::db::queries::infer_flow_binding_baseline`]
    /// returns it, but resolved in this context.
    fn local_binding_baseline(&mut self, binding: TextRange) -> Option<TypeData<'db>> {
        if let Some(baseline) = self.local_flow.as_ref()?.baselines.get(&binding) {
            return *baseline;
        }
        // As in `flow_binding_baseline`, the raw binding type gives the
        // declared type.
        let baseline = self
            .js_info
            .raw_binding_types
            .get(&binding)
            .cloned()
            .and_then(|reference| {
                let declared = self.resolve(&reference);
                self.binding_baseline(binding, declared)
            });
        self.local_flow
            .as_mut()?
            .baselines
            .insert(binding, baseline);
        baseline
    }

    /// Returns the narrowed type of the property path read by the member
    /// expression at `member`, given its `declared` type there: the type of
    /// the member access with its object's narrowed type.
    ///
    /// Returns `None` when collection kept no flow for the read, when no
    /// runtime test can change `declared`, or when narrowing leaves it
    /// unchanged. Reads of the same path with equal declared types share one
    /// solve.
    pub(super) fn narrow_path(
        &mut self,
        member: TextRange,
        declared: TypeData<'db>,
    ) -> Option<TypeData<'db>> {
        let read = self.js_info.flow.read(member)?;
        let root = self.js_info.flow.root(read.root)?;
        let baseline = if root.is_property_only(read.subject) {
            self.property_baseline(declared)
        } else {
            self.narrowing_baseline(declared)
        }?;
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
                ),
                read.point,
            ),
        )
    }

    /// Returns the type that narrowing of a subject whose declared type is
    /// `declared` starts from: `declared` with its unions, aliases, and
    /// `typeof` types flattened.
    ///
    /// Returns `None` when no runtime test can change the type, because it
    /// contains `any` or an undetermined member, and `Unknown` when flattening
    /// exceeds its work limit.
    fn narrowing_baseline(&mut self, declared: TypeData<'db>) -> Option<TypeData<'db>> {
        match self.flow_baseline(declared) {
            Some(baseline) if is_narrowing_invariant(baseline) => None,
            Some(baseline) => Some(baseline),
            None => Some(TypeData::Unknown),
        }
    }

    /// Returns the type that narrowing of the binding declared at `binding`
    /// starts from, given its `declared` type: [`Self::property_baseline`]
    /// when conditions narrow the binding only through tests of its
    /// properties, and [`Self::narrowing_baseline`] otherwise.
    fn binding_baseline(
        &mut self,
        binding: TextRange,
        declared: TypeData<'db>,
    ) -> Option<TypeData<'db>> {
        if self.js_info.flow.is_property_only_binding(binding) {
            self.property_baseline(declared)
        } else {
            self.narrowing_baseline(declared)
        }
    }

    /// Returns the type that narrowing of a subject in
    /// [`RootFlow::property_only`] starts from, given its `declared` type, or
    /// `None` when no test of a property can narrow it.
    ///
    /// Such a test narrows only a union with two object types, as
    /// [`may_narrow_by_property`] describes, so this is
    /// [`Self::narrowing_baseline`] when the flattened type is one.
    ///
    /// Unlike [`Self::narrowing_baseline`], flattening resolves the local
    /// types of this context's module through this context, as
    /// [`Self::resolve_module_handle`] describes. Complete module inference
    /// has resolved all of them by the time it evaluates flow, so finding out
    /// that most such subjects can't narrow costs only cache reads there,
    /// where a lookup would resolve each type again in a fresh context.
    fn property_baseline(&mut self, declared: TypeData<'db>) -> Option<TypeData<'db>> {
        match self.flatten_baseline(declared, true) {
            Ok(baseline) => (!is_narrowing_invariant(baseline)
                && may_narrow_by_property(self.db, baseline))
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

    /// Returns the type of `subject` at each of `points`, given its `baseline`
    /// type at the root's start, or `None` when the walk or the solve exceeds
    /// the flow work limit.
    fn solve_subject(
        &mut self,
        root: &RootFlow,
        points: &[FlowNodeId],
        subject: FlowSubjectId,
        baseline: TypeData<'db>,
    ) -> Option<Vec<TypeData<'db>>> {
        let reaching = ReachingFlow::new(root, points)?;
        let states = self.solve_flow(root, &reaching, subject, baseline)?;
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
    fn flow_baseline(&mut self, ty: TypeData<'db>) -> Option<TypeData<'db>> {
        self.flatten_baseline(ty, false).ok()
    }

    /// Flattens `ty` as [`Self::flow_baseline`] does. When `in_context` is
    /// true, handles to local types of this context's module are resolved
    /// with [`Self::resolve_module_handle`] instead of lookups, and a cycle
    /// of such handles stops the flattening.
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

    /// Returns the type of `subject` at every node of `reaching`, given its
    /// `baseline` type at the root's start, or `None` when the solve exceeds
    /// the flow work limit.
    fn solve_flow(
        &mut self,
        root: &RootFlow,
        reaching: &ReachingFlow,
        subject: FlowSubjectId,
        baseline: TypeData<'db>,
    ) -> Option<Vec<TypeData<'db>>> {
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
        let mut guards = GuardSolve {
            subject,
            declared: baseline,
            properties: FxHashMap::default(),
        };
        while let Some(number) = queue.pop_front() {
            let index = number as usize;
            queued[index] = false;
            remaining = remaining.checked_sub(1)?;
            let predecessors = reaching.predecessors.get(number);
            let ty = match &root.nodes[reaching.nodes[index] as usize] {
                FlowNode::Start => baseline,
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
                if *tested != subject {
                    return Some(ty);
                }
                match self.test_predicate(*test) {
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
            } => {
                if *object != subject {
                    return Some(ty);
                }
                // Loops revisit conditions, and each application looks up the
                // property in every member of the union.
                if let Some(narrowed) = solve.properties.get(&(index, ty)) {
                    return Some(*narrowed);
                }
                let narrowed = match self.test_predicate(*test) {
                    Some(predicate) => {
                        let test = PropertyTest {
                            name: name.text(),
                            predicate,
                            positive: *positive,
                            optional_chain: *optional_chain,
                        };
                        narrow_by_property(
                            self.db,
                            ty,
                            solve.declared,
                            test,
                            self.narrowing_resolver(),
                        )
                    }
                    None => ty,
                };
                solve.properties.insert((index, ty), narrowed);
                narrowed
            }
            // Collection emits property guards only for named properties.
            FlowGuard::Property { .. } => ty,
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

    fn test_predicate(&mut self, test: FlowTest) -> Option<NarrowingPredicate<'db>> {
        Some(match test {
            FlowTest::Truthy => NarrowingPredicate::Truthy,
            FlowTest::Nullish => NarrowingPredicate::Nullish,
            FlowTest::Typeof(kind) => NarrowingPredicate::Typeof(kind),
            FlowTest::Null => NarrowingPredicate::Literal(TypeData::Null),
            FlowTest::Literal(range) => {
                let reference = self.js_info.raw_expressions.get(&range)?.clone();
                NarrowingPredicate::Literal(self.resolve(&reference))
            }
            FlowTest::Undefined { strict: true } => {
                NarrowingPredicate::Literal(TypeData::Undefined)
            }
            FlowTest::Undefined { strict: false } => NarrowingPredicate::Nullish,
            FlowTest::Predicate(function) => return self.guard_predicate(function),
        })
    }

    /// Returns the test that the guard declared at `function` applies to its
    /// tested argument, whether a call reports it or asserts it.
    ///
    /// Returns `None` when the guard's target type is neither a primitive type
    /// nor a primitive literal.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Only primitive types and primitive literals map to a runtime test."
    )]
    fn guard_predicate(&mut self, function: TextRange) -> Option<NarrowingPredicate<'db>> {
        let info = self.js_info;
        let (_, contract) = guard_signature(&info.raw_types, &info.raw_binding_types, function)?;
        let target = self.resolve(contract.target?);
        // An alias resolves to an instance of its target. The baseline flattens
        // that instance, as it does for a variable declared with the alias.
        let target = self.flow_baseline(target)?;
        Some(match target {
            TypeData::BigInt => NarrowingPredicate::Typeof(TypeofKind::BigInt),
            TypeData::Boolean => NarrowingPredicate::Typeof(TypeofKind::Boolean),
            TypeData::Number => NarrowingPredicate::Typeof(TypeofKind::Number),
            TypeData::String => NarrowingPredicate::Typeof(TypeofKind::String),
            TypeData::Symbol => NarrowingPredicate::Typeof(TypeofKind::Symbol),
            TypeData::Null | TypeData::Undefined => NarrowingPredicate::Literal(target),
            TypeData::Literal(literal)
                if matches!(
                    literal.literal(self.db),
                    Literal::BigInt(_)
                        | Literal::Boolean(_)
                        | Literal::Number(_)
                        | Literal::String(_)
                ) =>
            {
                NarrowingPredicate::Literal(target)
            }
            _ => return None,
        })
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

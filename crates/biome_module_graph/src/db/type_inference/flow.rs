//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Collection records the incoming flow of each refinable read, the guards
//! that the conditions of its execution root apply, and its writes (see
//! [`crate::js_module_info::flow`]). This module resolves the type that the
//! read's subject starts from and the types that its writes assign, and
//! applies the guards and the writes along every path that reaches the read.
//! A variable starts from its declared type. The declared
//! type of a property path at a read is the type of the member access there,
//! given the narrowed type of its object, as TypeScript types property reads.
//! Likewise, the declared type of an element of a destructuring pattern at a
//! read is its field on the pattern narrowed there.

use super::flow_expressions::collected_member_name;
use super::imports::{ExportOriginResult, find_export_origin};
use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, FlowAssignedTypes, FlowSubjectRootInput, FlowSubjectTypeInput,
    FlowSubjectWritesInput, FlowSwitchInput, LocalTypeInput, global_contributions,
    infer_flow_assigned_types, infer_flow_binding_baseline, infer_flow_subject_states,
    infer_flow_subject_type, infer_flow_switch_exhaustive, infer_local_type,
};
use crate::js_module_info::flow::{
    AnyMemberDeclaration, CalleeReturn, FlowAssigned, FlowAssignedValue, FlowCallee, FlowGuard,
    FlowNode, FlowNodeId, FlowRead, FlowSubject, FlowSubjectId, FlowTest, FlowWriteEffect,
    PropertyKey, RootFlow, UnitValue, binding_unit_value, declaration_return, guard_signature,
    member_return,
};
use crate::module_graph::ModuleInfoKind;
use crate::{
    JsExport, JsModuleInfo, JsOwnExport, ModuleDb, ModuleInfo, ResolutionMode, module_for_key,
    resolve_module_import,
};
use biome_js_semantic::ScopeId;
use biome_js_syntax::{AnyJsExpression, JsIdentifierExpression};
use biome_js_type_info::interned_types::{ClassParents, Literal, TypeData, TypeMember};
use biome_js_type_info::{
    ImportSymbol, Literal as RawLiteral, NarrowingPredicate, PropertyTest, RawTypeData, RawTypeId,
    TypeId, TypeReference, TypeofExpression, base_literal_type, has_readonly_property,
    is_narrowing_invariant, may_narrow_by_instanceof, may_narrow_by_property, narrow_by_assignment,
    narrow_by_in_operator, narrow_by_instanceof, narrow_by_property, narrow_by_type_predicate,
    narrow_type,
};
use biome_rowan::{AstNode, Text, TextRange, TextSize};
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
    // A candidate made only by reads of a binding that no test of a property
    // can narrow keeps its type, as in `local_flow_expression_type`.
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

/// Returns whether `ty` is `undefined` or a union with an `undefined` member.
fn contains_undefined(db: &dyn ModuleDb, ty: TypeData) -> bool {
    if let TypeData::Union(union) = ty {
        union.types(db).contains(&TypeData::Undefined)
    } else {
        ty == TypeData::Undefined
    }
}

/// Returns whether a write can change a subject whose declared type is
/// `baseline`: an assignment narrows only a union, and a compound assignment
/// changes only a literal type.
fn may_narrow_by_assignment(baseline: TypeData) -> bool {
    matches!(
        baseline,
        TypeData::Union(_) | TypeData::Literal(_) | TypeData::Unknown
    )
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
/// each, the types that the subject's writes assign, the results of the
/// property guards, `in` tests, `instanceof` tests, and calls to type-guard
/// functions applied so far, keyed by guard index and incoming type, the
/// values of the imported variables that [`FlowTest::Imported`] compares
/// with, found so far, keyed by declaration, and the type that each write
/// of the subject that assigns a value gives it.
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
    assigned: Option<FlowAssignedTypes<'db>>,
    properties: FxHashMap<(usize, TypeData<'db>), TypeData<'db>>,
    /// The unit values of the imported variables that a [`FlowTest::Imported`]
    /// compares with, found so far, keyed by declaration. `None` records a
    /// variable that holds no unit value.
    imported: FxHashMap<TextRange, Option<TypeData<'db>>>,
    writes: FxHashMap<u32, TypeData<'db>>,
}

impl<'db> GuardSolve<'db> {
    fn new(
        subject: FlowSubjectId,
        declared: TypeData<'db>,
        readonly_path: bool,
        exhaustive: u64,
        assigned: Option<FlowAssignedTypes<'db>>,
    ) -> Self {
        Self {
            subject,
            declared,
            readonly_path,
            exhaustive,
            assigned,
            properties: FxHashMap::default(),
            imported: FxHashMap::default(),
            writes: FxHashMap::default(),
        }
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
    /// When the variable is an element of a destructuring pattern that a test
    /// narrows at the read, as `v` in `const { kind, v } = s` after
    /// `kind === "c"`, its declared type there is its property on the
    /// narrowed pattern. Tests of the variable itself then narrow it from that
    /// type, as tests of a property path narrow it from its type on the
    /// narrowed object.
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
        let root = info.flow.root(read.root)?;
        let &FlowSubject::Binding(binding) = root.subject(read.subject)? else {
            return None;
        };
        let start = identifier.range().start();
        let declared = root
            .element_pattern(read.subject)
            .and_then(|pattern| self.narrowed_element(read, start, pattern, binding));
        // The read is a candidate only through its pattern when no test of
        // the variable itself reaches it.
        if !root.reads_at(read.subject, read.point) {
            return declared;
        }
        let baseline = match declared {
            Some(declared) => {
                let baseline = if root.is_property_only(read.subject) {
                    self.property_baseline(declared, root.is_instance_tested(read.subject))
                } else {
                    self.narrowing_baseline(declared)
                };
                match baseline {
                    Some(baseline) => baseline,
                    None => return Some(declared),
                }
            }
            None => self.flow_binding_baseline(binding)?,
        };
        let exhaustive = self.exhaustive_switches(read, start);
        let assigned = self.flow_assigned_types(read.root, read.subject);
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
                ),
                read.point,
            ),
        )
        .or(declared)
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
        let baseline = if root.is_property_only(read.subject) {
            self.property_baseline(declared, root.is_instance_tested(read.subject))
        } else {
            self.narrowing_baseline(declared)
        }?;
        if root.is_assignment_only(read.subject) && !may_narrow_by_assignment(baseline) {
            return None;
        }
        let readonly_path =
            root.is_readonly_tested(read.subject) && self.is_readonly_path(read, member);
        if !readonly_path && root.is_only_readonly_tested(read.subject) {
            return None;
        }
        let exhaustive = self.exhaustive_switches(read, member.range().start());
        let assigned = self.flow_assigned_types(read.root, read.subject);
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
                ),
                read.point,
            ),
        )
    }

    /// Returns the types that the writes of `subject` in the root at index
    /// `root` assign, as [`crate::db::queries::infer_flow_assigned_types`]
    /// resolves them, or resolved in this context and kept per root and
    /// subject when it evaluates flow locally.
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
    /// default's type, as TypeScript's `getTypeWithDefault` does. A value
    /// that can't be resolved, or whose type may come from a return type that
    /// Biome infers, is `Unknown`, which reduces nothing.
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
        let types: Box<[_]> = values
            .iter()
            .map(|value| {
                root.assigned
                    .get(*value as usize)
                    .and_then(|assigned| self.assigned_type(assigned))
                    .unwrap_or(TypeData::Unknown)
            })
            .collect();
        Some(FlowAssignedTypes::new(self.db, types))
    }

    /// Returns the type of the value that `assigned` describes, as
    /// [`Self::assigned_types`] resolves it, or `None` when it can't be
    /// resolved or when a call that it can give may have a return type that
    /// Biome infers, as [`Self::has_annotated_return`] decides.
    fn assigned_type(&mut self, assigned: &FlowAssigned) -> Option<TypeData<'db>> {
        if !assigned
            .callees
            .iter()
            .all(|callee| self.has_annotated_return(*callee))
        {
            return None;
        }
        let mut ty = match assigned.value {
            // TypeScript types an object or array literal by the declared type
            // of its target, which keeps the literal types of its properties
            // where the declared type has them. Keeping all of them gives a
            // subtype of TypeScript's type, which the reduction never makes
            // narrower than TypeScript's.
            FlowAssignedValue::Expression(range) => {
                let reference = self.js_info.raw_expressions.get(&range)?.clone();
                let ty = self.resolve(&reference);
                self.literal_argument(&reference, ty).unwrap_or(ty)
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
        Some(ty)
    }

    /// Returns whether the return type of `callee` comes from an annotation,
    /// as [`CalleeReturn::Resolved`] describes.
    fn has_annotated_return(&mut self, callee: FlowCallee) -> bool {
        let info = self.js_info;
        match callee {
            FlowCallee::Import(binding) => {
                imported_return_is_annotated(self.db, self.module, info, binding)
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
    /// narrowing, it is the ordinary type. The statement is exhaustive when
    /// the guard of its bypass, which tests that no case matched, leaves that
    /// type `never`. For `switch (typeof x)` with `x: unknown` or `x: any`,
    /// the cases must test every kind of value, as in TypeScript.
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
        let Some(discriminant) = self
            .flow_operand_at(switch.discriminant)
            .or_else(|| self.raw_operand_at(switch.discriminant))
        else {
            return false;
        };
        let Some(ty) = self.flow_baseline(discriminant) else {
            return false;
        };
        if let (TypeData::AnyKeyword | TypeData::UnknownKeyword, Some(kinds)) =
            (ty, switch.typeof_kinds)
        {
            return kinds == u8::MAX;
        }
        if matches!(ty, TypeData::Unknown | TypeData::NeverKeyword) || is_narrowing_invariant(ty) {
            return false;
        }
        let mut solve = GuardSolve::new(switch.subject, ty, false, 0, None);
        if !self.compares_with_unit_values(root, switch.guard, &mut solve) {
            return false;
        }
        let mut remaining = MAX_FLOW_STEPS;
        self.apply_guard(root, switch.guard, &mut solve, ty, &mut remaining)
            == Some(TypeData::NeverKeyword)
    }

    /// Returns whether every imported variable that the guard at `guard`
    /// compares the subject of `solve` with holds a unit value in the module
    /// that declares it.
    ///
    /// TypeScript finds a `switch` statement exhaustive only when the type of
    /// every case is a unit type. A case that compares with an imported
    /// variable that holds no unit value narrows nothing, so without this
    /// check the remaining cases alone would make the statement exhaustive.
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
                Some(FlowGuard::ReadonlyPath(guard) | FlowGuard::Bypass { guard, .. }) => {
                    pending.push(*guard);
                }
                Some(FlowGuard::Test {
                    subject,
                    test: FlowTest::Imported { binding, .. },
                    ..
                }) if *subject == solve.subject => {
                    if self.imported_value(*binding, solve).is_none() {
                        return false;
                    }
                }
                Some(
                    FlowGuard::Keep
                    | FlowGuard::Incomplete
                    | FlowGuard::Test { .. }
                    | FlowGuard::Property { .. },
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
    /// contains `any` or an undetermined member, and `Unknown` when flattening
    /// exceeds its work limit.
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
        (!is_narrowing_invariant(baseline) && may_narrow_by_property(self.db, baseline))
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
    /// narrow `unknown` and a type with one object type, which
    /// [`may_narrow_by_instanceof`] checks.
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
    fn flow_baseline(&mut self, ty: TypeData<'db>) -> Option<TypeData<'db>> {
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
        // `undefined` at the start, as TypeScript's initial type gives it.
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
                if *tested != subject {
                    return Some(ty);
                }
                if let FlowTest::In(_) | FlowTest::Instanceof(_) | FlowTest::Predicate { .. } = test
                {
                    // Loops revisit conditions, and each application compares
                    // every member of the union.
                    if let Some(narrowed) = solve.properties.get(&(index, ty)) {
                        return Some(*narrowed);
                    }
                    let narrowed = match *test {
                        FlowTest::In(key) => self.narrow_by_in(ty, solve.declared, key, *positive),
                        FlowTest::Predicate {
                            function,
                            object_target,
                        } => self.narrow_by_predicate(
                            ty,
                            solve.declared,
                            function,
                            object_target,
                            *positive,
                        ),
                        test @ FlowTest::Instanceof(_) => {
                            self.narrow_by_instance(ty, solve.declared, test, *positive)
                        }
                        FlowTest::Truthy
                        | FlowTest::Nullish
                        | FlowTest::Typeof(_)
                        | FlowTest::Null
                        | FlowTest::Literal(_)
                        | FlowTest::Undefined { .. }
                        | FlowTest::Imported { .. } => ty,
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
                if *object != subject {
                    return Some(ty);
                }
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

    /// Returns the type of the subject that `solve` narrows right after the
    /// write at index `index`, given its incoming type `ty`, following
    /// TypeScript's `getTypeAtFlowAssignment`.
    ///
    /// A write of the subject itself gives the declared type reduced by the
    /// assigned type, as [`FlowWriteEffect`] describes. The result depends
    /// only on the declared type and the value, except for a compound
    /// assignment, so it is computed once per write. A write of a proper
    /// prefix of the subject's path, or of any property of one, gives the
    /// declared type, as TypeScript's `containsMatchingReference` does. Any
    /// other write leaves the incoming type.
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
        let (declared, value) = match write.effect {
            FlowWriteEffect::Wildcard => return ty,
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
        let narrowed = match assigned {
            Some(assigned) => {
                narrow_by_assignment(self.db, declared, assigned, self.narrowing_resolver())
            }
            None => declared,
        };
        solve.writes.insert(index, narrowed);
        narrowed
    }

    /// Narrows `ty` by `name in value`, where `name` is the string or number
    /// literal at `key`. When `positive` is true, the members of `ty` that
    /// lack the property are removed. When `positive` is false, the members
    /// that declare it as required are removed. See [`narrow_by_in_operator`].
    ///
    /// Returns `ty` unchanged when the subject's `declared` type is not a
    /// union with two object types, because the test could then only make a
    /// branch impossible. Also returns `ty` unchanged when the key doesn't
    /// resolve to a string or number literal.
    fn narrow_by_in(
        &mut self,
        ty: TypeData<'db>,
        declared: TypeData<'db>,
        key: TextRange,
        positive: bool,
    ) -> TypeData<'db> {
        if !may_narrow_by_property(self.db, declared) {
            return ty;
        }
        let Some(reference) = self.js_info.raw_expressions.get(&key) else {
            return ty;
        };
        let TypeData::Literal(literal) = self.resolve(reference) else {
            return ty;
        };
        let name = match literal.literal(self.db) {
            Literal::String(string) => string.as_str(),
            Literal::Number(number) => number.as_str(),
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
    /// type and is not `unknown`, because the test could then only make a
    /// branch impossible. `C` is resolved only after that check.
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

    /// Returns the predicate that `test` applies.
    ///
    /// Returns `None` for a call to a type-guard function, an `in` test, and an
    /// `instanceof` test, which have no predicate. Also returns `None` for a
    /// comparison with an imported variable that holds no unit value, or, with
    /// loose equality, that holds a unit value other than `null` or
    /// `undefined`, because such a comparison narrows nothing.
    fn test_predicate(
        &mut self,
        test: FlowTest,
        solve: &mut GuardSolve<'db>,
    ) -> Option<NarrowingPredicate<'db>> {
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
            FlowTest::Imported { binding, strict } => {
                match (self.imported_value(binding, solve)?, strict) {
                    (value, true) => NarrowingPredicate::Literal(value),
                    (TypeData::Null | TypeData::Undefined, false) => NarrowingPredicate::Nullish,
                    // Loose equality with any other value doesn't narrow the
                    // subject, as loose equality with a literal doesn't.
                    (_, false) => return None,
                }
            }
            FlowTest::Predicate { .. } | FlowTest::In(_) | FlowTest::Instanceof(_) => {
                return None;
            }
        })
    }

    /// Returns the unit value that the imported variable declared at `binding`
    /// holds in the module that declares it, or `None` when it holds none.
    ///
    /// The result is kept in `solve`, so a loop that applies the test again
    /// doesn't follow the import again. [`imported_unit_value`] lists the
    /// imports followed.
    fn imported_value(
        &mut self,
        binding: TextRange,
        solve: &mut GuardSolve<'db>,
    ) -> Option<TypeData<'db>> {
        if let Some(value) = solve.imported.get(&binding) {
            return *value;
        }
        let value = imported_unit_value(self.db, self.module, self.js_info, binding);
        solve.imported.insert(binding, value);
        value
    }

    /// Returns `ty` narrowed by a call to the type-guard function declared at
    /// `function`, given the subject's `declared` type, with
    /// [`narrow_by_type_predicate`].
    ///
    /// `object_target` is true when collection found that every member of the
    /// target is an object type. Biome narrows by such a target only a type
    /// that has an object type or is `unknown`, so when `declared` is not one,
    /// this returns `ty` unchanged without resolving the target. TypeScript
    /// also narrows a primitive by such a target, as `string` by
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

/// Limits the imports followed to find the value of an imported variable that
/// a comparison names.
const MAX_IMPORT_STEPS: usize = 16;

/// Returns the unit value that the imported variable declared at `binding` in
/// `js_info`, the module `module`, holds: `null`, `undefined`, or a string,
/// number, bigint, or boolean literal type.
///
/// The import is followed through re-exports to the module that declares the
/// variable. When that variable is a `const` that names another imported
/// variable, the search continues with that import, through at most 16
/// imports in all. The value comes from the syntax of the declarations and the
/// raw type of the literal, without resolving any type, so it is the same in
/// every resolution context.
///
/// Returns `None` for a type-only import, a namespace import, an import that
/// doesn't resolve to a single declaration, an export that isn't a variable, a
/// variable that holds no unit value, and more than 16 imports.
/// An export that isn't a variable includes `export default "a"`, which
/// TypeScript narrows by.
fn imported_unit_value<'db>(
    db: &'db dyn ModuleDb,
    mut module: ModuleInfo,
    js_info: &JsModuleInfo,
    mut binding: TextRange,
) -> Option<TypeData<'db>> {
    let mut info = js_info;
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
        match binding_unit_value(&declared, &origin_info.semantic_model)? {
            UnitValue::Null => return Some(TypeData::Null),
            UnitValue::Undefined => return Some(TypeData::Undefined),
            UnitValue::Literal(literal) => return raw_literal_type(db, origin_info, literal),
            UnitValue::Imported(next) => {
                module = origin;
                info = origin_info;
                binding = next;
            }
        }
    }
    None
}

/// Returns whether the return type of a call to the imported binding at
/// `binding` in `js_info`, the module `module`, comes from an annotation.
///
/// The import is followed through re-exports to the module that declares the
/// export, as `find_export_origin` follows exports. A binding that an import
/// declares there is followed in turn, at most 16 of them. Another binding
/// is annotated in a declaration file, which declares only annotated
/// functions, and elsewhere as [`declaration_return`] decides.
fn imported_return_is_annotated(
    db: &dyn ModuleDb,
    mut module: ModuleInfo,
    js_info: &JsModuleInfo,
    mut binding: TextRange,
) -> bool {
    let mut info = js_info;
    for _ in 0..MAX_IMPORT_STEPS {
        let Some(TypeReference::Import(import)) = info.raw_binding_types.get(&binding) else {
            return false;
        };
        let name = match &import.symbol {
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::Default => Text::new_static("default"),
            ImportSymbol::All => return false,
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
        let Some(declared) = origin_info.semantic_model.as_binding_by_range(*declaration) else {
            return false;
        };
        match declaration_return(&declared) {
            CalleeReturn::Resolved(FlowCallee::Import(next)) => {
                module = origin;
                info = origin_info;
                binding = next;
            }
            _ if is_declaration_file(db, origin) => return true,
            CalleeReturn::Annotated => return true,
            CalleeReturn::Inferred | CalleeReturn::Resolved(FlowCallee::Member(_)) => return false,
        }
    }
    false
}

/// Returns whether every type that `ty` can be gets its member `name` from
/// the lib, from a declaration file, or from methods with a return type
/// annotation, so that a call of the member has a return type that comes
/// from an annotation.
///
/// That holds for a lib global or an instance of one, unless a module other
/// than a declaration file merges an interface into it; a primitive, a
/// literal that isn't an object, and a tuple, whose members are those of
/// their lib types; a type that a declaration file declares, when the
/// classes and interfaces that it extends are such types too; and an
/// instance of a class or an interface that a module other than a
/// declaration file declares, as [`own_member`] decides. `null` and
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
                        OwnMember::Inherited
                    } else if instance {
                        own_member(
                            db,
                            module,
                            class.scope_id(db),
                            class.members(db),
                            class.contributions(db),
                            name,
                        )
                    } else {
                        OwnMember::Other
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
                        )
                    };
                    (interface.extends(db).to_vec(), own)
                } else if matches!(resolved, TypeData::Object(_)) {
                    if in_declaration_file {
                        continue;
                    }
                    return false;
                } else {
                    // A handle in another module or a type that an alias
                    // names, which its own module decides.
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
    /// Its own methods, each with a return type annotation.
    Annotated,
    /// The classes and interfaces that it extends: it declares no member
    /// with the name, and no computed key or index signature that could
    /// stand for it.
    Inherited,
    /// Anything else, including a static member, a member without an
    /// annotation, and a member of a merged declaration.
    Other,
}

/// Returns where an instance of a class or an interface that `module`, a
/// module other than a declaration file, declares gets its member `name`,
/// given the scope that the class or interface records, the `members` that
/// local inference collected for it, and how many `contributions` other
/// modules merge into it.
///
/// Its own members are those that [`member_return`] finds in the declaration
/// that opens the scope. Declarations merged into it can add members with
/// the name: member lookup finds a class's own members before them, but can
/// find those of a later interface declaration first. So the member is its
/// own only when local inference collected no more members with the name
/// than the declaration declares, and inherited only when it collected none.
fn own_member(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    scope_id: Option<ScopeId>,
    members: &[TypeMember],
    contributions: usize,
    name: &str,
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
    let collected = members
        .iter()
        .filter(|member| !member.kind.is_static() && member.kind.has_name(name))
        .count();
    match member_return(&declaration, name) {
        (0, _)
            if collected == 0
                && members.iter().all(|member| {
                    member.kind.computed_value_type().is_none()
                        && member.kind.index_signature_type().is_none()
                }) =>
        {
            OwnMember::Inherited
        }
        (declared, CalleeReturn::Annotated) if declared != 0 && collected <= declared => {
            OwnMember::Annotated
        }
        _ => OwnMember::Other,
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

/// Returns the type of the expression at `range` in `js_info` when its raw
/// type is a string, number, bigint, or boolean literal, converted without a
/// resolution context, because a literal refers to no other type.
fn raw_literal_type<'db>(
    db: &'db dyn ModuleDb,
    js_info: &JsModuleInfo,
    range: TextRange,
) -> Option<TypeData<'db>> {
    let TypeReference::Resolved(RawTypeId::Local(id)) = js_info.raw_expressions.get(&range)? else {
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

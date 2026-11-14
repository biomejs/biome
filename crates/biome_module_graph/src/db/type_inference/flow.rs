//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Collection records the incoming flow of each refinable read and the guards
//! that the conditions of its execution root apply (see
//! [`crate::js_module_info::flow`]). This module resolves the type that the
//! read's subject starts from and applies those guards along every path that
//! reaches the read. A variable starts from its declared type. The declared
//! type of a property path at a read is the type of the member access there,
//! given the narrowed type of its object, as TypeScript types property reads.
//! Likewise, the declared type of an element of a destructuring pattern at a
//! read is its field on the pattern narrowed there.

use super::flow_expressions::collected_member_name;
use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, FlowSubjectRootInput, FlowSubjectTypeInput, infer_flow_binding_baseline,
    infer_flow_subject_states, infer_flow_subject_type,
};
use crate::js_module_info::flow::{
    FlowGuard, FlowNode, FlowNodeId, FlowRead, FlowSubject, FlowSubjectId, FlowTest, PropertyKey,
    RootFlow, guard_signature,
};
use crate::{JsModuleInfo, ModuleDb, ModuleInfo};
use biome_js_syntax::{AnyJsExpression, JsIdentifierExpression};
use biome_js_type_info::interned_types::{Literal, TypeData};
use biome_js_type_info::{
    NarrowingPredicate, PropertyTest, RawTypeData, RawTypeId, TypeId, TypeReference,
    TypeofExpression, has_readonly_property, is_narrowing_invariant, may_narrow_by_instanceof,
    may_narrow_by_property, narrow_by_in_operator, narrow_by_instanceof, narrow_by_property,
    narrow_by_type_predicate, narrow_type,
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
    let states = ctx.solve_subject(root, points, subject, baseline, input.readonly_path(db))?;
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
            match ctx.solve_subject(
                root,
                &[point],
                states.subject(db),
                baseline,
                states.readonly_path(db),
            ) {
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
    /// Handles to local types of the module form a cycle or a chain longer
    /// than [`MAX_MODULE_HANDLE_STEPS`].
    Cycle,
}

/// The state of one solve that its guards share: the subject it narrows, the
/// subject's declared type at the root's start, whether the subject is a
/// readonly path, as [`FlowGuard::ReadonlyPath`] requires, and the results of
/// the property guards, `in` tests, `instanceof` tests, and calls to
/// type-guard functions applied so far, keyed by guard index and incoming
/// type.
struct GuardSolve<'db> {
    /// The subject that the solve narrows.
    subject: FlowSubjectId,
    /// The subject's declared type at the root's start, which decides whether
    /// a tested property is a discriminant.
    declared: TypeData<'db>,
    readonly_path: bool,
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
    /// Whether each property path read so far is readonly, keyed by the
    /// index of its root and its subject, as
    /// [`ResolutionCtx::is_readonly_path`] decides it.
    readonly_paths: FxHashMap<(usize, FlowSubjectId), bool>,
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
        let declared = root
            .element_pattern(read.subject)
            .and_then(|pattern| self.narrowed_element(read, pattern, binding));
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
                ),
                read.point,
            ),
        )
        .or(declared)
    }

    /// Returns the type at `read` of the element declared at `element`, given
    /// the `pattern` that holds it. The type is the element's property on the
    /// pattern narrowed at the read, destructured the same way as the
    /// element's collected type destructures the pattern's declared type.
    ///
    /// Returns `None` when no test can narrow the pattern, when the pattern
    /// keeps its declared type at the read, and when the property can't be
    /// read, so the element keeps its declared type. Returns `Unknown` when
    /// the narrowing of the pattern exceeds a work limit or meets a cycle,
    /// and when no value of the pattern reaches the read.
    fn narrowed_element(
        &mut self,
        read: FlowRead,
        pattern: FlowSubjectId,
        element: TextRange,
    ) -> Option<TypeData<'db>> {
        let info = self.js_info;
        let &FlowSubject::Pattern(range) = info.flow.root(read.root)?.subject(pattern)? else {
            return None;
        };
        let baseline = self.flow_binding_baseline(range)?;
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
    /// only when the path is readonly, as [`Self::is_readonly_path`] decides
    /// once the declared type can narrow.
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
        let readonly_path =
            root.is_readonly_tested(read.subject) && self.is_readonly_path(read, member);
        if !readonly_path && root.is_only_readonly_tested(read.subject) {
            return None;
        }
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
                ),
                read.point,
            ),
        )
    }

    /// Returns whether the property path that `member` reads, at `read`, is
    /// readonly, as [`FlowGuard::ReadonlyPath`] requires: whether, at every
    /// step of the path, the declared type of the object, the type of its
    /// expression without narrowing, declares the property readonly in every
    /// member other than `null` and `undefined`, as [`has_readonly_property`]
    /// decides it. Every read of a path gives the same answer, so the answer
    /// is kept per root and subject when this context evaluates flow locally.
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

    /// Checks the steps of the path as [`Self::is_readonly_path`] describes,
    /// from the last property to the first, or returns `None` when the syntax
    /// of `member` doesn't match the path.
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

    /// Returns the type of `subject` at each of `points`, given its `baseline`
    /// type at the root's start and whether it is a readonly path, or `None`
    /// when the walk or the solve exceeds the flow work limit.
    fn solve_subject(
        &mut self,
        root: &RootFlow,
        points: &[FlowNodeId],
        subject: FlowSubjectId,
        baseline: TypeData<'db>,
        readonly_path: bool,
    ) -> Option<Vec<TypeData<'db>>> {
        let reaching = ReachingFlow::new(root, points)?;
        let states = self.solve_flow(root, &reaching, subject, baseline, readonly_path)?;
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

    /// Returns the type of `subject` at every node of `reaching`, given its
    /// `baseline` type at the root's start and whether it is a readonly path,
    /// or `None` when the solve exceeds the flow work limit.
    fn solve_flow(
        &mut self,
        root: &RootFlow,
        reaching: &ReachingFlow,
        subject: FlowSubjectId,
        baseline: TypeData<'db>,
        readonly_path: bool,
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
            readonly_path,
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
                        | FlowTest::Undefined { .. } => ty,
                    };
                    solve.properties.insert((index, ty), narrowed);
                    return Some(narrowed);
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
            FlowGuard::ReadonlyPath(inner) => {
                if !solve.readonly_path {
                    return Some(ty);
                }
                return self.apply_guard(root, *inner, solve, ty, remaining);
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
            FlowTest::Predicate { .. } | FlowTest::In(_) | FlowTest::Instanceof(_) => {
                return None;
            }
        })
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

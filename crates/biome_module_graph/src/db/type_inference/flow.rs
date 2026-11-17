//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Collection records the incoming flow of each refinable read and the guards
//! that the conditions of its execution root apply (see
//! [`crate::js_module_info::flow`]). This module resolves the read's binding
//! type and applies those guards along every path that reaches the read.

use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, FlowSubjectRootInput, FlowSubjectTypeInput, infer_flow_binding_baseline,
    infer_flow_subject_states, infer_flow_subject_type,
};
use crate::js_module_info::flow::{
    FlowGuard, FlowNode, FlowNodeId, FlowSubject, FlowSubjectId, FlowTest, RootFlow,
    guard_signature,
};
use crate::{JsModuleInfo, ModuleDb, ModuleInfo};
use biome_js_syntax::JsIdentifierExpression;
use biome_js_type_info::interned_types::{Literal, TypeData};
use biome_js_type_info::{NarrowingPredicate, TypeofKind, is_narrowing_invariant, narrow_type};
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

const MAX_FLOW_STEPS: usize = 16_384;
/// Limits the types visited while flattening a flow baseline or checking the
/// members of a refined union.
pub(super) const MAX_FLOW_TYPE_STEPS: usize = 1024;

pub(in crate::db) fn flow_expression_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    range: TextRange,
) -> Option<TypeData<'db>> {
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
    let reference = info.raw_binding_types.get(&range)?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    let declared = ctx.resolve(reference);
    ctx.narrowing_baseline(declared)
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
        let &FlowSubject::Binding(binding) = info.flow.root(read.root)?.subject(read.subject)?;
        let baseline = if self.evaluates_flow_locally() {
            self.local_binding_baseline(binding)
        } else {
            infer_flow_binding_baseline(
                self.db,
                BindingTypeInput::new(self.db, self.module, binding),
            )
        };
        infer_flow_subject_type(
            self.db,
            FlowSubjectTypeInput::new(
                self.db,
                FlowSubjectRootInput::new(self.db, self.module, read.root, read.subject, baseline),
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
        let baseline = self
            .js_info
            .raw_binding_types
            .get(&binding)
            .cloned()
            .and_then(|reference| {
                let declared = self.resolve(&reference);
                self.narrowing_baseline(declared)
            });
        self.local_flow
            .as_mut()?
            .baselines
            .insert(binding, baseline);
        baseline
    }

    /// Returns the type that narrowing of a binding whose declared type is
    /// `declared` starts from: `declared` with its unions, aliases, and
    /// `typeof` types flattened.
    ///
    /// Returns `None` when no runtime test can change the type, because it
    /// contains `any` or an undetermined member, and `Unknown` when flattening
    /// exceeds its work limit.
    fn narrowing_baseline(&mut self, declared: TypeData<'db>) -> Option<TypeData<'db>> {
        // The declared type is resolved from the raw binding type, not through
        // a declaration query: declaration queries can erase a cyclic object's
        // shape, while the raw type still lets flow detect an impossible
        // branch and return an unknown override.
        match self.flow_baseline(declared) {
            Some(baseline) if is_narrowing_invariant(baseline) => None,
            Some(baseline) => Some(baseline),
            None => Some(TypeData::Unknown),
        }
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

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Other type shapes remain unchanged."
    )]
    fn flow_baseline(&mut self, ty: TypeData<'db>) -> Option<TypeData<'db>> {
        let mut pending = vec![ty];
        let mut types = Vec::new();
        for step in 0..MAX_FLOW_TYPE_STEPS {
            let Some(ty) = pending.pop() else {
                return Some(TypeData::union_from_types(self.db, types));
            };
            let ty = resolve_local_type_on_demand(self.db, ty).expand_global_local(self.db);
            match ty {
                TypeData::Union(union) => {
                    let children = union.types(self.db);
                    if children.len()
                        > (MAX_FLOW_TYPE_STEPS - step - 1).saturating_sub(pending.len())
                    {
                        return None;
                    }
                    pending.extend(children.iter().rev().copied());
                }
                TypeData::TypeofType(wrapper) => pending.push(wrapper.ty(self.db)),
                TypeData::TypeofValue(wrapper) => pending.push(wrapper.ty(self.db)),
                TypeData::InstanceOf(instance) => {
                    let target = resolve_local_type_on_demand(self.db, instance.ty(self.db));
                    if target.should_flatten_instance(instance.type_parameters(self.db)) {
                        pending.push(target);
                    } else {
                        types.push(ty);
                    }
                }
                _ => types.push(ty),
            }
        }
        None
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
                        self.apply_guard(root, *guard, subject, incoming, &mut remaining)?
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

    /// Returns the type of `subject` after the guard at `guard` holds, given
    /// its incoming type `ty`, or `None` when the flow work limit runs out.
    fn apply_guard(
        &mut self,
        root: &RootFlow,
        guard: usize,
        subject: FlowSubjectId,
        ty: TypeData<'db>,
        remaining: &mut usize,
    ) -> Option<TypeData<'db>> {
        *remaining = remaining.checked_sub(1)?;
        let Some(guard) = root.guards.get(guard) else {
            return Some(TypeData::Unknown);
        };
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
                    Some(predicate) => narrow_type(self.db, ty, predicate, *positive),
                    None => ty,
                }
            }
            FlowGuard::Both {
                left,
                right,
                sequential,
            } => {
                let left = self.apply_guard(root, *left, subject, ty, remaining)?;
                let right = self.apply_guard(
                    root,
                    *right,
                    subject,
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

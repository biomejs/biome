//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Collection records the incoming flow of each refinable read and the guards
//! that the conditions of its execution root apply (see
//! [`crate::js_module_info::flow`]). This module resolves the read's binding
//! type and applies those guards along every path that reaches the read.

use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, FlowBindingTypeInput, infer_flow_binding_baseline, infer_flow_binding_type,
};
use crate::js_module_info::flow::{FlowGuard, FlowNode, FlowNodeId, FlowTest, RootFlow};
use crate::{JsModuleInfo, ModuleDb, ModuleInfo};
use biome_js_syntax::JsIdentifierExpression;
use biome_js_type_info::interned_types::TypeData;
use biome_js_type_info::{NarrowingPredicate, is_narrowing_invariant, narrow_type};
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::collections::hash_map::Entry;

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
    // Declaration queries can erase a cyclic object's shape. The raw baseline
    // still lets flow detect an impossible branch and return an unknown override.
    let reference = info.raw_binding_types.get(&range)?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    let baseline = ctx.resolve(reference);
    match ctx.flow_baseline(baseline) {
        Some(baseline) if is_narrowing_invariant(baseline) => None,
        Some(baseline) => Some(baseline),
        None => Some(TypeData::Unknown),
    }
}

pub(in crate::db) fn flow_binding_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    input: FlowBindingTypeInput<'db>,
) -> Option<TypeData<'db>> {
    let root = info.flow.root(input.root(db))?;
    let point = input.point(db);
    if point as usize >= root.nodes.len() {
        return None;
    }
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.narrow_binding_at_flow(root, point, input.binding(db))
}

/// The nodes of a root's flow graph that lie on some path from its start to
/// one point.
///
/// Reached nodes are numbered from zero in the order a backward walk from the
/// point first visits them, so the point is node `0`. Solving a read over these
/// numbers takes work and memory proportional to the reached nodes rather than
/// to the whole root.
struct ReachingFlow {
    /// The root's id of each reached node.
    nodes: Vec<FlowNodeId>,
    /// The numbers of every reached node's predecessors, grouped by node.
    predecessors: Vec<u32>,
    /// The end of each reached node's group in `predecessors`; the group starts
    /// where the previous node's group ends.
    ends: Vec<u32>,
}

impl ReachingFlow {
    /// Walks backward from `point` through the predecessors in `root`.
    ///
    /// Returns `None` when the walk exceeds its work limit or meets a node
    /// that `root` does not contain.
    fn new(root: &RootFlow, point: FlowNodeId) -> Option<Self> {
        let mut nodes = Vec::new();
        let mut numbers = FxHashMap::default();
        let mut pending = vec![point];
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(node) = pending.pop() {
            remaining = remaining.checked_sub(1)?;
            let Entry::Vacant(entry) = numbers.entry(node) else {
                continue;
            };
            entry.insert(u32::try_from(nodes.len()).ok()?);
            nodes.push(node);
            pending.extend(predecessor_ids(root.nodes.get(node as usize)?));
        }
        let mut predecessors = Vec::new();
        let mut ends = Vec::with_capacity(nodes.len());
        for node in &nodes {
            for id in predecessor_ids(&root.nodes[*node as usize]) {
                predecessors.push(*numbers.get(id)?);
            }
            ends.push(u32::try_from(predecessors.len()).ok()?);
        }
        Some(Self {
            nodes,
            predecessors,
            ends,
        })
    }

    /// Returns the numbers of the predecessors of the reached node `number`.
    fn predecessors(&self, number: u32) -> &[u32] {
        let number = number as usize;
        let start = number
            .checked_sub(1)
            .map_or(0, |previous| self.ends[previous]);
        &self.predecessors[start as usize..self.ends[number] as usize]
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

impl<'db> ResolutionCtx<'db, '_> {
    pub(super) fn narrow_reference(
        &mut self,
        identifier: &JsIdentifierExpression,
    ) -> Option<TypeData<'db>> {
        let read = self.js_info.flow.read(identifier.range())?;
        infer_flow_binding_type(
            self.db,
            FlowBindingTypeInput::new(
                self.db,
                BindingTypeInput::new(self.db, self.module, read.binding),
                read.root,
                read.point,
            ),
        )
    }

    /// Applies the guards on every path to `point` to the binding's baseline.
    ///
    /// Collection records only reads that a test of their binding can reach, so
    /// the baseline is resolved without a separate relevance check.
    fn narrow_binding_at_flow(
        &mut self,
        root: &RootFlow,
        point: FlowNodeId,
        binding: BindingTypeInput<'db>,
    ) -> Option<TypeData<'db>> {
        let baseline = infer_flow_binding_baseline(self.db, binding)?;
        if baseline == TypeData::Unknown {
            return Some(TypeData::Unknown);
        }
        let Some(reaching) = ReachingFlow::new(root, point) else {
            return Some(TypeData::Unknown);
        };
        let narrowed = self.solve_flow(root, &reaching, binding.range(self.db), baseline);
        if narrowed == TypeData::NeverKeyword {
            return Some(TypeData::Unknown);
        }
        (narrowed != baseline).then_some(narrowed)
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

    /// Returns the type of `binding` at the point of `reaching`, given its
    /// `baseline` type at the root's start.
    fn solve_flow(
        &mut self,
        root: &RootFlow,
        reaching: &ReachingFlow,
        binding: TextRange,
        baseline: TypeData<'db>,
    ) -> TypeData<'db> {
        let count = reaching.nodes.len();
        let mut successors = vec![Vec::new(); count];
        let mut queue = VecDeque::new();
        for (number, node) in (0..).zip(&reaching.nodes) {
            if matches!(root.nodes[*node as usize], FlowNode::Start) {
                queue.push_back(number);
            }
            for &predecessor in reaching.predecessors(number) {
                successors[predecessor as usize].push(number);
            }
        }
        let mut states = vec![TypeData::NeverKeyword; count];
        let mut queued = vec![false; count];
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(number) = queue.pop_front() {
            let index = number as usize;
            queued[index] = false;
            let Some(next) = remaining.checked_sub(1) else {
                return TypeData::Unknown;
            };
            remaining = next;
            let predecessors = reaching.predecessors(number);
            let ty = match &root.nodes[reaching.nodes[index] as usize] {
                FlowNode::Start => baseline,
                FlowNode::Join(_) => TypeData::union_from_types(
                    self.db,
                    predecessors
                        .iter()
                        .map(|predecessor| states[*predecessor as usize])
                        .collect(),
                ),
                FlowNode::Condition { guard, .. } => {
                    let incoming = states[predecessors[0] as usize];
                    if incoming == TypeData::NeverKeyword {
                        incoming
                    } else {
                        self.apply_guard(root, *guard, binding, incoming, &mut remaining)
                    }
                }
            };
            if states[index] != ty {
                states[index] = ty;
                for &successor in &successors[index] {
                    if !queued[successor as usize] {
                        queued[successor as usize] = true;
                        queue.push_back(successor);
                    }
                }
            }
        }
        states[0]
    }

    /// Returns the type of `binding` after the guard at `guard` holds, given
    /// its incoming type `ty`.
    fn apply_guard(
        &mut self,
        root: &RootFlow,
        guard: usize,
        binding: TextRange,
        ty: TypeData<'db>,
        remaining: &mut usize,
    ) -> TypeData<'db> {
        let (Some(next), Some(guard)) = (remaining.checked_sub(1), root.guards.get(guard)) else {
            return TypeData::Unknown;
        };
        *remaining = next;
        match guard {
            FlowGuard::Keep => ty,
            FlowGuard::Incomplete => TypeData::Unknown,
            FlowGuard::Test {
                binding: tested,
                test,
                positive,
            } => {
                if *tested != binding {
                    return ty;
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
                let left = self.apply_guard(root, *left, binding, ty, remaining);
                let right = self.apply_guard(
                    root,
                    *right,
                    binding,
                    if *sequential { left } else { ty },
                    remaining,
                );
                if *sequential {
                    right
                } else {
                    TypeData::union_from_types(self.db, vec![left, right])
                }
            }
        }
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reaching_flow_numbers_only_the_nodes_before_the_point() {
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
        };
        let reaching = ReachingFlow::new(&root, 2).unwrap();
        assert_eq!(reaching.nodes, [2, 1, 3, 0]);
        let predecessors: Vec<_> = (0..4).map(|number| reaching.predecessors(number)).collect();
        assert_eq!(predecessors, [&[1][..], &[3, 2], &[0], &[]]);
        assert!(ReachingFlow::new(&root, 5).is_none());
    }
}

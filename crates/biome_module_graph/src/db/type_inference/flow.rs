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
use crate::js_module_info::flow::{
    AnyFlowExpression, FlowGuard, FlowNode, FlowNodeId, FlowTest, RootFlow,
};
use crate::{JsModuleInfo, ModuleDb, ModuleInfo};
use biome_js_syntax::{AnyJsExpression, JsIdentifierExpression, JsSyntaxNode};
use biome_js_type_info::interned_types::TypeData;
use biome_js_type_info::{NarrowingPredicate, is_narrowing_invariant, narrow_type};
use biome_rowan::{AstNode, TextRange};
use std::collections::VecDeque;

const MAX_FLOW_STEPS: usize = 16_384;
const MAX_FLOW_TYPE_STEPS: usize = 1024;

fn expression_at(root: &JsSyntaxNode, range: TextRange) -> Option<AnyJsExpression> {
    if range.is_empty() || !root.text_range_with_trivia().contains_range(range) {
        return None;
    }
    root.covering_element(range).ancestors().find_map(|node| {
        (node.text_trimmed_range() == range)
            .then(|| AnyJsExpression::cast(node))
            .flatten()
    })
}

pub(in crate::db) fn flow_expression_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    range: TextRange,
) -> Option<TypeData<'db>> {
    let tree = info.semantic_model.root();
    let expression = AnyFlowExpression::cast(expression_at(tree.syntax(), range)?.into_syntax())?;
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
    if point >= root.nodes.len() {
        return None;
    }
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.narrow_binding_at_flow(root, point, input.binding(db))
}

/// Returns the nodes on some path from the root's start to `point`, or `None`
/// when the walk exceeds its work limit.
fn reaching_nodes(root: &RootFlow, point: FlowNodeId) -> Option<Vec<bool>> {
    let mut pending = vec![point];
    let mut reaching = vec![false; root.nodes.len()];
    let mut remaining = MAX_FLOW_STEPS;
    while let Some(node) = pending.pop() {
        remaining = remaining.checked_sub(1)?;
        if std::mem::replace(reaching.get_mut(node)?, true) {
            continue;
        }
        match &root.nodes[node] {
            FlowNode::Start => {}
            FlowNode::Join(predecessors) => pending.extend(predecessors),
            FlowNode::Condition { antecedent, .. } => pending.push(*antecedent),
        }
    }
    Some(reaching)
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
        let Some(reaching) = reaching_nodes(root, point) else {
            return Some(TypeData::Unknown);
        };
        let narrowed = self.solve_flow(root, &reaching, point, binding.range(self.db), baseline);
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

    fn solve_flow(
        &mut self,
        root: &RootFlow,
        reaching: &[bool],
        point: FlowNodeId,
        binding: TextRange,
        baseline: TypeData<'db>,
    ) -> TypeData<'db> {
        let mut successors = vec![Vec::new(); root.nodes.len()];
        let mut queue = VecDeque::new();
        for (index, node) in root.nodes.iter().enumerate() {
            if !reaching[index] {
                continue;
            }
            match node {
                FlowNode::Start => queue.push_back(index),
                FlowNode::Join(predecessors) => {
                    for predecessor in predecessors {
                        successors[*predecessor].push(index);
                    }
                }
                FlowNode::Condition { antecedent, .. } => successors[*antecedent].push(index),
            }
        }
        let mut states = vec![TypeData::NeverKeyword; root.nodes.len()];
        let mut queued = vec![false; root.nodes.len()];
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(index) = queue.pop_front() {
            queued[index] = false;
            let Some(next) = remaining.checked_sub(1) else {
                return TypeData::Unknown;
            };
            remaining = next;
            let ty = match &root.nodes[index] {
                FlowNode::Start => baseline,
                FlowNode::Join(predecessors) => TypeData::union_from_types(
                    self.db,
                    predecessors
                        .iter()
                        .map(|predecessor| states[*predecessor])
                        .collect(),
                ),
                FlowNode::Condition { antecedent, guard } => {
                    let incoming = states[*antecedent];
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
                    if !queued[successor] {
                        queued[successor] = true;
                        queue.push_back(successor);
                    }
                }
            }
        }
        states[point]
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

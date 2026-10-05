//! Refines runtime reads without changing declaration types or raw type identities.
//!
//! Semantic bindings identify values; the syntax-only flow graph identifies the
//! incoming state of each occurrence. Only unwritten bindings declared in the
//! same execution root participate, so refinements cannot cross closure boundaries.

use super::flow_conditions::{
    ConditionStep, MAX_CONDITION_DEPTH, SyntaxGuard, condition_step, condition_subjects,
};
use super::{ImportResolution, ResolutionCtx, resolve_local_type_on_demand};
use crate::db::queries::{
    BindingTypeInput, FlowBindingTypeInput, FlowRootInput, infer_flow_binding_type,
    narrowing_flow_for_root,
};
use crate::{JsModuleInfo, ModuleDb, ModuleInfo};
use biome_js_control_flow::{
    AnyJsControlFlowRoot, FlowNode, FlowNodeId, FlowOutcome, NarrowingFlowGraph,
};
use biome_js_semantic::{Binding, JsDeclarationKind};
use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, JsSyntaxNode};
use biome_js_type_info::interned_types::TypeData;
use biome_js_type_info::{NarrowingPredicate, narrow_type};
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
    let expression = expression_at(tree.syntax(), range)?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.resolve_flow_expression(&expression)
}

pub(in crate::db) fn flow_binding_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    info: &JsModuleInfo,
    input: FlowBindingTypeInput<'db>,
) -> Option<TypeData<'db>> {
    let range = input.binding(db).range(db);
    let binding = info.semantic_model.as_binding_by_range(range)?;
    // The semantic index matches the start offset, not the entire range.
    if binding.range() != range {
        return None;
    }
    let root = binding_flow_root(&binding)?;
    let root_input = input.root(db);
    if root.range() != root_input.root(db) {
        return None;
    }
    let graph = narrowing_flow_for_root(db, root_input).as_ref()?;
    let mut ctx = ResolutionCtx::new(db, module, info, ImportResolution::on_demand());
    ctx.narrow_binding_at_flow(graph, input.point(db), &root, &binding)
}

fn binding_flow_root(binding: &Binding) -> Option<AnyJsControlFlowRoot> {
    // A `var` initializer is not a semantic write and may follow an earlier guard.
    if binding.is_imported()
        || binding.declaration_kind() == JsDeclarationKind::HoistedValue
        || binding.all_writes().next().is_some()
    {
        return None;
    }
    binding
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsControlFlowRoot::cast)
}

impl<'db> ResolutionCtx<'db, '_> {
    pub(super) fn narrow_reference(
        &mut self,
        expression: &AnyJsExpression,
    ) -> Option<TypeData<'db>> {
        let AnyJsExpression::JsIdentifierExpression(identifier) = expression else {
            return None;
        };
        let binding = self
            .js_info
            .semantic_model
            .binding(&identifier.name().ok()?)?;
        let declaration_root = binding_flow_root(&binding)?;
        let root = expression
            .syntax()
            .ancestors()
            .find_map(AnyJsControlFlowRoot::cast)?;
        if root.syntax() != declaration_root.syntax() {
            return None;
        }
        let input = FlowRootInput::new(self.db, self.module, root.range());
        let graph = narrowing_flow_for_root(self.db, input).as_ref()?;
        let point = *graph.expression_flows.get(&expression.range())?;
        infer_flow_binding_type(
            self.db,
            FlowBindingTypeInput::new(
                self.db,
                BindingTypeInput::new(self.db, self.module, binding.range()),
                input,
                point,
            ),
        )
    }

    fn narrow_binding_at_flow(
        &mut self,
        graph: &NarrowingFlowGraph,
        point: FlowNodeId,
        root: &AnyJsControlFlowRoot,
        binding: &Binding,
    ) -> Option<TypeData<'db>> {
        let mut pending = vec![point];
        let mut seen = vec![false; graph.nodes.len()];
        let mut relevant = false;
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(node) = pending.pop() {
            if remaining == 0 {
                return Some(TypeData::Unknown);
            }
            remaining -= 1;
            if *seen.get(node)? {
                continue;
            }
            seen[node] = true;
            match &graph.nodes[node] {
                FlowNode::Start => {}
                FlowNode::Join(predecessors) => pending.extend(predecessors),
                FlowNode::Condition {
                    antecedent,
                    expression,
                    outcome,
                } => {
                    pending.push(*antecedent);
                    if !relevant {
                        let mut condition_remaining = MAX_FLOW_TYPE_STEPS;
                        let subjects = expression_at(root.syntax(), *expression).map(|condition| {
                            condition_subjects(
                                condition.clone(),
                                *outcome,
                                &self.js_info.semantic_model,
                                &mut condition_remaining,
                            )
                            .unwrap_or_else(|| {
                                // A matching limit does not make unrelated variables flow-dependent.
                                condition
                                    .syntax()
                                    .descendants()
                                    .take(MAX_FLOW_TYPE_STEPS)
                                    .filter_map(AnyJsExpression::cast)
                                    .filter_map(|expression| {
                                        let AnyJsExpression::JsIdentifierExpression(identifier) =
                                            expression
                                        else {
                                            return None;
                                        };
                                        identifier.name().ok()
                                    })
                                    .collect()
                            })
                        });
                        relevant = subjects.is_some_and(|subjects| {
                            subjects.iter().any(|subject| {
                                self.js_info
                                    .semantic_model
                                    .binding(subject)
                                    .is_some_and(|candidate| candidate == *binding)
                            })
                        });
                    }
                }
            }
        }
        if !relevant {
            return None;
        }
        // Declaration queries can erase a cyclic object's shape. The raw baseline
        // still lets flow detect an impossible branch and return an unknown override.
        let reference = self
            .js_info
            .raw_binding_types
            .get(&binding.range())?
            .clone();
        let baseline = self.resolve(&reference);
        let Some(baseline) = self.flow_baseline(baseline) else {
            return Some(TypeData::Unknown);
        };
        let narrowed = self.solve_flow(graph, &seen, point, root.syntax(), binding, baseline);
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
        graph: &NarrowingFlowGraph,
        relevant: &[bool],
        point: FlowNodeId,
        root: &JsSyntaxNode,
        binding: &Binding,
        baseline: TypeData<'db>,
    ) -> TypeData<'db> {
        let mut successors = vec![Vec::new(); graph.nodes.len()];
        let mut queue = VecDeque::new();
        for (index, node) in graph.nodes.iter().enumerate() {
            if !relevant[index] {
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
        let mut states = vec![TypeData::NeverKeyword; graph.nodes.len()];
        let mut queued = vec![false; graph.nodes.len()];
        let mut remaining = MAX_FLOW_STEPS;
        while let Some(index) = queue.pop_front() {
            queued[index] = false;
            if remaining == 0 {
                return TypeData::Unknown;
            }
            remaining -= 1;
            let ty = match &graph.nodes[index] {
                FlowNode::Start => baseline,
                FlowNode::Join(predecessors) => TypeData::union_from_types(
                    self.db,
                    predecessors
                        .iter()
                        .map(|predecessor| states[*predecessor])
                        .collect(),
                ),
                FlowNode::Condition {
                    antecedent,
                    expression,
                    outcome,
                } => {
                    let incoming = states[*antecedent];
                    if incoming == TypeData::NeverKeyword {
                        incoming
                    } else if let Some(condition) = expression_at(root, *expression) {
                        self.narrow_condition(
                            binding,
                            incoming,
                            condition,
                            *outcome,
                            0,
                            &mut remaining,
                        )
                    } else {
                        incoming
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

    fn narrow_condition(
        &mut self,
        binding: &Binding,
        ty: TypeData<'db>,
        expression: AnyJsExpression,
        outcome: FlowOutcome,
        depth: usize,
        remaining: &mut usize,
    ) -> TypeData<'db> {
        if depth >= MAX_CONDITION_DEPTH {
            return ty;
        }
        let Some(step) = condition_step(expression, outcome, remaining) else {
            return TypeData::Unknown;
        };
        match step {
            ConditionStep::Unsupported => ty,
            ConditionStep::Guard {
                subject,
                guard,
                positive,
            } => {
                if !self
                    .js_info
                    .semantic_model
                    .binding(&subject)
                    .is_some_and(|candidate| candidate == *binding)
                {
                    return ty;
                }
                let Some(predicate) = self.guard_predicate(guard) else {
                    return ty;
                };
                narrow_type(self.db, ty, predicate, positive)
            }
            ConditionStep::Negated { argument, outcome } => {
                self.narrow_condition(binding, ty, argument, outcome, depth + 1, remaining)
            }
            ConditionStep::Logical {
                left,
                right,
                outcome,
                sequential,
            } => {
                let left = self.narrow_condition(binding, ty, left, outcome, depth + 1, remaining);
                let right = self.narrow_condition(
                    binding,
                    if sequential { left } else { ty },
                    right,
                    outcome,
                    depth + 1,
                    remaining,
                );
                if sequential {
                    right
                } else {
                    TypeData::union_from_types(self.db, vec![left, right])
                }
            }
        }
    }

    fn guard_predicate(&mut self, guard: SyntaxGuard) -> Option<NarrowingPredicate<'db>> {
        if !guard.is_applicable(&self.js_info.semantic_model) {
            return None;
        }
        Some(match guard {
            SyntaxGuard::Truthy => NarrowingPredicate::Truthy,
            SyntaxGuard::Nullish => NarrowingPredicate::Nullish,
            SyntaxGuard::Typeof(kind) => NarrowingPredicate::Typeof(kind),
            SyntaxGuard::Literal(literal) => {
                let ty = if matches!(literal, AnyJsLiteralExpression::JsNullLiteralExpression(_)) {
                    TypeData::Null
                } else {
                    let reference = self.js_info.raw_expressions.get(&literal.range())?.clone();
                    self.resolve(&reference)
                };
                NarrowingPredicate::Literal(ty)
            }
            SyntaxGuard::Undefined { strict, .. } => {
                if strict {
                    NarrowingPredicate::Literal(TypeData::Undefined)
                } else {
                    NarrowingPredicate::Nullish
                }
            }
        })
    }
}

//! Evaluates affected runtime operands by source occurrence, leaving structural
//! type resolution and its context-independent caches unchanged.

use super::ResolutionCtx;
use crate::db::queries::{ExpressionTypeInput, infer_flow_expression_type};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsAwaitExpression, JsBinaryExpression, JsBinaryOperator,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsComputedMemberExpression,
    JsConditionalExpression, JsIdentifierExpression, JsLogicalExpression, JsLogicalOperator,
    JsParenthesizedExpression, JsSequenceExpression, JsStaticMemberExpression,
};
use biome_js_type_info::{
    NarrowingPredicate, RawTypeData, RawTypeId, TypeReference, TypeofExpression,
    interned_types::{Literal, TypeData},
    narrow_type,
};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};

/// Limits how many [evaluated ancestors](AnyFlowExpression::evaluated_ancestors)
/// an evaluated expression may have.
///
/// Evaluating an expression queries its operands, and nested tracked queries
/// run on the call stack. Each operand has one more evaluated ancestor than the
/// expression that queries it, so this bounds the depth of that recursion.
const MAX_FLOW_EXPRESSION_DEPTH: usize = 48;

declare_node_union! {
    /// Expressions whose type at a source occurrence can follow a refined
    /// operand.
    ///
    /// [`ResolutionCtx::resolve_flow_expression`] evaluates exactly the members
    /// for which [`Self::is_evaluated`] returns true; every other expression
    /// keeps its raw type.
    pub(super) AnyFlowExpression = JsIdentifierExpression
        | JsParenthesizedExpression
        | JsStaticMemberExpression
        | JsComputedMemberExpression
        | JsLogicalExpression
        | JsConditionalExpression
        | JsCallExpression
        | JsAwaitExpression
        | JsSequenceExpression
        | JsBinaryExpression
}

impl AnyFlowExpression {
    /// Returns whether flow evaluation supports this expression's shape.
    ///
    /// Among binary expressions only addition is evaluated, and calls are
    /// evaluated only without explicit type arguments or spread arguments.
    pub(super) fn is_evaluated(&self) -> bool {
        match self {
            Self::JsBinaryExpression(binary) => {
                binary.operator().ok() == Some(JsBinaryOperator::Plus)
            }
            Self::JsCallExpression(call) => {
                call.type_arguments().is_none()
                    && call.arguments().is_ok_and(|arguments| {
                        arguments.args().iter().all(|argument| {
                            matches!(argument, Ok(AnyJsCallArgument::AnyJsExpression(_)))
                        })
                    })
            }
            Self::JsIdentifierExpression(_)
            | Self::JsParenthesizedExpression(_)
            | Self::JsStaticMemberExpression(_)
            | Self::JsComputedMemberExpression(_)
            | Self::JsLogicalExpression(_)
            | Self::JsConditionalExpression(_)
            | Self::JsAwaitExpression(_)
            | Self::JsSequenceExpression(_) => true,
        }
    }

    /// Returns the enclosing expressions whose evaluation queries this one,
    /// directly or through other operands, innermost first.
    ///
    /// A refined type can only propagate along this chain, and its length
    /// bounds the operand recursion that can reach this expression.
    pub(super) fn evaluated_ancestors(&self) -> impl Iterator<Item = Self> + use<> {
        std::iter::successors(self.evaluated_parent(), Self::evaluated_parent)
    }

    /// Returns the parent expression whose evaluation queries this one as an
    /// operand.
    ///
    /// An operand is a direct child or a call argument. A sequence reads only
    /// its right side; every other evaluated expression reads all of its operands.
    fn evaluated_parent(&self) -> Option<Self> {
        let mut parent = self.syntax().parent()?;
        while JsCallArguments::can_cast(parent.kind())
            || JsCallArgumentList::can_cast(parent.kind())
        {
            parent = parent.parent()?;
        }
        let parent = Self::cast(parent)?;
        let reads_operand = match &parent {
            Self::JsSequenceExpression(sequence) => sequence
                .right()
                .is_ok_and(|right| right.syntax() == self.syntax()),
            Self::JsIdentifierExpression(_)
            | Self::JsParenthesizedExpression(_)
            | Self::JsStaticMemberExpression(_)
            | Self::JsComputedMemberExpression(_)
            | Self::JsLogicalExpression(_)
            | Self::JsConditionalExpression(_)
            | Self::JsCallExpression(_)
            | Self::JsAwaitExpression(_)
            | Self::JsBinaryExpression(_) => true,
        };
        (reads_operand && parent.is_evaluated()).then_some(parent)
    }
}

impl<'db> ResolutionCtx<'db, '_> {
    /// Infers `expression` at its source occurrence from refined operands.
    ///
    /// Returns `None` when no operand is refined, when [`AnyFlowExpression::is_evaluated`]
    /// rejects the expression, or when it has more than
    /// [`MAX_FLOW_EXPRESSION_DEPTH`] evaluated ancestors.
    pub(super) fn resolve_flow_expression(
        &mut self,
        expression: &AnyFlowExpression,
    ) -> Option<TypeData<'db>> {
        if !expression.is_evaluated()
            || expression
                .evaluated_ancestors()
                .nth(MAX_FLOW_EXPRESSION_DEPTH)
                .is_some()
        {
            return None;
        }
        match expression {
            AnyFlowExpression::JsIdentifierExpression(identifier) => {
                self.narrow_reference(identifier)
            }
            AnyFlowExpression::JsParenthesizedExpression(parenthesized) => {
                self.flow_operand(&parenthesized.expression().ok()?)
            }
            AnyFlowExpression::JsStaticMemberExpression(member) => {
                let object = self.flow_operand(&member.object().ok()?)?;
                let name = member.member().ok()?.value_token().ok()?;
                let result = self
                    .resolve_static_member_expression(object, name.text_trimmed())
                    .unwrap_or(TypeData::Unknown);
                Some(if member.is_optional_chain() {
                    self.optional_chain_result(object, result)
                } else {
                    result
                })
            }
            AnyFlowExpression::JsComputedMemberExpression(member) => {
                let object_expression = member.object().ok()?;
                let member_expression = member.member().ok()?;
                let object = self.flow_operand(&object_expression);
                let key = self.flow_operand(&member_expression);
                if object.is_none() && key.is_none() {
                    return None;
                }
                let object = object.or_else(|| self.raw_operand(&object_expression))?;
                let key = key.or_else(|| self.raw_operand(&member_expression))?;
                let result = self
                    .resolve_computed_member_expression(object, key)
                    .unwrap_or(TypeData::Unknown);
                Some(if member.is_optional_chain() {
                    self.optional_chain_result(object, result)
                } else {
                    result
                })
            }
            AnyFlowExpression::JsLogicalExpression(logical) => {
                let left_expression = logical.left().ok()?;
                let right_expression = logical.right().ok()?;
                let left = self.flow_operand(&left_expression);
                let right = self.flow_operand(&right_expression);
                if left.is_none() && right.is_none() {
                    return None;
                }
                let left = left.or_else(|| self.raw_operand(&left_expression))?;
                let (predicate, retained) = match logical.operator().ok()? {
                    JsLogicalOperator::LogicalAnd => (NarrowingPredicate::Truthy, false),
                    JsLogicalOperator::LogicalOr => (NarrowingPredicate::Truthy, true),
                    JsLogicalOperator::NullishCoalescing => (NarrowingPredicate::Nullish, false),
                };
                let continuing = narrow_type(self.db, left, predicate, !retained);
                if continuing == TypeData::NeverKeyword {
                    return Some(left);
                }
                let right = right.or_else(|| self.raw_operand(&right_expression))?;
                let retained = narrow_type(self.db, left, predicate, retained);
                Some(TypeData::union_from_types(self.db, vec![retained, right]))
            }
            AnyFlowExpression::JsConditionalExpression(conditional) => {
                let test_expression = conditional.test().ok()?;
                let consequent_expression = conditional.consequent().ok()?;
                let alternate_expression = conditional.alternate().ok()?;
                let test = self.flow_operand(&test_expression);
                let consequent = self.flow_operand(&consequent_expression);
                let alternate = self.flow_operand(&alternate_expression);
                if test.is_none() && consequent.is_none() && alternate.is_none() {
                    return None;
                }
                let test = test.or_else(|| self.raw_operand(&test_expression))?;
                if narrow_type(self.db, test, NarrowingPredicate::Truthy, true)
                    == TypeData::NeverKeyword
                {
                    return alternate.or_else(|| self.raw_operand(&alternate_expression));
                }
                if narrow_type(self.db, test, NarrowingPredicate::Truthy, false)
                    == TypeData::NeverKeyword
                {
                    return consequent.or_else(|| self.raw_operand(&consequent_expression));
                }
                let consequent = consequent.or_else(|| self.raw_operand(&consequent_expression))?;
                let alternate = alternate.or_else(|| self.raw_operand(&alternate_expression))?;
                Some(TypeData::union_from_types(
                    self.db,
                    vec![consequent, alternate],
                ))
            }
            AnyFlowExpression::JsCallExpression(call) => {
                let callee_expression = call.callee().ok()?;
                let callee = self.flow_operand(&callee_expression);
                let mut overrides = Vec::new();
                for argument in call.arguments().ok()?.args().iter() {
                    let AnyJsCallArgument::AnyJsExpression(argument) = argument.ok()? else {
                        return None;
                    };
                    overrides.push(self.flow_operand(&argument));
                }
                if callee.is_none() && overrides.iter().all(Option::is_none) {
                    return None;
                }
                let TypeReference::Resolved(RawTypeId::Local(id)) =
                    self.js_info.raw_expressions.get(&call.range())?
                else {
                    return None;
                };
                let RawTypeData::TypeofExpression(raw) = self.js_info.raw_types.get(id.index())?
                else {
                    return None;
                };
                let TypeofExpression::Call(raw) = raw.as_ref() else {
                    return None;
                };
                let arguments = raw.arguments.clone();
                let raw_callee = raw.callee.clone();
                let callee = callee.unwrap_or_else(|| self.resolve(&raw_callee));
                let result =
                    self.resolve_call_expression_with_overrides(callee, &arguments, &overrides);
                Some(if call.is_optional_chain() {
                    self.optional_chain_result(callee, result)
                } else {
                    result
                })
            }
            AnyFlowExpression::JsAwaitExpression(await_expression) => {
                let argument = self.flow_operand(&await_expression.argument().ok()?)?;
                Some(
                    self.resolve_await_expression(argument)
                        .unwrap_or(TypeData::Unknown),
                )
            }
            AnyFlowExpression::JsSequenceExpression(sequence) => {
                self.flow_operand(&sequence.right().ok()?)
            }
            AnyFlowExpression::JsBinaryExpression(binary) => {
                let left_expression = binary.left().ok()?;
                let right_expression = binary.right().ok()?;
                let left = self.flow_operand(&left_expression);
                let right = self.flow_operand(&right_expression);
                if left.is_none() && right.is_none() {
                    return None;
                }
                let left = left.or_else(|| self.raw_operand(&left_expression))?;
                let right = right.or_else(|| self.raw_operand(&right_expression))?;
                if !self.primitive_addition_operand(left) || !self.primitive_addition_operand(right)
                {
                    return Some(TypeData::Unknown);
                }
                Some(
                    self.resolve_addition_expression(left, right)
                        .unwrap_or(TypeData::Unknown),
                )
            }
        }
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Only primitive addition coercions are modeled."
    )]
    fn primitive_addition_operand(&self, ty: TypeData<'db>) -> bool {
        match ty {
            TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Undefined => true,
            TypeData::Literal(literal) => matches!(
                literal.literal(self.db),
                Literal::BigInt(_)
                    | Literal::Boolean(_)
                    | Literal::Number(_)
                    | Literal::String(_)
                    | Literal::Template(_)
            ),
            _ => false,
        }
    }

    fn flow_operand(&self, expression: &AnyJsExpression) -> Option<TypeData<'db>> {
        infer_flow_expression_type(
            self.db,
            ExpressionTypeInput::new(self.db, self.module, expression.range()),
        )
    }

    fn raw_operand(&mut self, expression: &AnyJsExpression) -> Option<TypeData<'db>> {
        let reference = self
            .js_info
            .raw_expressions
            .get(&expression.range())?
            .clone();
        Some(self.resolve(&reference))
    }
}

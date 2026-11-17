//! Evaluates affected runtime operands by source occurrence, leaving structural
//! type resolution and its context-independent caches unchanged.

use super::ResolutionCtx;
use super::flow::MAX_FLOW_TYPE_STEPS;
use crate::JsModuleInfo;
use crate::db::queries::{ExpressionTypeInput, infer_flow_expression_type};
use crate::js_module_info::flow::AnyFlowExpression;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsName, JsLogicalOperator, JsStaticMemberExpression,
};
use biome_js_type_info::{
    NarrowingPredicate, RawTypeData, RawTypeId, TypeReference, TypeofExpression, falsy_values,
    interned_types::{Literal, TypeData},
    narrow_type,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

/// Limits how many [evaluated ancestors](AnyFlowExpression::evaluated_ancestors)
/// an evaluated expression may have.
///
/// Evaluating an expression queries its operands, and nested tracked queries
/// run on the call stack. Each operand has one more evaluated ancestor than the
/// expression that queries it, so this bounds the depth of that recursion.
const MAX_FLOW_EXPRESSION_DEPTH: usize = 48;

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
                let object = self.flow_operand(&member.object().ok()?);
                // The declared type of a property path is the type of this
                // access, computed on the object's narrowed type when the
                // object has one.
                let declared = match object {
                    Some(object) => {
                        let js_info = self.js_info;
                        let name = member.member().ok()?;
                        let token = name.value_token().ok()?;
                        // Class members store a private name with its `#`, which
                        // its token lacks.
                        let name = if matches!(name, AnyJsName::JsPrivateName(_)) {
                            collected_member_name(js_info, member)?
                        } else {
                            token.text_trimmed()
                        };
                        let result = self
                            .resolve_static_member_expression(object, name)
                            .unwrap_or(TypeData::Unknown);
                        if member.is_optional_chain() {
                            self.optional_chain_result(object, result)
                        } else {
                            result
                        }
                    }
                    None if self.js_info.flow.read(member.range()).is_some() => {
                        self.raw_operand(&AnyJsExpression::from(member.clone()))?
                    }
                    None => return None,
                };
                match self.narrow_path(&AnyJsExpression::from(member.clone()), declared) {
                    Some(narrowed) => Some(narrowed),
                    None => object.is_some().then_some(declared),
                }
            }
            AnyFlowExpression::JsComputedMemberExpression(member) => {
                let object_expression = member.object().ok()?;
                let member_expression = member.member().ok()?;
                let object = self.flow_operand(&object_expression);
                let key = self.flow_operand(&member_expression);
                // The declared type of a property path with a literal key is
                // the type of this access, computed on the narrowed types of
                // the object and the key when they have them.
                let declared = if object.is_none() && key.is_none() {
                    self.js_info.flow.read(member.range())?;
                    self.raw_operand(&AnyJsExpression::from(member.clone()))?
                } else {
                    let object = object.or_else(|| self.raw_operand(&object_expression))?;
                    let key = key.or_else(|| self.raw_operand(&member_expression))?;
                    let result = self
                        .resolve_computed_member_expression(object, key)
                        .unwrap_or(TypeData::Unknown);
                    if member.is_optional_chain() {
                        self.optional_chain_result(object, result)
                    } else {
                        result
                    }
                };
                match self.narrow_path(&AnyJsExpression::from(member.clone()), declared) {
                    Some(narrowed) => Some(narrowed),
                    None => (object.is_some() || key.is_some()).then_some(declared),
                }
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
                let operator = logical.operator().ok()?;
                let (predicate, retained) = match operator {
                    JsLogicalOperator::LogicalAnd => (NarrowingPredicate::Truthy, false),
                    JsLogicalOperator::LogicalOr => (NarrowingPredicate::Truthy, true),
                    JsLogicalOperator::NullishCoalescing => (NarrowingPredicate::Nullish, false),
                };
                let continuing = narrow_type(
                    self.db,
                    left,
                    predicate,
                    !retained,
                    self.narrowing_resolver(),
                );
                if continuing == TypeData::NeverKeyword {
                    return Some(left);
                }
                let right = right.or_else(|| self.raw_operand(&right_expression))?;
                // `&&` returns its left operand only when the operand is falsy,
                // so a `string` or `bigint` operand contributes `""` or `0n`,
                // although a read in a falsy branch keeps the broad type.
                let retained = if operator == JsLogicalOperator::LogicalAnd {
                    falsy_values(self.db, left, self.narrowing_resolver())
                } else {
                    narrow_type(
                        self.db,
                        left,
                        predicate,
                        retained,
                        self.narrowing_resolver(),
                    )
                };
                Some(TypeData::union_from_types(self.db, vec![retained, right]))
            }
            AnyFlowExpression::JsConditionalExpression(conditional) => {
                let consequent_expression = conditional.consequent().ok()?;
                let alternate_expression = conditional.alternate().ok()?;
                let consequent = self.flow_operand(&consequent_expression);
                let alternate = self.flow_operand(&alternate_expression);
                // The test doesn't change the type of the expression, so
                // when neither branch is narrowed, the expression has the type
                // that inference gives it without flow.
                if consequent.is_none() && alternate.is_none() {
                    return None;
                }
                let consequent = consequent.or_else(|| self.raw_operand(&consequent_expression))?;
                let alternate = alternate.or_else(|| self.raw_operand(&alternate_expression))?;
                let test_expression = conditional.test().ok()?;
                let consequent = self.conditional_branch(
                    &test_expression,
                    &consequent_expression,
                    consequent,
                    true,
                );
                let alternate = self.conditional_branch(
                    &test_expression,
                    &alternate_expression,
                    alternate,
                    false,
                );
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
                let result = self.resolve_call_expression_with_overrides(
                    callee,
                    &arguments,
                    &overrides,
                    &raw_callee,
                );
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

    /// Returns the type that `branch` adds to the type of a conditional
    /// expression with the test `test`. `ty` is the type of `branch`, and
    /// `positive` is true for the branch that runs when the test is truthy.
    ///
    /// TypeScript types a conditional expression as the union of its
    /// branches, whatever the test, so this returns `ty` even when the test
    /// can never select `branch`. With `x: { a: number }`, `x ? 1 : "s"` is
    /// `1 | "s"`.
    ///
    /// Returns `never`, which adds nothing to the union, when `branch` only
    /// reads the variable that `test` reads, Biome reads `branch` as Unknown,
    /// and the test leaves the variable no value in that branch. TypeScript
    /// types such a read as `never`, so with the same `x`, `x ? 1 : x` is `1`.
    fn conditional_branch(
        &mut self,
        test: &AnyJsExpression,
        branch: &AnyJsExpression,
        ty: TypeData<'db>,
        positive: bool,
    ) -> TypeData<'db> {
        if ty != TypeData::Unknown || !reads_same_variable(test, branch) {
            return ty;
        }
        let Some(test) = self.flow_operand(test).or_else(|| self.raw_operand(test)) else {
            return ty;
        };
        let narrowed = narrow_type(
            self.db,
            test,
            NarrowingPredicate::Truthy,
            positive,
            self.narrowing_resolver(),
        );
        if narrowed == TypeData::NeverKeyword {
            narrowed
        } else {
            ty
        }
    }

    /// Returns whether `ty` is a primitive or a union of primitives, so adding
    /// it cannot run a user-defined `valueOf` or `toString` method.
    ///
    /// Returns false for an empty union and when the union members, nested
    /// unions included, exceed [`MAX_FLOW_TYPE_STEPS`].
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Only primitive addition coercions are modeled."
    )]
    fn primitive_addition_operand(&self, ty: TypeData<'db>) -> bool {
        let mut pending = vec![ty];
        for _ in 0..MAX_FLOW_TYPE_STEPS {
            let Some(ty) = pending.pop() else {
                return true;
            };
            let primitive = match ty {
                TypeData::Union(union) => {
                    let types = union.types(self.db);
                    pending.extend(types.iter().copied());
                    !types.is_empty()
                }
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
            };
            if !primitive {
                return false;
            }
        }
        false
    }

    fn flow_operand(&mut self, expression: &AnyJsExpression) -> Option<TypeData<'db>> {
        self.flow_operand_at(expression.range())
    }

    /// Returns the flow-sensitive type of the expression at `range`, or
    /// `None` when it is not a flow candidate or narrowing leaves its type
    /// unchanged.
    ///
    /// While the arguments of a call point are read (see
    /// [`ResolutionCtx::calls_before`]), the type is evaluated in this
    /// context without caching.
    pub(super) fn flow_operand_at(&mut self, range: TextRange) -> Option<TypeData<'db>> {
        if self.calls_before.is_some() {
            return self.uncached_flow_expression_type(range);
        }
        if self.evaluates_flow_locally() {
            return self.local_flow_expression_type(range);
        }
        infer_flow_expression_type(
            self.db,
            ExpressionTypeInput::new(self.db, self.module, range),
        )
    }

    pub(super) fn raw_operand(&mut self, expression: &AnyJsExpression) -> Option<TypeData<'db>> {
        self.raw_operand_at(expression.range())
    }

    /// Returns the type of the expression at `range` without narrowing, or
    /// `None` when collection recorded no expression there.
    pub(super) fn raw_operand_at(&mut self, range: TextRange) -> Option<TypeData<'db>> {
        let reference = self.js_info.raw_expressions.get(&range)?.clone();
        Some(self.resolve(&reference))
    }
}

/// Returns whether `test` and `branch`, without parentheses, are both a read of
/// one variable, as in `x ? 1 : x`. Both are in one expression and neither is
/// inside a function, so the same name means the same variable.
fn reads_same_variable(test: &AnyJsExpression, branch: &AnyJsExpression) -> bool {
    let (
        AnyJsExpression::JsIdentifierExpression(test),
        AnyJsExpression::JsIdentifierExpression(branch),
    ) = (
        test.clone().omit_parentheses(),
        branch.clone().omit_parentheses(),
    )
    else {
        return false;
    };
    let (Ok(test), Ok(branch)) = (test.name(), branch.name()) else {
        return false;
    };
    let (Ok(test), Ok(branch)) = (test.value_token(), branch.value_token()) else {
        return false;
    };
    test.text_trimmed() == branch.text_trimmed()
}

/// Returns the member name that local inference collected for `member`, or
/// `None` when it collected no static member expression for `member`. The
/// name spells a private name with its `#`, as class members store it.
pub(super) fn collected_member_name<'a>(
    js_info: &'a JsModuleInfo,
    member: &JsStaticMemberExpression,
) -> Option<&'a str> {
    let TypeReference::Resolved(RawTypeId::Local(id)) =
        js_info.raw_expressions.get(&member.range())?
    else {
        return None;
    };
    let RawTypeData::TypeofExpression(raw) = js_info.raw_types.get(id.index())? else {
        return None;
    };
    let (TypeofExpression::StaticMember(expression)
    | TypeofExpression::OptionalChainStaticMember(expression)) = raw.as_ref()
    else {
        return None;
    };
    Some(expression.member.text())
}

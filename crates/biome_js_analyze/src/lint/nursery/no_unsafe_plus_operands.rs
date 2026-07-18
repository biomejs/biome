use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsAssignmentExpression,
    JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator, JsParenthesizedExpression,
};
use biome_js_type_info::InferredType;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_unsafe_plus_operands::NoUnsafePlusOperandsOptions;

declare_lint_rule! {
    /// Disallow `+` operations with operands that are known to be unsafe.
    ///
    /// This rule uses type information to report `+` and `+=` operations that are
    /// very likely mistakes at runtime, such as mixing `number` with `bigint` or
    /// using object-like, `symbol`, `unknown`, or `never` values as operands.
    ///
    /// This port intentionally does not support the original rule's options.
    /// It keeps the upstream default behavior for no-option usage and always checks
    /// compound `+=` assignments.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-bigint-plus-number.ts
    /// const value = 1n + 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-number-plus-bigint.ts
    /// const value = 1 + 1n;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-bigint-add-assign.ts
    /// declare let count: number;
    /// count += 1n;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-number-plus-number.ts
    /// const sum = 1 + 2;
    /// ```
    ///
    /// ```ts,file=valid-string-plus-number.ts
    /// const message = "value: " + 1;
    /// ```
    ///
    /// ```ts,file=valid-bigint-add-assign.ts
    /// let total = 1n;
    /// total += 2n;
    /// ```
    pub NoUnsafePlusOperands {
        version: "2.4.10",
        name: "noUnsafePlusOperands",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("restrict-plus-operands").same()],
        recommended: false,
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub NoUnsafePlusOperandsQuery = JsBinaryExpression | JsAssignmentExpression
}

pub enum NoUnsafePlusOperandsState {
    InvalidOperand {
        range: TextRange,
    },
    MixedBigIntAndNumber {
        range: TextRange,
        left_range: TextRange,
        right_range: TextRange,
    },
}

struct OperandInfo<'db> {
    range: TextRange,
    ty: InferredType<'db>,
}

impl Rule for NoUnsafePlusOperands {
    type Query = Typed<NoUnsafePlusOperandsQuery>;
    type State = NoUnsafePlusOperandsState;
    type Signals = Option<Self::State>;
    type Options = NoUnsafePlusOperandsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            NoUnsafePlusOperandsQuery::JsBinaryExpression(binary) => {
                run_binary(ctx, binary).flatten()
            }
            NoUnsafePlusOperandsQuery::JsAssignmentExpression(assignment) => {
                run_assignment(ctx, assignment).flatten()
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match state {
            NoUnsafePlusOperandsState::InvalidOperand { range } => {
                let ty = type_for_range(ctx, *range)?.plus_operand_description();

                Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "Invalid operand for a "<Emphasis>"+"</Emphasis>" operation: "<Emphasis>{ty}</Emphasis>"."
                    },
                )
                .detail(node.range(), markup! {
                    "This operation uses a value whose type is not safely supported here."
                })
                .note(markup! {
                    "Avoid object-like, "<Emphasis>"symbol"</Emphasis>", "<Emphasis>"unknown"</Emphasis>", and "<Emphasis>"never"</Emphasis>" operands when using "<Emphasis>"+"</Emphasis>" or "<Emphasis>"+="</Emphasis>"."
                }),
            )
            }
            NoUnsafePlusOperandsState::MixedBigIntAndNumber {
                range,
                left_range,
                right_range,
            } => {
                let left = type_for_range(ctx, *left_range)?.plus_operand_description();
                let right = type_for_range(ctx, *right_range)?.plus_operand_description();

                Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "Numeric "<Emphasis>"+"</Emphasis>" operations must use either two "<Emphasis>"bigint"</Emphasis>" values or two "<Emphasis>"number"</Emphasis>" values."
                    },
                )
                .detail(range, markup! {
                    "This operation mixes "<Emphasis>{left}</Emphasis>" with "<Emphasis>{right}</Emphasis>"."
                })
                .note(markup! {
                    "Convert one side so both operands use the same numeric type before applying "<Emphasis>"+"</Emphasis>" or "<Emphasis>"+="</Emphasis>"."
                }),
            )
            }
        }
    }
}

fn run_binary(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    binary: &JsBinaryExpression,
) -> Option<Option<NoUnsafePlusOperandsState>> {
    if binary.operator() != Ok(JsBinaryOperator::Plus) || has_parent_plus_expression(binary) {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    Some(analyze_binary_operands(ctx, binary.range(), left, right))
}

fn run_assignment(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
) -> Option<Option<NoUnsafePlusOperandsState>> {
    if assignment.operator() != Ok(JsAssignmentOperator::AddAssign) {
        return None;
    }

    let left = assignment.left().ok()?;
    let right = assignment.right().ok()?;
    let left_ty = type_of_assignment_target(ctx, assignment, &left)?;

    let right_ty = ctx.inferred_type_of_expression(&right)?;

    let left = OperandInfo {
        range: left.range(),
        ty: left_ty,
    };
    let right = OperandInfo {
        range: right.range(),
        ty: right_ty,
    };

    Some(analyze_pair(assignment.range(), &left, &right))
}

fn type_of_assignment_target<'a>(
    ctx: &'a RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
    left: &AnyJsAssignmentPattern,
) -> Option<InferredType<'a>> {
    match left {
        AnyJsAssignmentPattern::AnyJsAssignment(assignment_target) => {
            type_of_assignment(ctx, assignment, assignment_target)
        }
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
        | AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => None,
    }
}

fn type_of_assignment<'a>(
    ctx: &'a RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
    target: &AnyJsAssignment,
) -> Option<InferredType<'a>> {
    match target {
        AnyJsAssignment::JsIdentifierAssignment(identifier) => {
            let name = identifier.name_token().ok()?;
            ctx.inferred_type_of_named_value(assignment.range(), name.text_trimmed())
        }
        AnyJsAssignment::JsParenthesizedAssignment(parenthesized) => {
            type_of_assignment(ctx, assignment, &parenthesized.assignment().ok()?)
        }
        AnyJsAssignment::TsAsAssignment(ts_as) => {
            type_of_assignment(ctx, assignment, &ts_as.assignment().ok()?)
        }
        AnyJsAssignment::TsNonNullAssertionAssignment(non_null) => {
            type_of_assignment(ctx, assignment, &non_null.assignment().ok()?)
        }
        AnyJsAssignment::TsSatisfiesAssignment(satisfies) => {
            type_of_assignment(ctx, assignment, &satisfies.assignment().ok()?)
        }
        AnyJsAssignment::TsTypeAssertionAssignment(assertion) => {
            type_of_assignment(ctx, assignment, &assertion.assignment().ok()?)
        }
        AnyJsAssignment::JsBogusAssignment(_)
        | AnyJsAssignment::JsComputedMemberAssignment(_)
        | AnyJsAssignment::JsStaticMemberAssignment(_) => None,
    }
}

fn has_parent_plus_expression(node: &JsBinaryExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
        .and_then(JsBinaryExpression::cast)
        .is_some_and(|parent| parent.operator() == Ok(JsBinaryOperator::Plus))
}

fn analyze_binary_operands(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    range: TextRange,
    left: AnyJsExpression,
    right: AnyJsExpression,
) -> Option<NoUnsafePlusOperandsState> {
    let mut first_number_range = None;
    let mut first_bigint_range = None;

    analyze_expression(ctx, left, &mut first_number_range, &mut first_bigint_range)?;
    analyze_expression(ctx, right, &mut first_number_range, &mut first_bigint_range)?;

    if let (Some(left_range), Some(right_range)) = (first_number_range, first_bigint_range) {
        return Some(NoUnsafePlusOperandsState::MixedBigIntAndNumber {
            range,
            left_range,
            right_range,
        });
    }

    None
}

fn analyze_expression(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    expression: AnyJsExpression,
    first_number_range: &mut Option<TextRange>,
    first_bigint_range: &mut Option<TextRange>,
) -> Option<Option<NoUnsafePlusOperandsState>> {
    let expression = expression.omit_parentheses();

    if let AnyJsExpression::JsBinaryExpression(binary) = &expression
        && binary.operator().ok()? == JsBinaryOperator::Plus
    {
        if let Some(state) = analyze_expression(
            ctx,
            binary.left().ok()?,
            first_number_range,
            first_bigint_range,
        )? {
            return Some(Some(state));
        }

        if let Some(state) = analyze_expression(
            ctx,
            binary.right().ok()?,
            first_number_range,
            first_bigint_range,
        )? {
            return Some(Some(state));
        }

        return Some(None);
    }

    let operand = OperandInfo {
        range: expression.range(),
        ty: ctx.inferred_type_of_expression(&expression)?,
    };

    if operand.ty.has_invalid_plus_operand_variant() {
        return Some(Some(NoUnsafePlusOperandsState::InvalidOperand {
            range: operand.range,
        }));
    }

    if operand.ty.has_number_like_variant() {
        first_number_range.get_or_insert(operand.range);
    }
    if operand.ty.has_bigint_like_variant() {
        first_bigint_range.get_or_insert(operand.range);
    }

    Some(None)
}

fn analyze_pair(
    range: TextRange,
    left: &OperandInfo,
    right: &OperandInfo,
) -> Option<NoUnsafePlusOperandsState> {
    if left.ty.has_invalid_plus_operand_variant() {
        return Some(NoUnsafePlusOperandsState::InvalidOperand { range: left.range });
    }

    if right.ty.has_invalid_plus_operand_variant() {
        return Some(NoUnsafePlusOperandsState::InvalidOperand { range: right.range });
    }

    if (left.ty.has_number_like_variant() && right.ty.has_bigint_like_variant())
        || (left.ty.has_bigint_like_variant() && right.ty.has_number_like_variant())
    {
        return Some(NoUnsafePlusOperandsState::MixedBigIntAndNumber {
            range,
            left_range: left.range,
            right_range: right.range,
        });
    }

    None
}

fn type_for_range<'a>(
    ctx: &'a RuleContext<NoUnsafePlusOperands>,
    range: TextRange,
) -> Option<InferredType<'a>> {
    match ctx.query() {
        NoUnsafePlusOperandsQuery::JsBinaryExpression(binary) => {
            type_for_range_in_binary(ctx, binary, range)
        }
        NoUnsafePlusOperandsQuery::JsAssignmentExpression(assignment) => {
            type_for_range_in_assignment(ctx, assignment, range)
        }
    }
}

fn type_for_range_in_binary<'a>(
    ctx: &'a RuleContext<NoUnsafePlusOperands>,
    binary: &JsBinaryExpression,
    range: TextRange,
) -> Option<InferredType<'a>> {
    type_for_range_in_expression(ctx, binary.left().ok()?, range)
        .or_else(|| type_for_range_in_expression(ctx, binary.right().ok()?, range))
}

fn type_for_range_in_expression<'a>(
    ctx: &'a RuleContext<NoUnsafePlusOperands>,
    expression: AnyJsExpression,
    range: TextRange,
) -> Option<InferredType<'a>> {
    let expression = expression.omit_parentheses();

    if expression.range() == range {
        return ctx.inferred_type_of_expression(&expression);
    }

    if let AnyJsExpression::JsBinaryExpression(binary) = &expression
        && binary.operator().ok()? == JsBinaryOperator::Plus
    {
        return type_for_range_in_expression(ctx, binary.left().ok()?, range)
            .or_else(|| type_for_range_in_expression(ctx, binary.right().ok()?, range));
    }

    None
}

fn type_for_range_in_assignment<'a>(
    ctx: &'a RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
    range: TextRange,
) -> Option<InferredType<'a>> {
    let left = assignment.left().ok()?;
    let right = assignment.right().ok()?;

    if left.range() == range {
        return type_of_assignment_target(ctx, assignment, &left);
    }

    if right.range() == range {
        return ctx.inferred_type_of_expression(&right);
    }

    None
}

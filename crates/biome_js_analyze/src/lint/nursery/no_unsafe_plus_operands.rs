use std::io;

use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{fmt, markup};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsAssignmentExpression,
    JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator, JsParenthesizedExpression,
};
use biome_js_type_info::{Literal, Type, TypeData};
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

struct OperandInfo {
    range: TextRange,
    ty: Type,
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
                let ty = type_for_range(ctx, *range)?;

                Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "Invalid operand for a "<Emphasis>"+"</Emphasis>" operation: "<Emphasis>{TypeDescription(&ty)}</Emphasis>"."
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
                let left = type_for_range(ctx, *left_range)?;
                let right = type_for_range(ctx, *right_range)?;

                Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "Numeric "<Emphasis>"+"</Emphasis>" operations must use either two "<Emphasis>"bigint"</Emphasis>" values or two "<Emphasis>"number"</Emphasis>" values."
                    },
                )
                .detail(range, markup! {
                    "This operation mixes "<Emphasis>{TypeDescription(&left)}</Emphasis>" with "<Emphasis>{TypeDescription(&right)}</Emphasis>"."
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

    let right_ty = ctx.type_of_expression(&right);

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

fn type_of_assignment_target(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
    left: &AnyJsAssignmentPattern,
) -> Option<Type> {
    match left {
        AnyJsAssignmentPattern::AnyJsAssignment(assignment_target) => {
            type_of_assignment(ctx, assignment, assignment_target)
        }
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
        | AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => None,
    }
}

fn type_of_assignment(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
    target: &AnyJsAssignment,
) -> Option<Type> {
    match target {
        AnyJsAssignment::JsIdentifierAssignment(identifier) => {
            let name = identifier.name_token().ok()?;
            Some(ctx.type_of_named_value(assignment.range(), name.text_trimmed()))
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

fn has_invalid_variant(ty: &Type) -> bool {
    if ty.is_union() {
        ty.flattened_union_variants()
            .any(|variant| is_invalid_variant(&variant))
    } else {
        is_invalid_variant(ty)
    }
}

fn is_invalid_variant(ty: &Type) -> bool {
    let Some(data) = ty.resolved_data().map(|resolved| resolved.as_raw_data()) else {
        return true;
    };

    match data {
        TypeData::NeverKeyword | TypeData::Symbol | TypeData::UnknownKeyword => true,
        TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::Object(_)),
        TypeData::Reference(reference) => ty
            .resolve(reference)
            .is_some_and(|resolved| is_invalid_variant(&resolved)),
        TypeData::Intersection(intersection) => intersection.types().iter().all(|reference| {
            ty.resolve(reference)
                .is_some_and(|ty| is_object_like_variant(&ty))
        }),
        _ => is_object_like_variant(ty),
    }
}

fn is_object_like_variant(ty: &Type) -> bool {
    let Some(data) = ty.resolved_data().map(|resolved| resolved.as_raw_data()) else {
        return true;
    };

    match data {
        TypeData::Class(_)
        | TypeData::Constructor(_)
        | TypeData::Function(_)
        | TypeData::Interface(_)
        | TypeData::Module(_)
        | TypeData::Namespace(_)
        | TypeData::Object(_)
        | TypeData::ObjectKeyword
        | TypeData::Tuple(_) => true,
        TypeData::Reference(reference) => ty
            .resolve(reference)
            .is_some_and(|resolved| is_object_like_variant(&resolved)),
        TypeData::InstanceOf(instance) => ty
            .resolve(&instance.ty)
            .is_some_and(|resolved| is_object_like_variant(&resolved)),
        _ => false,
    }
}

fn has_number_like(ty: &Type) -> bool {
    ty.is_number_or_number_literal()
        || ty.has_variant(|variant| variant.is_number_or_number_literal())
}

fn has_bigint_like(ty: &Type) -> bool {
    is_bigint_like(ty) || ty.has_variant(|variant| is_bigint_like(&variant))
}

fn is_bigint_like(ty: &Type) -> bool {
    match ty.resolved_data().map(|resolved| resolved.as_raw_data()) {
        Some(TypeData::BigInt) => true,
        Some(TypeData::Literal(literal)) => matches!(literal.as_ref(), Literal::BigInt(_)),
        _ => false,
    }
}

struct TypeDescription<'a>(&'a Type);

impl fmt::Display for TypeDescription<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> io::Result<()> {
        self.write_type(self.0, fmt)
    }
}

impl TypeDescription<'_> {
    fn write_type(&self, ty: &Type, fmt: &mut fmt::Formatter) -> io::Result<()> {
        if ty.is_union() {
            let mut variants = ty.flattened_union_variants();
            let Some(first) = variants.next() else {
                return fmt.write_str("unknown");
            };

            self.write_variant(&first, fmt)?;
            for variant in variants {
                fmt.write_str(" | ")?;
                self.write_variant(&variant, fmt)?;
            }
            return Ok(());
        }

        self.write_variant(ty, fmt)
    }

    fn write_variant(&self, ty: &Type, fmt: &mut fmt::Formatter) -> io::Result<()> {
        let Some(data) = ty.resolved_data().map(|resolved| resolved.as_raw_data()) else {
            return fmt.write_str("unknown");
        };

        match data {
            TypeData::Unknown | TypeData::UnknownKeyword => fmt.write_str("unknown"),
            TypeData::AnyKeyword => fmt.write_str("any"),
            TypeData::NeverKeyword => fmt.write_str("never"),
            TypeData::Null => fmt.write_str("null"),
            TypeData::Undefined => fmt.write_str("undefined"),
            TypeData::Boolean => fmt.write_str("boolean"),
            TypeData::Number => fmt.write_str("number"),
            TypeData::String => fmt.write_str("string"),
            TypeData::BigInt => fmt.write_str("bigint"),
            TypeData::Symbol => fmt.write_str("symbol"),
            TypeData::ObjectKeyword | TypeData::Object(_) => fmt.write_str("object"),
            TypeData::Interface(_) => fmt.write_str("interface"),
            TypeData::Class(_) => fmt.write_str("class"),
            TypeData::Function(_) => fmt.write_str("function"),
            TypeData::Tuple(_) => fmt.write_str("tuple"),
            TypeData::Module(_) | TypeData::Namespace(_) => fmt.write_str("namespace"),
            TypeData::Constructor(_) => fmt.write_str("constructor"),
            TypeData::InstanceOf(instance) => match ty.resolve(&instance.ty) {
                Some(resolved) => self.write_variant(&resolved, fmt),
                None => fmt.write_str("object"),
            },
            TypeData::Intersection(intersection) => {
                let mut resolved = intersection
                    .types()
                    .iter()
                    .filter_map(|reference| ty.resolve(reference));
                let Some(first) = resolved.next() else {
                    return fmt.write_str("intersection");
                };

                self.write_variant(&first, fmt)?;
                for ty in resolved {
                    fmt.write_str(" & ")?;
                    self.write_variant(&ty, fmt)?;
                }
                Ok(())
            }
            TypeData::Literal(literal) => match literal.as_ref() {
                Literal::BigInt(_) => fmt.write_str("bigint"),
                Literal::Boolean(_) => fmt.write_str("boolean"),
                Literal::Number(_) => fmt.write_str("number"),
                Literal::String(_) | Literal::Template(_) => fmt.write_str("string"),
                Literal::Object(_) => fmt.write_str("object"),
                Literal::RegExp(_) => fmt.write_str("RegExp"),
            },
            _ => fmt.write_fmt(format_args!("{ty}")),
        }
    }
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
        ty: ctx.type_of_expression(&expression),
    };

    if has_invalid_variant(&operand.ty) {
        return Some(Some(NoUnsafePlusOperandsState::InvalidOperand {
            range: operand.range,
        }));
    }

    if has_number_like(&operand.ty) {
        first_number_range.get_or_insert(operand.range);
    }
    if has_bigint_like(&operand.ty) {
        first_bigint_range.get_or_insert(operand.range);
    }

    Some(None)
}

fn analyze_pair(
    range: TextRange,
    left: &OperandInfo,
    right: &OperandInfo,
) -> Option<NoUnsafePlusOperandsState> {
    if has_invalid_variant(&left.ty) {
        return Some(NoUnsafePlusOperandsState::InvalidOperand { range: left.range });
    }

    if has_invalid_variant(&right.ty) {
        return Some(NoUnsafePlusOperandsState::InvalidOperand { range: right.range });
    }

    if (has_number_like(&left.ty) && has_bigint_like(&right.ty))
        || (has_bigint_like(&left.ty) && has_number_like(&right.ty))
    {
        return Some(NoUnsafePlusOperandsState::MixedBigIntAndNumber {
            range,
            left_range: left.range,
            right_range: right.range,
        });
    }

    None
}

fn type_for_range(ctx: &RuleContext<NoUnsafePlusOperands>, range: TextRange) -> Option<Type> {
    match ctx.query() {
        NoUnsafePlusOperandsQuery::JsBinaryExpression(binary) => {
            type_for_range_in_binary(ctx, binary, range)
        }
        NoUnsafePlusOperandsQuery::JsAssignmentExpression(assignment) => {
            type_for_range_in_assignment(ctx, assignment, range)
        }
    }
}

fn type_for_range_in_binary(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    binary: &JsBinaryExpression,
    range: TextRange,
) -> Option<Type> {
    type_for_range_in_expression(ctx, binary.left().ok()?, range)
        .or_else(|| type_for_range_in_expression(ctx, binary.right().ok()?, range))
}

fn type_for_range_in_expression(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    expression: AnyJsExpression,
    range: TextRange,
) -> Option<Type> {
    let expression = expression.omit_parentheses();

    if expression.range() == range {
        return Some(ctx.type_of_expression(&expression));
    }

    if let AnyJsExpression::JsBinaryExpression(binary) = &expression
        && binary.operator().ok()? == JsBinaryOperator::Plus
    {
        return type_for_range_in_expression(ctx, binary.left().ok()?, range)
            .or_else(|| type_for_range_in_expression(ctx, binary.right().ok()?, range));
    }

    None
}

fn type_for_range_in_assignment(
    ctx: &RuleContext<NoUnsafePlusOperands>,
    assignment: &JsAssignmentExpression,
    range: TextRange,
) -> Option<Type> {
    let left = assignment.left().ok()?;
    let right = assignment.right().ok()?;

    if left.range() == range {
        return type_of_assignment_target(ctx, assignment, &left);
    }

    if right.range() == range {
        return Some(ctx.type_of_expression(&right));
    }

    None
}

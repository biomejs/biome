use crate::services::typed::Typed;

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsTemplateElement,
    JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator,
    JsCallExpression, JsTemplateExpression, global_identifier,
};
use biome_js_type_info::{InferredType, StringificationMode, StringificationUsefulness};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_base_to_string::NoBaseToStringOptions;

const DEFAULT_IGNORED_TYPE_NAMES: &[&str] = &["Error", "RegExp", "URL", "URLSearchParams"];

declare_lint_rule! {
    /// Require stringification to avoid values that only use the default object representation.
    ///
    /// JavaScript coerces values to strings in several places, such as `String(value)`,
    /// `value.toString()`, string concatenation, template interpolation, and `Array#join()`.
    /// When the value only inherits the default object stringification, that often produces
    /// `"[object Object]"` instead of something intentionally readable.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-string.ts
    /// const value: {} = {};
    /// String(value);
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-template.ts
    /// const value: {} = {};
    /// `${value}`;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-join.ts
    /// const values: {}[] = [{}];
    /// values.join(",");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// String(1);
    /// ```
    ///
    /// ```ts
    /// class CustomToString {
    ///     toString() {
    ///         return "ok";
    ///     }
    /// }
    ///
    /// `${new CustomToString()}`;
    /// ```
    pub NoBaseToString {
        version: "2.4.15",
        name: "noBaseToString",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-base-to-string").same()],
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyNoBaseToStringQuery =
        JsAssignmentExpression
        | JsBinaryExpression
        | JsCallExpression
        | JsTemplateExpression
}

#[derive(Clone, Copy, Debug)]
pub struct RuleState {
    kind: DiagnosticKind,
    certainty: StringificationUsefulness,
    range: TextRange,
}

impl Rule for NoBaseToString {
    type Query = Typed<AnyNoBaseToStringQuery>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoBaseToStringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyNoBaseToStringQuery::JsCallExpression(call) => {
                run_call_expression(ctx, call).into_iter().collect()
            }
            AnyNoBaseToStringQuery::JsBinaryExpression(binary) => {
                run_binary_expression(ctx, binary).into_iter().collect()
            }
            AnyNoBaseToStringQuery::JsAssignmentExpression(assignment) => {
                run_assignment_expression(ctx, assignment)
                    .into_iter()
                    .collect()
            }
            AnyNoBaseToStringQuery::JsTemplateExpression(template) => {
                run_template_expression(ctx, template).into_iter().collect()
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let certainty = match state.certainty {
            StringificationUsefulness::Always => return None,
            StringificationUsefulness::Sometimes => "may",
            StringificationUsefulness::Never => "will",
        };

        let diagnostic = match state.kind {
            DiagnosticKind::BaseToString => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This expression "{certainty}" use the default object stringification format, such as "<Emphasis>"[object Object]"</Emphasis>", when stringified."
                },
            )
            .note(markup! {
                "Stringify a primitive value, define a custom stringification method, or serialize the object explicitly."
            }),
            DiagnosticKind::BaseArrayJoin => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This "<Emphasis>"join()"</Emphasis>" call "{certainty}" stringify one or more elements with the default object format, such as "<Emphasis>"[object Object]"</Emphasis>"."
                },
            )
            .note(markup! {
                "Ensure every joined element has a useful string representation, or map the values before joining."
            }),
        };

        Some(diagnostic)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DiagnosticKind {
    BaseToString,
    BaseArrayJoin,
}

fn run_call_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    run_builtin_string_call(ctx, node).or_else(|| run_member_call(ctx, node))
}

/// Handle `String(value)` calls.
fn run_builtin_string_call(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?;
    let (reference, name) = global_identifier(
        &callee
            .clone()
            .omit_parentheses()
            .as_any_global_identifier_expression()?,
    )?;
    if name.text() != "String" || ctx.has_binding(&reference) {
        return None;
    }

    let args = node.arguments().ok()?.args();
    if args.len() != 1 {
        return None;
    }

    let first_arg = args.first()?.ok()?;
    let arg = first_arg.as_any_js_expression()?;
    check_expression(ctx, arg, DiagnosticKind::BaseToString)
}

/// Handle `value.join()`, `value.toString()`, and `value.toLocaleString()` calls.
fn run_member_call(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    let property = member.member().ok()?;
    let property_token = property.value_token().ok()?;
    let property_name = property_token.text_trimmed();
    let object = member.object().ok()?;

    match property_name {
        "join" => check_expression(ctx, &object, DiagnosticKind::BaseArrayJoin),
        "toString" | "toLocaleString" => {
            check_expression(ctx, &object, DiagnosticKind::BaseToString)
        }
        _ => None,
    }
}

/// Handle `value + ""` and `"" + value`.
fn run_binary_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsBinaryExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsBinaryOperator::Plus {
        return None;
    }

    let left = node.left().ok()?;
    let right = node.right().ok()?;

    if ctx
        .inferred_type_of_expression(&left)
        .is_some_and(InferredType::is_all_string_like)
    {
        return check_expression(ctx, &right, DiagnosticKind::BaseToString);
    }

    if ctx
        .inferred_type_of_expression(&right)
        .is_some_and(InferredType::is_all_string_like)
    {
        return check_expression(ctx, &left, DiagnosticKind::BaseToString);
    }

    None
}

/// Handle `value += ""`.
fn run_assignment_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsAssignmentExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsAssignmentOperator::AddAssign {
        return None;
    }

    let left = node.left().ok()?;
    let left_ty = type_of_assignment_target(ctx, node, &left)?;
    if !left_ty.is_all_string_like() {
        return None;
    }

    check_expression(ctx, &node.right().ok()?, DiagnosticKind::BaseToString)
}

/// Handle template strings like `${value}`.
fn run_template_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsTemplateExpression,
) -> Option<RuleState> {
    if node.tag().is_some() {
        return None;
    }

    node.elements()
        .into_iter()
        .find_map(|element| match element {
            AnyJsTemplateElement::JsTemplateElement(template_element) => {
                template_element.expression().ok().and_then(|expression| {
                    check_expression(ctx, &expression, DiagnosticKind::BaseToString)
                })
            }
            AnyJsTemplateElement::JsTemplateChunkElement(_) => None,
        })
}

fn type_of_assignment_target<'a>(
    ctx: &'a RuleContext<NoBaseToString>,
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
    ctx: &'a RuleContext<NoBaseToString>,
    _assignment: &JsAssignmentExpression,
    target: &AnyJsAssignment,
) -> Option<InferredType<'a>> {
    let mut current = target.clone();

    loop {
        match current {
            AnyJsAssignment::JsIdentifierAssignment(identifier) => {
                let name = identifier.name_token().ok()?;
                return ctx.inferred_type_of_named_value(
                    name.text_trimmed_range(),
                    name.text_trimmed(),
                );
            }
            AnyJsAssignment::JsParenthesizedAssignment(parenthesized) => {
                current = parenthesized.assignment().ok()?;
            }
            AnyJsAssignment::TsAsAssignment(ts_as) => {
                current = ts_as.assignment().ok()?;
            }
            AnyJsAssignment::TsNonNullAssertionAssignment(non_null) => {
                current = non_null.assignment().ok()?;
            }
            AnyJsAssignment::TsSatisfiesAssignment(satisfies) => {
                current = satisfies.assignment().ok()?;
            }
            AnyJsAssignment::TsTypeAssertionAssignment(assertion) => {
                current = assertion.assignment().ok()?;
            }
            AnyJsAssignment::JsBogusAssignment(_)
            | AnyJsAssignment::JsComputedMemberAssignment(_)
            | AnyJsAssignment::JsStaticMemberAssignment(_) => return None,
        }
    }
}

fn check_expression(
    ctx: &RuleContext<NoBaseToString>,
    expression: &AnyJsExpression,
    kind: DiagnosticKind,
) -> Option<RuleState> {
    if is_literal_expression(expression) {
        return None;
    }

    let ty = ctx.inferred_type_of_expression(expression)?;
    let ignored_type_names = ctx
        .options()
        .ignored_type_names
        .as_deref()
        .map_or_else(
            || DEFAULT_IGNORED_TYPE_NAMES.to_vec(),
            |names| names.iter().map(AsRef::as_ref).collect(),
        );
    let certainty = match kind {
        DiagnosticKind::BaseArrayJoin => ty.stringification_usefulness(
            StringificationMode::Join,
            &ignored_type_names,
        ),
        DiagnosticKind::BaseToString => ty.stringification_usefulness(
            StringificationMode::ToString,
            &ignored_type_names,
        ),
    };

    if certainty == StringificationUsefulness::Always {
        None
    } else {
        Some(RuleState {
            kind,
            certainty,
            range: expression.range(),
        })
    }
}

fn is_literal_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(_) | AnyJsExpression::JsArrayExpression(_)
    )
}

use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsSyntaxNode,
    JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast};
use biome_rule_options::use_includes::UseIncludesOptions;

declare_lint_rule! {
    /// Prefer `includes()` over `indexOf()` comparisons.
    ///
    /// When checking for the presence of a value in a string or array, using
    /// `includes()` is more expressive and readable than comparing the result of
    /// `indexOf()` against `-1` or `0`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// str.indexOf("foo") !== -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// arr.indexOf(item) === -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// str.indexOf("foo") >= 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// str.indexOf("foo") > -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// str.indexOf("foo") < 0;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// str.includes("foo");
    /// ```
    ///
    /// ```js
    /// !arr.includes(item);
    /// ```
    ///
    /// ```js
    /// // Using indexOf for position is fine
    /// const idx = str.indexOf("foo");
    /// ```
    ///
    pub UseIncludes {
        version: "next",
        name: "useIncludes",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("prefer-includes")],
        fix_kind: FixKind::Unsafe,
    }
}

/// Whether the replacement is a positive or negated `includes()` call.
#[derive(Clone, Copy)]
pub enum IncludesKind {
    /// `x.includes(y)` — the indexOf was checking for presence.
    Positive,
    /// `!x.includes(y)` — the indexOf was checking for absence.
    Negated,
}

pub struct UseIncludesState {
    call: JsCallExpression,
    comparison: JsBinaryExpression,
    kind: IncludesKind,
}

impl Rule for UseIncludes {
    type Query = Ast<JsCallExpression>;
    type State = UseIncludesState;
    type Signals = Option<Self::State>;
    type Options = UseIncludesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let method_name = call_method_name(call)?;

        if method_name != "indexOf" {
            return None;
        }

        let comparison = nearest_parent_binary_expression(call.syntax())?;
        let operator = comparison.operator().ok()?;
        let left = comparison.left().ok()?;
        let right = comparison.right().ok()?;

        let left_is_call = expression_matches_target(&left, call.syntax());
        let right_is_call = expression_matches_target(&right, call.syntax());

        if !left_is_call && !right_is_call {
            return None;
        }

        let (literal, normalized_op) = if left_is_call {
            // indexOf(...) OP literal
            (right, operator)
        } else {
            // literal OP indexOf(...) — flip the operator
            let flipped = match operator {
                JsBinaryOperator::GreaterThan => JsBinaryOperator::LessThan,
                JsBinaryOperator::GreaterThanOrEqual => JsBinaryOperator::LessThanOrEqual,
                JsBinaryOperator::LessThan => JsBinaryOperator::GreaterThan,
                JsBinaryOperator::LessThanOrEqual => JsBinaryOperator::GreaterThanOrEqual,
                other => other,
            };
            (left, flipped)
        };

        // Determine if this is a presence or absence check.
        let kind = match normalized_op {
            // indexOf(x) !== -1  →  includes(x)
            // indexOf(x) != -1   →  includes(x)
            JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality => {
                if is_negative_one_literal(&literal) {
                    Some(IncludesKind::Positive)
                } else {
                    None
                }
            }
            // indexOf(x) >= 0    →  includes(x)
            JsBinaryOperator::GreaterThanOrEqual => {
                if is_number_literal_value(&literal, 0.0) {
                    Some(IncludesKind::Positive)
                } else {
                    None
                }
            }
            // indexOf(x) > -1   →  includes(x)
            JsBinaryOperator::GreaterThan => {
                if is_negative_one_literal(&literal) {
                    Some(IncludesKind::Positive)
                } else {
                    None
                }
            }
            // indexOf(x) === -1  →  !includes(x)
            // indexOf(x) == -1   →  !includes(x)
            JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality => {
                if is_negative_one_literal(&literal) {
                    Some(IncludesKind::Negated)
                } else {
                    None
                }
            }
            // indexOf(x) < 0    →  !includes(x)
            JsBinaryOperator::LessThan => {
                if is_number_literal_value(&literal, 0.0) {
                    Some(IncludesKind::Negated)
                } else {
                    None
                }
            }
            _ => None,
        }?;

        Some(UseIncludesState {
            call: call.clone(),
            comparison,
            kind,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.comparison.range(),
                markup! {
                    "Use "<Emphasis>".includes()"</Emphasis>" rather than "<Emphasis>".indexOf()"</Emphasis>" to check for the presence of a value."
                },
            )
            .note(markup! {
                <Emphasis>".includes()"</Emphasis>" is more readable and expressive than comparing "<Emphasis>".indexOf()"</Emphasis>" against "<Emphasis>"-1"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let callee = state.call.callee().ok()?;
        let static_member = callee.as_js_static_member_expression()?;
        let updated_callee = AnyJsExpression::JsStaticMemberExpression(
            static_member
                .clone()
                .with_member(make::js_name(make::ident("includes")).into()),
        );
        let includes_call =
            AnyJsExpression::JsCallExpression(state.call.clone().with_callee(updated_callee));

        let replacement = match state.kind {
            IncludesKind::Positive => includes_call,
            IncludesKind::Negated => {
                make::js_unary_expression(make::token(T![!]), includes_call).into()
            }
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsBinaryExpression(state.comparison.clone()),
            replacement,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>".includes()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn call_method_name(call: &JsCallExpression) -> Option<String> {
    let callee = call.callee().ok()?;
    let member = callee.as_js_static_member_expression()?;
    let member_name = member.member().ok()?;
    let name = member_name.as_js_name()?;
    Some(name.value_token().ok()?.text_trimmed().to_string())
}

fn nearest_parent_binary_expression(node: &JsSyntaxNode) -> Option<JsBinaryExpression> {
    let mut current = node.parent()?;
    loop {
        if let Some(binary) = JsBinaryExpression::cast(current.clone()) {
            return Some(binary);
        }
        if biome_js_syntax::JsParenthesizedExpression::can_cast(current.kind()) {
            current = current.parent()?;
            continue;
        }
        return None;
    }
}

fn expression_matches_target(expression: &AnyJsExpression, target: &JsSyntaxNode) -> bool {
    expression.clone().omit_parentheses().syntax().eq(target)
}

fn is_number_literal_value(expression: &AnyJsExpression, value: f64) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression().cloned())
        .and_then(|literal| literal.as_number())
        .is_some_and(|number| number == value)
}

fn is_negative_one_literal(expression: &AnyJsExpression) -> bool {
    let expression = expression.clone().omit_parentheses();
    if is_number_literal_value(&expression, -1.0) {
        return true;
    }

    let unary = match expression.as_js_unary_expression() {
        Some(unary) => unary,
        None => return false,
    };

    if unary.operator().ok() != Some(JsUnaryOperator::Minus) {
        return false;
    }

    unary
        .argument()
        .ok()
        .is_some_and(|arg| is_number_literal_value(&arg, 1.0))
}

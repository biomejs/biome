use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsSyntaxNode,
    is_in_boolean_context,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast, TextRange};
use biome_rule_options::use_array_some::UseArraySomeOptions;

declare_lint_rule! {
    /// Prefer `Array.prototype.some()` over verbose existence checks.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.filter(predicate).length > 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.findIndex(predicate) !== -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.find(predicate)) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.find(predicate) != null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.findLastIndex(predicate) !== -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.findLast(predicate)) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// array.some(predicate);
    /// ```
    ///
    pub UseArraySome {
        version: "next",
        name: "useArraySome",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-array-some").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

/// The kind of pattern detected by the rule.
#[derive(Clone, Copy)]
pub enum DetectedPattern {
    FilterLengthComparison,
    FindIndexComparison,
    FindLastIndexComparison,
    FindExistenceComparison,
    FindLastExistenceComparison,
    FindFamilyAsBoolean,
}

pub enum UseArraySomeState {
    Fix {
        call: JsCallExpression,
        replace_node: AnyJsExpression,
        pattern: DetectedPattern,
    },
    Suggest {
        range: TextRange,
        pattern: DetectedPattern,
    },
}

impl Rule for UseArraySome {
    type Query = Ast<JsCallExpression>;
    type State = UseArraySomeState;
    type Signals = Option<Self::State>;
    type Options = UseArraySomeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let method_name = call_method_name(call)?;

        if method_name == "filter" {
            return detect_filter_length_pattern(call);
        }

        if method_name == "findIndex" || method_name == "findLastIndex" {
            return detect_find_index_comparison_pattern(call, &method_name);
        }

        if (method_name == "find" || method_name == "findLast")
            && let Some(state) = detect_find_existence_comparison_pattern(call, &method_name)
        {
            return Some(state);
        }

        if (method_name == "find" || method_name == "findLast")
            && is_in_boolean_context(call.syntax()).unwrap_or(false)
        {
            return Some(UseArraySomeState::Suggest {
                range: call.range(),
                pattern: DetectedPattern::FindFamilyAsBoolean,
            });
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            UseArraySomeState::Fix { replace_node, .. } => replace_node.range(),
            UseArraySomeState::Suggest { range, .. } => *range,
        };

        let pattern = match state {
            UseArraySomeState::Fix { pattern, .. } | UseArraySomeState::Suggest { pattern, .. } => {
                *pattern
            }
        };
        let description = match pattern {
            DetectedPattern::FilterLengthComparison => "filter(...).length comparison",
            DetectedPattern::FindIndexComparison => "findIndex(...) !== -1",
            DetectedPattern::FindLastIndexComparison => "findLastIndex(...) !== -1",
            DetectedPattern::FindExistenceComparison => "find(...) existence comparison",
            DetectedPattern::FindLastExistenceComparison => "findLast(...) existence comparison",
            DetectedPattern::FindFamilyAsBoolean => "find-family used as boolean",
        };

        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Prefer "<Emphasis>".some()"</Emphasis>" over "<Emphasis>{description}</Emphasis>"."
            },
        );
        if matches!(pattern, DetectedPattern::FilterLengthComparison) {
            diag = diag.note(markup! {
                "Using "<Emphasis>".filter()"</Emphasis>" followed by a length check iterates the entire array unnecessarily. "<Emphasis>".some()"</Emphasis>" stops at the first match."
            });
        }
        diag = diag.note(markup! {
            "Use "<Emphasis>".some()"</Emphasis>" when you only need to know if any element matches."
        });
        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let UseArraySomeState::Fix {
            call, replace_node, ..
        } = state
        else {
            return None;
        };

        let callee = call.callee().ok()?;
        let static_member = callee.as_js_static_member_expression()?;
        let updated_callee = AnyJsExpression::JsStaticMemberExpression(
            static_member
                .clone()
                .with_member(make::js_name(make::ident("some")).into()),
        );
        let updated_call = call.clone().with_callee(updated_callee);

        let mut mutation = ctx.root().begin();
        mutation.replace_node(replace_node.clone(), AnyJsExpression::JsCallExpression(updated_call));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>".some()"</Emphasis>" instead." }.to_owned(),
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

fn detect_filter_length_pattern(call: &JsCallExpression) -> Option<UseArraySomeState> {
    let parent_member = call.syntax().parent().and_then(|parent| {
        let member = parent.cast::<biome_js_syntax::JsStaticMemberExpression>()?;
        let member_name = member.member().ok()?;
        let name = member_name.as_js_name()?;
        (name.value_token().ok()?.text_trimmed() == "length").then_some(member)
    })?;

    let comparison = nearest_parent_binary_expression(parent_member.syntax())?;
    let left = comparison.left().ok()?;
    let right = comparison.right().ok()?;
    let operator = comparison.operator().ok()?;

    // Determine which side is the length expression and normalize the operator
    let (literal, normalized_op) =
        if expression_matches_target(&left, parent_member.syntax()) {
            // arr.filter(...).length OP literal
            (right, operator)
        } else if expression_matches_target(&right, parent_member.syntax()) {
            // literal OP arr.filter(...).length — swap the operator
            let swapped = match operator {
                JsBinaryOperator::GreaterThan => JsBinaryOperator::LessThan,
                JsBinaryOperator::GreaterThanOrEqual => JsBinaryOperator::LessThanOrEqual,
                JsBinaryOperator::LessThan => JsBinaryOperator::GreaterThan,
                JsBinaryOperator::LessThanOrEqual => JsBinaryOperator::GreaterThanOrEqual,
                other => other, // symmetric operators like !== stay the same
            };
            (left, swapped)
        } else {
            return None;
        };

    let matches = match normalized_op {
        JsBinaryOperator::GreaterThan => is_number_literal_value(&literal, 0.0),
        JsBinaryOperator::GreaterThanOrEqual => is_number_literal_value(&literal, 1.0),
        JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality => {
            is_number_literal_value(&literal, 0.0)
        }
        _ => false,
    };

    matches.then_some(UseArraySomeState::Fix {
        call: call.clone(),
        replace_node: AnyJsExpression::JsBinaryExpression(comparison),
        pattern: DetectedPattern::FilterLengthComparison,
    })
}

fn detect_find_index_comparison_pattern(
    call: &JsCallExpression,
    method_name: &str,
) -> Option<UseArraySomeState> {
    let comparison = nearest_parent_binary_expression(call.syntax())?;
    let operator = comparison.operator().ok()?;
    if !matches!(
        operator,
        JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality
    ) {
        return None;
    }

    let left = comparison.left().ok()?;
    let right = comparison.right().ok()?;
    let left_matches_call = expression_matches_target(&left, call.syntax());
    let right_matches_call = expression_matches_target(&right, call.syntax());

    let is_match = (left_matches_call && is_negative_one_literal(&right))
        || (right_matches_call && is_negative_one_literal(&left));

    is_match.then_some(UseArraySomeState::Fix {
        call: call.clone(),
        replace_node: AnyJsExpression::JsBinaryExpression(comparison),
        pattern: if method_name == "findLastIndex" {
            DetectedPattern::FindLastIndexComparison
        } else {
            DetectedPattern::FindIndexComparison
        },
    })
}

fn detect_find_existence_comparison_pattern(
    call: &JsCallExpression,
    method_name: &str,
) -> Option<UseArraySomeState> {
    let comparison = nearest_parent_binary_expression(call.syntax())?;
    let operator = comparison.operator().ok()?;

    let left = comparison.left().ok()?;
    let right = comparison.right().ok()?;
    let left_matches_call = expression_matches_target(&left, call.syntax());
    let right_matches_call = expression_matches_target(&right, call.syntax());
    if !left_matches_call && !right_matches_call {
        return None;
    }

    let other = if left_matches_call { right } else { left };
    let strict_undefined = matches!(operator, JsBinaryOperator::StrictInequality)
        && is_undefined_expression(&other);
    let loose_nullish =
        matches!(operator, JsBinaryOperator::Inequality) && is_nullish_expression(&other);

    (strict_undefined || loose_nullish).then_some(UseArraySomeState::Suggest {
        range: comparison.range(),
        pattern: if method_name == "findLast" {
            DetectedPattern::FindLastExistenceComparison
        } else {
            DetectedPattern::FindExistenceComparison
        },
    })
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
    expression
        .clone()
        .omit_parentheses()
        .syntax()
        .eq(target)
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

    if unary.operator().ok() != Some(biome_js_syntax::JsUnaryOperator::Minus) {
        return false;
    }

    unary
        .argument()
        .ok()
        .is_some_and(|arg| is_number_literal_value(&arg, 1.0))
}

fn is_undefined_expression(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_static_value()
        .is_some_and(|value| matches!(value, biome_js_syntax::static_value::StaticValue::Undefined(_)))
}

fn is_nullish_expression(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_static_value()
        .is_some_and(|value| value.is_null_or_undefined())
}

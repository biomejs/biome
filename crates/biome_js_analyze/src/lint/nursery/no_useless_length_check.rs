use std::collections::{HashMap, HashSet};

use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsBinaryExpression, JsBinaryOperator,
    JsLogicalExpression, JsParenthesizedExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary length checks within logical expressions.
    ///
    /// When using the function [`Array#some()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some), it returns `false` when an array is empty; hence, there isn't need to check if the array is not empty.
    ///
    /// When using the function [`Array#every()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every), it returns `true` when an array is empty; hence, there isn't need to check if the array is empty.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (array.length === 0 || array.every(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length !== 0 && array.some(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length > 0 && array.some(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const isAllTrulyOrEmpty = array.length === 0 || array.every(Boolean);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (array.every(Boolean));
    /// ```
    ///
    /// ```js
    /// if (array.some(Boolean));
    /// ```
    ///
    /// ```js
    /// const isAllTrulyOrEmpty = array.every(Boolean);
    /// ```
    ///
    /// ```js
    /// const isAllTrulyOrEmpty = array.every(Boolean);
    /// ```
    ///
    /// ```js
    /// if (array.length === 0 || anotherCheck() || array.every(Boolean));
    /// ```
    ///
    /// ```js
    /// const isNonEmptyAllTrulyArray = array.length > 0 && array.every(Boolean);
    /// ```
    ///
    /// ```js
    /// const isEmptyArrayOrAllTruly = array.length === 0 || array.some(Boolean);
    /// ```
    ///
    pub NoUselessLengthCheck {
        version: "next",
        name: "noUselessLengthCheck",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("no-useless-length-check")],
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Clone, Debug)]
pub enum FunctionKind {
    /// `Array.some()` was used
    Some,
    /// `Array.every()` was used
    Every,
}

/// Whether the node is a descendant of a logical expression.
fn is_logical_exp_descendant(node: &AnyJsExpression, operator: JsSyntaxKind) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    parent
        .ancestors()
        .find_map(|ancestor| {
            if let Some(logical_exp) = JsLogicalExpression::cast_ref(&ancestor) {
                return logical_exp
                    .operator_token()
                    .ok()
                    .map(|token| token.kind() == operator)
                    .or(Some(false));
            }
            (!JsParenthesizedExpression::can_cast(ancestor.kind())).then_some(false)
        })
        .unwrap_or(false)
}

/// Extract the expressions that perform length comparisons corresponding to the errors you want to check.
/// # Examples
/// ## `foo.every()`
/// `foo.length === 0` -> `Some(foo)`
/// `foo.length !== 0` -> `None`
/// ## `foo.some()`
/// `foo.length !== 0` -> `Some(foo)`
/// `foo.length >= 1` -> `Some(foo)`
/// `foo.length === 0` -> `None`
fn get_comparing_length_exp(
    binary_exp: &JsBinaryExpression,
    function_kind: &FunctionKind,
) -> Option<AnyJsExpression> {
    let operator = binary_exp.operator().ok()?;
    // Check only when the number appears on the right side according to the original rules.
    // We assume that you have already complied with useExplicitLengthCheck
    let compare_exp = binary_exp.left().ok()?;
    let value_exp = binary_exp.right().ok()?;

    let member_exp = compare_exp.as_js_static_member_expression()?;
    let target = member_exp.object().ok()?;
    let member = member_exp.member().ok()?;
    if member.syntax().text_trimmed() != "length" || member_exp.is_optional_chain() {
        return None;
    }
    let literal = value_exp.as_any_js_literal_expression()?;
    let literal = literal.as_js_number_literal_expression()?;
    match function_kind {
        FunctionKind::Every => {
            // .length === 0
            (literal.syntax().text_trimmed() == "0"
                && (operator == JsBinaryOperator::StrictEquality
                    || operator == JsBinaryOperator::LessThan))
                .then_some(target)
        }
        FunctionKind::Some => {
            // .length !== 0
            (literal.syntax().text_trimmed() == "0"
                && (operator == JsBinaryOperator::StrictInequality
                    || operator == JsBinaryOperator::GreaterThan)
                || literal.syntax().text_trimmed() == "1"
                    && operator == JsBinaryOperator::GreaterThanOrEqual)
                .then_some(target)
        }
    }
}

#[derive(Clone)]
/// A struct that manages the form before and after replacement.
pub struct FixablePoint {
    /// The node before the replacement.
    prev_node: JsLogicalExpression,
    /// The node after the replacement.
    next_node: AnyJsExpression,
    /// Error occurrence location.
    range: TextRange,
}

/// Search for logical expressions and list expressions that compare to 0 and Array APIs (`.some()`, `.every()`).
fn search_logical_exp(
    any_exp: &AnyJsExpression,
    fixable_point: Option<FixablePoint>,
    function_kind: &FunctionKind,
    comparing_zeros: &mut HashMap<String, Vec<FixablePoint>>,
    array_tokens_used_api: &mut HashSet<String>,
) -> Option<()> {
    match any_exp {
        // || or &&
        AnyJsExpression::JsLogicalExpression(logical_exp) => {
            let operator = match function_kind {
                FunctionKind::Every => T![||],
                FunctionKind::Some => T![&&],
            };
            if logical_exp.operator_token().ok()?.kind() != operator {
                return None;
            };
            let left = logical_exp.left().ok()?;
            let left_fixable_point = FixablePoint {
                prev_node: logical_exp.clone(),
                next_node: logical_exp.right().ok()?,
                range: left.range(),
            };
            search_logical_exp(
                &left,
                Some(left_fixable_point),
                function_kind,
                comparing_zeros,
                array_tokens_used_api,
            )?;

            let right = logical_exp.right().ok()?;
            let right_fixable_point = FixablePoint {
                prev_node: logical_exp.clone(),
                next_node: logical_exp.left().ok()?,
                range: right.range(),
            };
            search_logical_exp(
                &right,
                Some(right_fixable_point),
                function_kind,
                comparing_zeros,
                array_tokens_used_api,
            )
        }
        // a === 0 ext.
        AnyJsExpression::JsBinaryExpression(binary_exp) => {
            let comparing_zero = get_comparing_length_exp(binary_exp, function_kind)?;
            let AnyJsExpression::JsIdentifierExpression(array_token) = comparing_zero else {
                return None;
            };
            let key = array_token.text();
            if let Some(comparing_zero_list) = comparing_zeros.get_mut(&key) {
                comparing_zero_list.push(fixable_point?);
            } else {
                comparing_zeros.insert(key, vec![fixable_point?]);
            }
            Some(())
        }
        // .some() or .every() etc.
        AnyJsExpression::JsCallExpression(task_exp) => {
            if task_exp.is_optional_chain() {
                return None;
            }
            let task_member_exp =
                AnyJsMemberExpression::cast(task_exp.callee().ok()?.into_syntax())?;
            let task_target = task_member_exp.object().ok()?;
            let task_target_token = task_target.as_js_identifier_expression()?;

            let task_member_name_node = task_member_exp.member_name()?;
            let task_member_name = task_member_name_node.text();
            match function_kind {
                FunctionKind::Every => {
                    if task_member_name == "every" {
                        array_tokens_used_api.insert(task_target_token.text());
                        Some(())
                    } else {
                        None
                    }
                }
                FunctionKind::Some => {
                    if task_member_name == "some" {
                        array_tokens_used_api.insert(task_target_token.text());
                        Some(())
                    } else {
                        None
                    }
                }
            }
        }
        // ( foo )
        AnyJsExpression::JsParenthesizedExpression(parent_exp) => search_logical_exp(
            &parent_exp.expression().ok()?,
            fixable_point,
            function_kind,
            comparing_zeros,
            array_tokens_used_api,
        ),
        AnyJsExpression::JsIdentifierExpression(_) => Some(()),
        _ => None,
    }
}

fn get_parenthesized_parent(exp: AnyJsExpression) -> AnyJsExpression {
    let Some(parent) = exp.syntax().parent() else {
        return exp;
    };
    let Some(parent) = JsParenthesizedExpression::cast(parent) else {
        return exp;
    };
    get_parenthesized_parent(AnyJsExpression::from(parent))
}

pub struct NoUselessLengthCheckState {
    /// The kind of function used in the logical expression.
    function_kind: FunctionKind,
    /// The form before and after replacement.
    fixable_point: FixablePoint,
}

impl Rule for NoUselessLengthCheck {
    type Query = Ast<JsLogicalExpression>;
    type State = NoUselessLengthCheckState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut fixable_list = Vec::new();

        let Some(operator) = node.operator_token().ok() else {
            return fixable_list;
        };
        // node must not be a child of a logical expression
        if is_logical_exp_descendant(&AnyJsExpression::from(node.clone()), operator.kind()) {
            return fixable_list;
        }

        for function_kind in [FunctionKind::Every, FunctionKind::Some] {
            let mut comparing_zeros = HashMap::new();
            let mut array_tokens_used_api = HashSet::new();
            let search_result = search_logical_exp(
                &AnyJsExpression::from(node.clone()),
                None,
                &function_kind,
                &mut comparing_zeros,
                &mut array_tokens_used_api,
            );
            if search_result.is_some() {
                for array_token in array_tokens_used_api {
                    if let Some(fixable_points) = comparing_zeros.get(&array_token) {
                        for fixable_point in fixable_points {
                            fixable_list.push(NoUselessLengthCheckState {
                                function_kind: function_kind.clone(),
                                fixable_point: fixable_point.clone(),
                            });
                        }
                    }
                }
            }
        }
        fixable_list
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        NoUselessLengthCheckState {
            function_kind,
            fixable_point,
        }: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                fixable_point.range,
                markup! {
                    "This length check is unnecessary."
                },
            )
            .note(
                match function_kind {
                    FunctionKind::Every => markup! {
                        "The empty check is useless as "<Emphasis>"`Array#every()`"</Emphasis>" returns "<Emphasis>"`true`"</Emphasis>" for an empty array."
                    },
                    FunctionKind::Some => markup! {
                        "The non-empty check is useless as "<Emphasis>"`Array#some()`"</Emphasis>" returns "<Emphasis>"`false`"</Emphasis>" for an empty array."
                    },
                }
            ),
        )
    }

    fn action(
        ctx: &RuleContext<Self>,
        NoUselessLengthCheckState { fixable_point, .. }: &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let FixablePoint {
            prev_node,
            next_node,
            ..
        } = fixable_point;

        mutation.replace_node(
            get_parenthesized_parent(AnyJsExpression::from(prev_node.clone())),
            next_node.clone().omit_parentheses(),
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Remove the length check" }.to_owned(),
            mutation,
        ))
    }
}

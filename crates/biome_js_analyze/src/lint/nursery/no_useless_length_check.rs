use std::collections::{HashMap, HashSet};

use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, JsBinaryExpression,
    JsLogicalExpression, JsParenthesizedExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary length checks within logical expressions.
    ///
    /// - `Array#some()` returns `false` for an empty array. There is no need to check if the array is not empty.
    /// - `Array#every()` returns `true`` for an empty array. There is no need to check if the array is empty.
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
pub enum ErrorType {
    UselessLengthCheckWithSome,
    UselessLengthCheckWithEvery,
}

/// Whether the node is a descendant of a logical expression.
fn is_logical_exp_descendant(node: &AnyJsExpression, operator: JsSyntaxKind) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    parent
        .ancestors()
        .find_map(|ancestor| {
            if let Some(logical_exp) = JsLogicalExpression::cast(ancestor.clone()) {
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
fn get_comparing_length_exp(
    binary_exp: &JsBinaryExpression,
    expect_error: &ErrorType,
) -> Option<AnyJsExpression> {
    let left = binary_exp.left().ok()?;
    let operator = binary_exp.operator_token().ok()?.kind();
    let right = binary_exp.right().ok()?;

    // Check only when the number appears on the right side according to the original rules.
    // We assume that you have already complied with useExplicitLengthCheck
    let compare_exp = left;
    let value_exp = right;

    let AnyJsExpression::JsStaticMemberExpression(member_exp) = compare_exp else {
        return None;
    };
    let target = member_exp.object().ok()?;
    let member = member_exp.member().ok()?;
    if member.text() != "length" || member_exp.is_optional_chain() {
        return None;
    }
    let AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(
        literal,
    )) = value_exp
    else {
        return None;
    };
    let number = literal.as_number()?.round() as i64;
    // .length === 0
    if matches!(expect_error, ErrorType::UselessLengthCheckWithEvery)
        && literal.to_string().trim() == "0"
        && (operator == T![===] || operator == T![<])
    {
        return Some(target);
    }
    // .length !== 0
    if matches!(expect_error, ErrorType::UselessLengthCheckWithSome)
        && (literal.to_string().trim() == "0" && (operator == T![!==] || operator == T![>])
            || number > 0 && operator == T![===]
            || literal.to_string().trim() == "1" && operator == T![>=])
    {
        return Some(target);
    }
    None
}

pub type Replacer = (JsLogicalExpression, AnyJsExpression, TextRange);

/// Search for logical expressions and list expressions that compare to 0 and Array APIs (`.some()`, `.every()`).
fn search_logical_exp(
    any_exp: &AnyJsExpression,
    replacer: Option<Replacer>,
    expect_error: &ErrorType,
    comparing_zeros: &mut HashMap<String, Vec<Replacer>>,
    array_tokens_used_api: &mut HashSet<String>,
) -> Option<()> {
    match any_exp {
        // || or &&
        AnyJsExpression::JsLogicalExpression(logical_exp) => {
            let operator = match expect_error {
                ErrorType::UselessLengthCheckWithEvery => T![||],
                ErrorType::UselessLengthCheckWithSome => T![&&],
            };
            if logical_exp.operator_token().ok()?.kind() != operator {
                return None;
            };
            let left = logical_exp.left().ok()?;
            let left_replacer = (logical_exp.clone(), logical_exp.right().ok()?, left.range());
            search_logical_exp(
                &left,
                Some(left_replacer),
                expect_error,
                comparing_zeros,
                array_tokens_used_api,
            )?;

            let right = logical_exp.right().ok()?;
            let right_replacer = (logical_exp.clone(), logical_exp.left().ok()?, right.range());
            search_logical_exp(
                &right,
                Some(right_replacer),
                expect_error,
                comparing_zeros,
                array_tokens_used_api,
            )
        }
        // a === 0 ext.
        AnyJsExpression::JsBinaryExpression(binary_exp) => {
            let comparing_zero = get_comparing_length_exp(binary_exp, expect_error)?;
            let AnyJsExpression::JsIdentifierExpression(array_token) = comparing_zero else {
                return None;
            };
            let key = array_token.text();
            if let Some(comparing_zero_list) = comparing_zeros.get_mut(&key) {
                comparing_zero_list.push(replacer?);
            } else {
                comparing_zeros.insert(key, vec![replacer?]);
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
            let AnyJsExpression::JsIdentifierExpression(task_target_token) = task_target else {
                return None;
            };

            let task_member_name_node = task_member_exp.member_name()?;
            let task_member_name = task_member_name_node.text();
            match expect_error {
                ErrorType::UselessLengthCheckWithEvery => {
                    if task_member_name == "every" {
                        array_tokens_used_api.insert(task_target_token.text());
                        Some(())
                    } else {
                        None
                    }
                }
                ErrorType::UselessLengthCheckWithSome => {
                    if task_member_name == "some" {
                        array_tokens_used_api.insert(task_target_token.text());
                        Some(())
                    } else {
                        None
                    }
                }
            }
        }
        // ( hoge )
        AnyJsExpression::JsParenthesizedExpression(parent_exp) => search_logical_exp(
            &parent_exp.expression().ok()?,
            replacer,
            expect_error,
            comparing_zeros,
            array_tokens_used_api,
        ),
        // hoge
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

fn get_parenthesized_child(exp: &AnyJsExpression) -> AnyJsExpression {
    let AnyJsExpression::JsParenthesizedExpression(parenthesized) = exp else {
        return exp.clone();
    };
    let Some(child) = parenthesized.expression().ok() else {
        return exp.clone();
    };
    get_parenthesized_child(&child)
}

impl Rule for NoUselessLengthCheck {
    type Query = Ast<JsLogicalExpression>;
    type State = (ErrorType, Replacer);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let Some(operator) = node.operator_token().ok() else {
            return Vec::new();
        };
        // node must not be a child of a logical expression
        if is_logical_exp_descendant(&AnyJsExpression::from(node.clone()), operator.kind()) {
            return Vec::new();
        }

        let mut fixable_list: Vec<(ErrorType, Replacer)> = Vec::new();

        for err_type in [
            ErrorType::UselessLengthCheckWithEvery,
            ErrorType::UselessLengthCheckWithSome,
        ] {
            let mut comparing_zeros: HashMap<String, Vec<Replacer>> = HashMap::new();
            let mut array_tokens_used_api: HashSet<String> = HashSet::new();
            let search_result = search_logical_exp(
                &AnyJsExpression::from(node.clone()),
                None,
                &err_type,
                &mut comparing_zeros,
                &mut array_tokens_used_api,
            );
            if search_result.is_some() {
                for array_token in array_tokens_used_api {
                    if let Some(replacers) = comparing_zeros.get(&array_token) {
                        for replacer in replacers {
                            fixable_list.push((err_type.clone(), replacer.clone()));
                        }
                    }
                }
            }
        }
        fixable_list
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (error_type, (_, _, error_range)): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                error_range,
                markup! {
                    "This length check is unnecessary."
                },
            )
            .note(
                match error_type {
                    ErrorType::UselessLengthCheckWithEvery => markup! {
                        "The empty check is useless as `Array#every()` returns `true` for an empty array."
                    },
                    ErrorType::UselessLengthCheckWithSome => markup! {
                        "The non-empty check is useless as `Array#some()` returns `false` for an empty array."
                    },
                }
            ),
        )
    }

    fn action(
        ctx: &RuleContext<Self>,
        (_error_type, (prev_node, next_node, _error_range)): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.replace_node(
            get_parenthesized_parent(AnyJsExpression::from(prev_node.clone())),
            get_parenthesized_child(next_node),
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Remove the length check" }.to_owned(),
            mutation,
        ))
    }
}

use ::serde::{Deserialize, Serialize};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make::{self};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsCallExpression,
    JsComputedMemberExpression, JsParenthesizedExpression, JsUnaryExpression, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Enforce using .at to retrieve elements.
    ///
    /// When extracting elements from an array, especially when retrieving from the end, `.at` is convenient. Replace the previously used syntax with `.at()`.
    ///
    /// ## Options
    ///
    /// ### `checkAllIndexAccess`
    ///
    /// By default, only negative element accesses will use errors, but I will also generate errors for positive accesses.
    ///
    /// ```json,ignore
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "checkAllIndexAccess": true
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = array[array.length - 1];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = array[array.length - 5];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = array.slice(-1)[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = array.slice(-1).pop();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = array.slice(-5).shift();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = string.charAt(string.length - 5);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = lodash.last(array);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = array.at(-1);
    /// ```
    ///
    /// ```js
    /// const foo = array.at(-5);
    /// ```
    ///
    /// ```js
    /// const foo = array[100];
    /// ```
    ///
    /// ```js
    /// const foo = array.at(array.length - 1);
    /// ```
    ///
    /// ```js
    /// array[array.length - 1] = foo;
    /// ```
    pub UseAtIndex {
        version: "next",
        name: "useAtIndex",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-at")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsArrayAccess = JsComputedMemberExpression | JsCallExpression
}

pub struct UseAtIndexState {
    at_number_exp: AnyJsExpression,
    error_type: ErrorType,
    object: AnyJsExpression,
}

pub enum ErrorType {
    NegativeIndex,
    IdIndex,
    StringCharAtNegativeIndex,
    StringCharAt,
    Slice,
    GetLastFunction,
}

/// If the node is a parenthized expression, it returns the expression inside.
/// # Examples
/// ```js
///    a // Some(a)
///    (a) // Some(a)
///    (a + b) // Some(a + b)
/// ```
fn solve_parenthesized_expression(node: &AnyJsExpression) -> Option<AnyJsExpression> {
    if let AnyJsExpression::JsParenthesizedExpression(parenthesized_exp) = node {
        let exp = parenthesized_exp.expression().ok()?;
        solve_parenthesized_expression(&exp)
    } else {
        Some(node.clone())
    }
}

/// Check if two expressions reference the same value.
/// Only literals are allowed for members.
/// # Examples
/// ```js
///     a == a
///     a.b == a.b
///     a?.b == a.b
///     a[0] == a[0]
///     a['b'] == a['b']
/// ```
fn is_same_reference(left: &AnyJsExpression, right: &AnyJsExpression) -> Option<bool> {
    // solve JsParenthesizedExpression
    let left = solve_parenthesized_expression(left)?;
    let right = solve_parenthesized_expression(right)?;
    match left {
        // x[0]
        AnyJsExpression::JsComputedMemberExpression(left) => match right {
            AnyJsExpression::JsComputedMemberExpression(right) => {
                let AnyJsExpression::AnyJsLiteralExpression(left_member) =
                    solve_parenthesized_expression(&left.member().ok()?)?
                else {
                    return Some(false);
                };
                let AnyJsExpression::AnyJsLiteralExpression(right_member) =
                    solve_parenthesized_expression(&right.member().ok()?)?
                else {
                    return Some(false);
                };
                if left_member.text() != right_member.text() {
                    return Some(false);
                }
                is_same_reference(&left.object().ok()?, &right.object().ok()?)
            }
            _ => Some(false),
        },
        // x.y
        AnyJsExpression::JsStaticMemberExpression(left) => match right {
            AnyJsExpression::JsStaticMemberExpression(right) => {
                let left_member = left.member().ok()?;
                let right_member = right.member().ok()?;
                if left_member.text() != right_member.text() {
                    Some(false)
                } else {
                    is_same_reference(&left.object().ok()?, &right.object().ok()?)
                }
            }
            _ => Some(false),
        },
        // x
        AnyJsExpression::JsIdentifierExpression(left) => match right {
            AnyJsExpression::JsIdentifierExpression(right) => {
                Some(left.name().ok()?.text() == right.name().ok()?.text())
            }
            _ => Some(false),
        },
        // this
        AnyJsExpression::JsThisExpression(_) => match right {
            AnyJsExpression::JsThisExpression(_) => Some(true),
            _ => Some(false),
        },
        _ => Some(false),
    }
}

/// If the node is a length method, it returns the object of interest.
fn get_length_node(node: &AnyJsExpression) -> Option<AnyJsExpression> {
    let AnyJsExpression::JsStaticMemberExpression(node) = node else {
        return None;
    };
    let member_name = node.member().ok()?;
    let member_name = member_name
        .as_js_name()?
        .value_token()
        .ok()?
        .token_text_trimmed();
    if member_name.text() != "length" {
        return None;
    }
    node.object().ok()
}

/// AnyJsExpressiion -> Some(i64) if the expression is an integer literal, otherwise None.
fn get_integer_from_literal(node: &AnyJsExpression) -> Option<i64> {
    if let AnyJsExpression::JsUnaryExpression(unary) = node {
        let token = unary.operator_token().ok()?;
        if token.kind() != T![-] {
            return None;
        }
        return get_integer_from_literal(&solve_parenthesized_expression(&unary.argument().ok()?)?)
            .map(|num| -num);
    }
    let AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(
        number,
    )) = node
    else {
        return None;
    };
    let number = number.as_number()?;
    if number.fract() == 0.0 {
        Some(i64::try_from(number as i128).ok()?)
    } else {
        None
    }
}

/// If the node is a negative index, it returns the negative index.
/// # Examples
/// ```js
///     hoge[hoge.length - 0] // => None
///     hoge[hoge.length - 1] // => Some(-1)
///     hoge[fuga.length - 2] // => None
/// ```
fn get_negative_index(
    member: &AnyJsExpression,
    object: &AnyJsExpression,
) -> Option<AnyJsExpression> {
    let AnyJsExpression::JsBinaryExpression(member) = member else {
        return None;
    };
    let token = member.operator_token().ok()?;
    if token.kind() != T![-] {
        return None;
    }
    // left expression should be hoge.length
    let left = solve_parenthesized_expression(&member.left().ok()?)?;
    let length_parent = get_length_node(&left)?;
    // left expression should be the same as the object
    if !is_same_reference(object, &length_parent)? {
        return None;
    }
    let number_exp = solve_parenthesized_expression(&member.right().ok()?)?;
    // right expression should be integer
    let number = get_integer_from_literal(&number_exp)?;
    if number > 0 {
        Some(AnyJsExpression::JsUnaryExpression(
            make::js_unary_expression(make::token(T![-]), number_exp),
        ))
    } else {
        None
    }
}

/// Is the node a child node of `delete`?
fn is_delete_child(node: &AnyJsExpression) -> Option<bool> {
    node.syntax().parent()?.ancestors().find_map(|ancestor| {
        if let Some(unary) = JsUnaryExpression::cast(ancestor.clone()) {
            unary
                .operator_token()
                .ok()
                .map(|token| token.kind() == T![delete])
                .or(Some(false))
        } else {
            (!JsParenthesizedExpression::can_cast(ancestor.kind())).then_some(false)
        }
    })
}

fn make_number_literal(value: i64) -> AnyJsExpression {
    AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(
        make::js_number_literal_expression(make::js_number_literal(value)),
    ))
}

/// check if the node is a zero-argument method.
// fn check_zero_arg_method(node: &JsCallExpression) -> Option<(String, AnyJsExpression)> {
//     let member = node.callee().ok()?;
//     match member {
//         AnyJsExpression::JsStaticMemberExpression(member) => {
//             let member_name = member.member().ok()?;
//             let member_name = member_name.as_js_name()?.value_token().ok()?;
//             let member_name = member_name.token_text_trimmed();
//             let args = node.arguments().ok()?.args();
//             if args.into_iter().count() != 0 {
//                 return None;
//             } else {
//                 Some((member_name.text().to_string(), member.object().ok()?))
//             }

//         },
//         _ => None,
//     }
// }

/// check if the node is a slice
/// # Examples
/// ```js
///     .slice(0)[0]
///     .slice(0, 1).pop(0)
/// ```
fn check_get_element_by_slice(node: &AnyJsExpression) -> Option<UseAtIndexState> {
    if is_delete_child(node).unwrap_or(false) {
        return None;
    }
    // selector
    let (selected_exp, at_value): (AnyJsExpression, i64) = match node {
        // .pop() or .shift()
        AnyJsExpression::JsCallExpression(call_exp) => {
            let arg_length = call_exp.arguments().ok()?.args().into_iter().count();
            if arg_length != 0 {
                return None;
            }
            let member = solve_parenthesized_expression(&call_exp.callee().ok()?)?;
            let AnyJsExpression::JsStaticMemberExpression(member) = member else {
                return None;
            };
            if call_exp.is_optional_chain() || member.is_optional_chain() {
                return None;
            }
            let member_name = member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed();
            let object = solve_parenthesized_expression(&member.object().ok()?)?;
            if member_name == "pop" {
                (object, -1)
            } else if member_name == "shift" {
                (object, 0)
            } else {
                return None;
            }
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let object = solve_parenthesized_expression(&member.object().ok()?)?;
            if member.is_optional_chain() {
                return None;
            }
            let value =
                get_integer_from_literal(&solve_parenthesized_expression(&member.member().ok()?)?)?;
            // enable only x[0]
            if value != 0 {
                return None;
            }
            (object, value)
        }
        _ => return None,
    };
    // .slice(0,1)
    let AnyJsExpression::JsCallExpression(call_exp) = selected_exp else {
        return None;
    };
    let AnyJsExpression::JsStaticMemberExpression(member) = call_exp.callee().ok()? else {
        return None;
    };
    let member_name = member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .token_text_trimmed();
    if member_name != "slice" {
        return None;
    }
    // arg length should be 1 or 2
    let args: Vec<_> = call_exp
        .arguments()
        .ok()?
        .args()
        .into_iter()
        .flatten()
        .collect();
    if args.is_empty() || args.len() > 2 {
        return None;
    }
    let AnyJsCallArgument::AnyJsExpression(arg0) = &args[0] else {
        return None;
    };
    let start_exp = solve_parenthesized_expression(arg0)?;
    let start_index = get_integer_from_literal(&start_exp)?;

    let sliced_exp = member.object().ok()?;

    if args.len() == 1 {
        if (at_value == 0) || (start_index == -1 && at_value == -1) {
            return Some(UseAtIndexState {
                at_number_exp: start_exp.trim_trivia()?,
                error_type: ErrorType::Slice,
                object: sliced_exp,
            });
        }
        return None;
    }
    let AnyJsCallArgument::AnyJsExpression(arg1) = &args[1] else {
        return None;
    };
    let end_exp = solve_parenthesized_expression(arg1)?;
    let end_index = get_integer_from_literal(&end_exp)?;
    // enable only x.slice(2, 4)
    if start_index * end_index >= 0 && start_index < end_index {
        if at_value == 0 {
            Some(UseAtIndexState {
                at_number_exp: start_exp.trim_trivia()?,
                error_type: ErrorType::Slice,
                object: sliced_exp,
            })
        } else {
            Some(UseAtIndexState {
                at_number_exp: make_number_literal(end_index - 1),
                error_type: ErrorType::Slice,
                object: sliced_exp,
            })
        }
    } else {
        None
    }
}

/// make `object.at(arg)`
fn make_at_method(object: AnyJsExpression, arg: AnyJsExpression) -> JsCallExpression {
    let at_member = make::js_static_member_expression(
        object,
        make::token(T![.]),
        make::js_name(make::ident("at")).into(),
    );
    let args = make::js_call_arguments(
        make::token(T!['(']),
        make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(arg)], []),
        make::token(T![')']),
    );
    make::js_call_expression(at_member.into(), args).build()
}

#[derive(
    Clone,
    Debug,
    Default,
    biome_deserialize_macros::Deserializable,
    Deserialize,
    Serialize,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseAtIndexOptions {
    // Force the use of the `.at()` method in cases other than positive integers.
    pub check_all_index_access: bool,
}

impl Rule for UseAtIndex {
    type Query = Ast<AnyJsArrayAccess>;
    type State = UseAtIndexState;
    type Signals = Option<Self::State>;
    type Options = UseAtIndexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let exp = ctx.query();

        let result: Option<UseAtIndexState> = match exp {
            // hoge[a]
            AnyJsArrayAccess::JsComputedMemberExpression(exp) => {
                // check slice
                if let Some(slice_err) = check_get_element_by_slice(
                    &AnyJsExpression::JsComputedMemberExpression(exp.clone()),
                ) {
                    return Some(slice_err);
                }
                // invalid optional chain
                if exp.is_optional_chain() {
                    return None;
                }
                // invalid mutable case
                if is_delete_child(&AnyJsExpression::JsComputedMemberExpression(exp.clone()))
                    .unwrap_or(false)
                {
                    return None;
                }
                // check member
                let member = solve_parenthesized_expression(&exp.member().ok()?)?;
                match member.clone() {
                    // hoge[hoge.length - 1]
                    AnyJsExpression::JsBinaryExpression(_binary) => Some(UseAtIndexState {
                        at_number_exp: get_negative_index(
                            &member,
                            &solve_parenthesized_expression(&exp.object().ok()?)?,
                        )?,
                        error_type: ErrorType::NegativeIndex,
                        object: exp.object().ok()?,
                    }),
                    // hoge[1]
                    AnyJsExpression::AnyJsLiteralExpression(member) => {
                        let AnyJsLiteralExpression::JsNumberLiteralExpression(member) = member
                        else {
                            return None;
                        };
                        let value_token = member.value_token().ok()?;
                        let number = value_token.text_trimmed();
                        if let Ok(number) = number.parse::<i64>() {
                            if number >= 0 {
                                let option = ctx.options();
                                option.check_all_index_access.then_some(UseAtIndexState {
                                    at_number_exp: make_number_literal(number),
                                    error_type: ErrorType::IdIndex,
                                    object: exp.object().ok()?,
                                })
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            // hoge.fuga()
            AnyJsArrayAccess::JsCallExpression(call_exp) => {
                // check slice
                if let Some(slice_err) =
                    check_get_element_by_slice(&AnyJsExpression::JsCallExpression(call_exp.clone()))
                {
                    return Some(slice_err);
                }

                if call_exp.is_optional_chain() {
                    return None;
                }

                let member = solve_parenthesized_expression(&call_exp.callee().ok()?)?;
                match member {
                    AnyJsExpression::JsStaticMemberExpression(member) => {
                        if member.is_optional_chain() {
                            return None;
                        }
                        let member_name = member
                            .member()
                            .ok()?
                            .as_js_name()?
                            .value_token()
                            .ok()?
                            .token_text_trimmed();
                        match member_name.text() {
                            "last" => {
                                let args: Vec<_> = call_exp
                                    .arguments()
                                    .ok()?
                                    .args()
                                    .into_iter()
                                    .flatten()
                                    .collect();
                                if args.len() != 1 {
                                    return None;
                                }
                                let object = member.object().ok()?;
                                let AnyJsExpression::JsIdentifierExpression(object) = object else {
                                    return None;
                                };
                                let lodash_function = ["_", "lodash", "underscore"];
                                let object_name = object.syntax().text().to_string();
                                if lodash_function.contains(&object_name.as_str()) {
                                    let AnyJsCallArgument::AnyJsExpression(arg0) = &args[0] else {
                                        return None;
                                    };
                                    Some(UseAtIndexState {
                                        at_number_exp: make_number_literal(-1),
                                        error_type: ErrorType::GetLastFunction,
                                        object: solve_parenthesized_expression(arg0)?,
                                    })
                                } else {
                                    None
                                }
                            }
                            "charAt" => {
                                let args: Vec<_> = call_exp
                                    .arguments()
                                    .ok()?
                                    .args()
                                    .into_iter()
                                    .flatten()
                                    .collect();
                                if args.len() != 1 {
                                    return None;
                                }
                                let AnyJsCallArgument::AnyJsExpression(arg0) = &args[0] else {
                                    return None;
                                };
                                let core_arg0 = solve_parenthesized_expression(arg0)?;
                                let char_at_parent =
                                    &solve_parenthesized_expression(&member.object().ok()?)?;
                                match core_arg0.clone() {
                                    // hoge.charAt(hoge.length - 1)
                                    AnyJsExpression::JsBinaryExpression(_) => {
                                        let at_number_exp_2 =
                                            get_negative_index(&core_arg0, char_at_parent)?;
                                        Some(UseAtIndexState {
                                            at_number_exp: at_number_exp_2,
                                            error_type: ErrorType::StringCharAtNegativeIndex,
                                            object: char_at_parent.clone(),
                                        })
                                    }
                                    // hoge.charAt(1)
                                    AnyJsExpression::AnyJsLiteralExpression(_member) => {
                                        let number = get_integer_from_literal(&core_arg0)?;
                                        let option = ctx.options();
                                        option.check_all_index_access.then_some(UseAtIndexState {
                                            at_number_exp: make_number_literal(number),
                                            error_type: ErrorType::StringCharAt,
                                            object: char_at_parent.clone(),
                                        })
                                    }
                                    _ => None,
                                }
                            }
                            //"lastIndexOf" => Some(ErrorType::GetLastFunction),
                            _ => None,
                        }
                    }
                    _ => return None,
                }
            }
        };
        result
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Replace index references with "<Emphasis>".at()"</Emphasis>"."
            },
        ).note(
            match state.error_type {
                ErrorType::NegativeIndex => {
                    markup! { "Prefer "<Emphasis>"X.at(-Y)"</Emphasis>" over "<Emphasis>"X[X.length - Y]"</Emphasis>"." }
                }
                ErrorType::IdIndex => {
                    markup! { "Prefer "<Emphasis>"X.at(Y)"</Emphasis>" over "<Emphasis>"X[Y]"</Emphasis>"." }
                }
                ErrorType::StringCharAtNegativeIndex => {
                    markup! { "Prefer "<Emphasis>"X.at(-Y)"</Emphasis>" over "<Emphasis>"X.charAt(X.length - Y)"</Emphasis>"." }
                }
                ErrorType::StringCharAt => {
                    markup! { "Prefer "<Emphasis>"X.at(Y)"</Emphasis>" over "<Emphasis>"X.charAt(Y)"</Emphasis>"." }
                }
                ErrorType::Slice => {
                    markup! { "Prefer "<Emphasis>"X.at(Y)"</Emphasis>" over "<Emphasis>"X.slice(Y)[0]"</Emphasis>"." }
                }
                ErrorType::GetLastFunction => {
                    markup! { "Prefer "<Emphasis>"X.at(-1)"</Emphasis>" over "<Emphasis>"_.last(X)"</Emphasis>"." }
                }
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let prev_node = match node {
            AnyJsArrayAccess::JsComputedMemberExpression(node) => {
                AnyJsExpression::JsComputedMemberExpression(node.clone())
            }
            AnyJsArrayAccess::JsCallExpression(node) => {
                AnyJsExpression::JsCallExpression(node.clone())
            }
        };
        let UseAtIndexState {
            at_number_exp,
            error_type: _,
            object,
        } = state;
        let object = match object {
            AnyJsExpression::JsArrayExpression(exp) => {
                AnyJsExpression::JsArrayExpression(exp.clone())
            }
            AnyJsExpression::JsCallExpression(exp) => {
                AnyJsExpression::JsCallExpression(exp.clone())
            }
            AnyJsExpression::JsComputedMemberExpression(exp) => {
                AnyJsExpression::JsComputedMemberExpression(exp.clone())
            }
            AnyJsExpression::JsIdentifierExpression(exp) => {
                AnyJsExpression::JsIdentifierExpression(exp.clone().trim_trivia()?)
            }
            AnyJsExpression::JsParenthesizedExpression(exp) => {
                AnyJsExpression::JsParenthesizedExpression(exp.clone())
            }
            AnyJsExpression::JsStaticMemberExpression(exp) => {
                AnyJsExpression::JsStaticMemberExpression(exp.clone())
            }
            _ => AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
                make::token(T!['(']),
                object.clone(),
                make::token(T![')']),
            )),
        };

        mutation.replace_node(
            prev_node,
            AnyJsExpression::JsCallExpression(make_at_method(object, at_number_exp.clone())),
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace index references with "<Emphasis>".at()"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

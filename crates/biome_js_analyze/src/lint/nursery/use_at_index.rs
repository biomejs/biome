use ::serde::{Deserialize, Serialize};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::{markup, MarkupBuf};
use biome_js_factory::make::{self};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression,
    JsCallExpression, JsComputedMemberExpression, JsParenthesizedExpression,
    JsStaticMemberExpression, JsUnaryExpression, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Use `at()` instead of integer index access.
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

/// If the node is a parenthized expression, it returns the expression inside.
/// # Examples
/// ```js
///    a // Some(a)
///    (a) // Some(a)
///    (a + b) // Some(a + b)
/// ```
fn solve_parenthesized_expression(mut node: AnyJsExpression) -> Option<AnyJsExpression> {
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized_exp) = node {
        let exp = parenthesized_exp.expression().ok()?;
        node = exp;
    }
    Some(node)
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
fn is_same_reference(left: AnyJsExpression, right: AnyJsExpression) -> Option<bool> {
    // solve JsParenthesizedExpression
    let left = solve_parenthesized_expression(left)?;
    let right = solve_parenthesized_expression(right)?;
    match (left, right) {
        // x[0]
        (
            AnyJsExpression::JsComputedMemberExpression(left),
            AnyJsExpression::JsComputedMemberExpression(right),
        ) => {
            let AnyJsExpression::AnyJsLiteralExpression(left_member) =
                solve_parenthesized_expression(left.member().ok()?)?
            else {
                return Some(false);
            };
            let AnyJsExpression::AnyJsLiteralExpression(right_member) =
                solve_parenthesized_expression(right.member().ok()?)?
            else {
                return Some(false);
            };
            if left_member.text() != right_member.text() {
                return Some(false);
            }
            is_same_reference(left.object().ok()?, right.object().ok()?)
        }
        // x.y
        (
            AnyJsExpression::JsStaticMemberExpression(left),
            AnyJsExpression::JsStaticMemberExpression(right),
        ) => {
            let left_member = left.member().ok()?;
            let right_member = right.member().ok()?;
            if left_member.text() != right_member.text() {
                Some(false)
            } else {
                is_same_reference(left.object().ok()?, right.object().ok()?)
            }
        }
        // x
        (
            AnyJsExpression::JsIdentifierExpression(left),
            AnyJsExpression::JsIdentifierExpression(right),
        ) => Some(left.name().ok()?.text() == right.name().ok()?.text()),
        // this
        (AnyJsExpression::JsThisExpression(_), AnyJsExpression::JsThisExpression(_)) => Some(true),
        _ => Some(false),
    }
}

/// When using this expression in other operations, enclose it in parentheses as needed.
fn overwrap_parentheses_expression(node: &AnyJsExpression) -> Option<AnyJsExpression> {
    match node {
        AnyJsExpression::JsArrayExpression(exp) => {
            Some(AnyJsExpression::JsArrayExpression(exp.clone()))
        }
        AnyJsExpression::JsCallExpression(exp) => {
            Some(AnyJsExpression::JsCallExpression(exp.clone()))
        }
        AnyJsExpression::JsComputedMemberExpression(exp) => {
            Some(AnyJsExpression::JsComputedMemberExpression(exp.clone()))
        }
        AnyJsExpression::JsIdentifierExpression(exp) => Some(
            AnyJsExpression::JsIdentifierExpression(exp.clone().trim_trivia()?),
        ),
        AnyJsExpression::JsParenthesizedExpression(exp) => {
            Some(AnyJsExpression::JsParenthesizedExpression(exp.clone()))
        }
        AnyJsExpression::JsStaticMemberExpression(exp) => {
            Some(AnyJsExpression::JsStaticMemberExpression(exp.clone()))
        }
        _ => Some(AnyJsExpression::JsParenthesizedExpression(
            make::js_parenthesized_expression(
                make::token(T!['(']),
                node.clone(),
                make::token(T![')']),
            ),
        )),
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
        return get_integer_from_literal(&solve_parenthesized_expression(unary.argument().ok()?)?)
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

/// Retrieve the value subtracted from the subtraction expression.
/// # Examples
/// ```js
///    a - b // => Some((a, [b]))
///    a - b - c // => Some((a, [b, c]))
/// ```
fn get_left_node_from_minus_binary_expressions(
    mut expression: AnyJsExpression,
) -> Option<(AnyJsExpression, Vec<AnyJsExpression>)> {
    let mut right_list = vec![];

    while let AnyJsExpression::JsBinaryExpression(binary) = expression {
        let token = binary.operator_token().ok()?;
        if token.kind() != T![-] {
            return Some((AnyJsExpression::JsBinaryExpression(binary), right_list));
        }

        right_list.push(binary.right().ok()?);
        expression = binary.left().ok()?;
    }
    Some((expression, right_list))
}

/// Combine the expressions in the list with the addition operator.
fn make_plus_binary_expression(list: Vec<AnyJsExpression>) -> Option<AnyJsExpression> {
    list.into_iter().rev().reduce(|left, right| {
        AnyJsExpression::JsBinaryExpression(make::js_binary_expression(
            left,
            make::token(T![+]),
            right,
        ))
    })
}

/// If the node is a negative index, it returns the negative index.
/// # Examples
/// ```js
///     hoge[hoge.length - 0] // => None
///     hoge[hoge.length - 1] // => Some(-1)
///     hoge[fuga.length - 2] // => None
/// ```
fn extract_negative_index_expression(
    member: &AnyJsExpression,
    object: AnyJsExpression,
) -> Option<AnyJsExpression> {
    let (left, right_list) = get_left_node_from_minus_binary_expressions(member.clone())?;
    if right_list.is_empty() {
        return None;
    }

    // left expression should be hoge.length
    let left = solve_parenthesized_expression(left)?;
    let length_parent = get_length_node(&left)?;
    // left expression should be the same as the object
    if !is_same_reference(object, length_parent)? {
        return None;
    }

    if right_list.len() == 1 {
        // right expression should be integer
        if let Some(number) =
            get_integer_from_literal(&solve_parenthesized_expression(right_list[0].clone())?)
        {
            if number > 0 {
                Some(AnyJsExpression::JsUnaryExpression(
                    make::js_unary_expression(make::token(T![-]), right_list[0].clone()),
                ))
            } else {
                None
            }
        } else {
            Some(AnyJsExpression::JsUnaryExpression(
                make::js_unary_expression(
                    make::token(T![-]),
                    overwrap_parentheses_expression(&right_list[0])?,
                ),
            ))
        }
    } else {
        make_plus_binary_expression(right_list)
    }
}

/// Is the node a child node of `delete`?
fn is_within_delete_expression(node: &AnyJsExpression) -> Option<bool> {
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

/// check if the node is a slice
/// # Examples
/// ```js
///     .slice(0)[0]
///     .slice(0, 1).pop(0)
/// ```
fn analyze_slice_element_access(node: &AnyJsExpression) -> Option<UseAtIndexState> {
    if is_within_delete_expression(node).unwrap_or(false) {
        return None;
    }
    // selector
    let (selected_exp, extract_type): (AnyJsExpression, SliceExtractType) = match node {
        // .pop() or .shift()
        AnyJsExpression::JsCallExpression(call_exp) => {
            let arg_length = call_exp.arguments().ok()?.args().into_iter().count();
            if arg_length != 0 {
                return None;
            }
            let member = solve_parenthesized_expression(call_exp.callee().ok()?)?;
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
            let object = solve_parenthesized_expression(member.object().ok()?)?;
            if member_name == "pop" {
                (object, SliceExtractType::Pop)
            } else if member_name == "shift" {
                (object, SliceExtractType::Shift)
            } else {
                return None;
            }
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let object = solve_parenthesized_expression(member.object().ok()?)?;
            if member.is_optional_chain() {
                return None;
            }
            let value =
                get_integer_from_literal(&solve_parenthesized_expression(member.member().ok()?)?)?;
            // enable only x[0]
            if value != 0 {
                return None;
            }
            (object, SliceExtractType::ZeroMember)
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
    let AnyJsCallArgument::AnyJsExpression(arg0) = args[0].clone() else {
        return None;
    };
    let start_exp = solve_parenthesized_expression(arg0)?;
    let sliced_exp = member.object().ok()?;

    match (extract_type.clone(), args.len()) {
        (SliceExtractType::ZeroMember | SliceExtractType::Shift, 1) => Some(UseAtIndexState::new(
            start_exp,
            ErrorType::Slice {
                arg_type: SliceArgType::OneArg,
                extract_type,
            },
            sliced_exp,
        )),
        (SliceExtractType::Pop, 1) if get_integer_from_literal(&start_exp)? < 0 => {
            Some(UseAtIndexState::new(
                make_number_literal(-1),
                ErrorType::Slice {
                    arg_type: SliceArgType::OneArg,
                    extract_type: SliceExtractType::Pop,
                },
                sliced_exp,
            ))
        }
        (SliceExtractType::ZeroMember | SliceExtractType::Shift, 2) => {
            let start_index = get_integer_from_literal(&start_exp)?;
            let end_index = get_integer_from_literal(&solve_parenthesized_expression(
                args[1].as_any_js_expression()?.clone(),
            )?)?;
            (start_index * end_index >= 0 && start_index < end_index).then_some(
                UseAtIndexState::new(
                    start_exp,
                    ErrorType::Slice {
                        arg_type: SliceArgType::TwoArg,
                        extract_type,
                    },
                    sliced_exp,
                ),
            )
        }
        (SliceExtractType::Pop, 2) => {
            let start_index = get_integer_from_literal(&start_exp)?;
            let end_index = get_integer_from_literal(&solve_parenthesized_expression(
                args[1].as_any_js_expression()?.clone(),
            )?)?;
            (start_index * end_index >= 0 && start_index < end_index).then_some(
                UseAtIndexState::new(
                    make_number_literal(end_index - 1),
                    ErrorType::Slice {
                        arg_type: SliceArgType::TwoArg,
                        extract_type: SliceExtractType::Pop,
                    },
                    sliced_exp,
                ),
            )
        }
        _ => None,
    }
}

fn check_binary_expression_member(
    member: JsBinaryExpression,
    object: AnyJsExpression,
    option: &UseAtIndexOptions,
) -> Option<UseAtIndexState> {
    let member = AnyJsExpression::JsBinaryExpression(member);
    let negative_index_exp =
        extract_negative_index_expression(&member, solve_parenthesized_expression(object.clone())?);
    if let Some(negative_index) = negative_index_exp {
        return Some(UseAtIndexState::new(
            negative_index,
            ErrorType::Index { is_negative: true },
            object,
        ));
    }
    option
        .check_all_index_access
        .then_some(UseAtIndexState::new(
            member,
            ErrorType::Index { is_negative: true },
            object,
        ))
}

fn check_literal_expression_member(
    member: AnyJsLiteralExpression,
    object: AnyJsExpression,
    option: &UseAtIndexOptions,
) -> Option<UseAtIndexState> {
    let AnyJsLiteralExpression::JsNumberLiteralExpression(member) = member else {
        return None;
    };
    let value_token = member.value_token().ok()?;
    let number = value_token.text_trimmed().parse::<i64>().ok()?;
    (number >= 0 && option.check_all_index_access).then_some(UseAtIndexState::new(
        make_number_literal(number),
        ErrorType::Index { is_negative: false },
        object,
    ))
}

fn check_unary_expression_member(
    member: JsUnaryExpression,
    object: AnyJsExpression,
    option: &UseAtIndexOptions,
) -> Option<UseAtIndexState> {
    if !option.check_all_index_access {
        return None;
    }
    // ignore -5
    let token = member.operator_token().ok()?;
    if token.kind() == T![-] {
        if let Some(arg) =
            get_integer_from_literal(&solve_parenthesized_expression(member.argument().ok()?)?)
        {
            if arg >= 0 {
                return None;
            }
        }
    }
    Some(UseAtIndexState::new(
        AnyJsExpression::JsUnaryExpression(member),
        ErrorType::Index { is_negative: false },
        object,
    ))
}

/// check hoge[0]
fn check_computed_member_expression(
    exp: &JsComputedMemberExpression,
    option: &UseAtIndexOptions,
) -> Option<UseAtIndexState> {
    // check slice
    if let Some(slice_err) =
        analyze_slice_element_access(&AnyJsExpression::JsComputedMemberExpression(exp.clone()))
    {
        return Some(slice_err);
    }
    // invalid optional chain, mutable case
    if exp.is_optional_chain()
        || is_within_delete_expression(&AnyJsExpression::JsComputedMemberExpression(exp.clone()))
            .unwrap_or(false)
    {
        return None;
    }
    // check member
    let member = solve_parenthesized_expression(exp.member().ok()?)?;
    let object = exp.object().ok()?;
    match member.clone() {
        // hoge[hoge.length - 1]
        AnyJsExpression::JsBinaryExpression(binary) => {
            check_binary_expression_member(binary, object, option)
        }
        // hoge[1]
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            check_literal_expression_member(literal, object, option)
        }
        // hoge[-x]
        AnyJsExpression::JsUnaryExpression(unary) => {
            check_unary_expression_member(unary, object, option)
        }
        AnyJsExpression::JsIdentifierExpression(_) => None,
        _ if option.check_all_index_access => Some(UseAtIndexState::new(
            member,
            ErrorType::Index { is_negative: false },
            object,
        )),
        _ => None,
    }
}

fn check_call_expression_last(
    call_exp: &JsCallExpression,
    member: &JsStaticMemberExpression,
) -> Option<UseAtIndexState> {
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
        let AnyJsCallArgument::AnyJsExpression(arg0) = args[0].clone() else {
            return None;
        };
        Some(UseAtIndexState::new(
            make_number_literal(-1),
            ErrorType::GetLastFunction,
            solve_parenthesized_expression(arg0)?,
        ))
    } else {
        None
    }
}

fn check_call_expression_char_at(
    call_exp: &JsCallExpression,
    member: &JsStaticMemberExpression,
    option: &UseAtIndexOptions,
) -> Option<UseAtIndexState> {
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
    let AnyJsCallArgument::AnyJsExpression(arg0) = args[0].clone() else {
        return None;
    };
    let core_arg0 = solve_parenthesized_expression(arg0)?;
    let char_at_parent = solve_parenthesized_expression(member.object().ok()?)?;
    match core_arg0.clone() {
        // hoge.charAt(hoge.length - 1)
        AnyJsExpression::JsBinaryExpression(_) => {
            let at_number_exp =
                extract_negative_index_expression(&core_arg0, char_at_parent.clone());
            if let Some(at_number_exp) = at_number_exp {
                Some(UseAtIndexState::new(
                    at_number_exp,
                    ErrorType::StringCharAt { is_negative: true },
                    char_at_parent,
                ))
            } else {
                option
                    .check_all_index_access
                    .then_some(UseAtIndexState::new(
                        core_arg0,
                        ErrorType::StringCharAt { is_negative: false },
                        char_at_parent,
                    ))
            }
        }
        // hoge.charAt(1)
        AnyJsExpression::AnyJsLiteralExpression(_member) => option
            .check_all_index_access
            .then_some(UseAtIndexState::new(
                core_arg0,
                ErrorType::StringCharAt { is_negative: false },
                char_at_parent.clone(),
            )),
        _ => option
            .check_all_index_access
            .then_some(UseAtIndexState::new(
                core_arg0,
                ErrorType::StringCharAt { is_negative: false },
                char_at_parent.clone(),
            )),
    }
}

/// check hoge.fuga()
fn check_call_expression(
    call_exp: &JsCallExpression,
    option: &UseAtIndexOptions,
) -> Option<UseAtIndexState> {
    // check slice
    if let Some(slice_err) =
        analyze_slice_element_access(&AnyJsExpression::JsCallExpression(call_exp.clone()))
    {
        return Some(slice_err);
    }

    if call_exp.is_optional_chain() {
        return None;
    }

    let member = solve_parenthesized_expression(call_exp.callee().ok()?)?;
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
                "last" => check_call_expression_last(call_exp, &member),
                "charAt" => check_call_expression_char_at(call_exp, &member, option),
                //"lastIndexOf" => Some(ErrorType::GetLastFunction),
                _ => None,
            }
        }
        _ => None,
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

/// The method to retrieve values from `.slice()`
#[derive(Clone)]
pub enum SliceExtractType {
    Pop,
    Shift,
    ZeroMember,
}

/// The number of arguments for `.slice()`
#[derive(Clone)]
pub enum SliceArgType {
    OneArg,
    TwoArg,
}

/// Type of Code to Fix
#[derive(Clone)]
pub enum ErrorType {
    Index {
        is_negative: bool,
    },
    StringCharAt {
        is_negative: bool,
    },
    Slice {
        arg_type: SliceArgType,
        extract_type: SliceExtractType,
    },
    GetLastFunction,
}

impl ErrorType {
    /// Return the error message corresponding to the ErrorType.
    fn get_error_message(self: &ErrorType) -> MarkupBuf {
        match self {
            ErrorType::Index { is_negative } | ErrorType::StringCharAt { is_negative } => {
                let (method, old_method) = if *is_negative {
                    ("X.at(-Y)", if matches!(self, ErrorType::StringCharAt { .. }) { "X.charAt(X.length - Y)" } else { "X[X.length - Y]" })
                } else {
                    ("X.at(Y)", if matches!(self, ErrorType::StringCharAt { .. }) { "X.charAt(Y)" } else { "X[Y]" })
                };
                markup! { "Prefer "<Emphasis>{method}</Emphasis>" over "<Emphasis>{old_method}</Emphasis>"." }.to_owned()
            },
            ErrorType::Slice { arg_type, extract_type } => {
                let extract_string = match extract_type {
                    SliceExtractType::Pop => ".pop()",
                    SliceExtractType::Shift => ".shift()",
                    SliceExtractType::ZeroMember => "[0]",
                };
                let (method, old_method) = match (arg_type, extract_type) {
                    (SliceArgType::OneArg, SliceExtractType::Pop) => ("X.at(-1)", format!("X.slice(-a){}", extract_string)),
                    (SliceArgType::TwoArg, SliceExtractType::Pop) => ("X.at(Y - 1)", format!("X.slice(a, Y){}", extract_string)),
                    _ => ("X.at(Y)", format!("X.slice({}){}", if matches!(arg_type, SliceArgType::OneArg) { "Y" } else { "Y, a" }, extract_string)),
                };
                markup! { "Prefer "<Emphasis>{method}</Emphasis>" over "<Emphasis>{old_method}</Emphasis>"." }.to_owned()
            },
            ErrorType::GetLastFunction => {
                markup! { "Prefer "<Emphasis>"X.at(-1)"</Emphasis>" over "<Emphasis>"_.last(X)"</Emphasis>"." }.to_owned()
            }
        }
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

impl UseAtIndexState {
    fn new(at_number_exp: AnyJsExpression, error_type: ErrorType, object: AnyJsExpression) -> Self {
        Self {
            at_number_exp,
            error_type,
            object,
        }
    }
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
        let option = ctx.options();

        let result: Option<UseAtIndexState> = match exp {
            // hoge[a]
            AnyJsArrayAccess::JsComputedMemberExpression(exp) => {
                check_computed_member_expression(exp, option)
            }
            // hoge.fuga()
            AnyJsArrayAccess::JsCallExpression(call_exp) => check_call_expression(call_exp, option),
        };
        result
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Replace index references with "<Emphasis>".at()"</Emphasis>"."
                }
                .to_owned(),
            )
            .note(state.error_type.get_error_message()),
        )
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
        let object = overwrap_parentheses_expression(object)?;

        mutation.replace_node(
            prev_node,
            AnyJsExpression::JsCallExpression(make_at_method(
                object,
                at_number_exp.clone().trim_trivia()?,
            )),
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace index references with "<Emphasis>".at()"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

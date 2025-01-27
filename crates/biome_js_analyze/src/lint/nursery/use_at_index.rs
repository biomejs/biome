use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::{markup, MarkupBuf};
use biome_js_factory::make::{self};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression,
    JsCallExpression, JsComputedMemberExpression, JsParenthesizedExpression,
    JsStaticMemberExpression, JsUnaryExpression, T,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, BatchMutationExt};

declare_lint_rule! {
    /// Use `at()` instead of integer index access.
    ///
    /// Accessing an element at the end of an array or a string is inconvenient because you have to subtract the length of the array or the string from the backward 1-based index of the element to access.
    /// For example, to access the last element of an array or a string, you would have to write `array[array.length - 1]`.
    /// A more convenient way to achieve the same thing is to use the `at()` method with a negative index.
    /// To access the last element of an array or a string just write `array.at(-1)`.
    ///
    /// This rule enforces the usage of `at()` over index access, `charAt()`, and `slice()[0]` when `at()` is more convenient.
    ///
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
        version: "1.9.4",
        name: "useAtIndex",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-at")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
    }
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
}

declare_node_union! {
    pub AnyJsArrayAccess = JsComputedMemberExpression | JsCallExpression
}

pub struct UseAtIndexState {
    at_number_exp: AnyJsExpression,
    error_type: ErrorType,
    object: AnyJsExpression,
}

impl Rule for UseAtIndex {
    type Query = Ast<AnyJsArrayAccess>;
    type State = UseAtIndexState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let exp = ctx.query();

        let result: Option<UseAtIndexState> = match exp {
            // foo[a]
            AnyJsArrayAccess::JsComputedMemberExpression(exp) => {
                check_computed_member_expression(exp)
            }
            // foo.bar()
            AnyJsArrayAccess::JsCallExpression(call_exp) => check_call_expression(call_exp),
        };
        result
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                state.error_type.get_error_message(),
            )
            .note(markup! {
                "Using "<Emphasis>".at()"</Emphasis>" is more convenient and is easier to read."
            }),
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
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>".at()"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

impl ErrorType {
    /// Return the error message corresponding to the ErrorType.
    fn get_error_message(self: &ErrorType) -> MarkupBuf {
        match self {
            ErrorType::Index { is_negative } | ErrorType::StringCharAt { is_negative } => {
                let (method, old_method) = if *is_negative {
                    (
                        "X.at(-Y)",
                        if matches!(self, ErrorType::StringCharAt { .. }) {
                            "X.charAt(X.length - Y)"
                        } else {
                            "X[X.length - Y]"
                        },
                    )
                } else {
                    (
                        "X.at(Y)",
                        if matches!(self, ErrorType::StringCharAt { .. }) {
                            "X.charAt(Y)"
                        } else {
                            "X[Y]"
                        },
                    )
                };
                markup! { "Prefer "<Emphasis>{method}</Emphasis>" over "<Emphasis>{old_method}</Emphasis>"." }.to_owned()
            }
            ErrorType::Slice {
                arg_type,
                extract_type,
            } => {
                let extract_string = match extract_type {
                    SliceExtractType::Pop => ".pop()",
                    SliceExtractType::Shift => ".shift()",
                    SliceExtractType::ZeroMember => "[0]",
                };
                let (method, old_method) = match (arg_type, extract_type) {
                    (SliceArgType::OneArg, SliceExtractType::Pop) => {
                        ("X.at(-1)", format!("X.slice(-a){extract_string}"))
                    }
                    (SliceArgType::TwoArg, SliceExtractType::Pop) => {
                        ("X.at(Y - 1)", format!("X.slice(a, Y){extract_string}"))
                    }
                    _ => (
                        "X.at(Y)",
                        format!(
                            "X.slice({}){}",
                            if matches!(arg_type, SliceArgType::OneArg) {
                                "Y"
                            } else {
                                "Y, a"
                            },
                            extract_string
                        ),
                    ),
                };
                markup! { "Prefer "<Emphasis>{method}</Emphasis>" over "<Emphasis>{old_method}</Emphasis>"." }.to_owned()
            }
        }
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
fn is_same_reference(left: AnyJsExpression, right: AnyJsExpression) -> Option<bool> {
    // solve JsParenthesizedExpression
    let left = left.omit_parentheses();
    let right = right.omit_parentheses();
    match (left, right) {
        // x[0]
        (
            AnyJsExpression::JsComputedMemberExpression(left),
            AnyJsExpression::JsComputedMemberExpression(right),
        ) => {
            let AnyJsExpression::AnyJsLiteralExpression(left_member) =
                left.member().ok()?.omit_parentheses()
            else {
                return Some(false);
            };
            let AnyJsExpression::AnyJsLiteralExpression(right_member) =
                right.member().ok()?.omit_parentheses()
            else {
                return Some(false);
            };
            if left_member.to_trimmed_string() != right_member.to_trimmed_string() {
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
            if left_member.to_trimmed_string() != right_member.to_trimmed_string() {
                Some(false)
            } else {
                is_same_reference(left.object().ok()?, right.object().ok()?)
            }
        }
        // x
        (
            AnyJsExpression::JsIdentifierExpression(left),
            AnyJsExpression::JsIdentifierExpression(right),
        ) => Some(left.name().ok()?.to_trimmed_string() == right.name().ok()?.to_trimmed_string()),
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
    let member_name = member_name.as_js_name()?.value_token().ok()?;
    if member_name.text_trimmed() != "length" {
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
        return get_integer_from_literal(&unary.argument().ok()?.omit_parentheses())
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
fn split_minus_binary_expressions(
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
///     foo[foo.length - 0] // => None
///     foo[foo.length - 1] // => Some(-1)
///     foo[bar.length - 2] // => None
/// ```
fn extract_negative_index_expression(
    member: AnyJsExpression,
    object: AnyJsExpression,
) -> Option<AnyJsExpression> {
    let (left, right_list) = split_minus_binary_expressions(member)?;
    if right_list.is_empty() {
        return None;
    }

    // left expression should be foo.length
    let left = left.omit_parentheses();
    let length_parent = get_length_node(&left)?;
    // left expression should be the same as the object
    if !is_same_reference(object, length_parent)? {
        return None;
    }

    if right_list.len() == 1 {
        // right expression should be integer
        if let Some(number) = get_integer_from_literal(&right_list[0].clone().omit_parentheses()) {
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
///     .slice(0, 1).pop()
/// ```
fn analyze_slice_element_access(node: &AnyJsExpression) -> Option<UseAtIndexState> {
    if is_within_delete_expression(node).unwrap_or(false) {
        return None;
    }
    // selector
    let (selected_exp, extract_type): (AnyJsExpression, SliceExtractType) = match node {
        // .pop() or .shift()
        AnyJsExpression::JsCallExpression(call_exp) => {
            let has_args = !call_exp.arguments().ok()?.args().is_empty();
            if has_args {
                return None;
            }
            let member = call_exp.callee().ok()?.omit_parentheses();
            let AnyJsExpression::JsStaticMemberExpression(member) = member else {
                return None;
            };
            if call_exp.is_optional_chain() || member.is_optional_chain() {
                return None;
            }
            let member_name = member.member().ok()?.as_js_name()?.value_token().ok()?;
            let object = member.object().ok()?.omit_parentheses();
            match member_name.text_trimmed() {
                "pop" => (object, SliceExtractType::Pop),
                "shift" => (object, SliceExtractType::Shift),
                _ => {
                    return None;
                }
            }
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let object = member.object().ok()?.omit_parentheses();
            if member.is_optional_chain() {
                return None;
            }
            let value = get_integer_from_literal(&member.member().ok()?.omit_parentheses())?;
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
    let member_name = member.member().ok()?.as_js_name()?.value_token().ok()?;
    if member_name.text_trimmed() != "slice" {
        return None;
    }
    // arg length should be 1 or 2
    let [Some(arg0), optional_arg1, None] =
        call_exp.arguments().ok()?.get_arguments_by_index([0, 1, 2])
    else {
        return None;
    };
    let AnyJsCallArgument::AnyJsExpression(arg0) = arg0.clone() else {
        return None;
    };
    let start_exp = arg0.omit_parentheses();
    let sliced_exp = member.object().ok()?;

    match (extract_type.clone(), optional_arg1) {
        (SliceExtractType::ZeroMember | SliceExtractType::Shift, None) => Some(UseAtIndexState {
            at_number_exp: start_exp,
            error_type: ErrorType::Slice {
                arg_type: SliceArgType::OneArg,
                extract_type,
            },
            object: sliced_exp,
        }),
        (SliceExtractType::Pop, None) if get_integer_from_literal(&start_exp)? < 0 => {
            Some(UseAtIndexState {
                at_number_exp: make_number_literal(-1),
                error_type: ErrorType::Slice {
                    arg_type: SliceArgType::OneArg,
                    extract_type: SliceExtractType::Pop,
                },
                object: sliced_exp,
            })
        }
        (SliceExtractType::ZeroMember | SliceExtractType::Shift, Some(arg1)) => {
            let start_index = get_integer_from_literal(&start_exp)?;
            let end_index =
                get_integer_from_literal(&arg1.as_any_js_expression()?.clone().omit_parentheses())?;
            (start_index * end_index >= 0 && start_index < end_index).then_some(UseAtIndexState {
                at_number_exp: start_exp,
                error_type: ErrorType::Slice {
                    arg_type: SliceArgType::TwoArg,
                    extract_type,
                },
                object: sliced_exp,
            })
        }
        (SliceExtractType::Pop, Some(arg1)) => {
            let start_index = get_integer_from_literal(&start_exp)?;
            let end_index =
                get_integer_from_literal(&arg1.as_any_js_expression()?.clone().omit_parentheses())?;
            (start_index * end_index >= 0 && start_index < end_index).then_some(UseAtIndexState {
                at_number_exp: make_number_literal(end_index - 1),
                error_type: ErrorType::Slice {
                    arg_type: SliceArgType::TwoArg,
                    extract_type: SliceExtractType::Pop,
                },
                object: sliced_exp,
            })
        }
        _ => None,
    }
}

fn check_binary_expression_member(
    member: JsBinaryExpression,
    object: AnyJsExpression,
) -> Option<UseAtIndexState> {
    let member = AnyJsExpression::JsBinaryExpression(member);
    let negative_index_exp =
        extract_negative_index_expression(member, object.clone().omit_parentheses());
    let negative_index = negative_index_exp?;

    Some(UseAtIndexState {
        at_number_exp: negative_index,
        error_type: ErrorType::Index { is_negative: true },
        object,
    })
}

/// check foo[foo.length - 1]
fn check_computed_member_expression(exp: &JsComputedMemberExpression) -> Option<UseAtIndexState> {
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
    let member = exp.member().ok()?.omit_parentheses();
    let object = exp.object().ok()?;
    match member.clone() {
        // foo[foo.length - 1]
        AnyJsExpression::JsBinaryExpression(binary) => {
            check_binary_expression_member(binary, object)
        }
        _ => None,
    }
}

/// check foo.charAt(foo.length - 1)
fn check_call_expression_char_at(
    call_exp: &JsCallExpression,
    member: &JsStaticMemberExpression,
) -> Option<UseAtIndexState> {
    let [Some(arg0), None] = call_exp.arguments().ok()?.get_arguments_by_index([0, 1]) else {
        return None;
    };
    let AnyJsCallArgument::AnyJsExpression(arg0) = arg0.clone() else {
        return None;
    };
    let arg0 = arg0.omit_parentheses();
    let char_at_parent = member.object().ok()?.omit_parentheses();
    match arg0.clone() {
        // foo.charAt(foo.length - 1)
        AnyJsExpression::JsBinaryExpression(_) => {
            let at_number_exp = extract_negative_index_expression(arg0, char_at_parent.clone());
            at_number_exp.map(|at_number_exp| UseAtIndexState {
                at_number_exp,
                error_type: ErrorType::StringCharAt { is_negative: true },
                object: char_at_parent,
            })
        }
        _ => None,
    }
}

/// check foo.bar()
fn check_call_expression(call_exp: &JsCallExpression) -> Option<UseAtIndexState> {
    // check slice
    if let Some(slice_err) =
        analyze_slice_element_access(&AnyJsExpression::JsCallExpression(call_exp.clone()))
    {
        return Some(slice_err);
    }

    if call_exp.is_optional_chain() {
        return None;
    }

    match call_exp.callee().ok()?.omit_parentheses() {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if member.is_optional_chain() {
                return None;
            }
            let member_name = member.member().ok()?.as_js_name()?.value_token().ok()?;
            match member_name.text_trimmed() {
                "charAt" => check_call_expression_char_at(call_exp, &member),
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

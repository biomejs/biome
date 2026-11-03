use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsLiteralExpression, AnyJsOptionalChainExpression, JsAssignmentExpression,
    JsAwaitExpression, JsBinaryOperator, JsCallArgumentList, JsCallExpression,
    JsExpressionStatement, JsImportCallExpression, JsLanguage, JsModuleItemList, JsNewExpression,
    JsPostUpdateExpression, JsPreUpdateExpression, JsSequenceExpression, JsStatementList,
    JsStaticMemberExpression, JsSyntaxKind, JsTemplateExpression, JsUnaryExpression,
    JsUnaryOperator, JsYieldExpression, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet};
use biome_rule_options::no_useless_splice::NoUselessSpliceOptions;

use crate::{JsRuleAction, utils::same_reference::is_same_reference};

declare_lint_rule! {
    /// Disallow `splice()` calls that have a simpler alternative.
    ///
    /// `splice()` can remove and add elements anywhere in an array.
    /// To understand a `splice()` call, the reader has to work out what each argument means.
    /// When a call only changes the start or the end of an array, a dedicated method says the same thing with its name.
    ///
    /// This rule reports `splice()` calls that:
    ///
    /// - don't change the array, because they remove no elements and add none.
    /// - remove the first element. Use `shift()` instead.
    /// - add elements to the start. Use `unshift()` instead.
    /// - remove the last element. Use `pop()` instead.
    /// - add elements to the end. Use `push()` instead.
    /// - remove every element. Set `length` to `0` instead.
    ///
    /// `splice()` returns an array that contains the removed elements.
    /// `shift()` and `pop()` return the removed element, and `unshift()` and `push()` return the new length of the array.
    /// For that reason, the rule only offers a fix when the code doesn't use the value that the call returns.
    /// A call that removes every element is only reported when its returned value isn't used.
    ///
    /// The rule ignores a call when the array is read with optional chaining (`?.`), such as `object?.items.splice(0, 1)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.splice(index, 0);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.splice(0, 1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.splice(0, 0, item);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.splice(array.length - 1, 1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.splice(array.length, 0, item);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.splice(0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// array.shift();
    /// array.unshift(item);
    /// array.pop();
    /// array.push(item);
    /// array.length = 0;
    /// array.splice(index, 1);
    /// array.splice(1, 0, item);
    /// const removed = array.splice(0);
    /// array?.splice(0, 1);
    /// ```
    ///
    pub NoUselessSplice {
        version: "2.6.3",
        name: "noUselessSplice",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-unnecessary-splice").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because the object may not be an array, and its `splice()` method may behave differently.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessSplice {
    type Query = Ast<JsCallExpression>;
    type State = UselessSplice;
    type Signals = Option<Self::State>;
    type Options = NoUselessSpliceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional() {
            return None;
        }
        let callee = static_callee(call)?;
        if callee.is_optional()
            || callee
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .text_trimmed()
                != "splice"
        {
            return None;
        }

        let object = callee.object().ok()?;
        let replacement =
            SpliceReplacement::from_arguments(&call.arguments().ok()?.args(), &object)?;
        if has_optional_chain(object) {
            return None;
        }

        let is_result_unused = is_result_unused(call);
        // Setting `length` doesn't produce the removed elements, so it can't
        // stand in for a call whose result is used.
        if replacement == SpliceReplacement::Empty && !is_result_unused {
            return None;
        }

        Some(UselessSplice {
            replacement,
            is_result_unused,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().range();
        let diagnostic = match state.replacement {
            SpliceReplacement::NoOp => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"splice()"</Emphasis>" call does not change the array."
                },
            )
            .note(markup! {
                "The call removes no elements and adds none. Remove it, or change its arguments if it is meant to modify the array."
            }),
            SpliceReplacement::Empty => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"splice()"</Emphasis>" call removes every element of the array."
                },
            )
            .note(markup! {
                "Setting "<Emphasis>"length"</Emphasis>" to "<Emphasis>"0"</Emphasis>" does the same thing and is easier to read."
            }),
            SpliceReplacement::Method(method) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"splice()"</Emphasis>" call does the same thing as "<Emphasis>{method}"()"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>{method}"()"</Emphasis>" is easier to read, because its name says what the call does."
            }),
        };
        if state.is_result_unused {
            return Some(diagnostic);
        }

        Some(match state.replacement {
            SpliceReplacement::NoOp => diagnostic.note(markup! {
                "The call always returns an empty array."
            }),
            SpliceReplacement::Method(method @ (ArrayMethod::Shift | ArrayMethod::Pop)) => {
                diagnostic.note(markup! {
                    <Emphasis>{method}"()"</Emphasis>" returns the removed element, while "<Emphasis>"splice()"</Emphasis>" returns an array that contains it."
                })
            }
            SpliceReplacement::Method(method @ (ArrayMethod::Unshift | ArrayMethod::Push)) => {
                diagnostic.note(markup! {
                    <Emphasis>{method}"()"</Emphasis>" returns the new length of the array, while "<Emphasis>"splice()"</Emphasis>" returns an empty array."
                })
            }
            SpliceReplacement::Empty => diagnostic,
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        if !state.is_result_unused || call.syntax().has_inner_comments() {
            return None;
        }

        let callee = static_callee(call)?;
        let arguments = call.arguments().ok()?.args();
        let mut mutation = ctx.root().begin();
        let message = match state.replacement {
            SpliceReplacement::NoOp => {
                let statement = call.parent::<JsExpressionStatement>()?;
                let is_in_statement_list = statement
                    .syntax()
                    .parent()
                    .is_some_and(|parent| STATEMENT_LIST_KINDS.matches(parent.kind()));
                let object = AnyJsCallArgument::AnyJsExpression(callee.object().ok()?);
                if !is_in_statement_list
                    || statement.syntax().has_comments_descendants()
                    || may_have_side_effects(&object)
                    || arguments
                        .iter()
                        .flatten()
                        .any(|argument| may_have_side_effects(&argument))
                {
                    return None;
                }
                mutation.remove_node(statement);
                markup! { "Remove the "<Emphasis>"splice()"</Emphasis>" call." }.to_owned()
            }
            SpliceReplacement::Empty => {
                let length = make::js_static_member_assignment(
                    callee.object().ok()?,
                    callee.operator_token().ok()?,
                    make::js_name(make::ident("length")).into(),
                );
                let assignment = make::js_assignment_expression(
                    AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::from(length)),
                    make::token_decorated_with_space(T![=]),
                    AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
                        make::js_number_literal_expression(make::js_number_literal("0")),
                    )),
                );
                mutation.replace_node(AnyJsExpression::from(call.clone()), assignment.into());
                markup! {
                    "Set "<Emphasis>"length"</Emphasis>" to "<Emphasis>"0"</Emphasis>" instead."
                }
                .to_owned()
            }
            SpliceReplacement::Method(method) => {
                // `push()` doesn't read `length` before the new elements are evaluated,
                // so an element that changes the array would be added at a different position.
                if method == ArrayMethod::Push
                    && arguments
                        .iter()
                        .skip(2)
                        .flatten()
                        .any(|argument| may_have_side_effects(&argument))
                {
                    return None;
                }

                let name = callee.member().ok()?.as_js_name()?.value_token().ok()?;
                mutation.replace_token(name, make::ident(method.name()));
                for element in arguments.elements().take(2) {
                    mutation.remove_node(element.node().ok()?.clone());
                    if let Some(separator) = element.trailing_separator().ok()? {
                        mutation.remove_token(separator.clone());
                    }
                }
                markup! { "Use "<Emphasis>{method}"()"</Emphasis>" instead." }.to_owned()
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

#[derive(Clone, Copy)]
pub struct UselessSplice {
    replacement: SpliceReplacement,
    /// Whether the surrounding code discards the value that the call returns.
    is_result_unused: bool,
}

/// The simpler operation that has the same effect on the array as a `splice()` call.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SpliceReplacement {
    /// The call removes no elements and adds none, so nothing replaces it.
    NoOp,
    /// The call removes every element, like setting `length` to `0`.
    Empty,
    Method(ArrayMethod),
}

impl SpliceReplacement {
    /// Returns the operation that can replace a `splice()` call on `object`
    /// with these `arguments`, or `None` if the call has no simpler alternative.
    fn from_arguments(arguments: &JsCallArgumentList, object: &AnyJsExpression) -> Option<Self> {
        let mut arguments = arguments.iter();
        let Some(start) = arguments.next() else {
            return Some(Self::NoOp);
        };
        let start = start.ok()?;
        let start = start.as_any_js_expression()?;
        let Some(delete_count) = arguments.next() else {
            return is_zero(start).then_some(Self::Empty);
        };
        let delete_count = delete_count.ok()?;
        let delete_count = delete_count.as_any_js_expression()?;
        let has_new_elements = arguments.next().is_some();

        let delete_count_value = static_number(delete_count);
        let removes_nothing = delete_count_value.is_some_and(|count| count <= 0.0);
        if has_new_elements {
            if !removes_nothing {
                return None;
            }
            return if is_zero(start) {
                Some(Self::Method(ArrayMethod::Unshift))
            } else if is_length_of(start, object) {
                Some(Self::Method(ArrayMethod::Push))
            } else {
                None
            };
        }

        if removes_nothing {
            Some(Self::NoOp)
        } else if delete_count_value == Some(1.0) {
            if is_zero(start) {
                Some(Self::Method(ArrayMethod::Shift))
            } else if is_length_minus_one_of(start, object) {
                Some(Self::Method(ArrayMethod::Pop))
            } else {
                None
            }
        } else {
            (is_zero(start) && is_length_of(delete_count, object)).then_some(Self::Empty)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArrayMethod {
    Shift,
    Unshift,
    Pop,
    Push,
}

impl ArrayMethod {
    const fn name(self) -> &'static str {
        match self {
            Self::Shift => "shift",
            Self::Unshift => "unshift",
            Self::Pop => "pop",
            Self::Push => "push",
        }
    }
}

impl biome_console::fmt::Display for ArrayMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.name())
    }
}

const STATEMENT_LIST_KINDS: SyntaxKindSet<JsLanguage> =
    JsStatementList::KIND_SET.union(JsModuleItemList::KIND_SET);

const SIDE_EFFECT_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsNewExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET);

/// Returns the callee of `call` if it reads a property with dot notation, such as `array.splice`.
fn static_callee(call: &JsCallExpression) -> Option<JsStaticMemberExpression> {
    call.callee()
        .ok()?
        .omit_parentheses()
        .as_js_static_member_expression()
        .cloned()
}

/// Returns `true` if `expression`, or an object it reads a property from or calls, uses `?.`.
///
/// Parentheses and TypeScript type assertions around each part are ignored,
/// so `(a?.b as string[])` counts.
fn has_optional_chain(expression: AnyJsExpression) -> bool {
    let mut current = expression.omit_type_wrappers();
    while let Some(chain) =
        current.and_then(|expression| AnyJsOptionalChainExpression::cast(expression.into_syntax()))
    {
        if chain.is_optional() {
            return true;
        }
        current = chain
            .object()
            .ok()
            .and_then(AnyJsExpression::omit_type_wrappers);
    }
    false
}

/// Returns `true` if the value of `call` is discarded.
///
/// That is the case when the call is a statement on its own, or an operand of
/// the comma operator whose value isn't the result of the whole expression.
fn is_result_unused(call: &JsCallExpression) -> bool {
    for ancestor in call.syntax().ancestors().skip(1) {
        match ancestor.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => return true,
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {}
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let is_in_left_operand = JsSequenceExpression::unwrap_cast(ancestor)
                    .left()
                    .is_ok_and(|left| left.range().contains_range(call.range()));
                if is_in_left_operand {
                    return true;
                }
            }
            _ => return false,
        }
    }
    false
}

/// Returns the value of `expression` if it is a number literal, optionally
/// preceded by `+` and `-` signs, such as `1`, `-1`, or `- -1`.
///
/// Parentheses and TypeScript type assertions are ignored.
fn static_number(expression: &AnyJsExpression) -> Option<f64> {
    let mut expression = expression.clone().omit_type_wrappers()?;
    let mut is_negative = false;
    loop {
        match expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) => {
                let value = literal.as_number()?;
                return Some(if is_negative { -value } else { value });
            }
            AnyJsExpression::JsUnaryExpression(unary) => {
                match unary.operator().ok()? {
                    JsUnaryOperator::Minus => is_negative = !is_negative,
                    JsUnaryOperator::Plus => {}
                    _ => return None,
                }
                expression = unary.argument().ok()?.omit_type_wrappers()?;
            }
            _ => return None,
        }
    }
}

fn is_zero(expression: &AnyJsExpression) -> bool {
    static_number(expression) == Some(0.0)
}

/// Returns `true` if `expression` reads the `length` property of `object`, such as
/// `array.length` for the object `array`.
fn is_length_of(expression: &AnyJsExpression, object: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_type_wrappers()
        .and_then(|expression| {
            let member = expression.as_js_static_member_expression()?;
            if member.is_optional()
                || member
                    .member()
                    .ok()?
                    .as_js_name()?
                    .value_token()
                    .ok()?
                    .text_trimmed()
                    != "length"
            {
                return Some(false);
            }
            is_same_reference(member.object().ok()?, object.clone())
        })
        .unwrap_or(false)
}

/// Returns `true` if `expression` is the `length` property of `object` minus one,
/// such as `array.length - 1` for the object `array`.
fn is_length_minus_one_of(expression: &AnyJsExpression, object: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_type_wrappers()
        .and_then(|expression| {
            let binary = expression.as_js_binary_expression()?;
            Some(
                binary.operator().ok()? == JsBinaryOperator::Minus
                    && static_number(&binary.right().ok()?) == Some(1.0)
                    && is_length_of(&binary.left().ok()?, object),
            )
        })
        .unwrap_or(false)
}

/// Returns `true` if evaluating `argument` may do more than produce a value,
/// such as calling a function or assigning to a variable.
fn may_have_side_effects(argument: &AnyJsCallArgument) -> bool {
    argument.syntax().descendants().any(|node| {
        SIDE_EFFECT_KINDS.matches(node.kind())
            || JsUnaryExpression::cast(node)
                .is_some_and(|unary| unary.operator() == Ok(JsUnaryOperator::Delete))
    })
}

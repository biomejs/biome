use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsLiteralExpression, JsAssignmentExpression, JsAwaitExpression, JsCallExpression,
    JsExpressionStatement, JsImportCallExpression, JsLanguage, JsNewExpression,
    JsPostUpdateExpression, JsPreUpdateExpression, JsStaticMemberExpression, JsSyntaxNode,
    JsTemplateExpression, JsUnaryExpression, JsUnaryOperator, JsYieldExpression, T,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet};
use biome_rule_options::no_confusing_array_splice::NoConfusingArraySpliceOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::{JsRuleAction, utils::same_reference::is_same_reference};

declare_lint_rule! {
    /// Disallow `splice()` and `toSpliced()` calls that are harder to read than the operation they perform.
    ///
    /// A `splice()` call that removes one element and adds one element back replaces a single element.
    /// Assigning to the index, as in `array[index] = element`, says the same thing more directly.
    /// In the same way, `toSpliced()` with these arguments creates a copy of the array with one element replaced,
    /// which is what `with()` does.
    ///
    /// A `splice()` or `toSpliced()` call that inserts at index `-1` without removing anything adds the elements
    /// before the last element. When the array is empty, it adds them at the start instead.
    /// Readers can easily mistake this for adding at the end, so it is clearer to work out the index explicitly.
    ///
    /// The rule ignores calls with spread arguments (`...`), and calls that use optional chaining (`?.`)
    /// to read or call the method, such as `array?.splice(index, 1, element)` or `array.splice?.(index, 1, element)`.
    ///
    /// The fixes are unsafe, because the replacements don't behave the same for every index:
    ///
    /// - `splice()` and `toSpliced()` count a negative index from the end of the array,
    ///   while `array[-1] = element` creates a property named `-1`.
    /// - `splice()` and `toSpliced()` add the element at the end when the index is past the end,
    ///   while an assignment leaves empty slots before the new element, and `with()` throws an error.
    /// - `splice()` returns the removed elements, while an assignment returns the new element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.splice(index, 1, element);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = array.toSpliced(index, 1, element);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.splice(-1, 0, element);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = array.toSpliced(-1, 0, element);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// array[index] = element;
    /// const result = array.with(index, element);
    /// const insertionIndex = Math.max(array.length - 1, 0);
    /// array.splice(insertionIndex, 0, element);
    /// array.splice(index, 2, element);
    /// array.splice(-1, 0, element, otherElement);
    /// array?.splice(index, 1, element);
    /// ```
    ///
    pub NoConfusingArraySplice {
        version: "next",
        name: "noConfusingArraySplice",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-confusing-array-splice").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConfusingArraySplice {
    type Query = Ast<JsCallExpression>;
    type State = ConfusingSplice;
    type Signals = Option<Self::State>;
    type Options = NoConfusingArraySpliceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional() {
            return None;
        }
        let callee = static_callee(call)?;
        if callee.is_optional() {
            return None;
        }
        splice_method(&callee)?;

        let [start, delete_count, _] = splice_arguments(call)?;
        let delete_count = static_number(&delete_count)?.trunc().max(0.0);
        if delete_count == 1.0 {
            Some(ConfusingSplice::ReplaceOneElement)
        } else if delete_count == 0.0 && static_number(&start)?.trunc() == -1.0 {
            Some(ConfusingSplice::InsertAtNegativeOne)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        let callee = static_callee(call)?;
        let name = callee.member().ok()?.as_js_name()?.value_token().ok()?;
        let name = name.text_trimmed();
        let diagnostic = match state {
            ConfusingSplice::ReplaceOneElement => {
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    call.range(),
                    markup! {
                        "This "<Emphasis>{name}"()"</Emphasis>" call replaces a single element."
                    },
                )
                .note(markup! {
                    "Readers have to work out what each argument means to understand the call."
                });
                match splice_method(&callee)? {
                    SpliceMethod::Splice => diagnostic.note(markup! {
                        "Assign to the index instead, as in "<Emphasis>"array[index] = element"</Emphasis>"."
                    }),
                    SpliceMethod::ToSpliced => diagnostic.note(markup! {
                        "Use "<Emphasis>"with()"</Emphasis>" instead, as in "<Emphasis>"array.with(index, element)"</Emphasis>"."
                    }),
                }
            }
            ConfusingSplice::InsertAtNegativeOne => RuleDiagnostic::new(
                rule_category!(),
                call.range(),
                markup! {
                    "This "<Emphasis>{name}"()"</Emphasis>" call inserts at index "<Emphasis>"-1"</Emphasis>"."
                },
            )
            .note(markup! {
                "It inserts before the last element, or at the start when the array is empty. Readers can mistake it for inserting at the end."
            })
            .note(markup! {
                "Work out the index explicitly, as in "<Emphasis>"Math.max(array.length - 1, 0)"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let callee = static_callee(call)?;
        let [start, delete_count, element] = splice_arguments(call)?;
        let mut mutation = ctx.root().begin();
        let message = match (state, splice_method(&callee)?) {
            (ConfusingSplice::ReplaceOneElement, SpliceMethod::Splice) => {
                let object = callee.object().ok()?;
                // Removing the parentheses around the callee could join the assignment
                // to a keyword before it, such as `else(array.splice)(...)`.
                if !JsExpressionStatement::can_cast(call.syntax().parent()?.kind())
                    || call.callee().ok()?.as_js_parenthesized_expression().is_some()
                    || !is_simple_receiver(&object)
                    || !is_replaceable_index(&start)
                    || has_comments_outside(
                        call.syntax(),
                        &[object.syntax(), start.syntax(), element.syntax()],
                    )
                {
                    return None;
                }
                let assignment = make::js_assignment_expression(
                    AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::from(
                        make::js_computed_member_assignment(
                            object,
                            make::token(T!['[']),
                            start,
                            make::token(T![']']),
                        ),
                    )),
                    make::token_decorated_with_space(T![=]),
                    element,
                );
                mutation.replace_node_discard_trivia(
                    AnyJsExpression::from(call.clone()),
                    assignment.into(),
                );
                markup! { "Assign to the index instead." }.to_owned()
            }
            (ConfusingSplice::ReplaceOneElement, SpliceMethod::ToSpliced) => {
                // `with()` throws an error for an index out of range,
                // while `toSpliced()` clamps it to the bounds of the array.
                if static_number(&start).is_some_and(|value| value.trunc() < 0.0)
                    || is_length_of(&start, &callee.object().ok()?)
                {
                    return None;
                }
                let arguments = call.arguments().ok()?.args();
                let mut elements = arguments.elements();
                let start_separator = elements.next()?.trailing_separator().ok()??.clone();
                let delete_count_separator = elements.next()?.trailing_separator().ok()??.clone();
                // A comment after the comma before the delete count describes the delete count,
                // and would otherwise end up in front of the element.
                if start_separator.has_trailing_comments()
                    || delete_count.syntax().has_comments_descendants()
                    || delete_count_separator.has_leading_comments()
                    || delete_count_separator.has_trailing_comments()
                {
                    return None;
                }
                let name = callee.member().ok()?.as_js_name()?.value_token().ok()?;
                mutation.replace_token(name, make::ident("with"));
                mutation.remove_node(delete_count);
                mutation.remove_token(delete_count_separator);
                markup! { "Use "<Emphasis>"with()"</Emphasis>" instead." }.to_owned()
            }
            (ConfusingSplice::InsertAtNegativeOne, _) => {
                let object = callee.object().ok()?.omit_parentheses();
                // The new index reads `length` before the element is evaluated,
                // so an element that changes the array would be inserted elsewhere.
                if !is_simple_receiver(&object)
                    || object.syntax().has_comments_descendants()
                    || start.syntax().has_comments_descendants()
                    || may_have_side_effects(&element)
                {
                    return None;
                }
                mutation.replace_node(start, last_index_or_zero(object.trim_trivia()?));
                markup! { "Work out the insertion index explicitly." }.to_owned()
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
pub enum ConfusingSplice {
    /// The call removes one element and adds one, such as `array.splice(index, 1, element)`.
    ReplaceOneElement,
    /// The call adds one element at index `-1` and removes none, such as `array.splice(-1, 0, element)`.
    InsertAtNegativeOne,
}

#[derive(Clone, Copy)]
enum SpliceMethod {
    Splice,
    ToSpliced,
}

const SIDE_EFFECT_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsNewExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET);

/// Returns the callee of `call` if it reads a property with dot notation, such as `array.splice`.
fn static_callee(call: &JsCallExpression) -> Option<JsStaticMemberExpression> {
    call.callee()
        .ok()?
        .omit_parentheses()
        .as_js_static_member_expression()
        .cloned()
}

/// Returns the method that `callee` reads, if it is `splice` or `toSpliced`.
fn splice_method(callee: &JsStaticMemberExpression) -> Option<SpliceMethod> {
    let name = callee.member().ok()?.as_js_name()?.value_token().ok()?;
    match name.text_trimmed() {
        "splice" => Some(SpliceMethod::Splice),
        "toSpliced" => Some(SpliceMethod::ToSpliced),
        _ => None,
    }
}

/// Returns the start index, the number of elements to remove, and the element to add,
/// if `call` has exactly these three arguments and none of them is spread.
fn splice_arguments(call: &JsCallExpression) -> Option<[AnyJsExpression; 3]> {
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 3 {
        return None;
    }
    let mut arguments = arguments
        .iter()
        .map(|argument| argument.ok()?.as_any_js_expression().cloned());
    Some([
        arguments.next()??,
        arguments.next()??,
        arguments.next()??,
    ])
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

/// Returns `true` if `expression` is an identifier, `this`, or a chain of
/// non-optional property reads with dot notation on one of them, such as `this.items`.
///
/// Parentheses are ignored.
fn is_simple_receiver(expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone().omit_parentheses();
    loop {
        expression = match expression {
            AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => {
                return true;
            }
            AnyJsExpression::JsStaticMemberExpression(member) if !member.is_optional() => {
                match member.object() {
                    Ok(object) => object.omit_parentheses(),
                    Err(_) => return false,
                }
            }
            _ => return false,
        }
    }
}

/// Returns `true` if `array[index]` refers to the same element that `splice()` replaces
/// when it receives `index` as its start index.
///
/// `splice()` converts its start index to a whole number, while `array[index]` converts the
/// index to a property name. For example, `splice()` reads `1.5` and `"1"` as index `1`, and
/// `NaN` and `undefined` as index `0`, and clamps `Infinity` to the length of the array,
/// while `array[1.5]`, `array[undefined]`, and `array[Infinity]` are properties named `1.5`,
/// `undefined`, and `Infinity`.
fn is_replaceable_index(index: &AnyJsExpression) -> bool {
    let Some(index) = index.clone().omit_type_wrappers() else {
        return false;
    };
    if let Some(value) = static_number(&index) {
        return value >= 0.0 && value.fract() == 0.0 && value <= MAX_SAFE_INTEGER;
    }
    if let AnyJsExpression::JsIdentifierExpression(identifier) = &index {
        return identifier
            .name()
            .and_then(|name| name.value_token())
            .is_ok_and(|name| {
                let name = name.text_trimmed();
                name != "NaN" && name != "undefined" && name != "Infinity"
            });
    }
    match index.as_static_value() {
        Some(value @ StaticValue::String(_)) => is_canonical_index_text(value.text()),
        Some(_) => false,
        None => !matches!(index, AnyJsExpression::AnyJsLiteralExpression(_)),
    }
}

/// The largest whole number that a JavaScript number can represent exactly.
const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

/// Returns `true` if `text` is a whole non-negative number written without leading zeros,
/// such as `0` or `12`.
fn is_canonical_index_text(text: &str) -> bool {
    match text.as_bytes() {
        [] => false,
        [b'0'] => true,
        [first, rest @ ..] => {
            lookup_byte(*first) == Dispatch::DIG
                && rest
                    .iter()
                    .all(|byte| matches!(lookup_byte(*byte), Dispatch::ZER | Dispatch::DIG))
        }
    }
}

/// Returns `true` if `expression` reads the `length` property of `object`, such as
/// `array.length` for the object `array`.
fn is_length_of(expression: &AnyJsExpression, object: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_type_wrappers()
        .and_then(|expression| {
            let member = expression.as_js_static_member_expression()?;
            let name = member.member().ok()?.as_js_name()?.value_token().ok()?;
            if member.is_optional() || name.text_trimmed() != "length" {
                return Some(false);
            }
            is_same_reference(member.object().ok()?, object.clone())
        })
        .unwrap_or(false)
}

/// Returns `true` if a token of `node` outside the `kept` nodes has a comment.
fn has_comments_outside(node: &JsSyntaxNode, kept: &[&JsSyntaxNode]) -> bool {
    node.descendants_tokens(Direction::Next).any(|token| {
        (token.has_leading_comments() || token.has_trailing_comments())
            && !kept.iter().any(|kept| {
                kept.text_range_with_trivia()
                    .contains_range(token.text_range())
            })
    })
}

/// Returns `true` if evaluating `expression` may do more than produce a value,
/// such as calling a function or assigning to a variable.
fn may_have_side_effects(expression: &AnyJsExpression) -> bool {
    expression
        .syntax()
        .preorder_of_kinds(SIDE_EFFECT_KINDS)
        .any(|event| match event {
            biome_rowan::WalkEvent::Enter(node) => JsUnaryExpression::cast(node)
                .is_none_or(|unary| unary.operator() == Ok(JsUnaryOperator::Delete)),
            biome_rowan::WalkEvent::Leave(_) => false,
        })
}

/// Builds `Math.max(object.length - 1, 0)`.
fn last_index_or_zero(object: AnyJsExpression) -> AnyJsExpression {
    let math_max = make::js_static_member_expression(
        make::js_identifier_expression(make::js_reference_identifier(make::ident("Math"))).into(),
        make::token(T![.]),
        make::js_name(make::ident("max")).into(),
    );
    let length = make::js_static_member_expression(
        object,
        make::token(T![.]),
        make::js_name(make::ident("length")).into(),
    );
    let last_index = make::js_binary_expression(
        length.into(),
        make::token_decorated_with_space(T![-]),
        number_literal("1"),
    );
    let arguments = make::js_call_arguments(
        make::token(T!['(']),
        make::js_call_argument_list(
            [
                AnyJsCallArgument::AnyJsExpression(last_index.into()),
                AnyJsCallArgument::AnyJsExpression(number_literal("0")),
            ],
            [make::token_with_trailing_space(T![,])],
        ),
        make::token(T![')']),
    );
    make::js_call_expression(math_max.into(), arguments)
        .build()
        .into()
}

fn number_literal(text: &str) -> AnyJsExpression {
    AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
        make::js_number_literal_expression(make::js_number_literal(text)),
    ))
}

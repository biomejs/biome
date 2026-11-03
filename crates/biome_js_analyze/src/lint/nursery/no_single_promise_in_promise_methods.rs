use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement, AnyJsAssignment,
    AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression,
    AnyJsMemberExpression, JsArrayExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsAwaitExpression, JsCallExpression, JsComputedMemberExpression, JsExpressionStatement,
    JsInitializerClause, JsLanguage, JsSyntaxNode, JsVariableDeclarator, T, TriviaPieceKind,
    global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, Direction};
use biome_rule_options::no_single_promise_in_promise_methods::NoSinglePromiseInPromiseMethodsOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow passing an array with a single element to `Promise.all()`, `Promise.any()`, or `Promise.race()`.
    ///
    /// `Promise.all()`, `Promise.any()`, and `Promise.race()` are meant to combine several promises into one.
    /// When the array contains only one element, there is nothing to combine.
    /// The array only makes the code harder to read, and it often means that another promise is missing from it.
    ///
    /// If the call is awaited, await the value directly.
    /// Otherwise, if you need a promise, pass the value to `Promise.resolve()`.
    /// `Promise.all()` is an exception, because it returns an array that holds the value.
    /// Await the value directly instead, and update the code that reads the array.
    ///
    /// The fix for `Promise.any()` changes the error that is thrown when the promise is rejected.
    /// `Promise.any()` rejects with an `AggregateError` that holds the original error in its `errors` property,
    /// while awaiting the value directly throws the original error.
    /// Check that no code depends on `AggregateError` before you apply the fix.
    ///
    /// `Promise.allSettled()` is not reported, because its result has a different shape than the promise it receives.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = await Promise.all([promise]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = await Promise.any([promise]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = await Promise.race([promise]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const promise = Promise.race([nonPromise]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = await promise;
    /// const promise = Promise.resolve(nonPromise);
    /// const foo = await Promise.all(promises);
    /// const foo = await Promise.any([promise, anotherPromise]);
    /// const [{ value: foo, reason: error }] = await Promise.allSettled([promise]);
    /// ```
    ///
    pub NoSinglePromiseInPromiseMethods {
        version: "next",
        name: "noSinglePromiseInPromiseMethods",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-single-promise-in-promise-methods").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because unwrapping changes when the value is settled, and unwrapping
        // `Promise.any()` changes the error that is thrown on rejection.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoSinglePromiseInPromiseMethods {
    type Query = Semantic<JsCallExpression>;
    type State = PromiseMethod;
    type Signals = Option<Self::State>;
    type Options = NoSinglePromiseInPromiseMethodsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional_chain() {
            return None;
        }
        let callee =
            AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        let method = PromiseMethod::from_name(callee.member_name()?.text())?;
        let (reference, name) = global_identifier(
            &callee
                .object()
                .ok()?
                .omit_parentheses()
                .as_any_global_identifier_expression()?,
        )?;
        if name.text() != "Promise" || ctx.model().binding(&reference).is_some() {
            return None;
        }
        single_element(call)?;
        Some(method)
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let (array, _) = single_element(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                array.range(),
                markup! {
                    "This array passed to "<Emphasis>"Promise."{method}"()"</Emphasis>" has only one element."
                },
            )
            .note(markup! {
                <Emphasis>"Promise."{method}"()"</Emphasis>" is meant to combine several promises. With only one element, the array is unnecessary, or another promise is missing from it."
            })
            .note(if *method == PromiseMethod::All {
                markup! {
                    "If the array is complete, await the value directly, and update the code that reads the array that "<Emphasis>"Promise.all()"</Emphasis>" returns."
                }
            } else {
                markup! {
                    "If the array is complete, use the value directly, or pass it to "<Emphasis>"Promise.resolve()"</Emphasis>" if you need a promise."
                }
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, method: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let (array, element) = single_element(call)?;
        let mut mutation = ctx.root().begin();

        let Some(await_expression) = call.parent::<JsAwaitExpression>() else {
            // `Promise.all()` produces an array, which `Promise.resolve()` can't reproduce.
            if *method == PromiseMethod::All {
                return None;
            }
            let callee = call.callee().ok()?.omit_parentheses();
            let name = callee
                .as_js_static_member_expression()?
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?;
            if has_lost_comments(array.syntax(), element.syntax()) {
                return None;
            }
            mutation.replace_token(name, make::ident("resolve"));
            mutation.replace_node(AnyJsExpression::from(array), element);
            return Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! { "Use "<Emphasis>"Promise.resolve()"</Emphasis>" instead." }.to_owned(),
                mutation,
            ));
        };

        let replaced = if *method == PromiseMethod::All {
            unwrapped_all_target(&mut mutation, &await_expression)?
        } else {
            await_expression.into()
        };
        if has_lost_comments(replaced.syntax(), element.syntax()) {
            return None;
        }
        let mut replacement = make::js_await_expression(
            make::token(T![await]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            element.with_leading_trivia_pieces([])?.with_trailing_trivia_pieces([])?,
        );
        let argument = replacement.argument().ok()?;
        if argument.needs_parentheses() {
            replacement = replacement.with_argument(make::parenthesized(argument).into());
        }
        separate_from_previous_keyword(&mut mutation, replaced.syntax());
        mutation.replace_node(replaced, replacement.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Await the value directly." }.to_owned(),
            mutation,
        ))
    }
}

/// The `Promise` methods that combine the promises of an array.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromiseMethod {
    All,
    Any,
    Race,
}

impl PromiseMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "all" => Some(Self::All),
            "any" => Some(Self::Any),
            "race" => Some(Self::Race),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for PromiseMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::All => "all",
            Self::Any => "any",
            Self::Race => "race",
        })
    }
}

/// Returns the array literal that is the only argument of `call`, and the
/// array's only element, if that element is neither a spread nor a hole.
fn single_element(call: &JsCallExpression) -> Option<(JsArrayExpression, AnyJsExpression)> {
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    let array = arguments
        .first()?
        .ok()?
        .as_any_js_expression()?
        .as_js_array_expression()?
        .clone();
    let elements = array.elements();
    if elements.len() != 1 {
        return None;
    }
    let element = elements.first()?.ok()?.as_any_js_expression()?.clone();
    Some((array, element))
}

/// Returns the expression that becomes `await <element>` when the result of
/// an awaited `Promise.all([element])` is either unused or only its first item
/// is read. Records the extra change needed to read the value instead of the
/// array, such as turning `const [foo] = ...` into `const foo = ...`.
///
/// Returns `None` in any other position, because there the code depends on
/// receiving an array.
fn unwrapped_all_target(
    mutation: &mut BatchMutation<JsLanguage>,
    await_expression: &JsAwaitExpression,
) -> Option<AnyJsExpression> {
    let parent = await_expression.syntax().parent()?;

    if JsExpressionStatement::can_cast(parent.kind()) {
        return Some(await_expression.clone().into());
    }

    if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
        let declarator = initializer.parent::<JsVariableDeclarator>()?;
        if declarator.variable_annotation().is_some() {
            return None;
        }
        let pattern = declarator.id().ok()?;
        let array_pattern = pattern.as_js_array_binding_pattern()?;
        let elements = array_pattern.elements();
        if elements.len() != 1 {
            return None;
        }
        let AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) =
            elements.first()?.ok()?
        else {
            return None;
        };
        let binding = element.pattern().ok()?;
        if element.init().is_some()
            || !matches!(
                binding,
                AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(_))
            )
            || has_lost_comments(pattern.syntax(), binding.syntax())
        {
            return None;
        }
        separate_from_previous_keyword(mutation, pattern.syntax());
        mutation.replace_node(pattern, binding);
        return Some(await_expression.clone().into());
    }

    if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
        if assignment.operator().ok()? != JsAssignmentOperator::Assign
            || !JsExpressionStatement::can_cast(assignment.syntax().parent()?.kind())
        {
            return None;
        }
        let pattern = assignment.left().ok()?;
        let array_pattern = pattern.as_js_array_assignment_pattern()?;
        let elements = array_pattern.elements();
        if elements.len() != 1 {
            return None;
        }
        let AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(element) =
            elements.first()?.ok()?
        else {
            return None;
        };
        let target = element.pattern().ok()?;
        if element.init().is_some()
            || !matches!(
                target,
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                    _
                ))
            )
            || has_lost_comments(pattern.syntax(), target.syntax())
        {
            return None;
        }
        separate_from_previous_keyword(mutation, pattern.syntax());
        mutation.replace_node(pattern, target);
        return Some(await_expression.clone().into());
    }

    // `(await Promise.all([promise]))[0]`
    let member = AnyJsExpression::cast(parent)?
        .outer_expression()?
        .syntax()
        .parent()
        .and_then(JsComputedMemberExpression::cast)?;
    let index = member.member().ok()?.omit_parentheses();
    let is_first_index = index
        .as_any_js_literal_expression()?
        .as_js_number_literal_expression()?
        .as_number()
        == Some(0.0);
    if member.is_optional()
        || !is_first_index
        || member.object().ok()?.omit_parentheses().syntax() != await_expression.syntax()
    {
        return None;
    }
    let member_parent = member.syntax().parent()?;
    let is_assigned = JsInitializerClause::cast_ref(&member_parent)
        .is_some_and(|initializer| initializer.parent::<JsVariableDeclarator>().is_some())
        || JsAssignmentExpression::cast(member_parent).is_some_and(|assignment| {
            assignment
                .right()
                .is_ok_and(|right| right.syntax() == member.syntax())
        });
    is_assigned.then(|| member.into())
}

/// Returns `true` if replacing `replaced` with `kept` would delete a comment.
///
/// The comments before `replaced` and after it are moved to the replacement,
/// and the comments inside `kept` stay. Any other comment is lost, including
/// the comments directly before and after `kept`, which are replaced.
fn has_lost_comments(replaced: &JsSyntaxNode, kept: &JsSyntaxNode) -> bool {
    let (Some(first), Some(last)) = (replaced.first_token(), replaced.last_token()) else {
        return false;
    };
    let kept_range = kept.text_trimmed_range();
    let kept_first = kept.first_token();
    let kept_last = kept.last_token();
    replaced.descendants_tokens(Direction::Next).any(|token| {
        let is_inside_kept = kept_range.contains_range(token.text_trimmed_range());
        let keeps_leading =
            token == first || (is_inside_kept && kept_first.as_ref() != Some(&token));
        let keeps_trailing =
            token == last || (is_inside_kept && kept_last.as_ref() != Some(&token));
        (!keeps_leading && token.has_leading_comments())
            || (!keeps_trailing && token.has_trailing_comments())
    })
}

/// Adds a space after the keyword that directly precedes `node`, when nothing
/// separates them.
///
/// The replacement of `node` starts with a name, so without the space,
/// `const[value]` would become `constvalue`.
fn separate_from_previous_keyword(mutation: &mut BatchMutation<JsLanguage>, node: &JsSyntaxNode) {
    let Some(first) = node.first_token() else {
        return;
    };
    let Some(previous) = first.prev_token() else {
        return;
    };
    if previous.kind().is_keyword()
        && previous.trailing_trivia().is_empty()
        && first.leading_trivia().is_empty()
    {
        let spaced = previous.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
        mutation.replace_token_discard_trivia(previous, spaced);
    }
}

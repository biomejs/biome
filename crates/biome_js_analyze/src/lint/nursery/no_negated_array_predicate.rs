use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, JsCallExpression,
    JsExportDefaultExpressionClause, JsExpressionStatement, JsSyntaxKind, JsSyntaxToken,
    JsUnaryExpression, JsUnaryOperator, OperatorPrecedence, T, TriviaPieceKind,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_negated_array_predicate::NoNegatedArrayPredicateOptions;

use crate::{
    JsRuleAction, services::semantic::Semantic,
    utils::indexed_collection::is_known_non_indexed_collection,
};

declare_lint_rule! {
    /// Disallow negating the result of `Array#some()` or `Array#every()`.
    ///
    /// `!array.some(callback)` checks that no item passes the test, which is the same as checking
    /// that every item fails it: `array.every((item) => !callback(item))`. In the same way,
    /// `!array.every(callback)` is the same as `array.some((item) => !callback(item))`.
    ///
    /// Putting the `!` inside the callback function keeps the condition next to the test it changes,
    /// and two `!` operators that cancel each other out, as in `!array.some((item) => !item.done)`,
    /// can be removed entirely: `array.every((item) => item.done)`.
    ///
    /// The rule only reports calls whose callback function returns a single expression, either
    /// directly, as in `(item) => item.done`, or with one `return` statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const isMissing = !array.some((item) => isUnicorn(item));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const isNotEveryUnicorn = !array.every((item) => isUnicorn(item));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (!array.some((item) => !isUnicorn(item))) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const isMissing = array.every((item) => !isUnicorn(item));
    ///
    /// const isNotEveryUnicorn = array.some((item) => !isUnicorn(item));
    ///
    /// if (array.every((item) => isUnicorn(item))) {}
    ///
    /// // The callback function isn't written inline, so there's nothing to negate.
    /// if (!array.every(Boolean)) {}
    /// ```
    ///
    pub NoNegatedArrayPredicate {
        version: "next",
        name: "noNegatedArrayPredicate",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintUnicorn("no-negated-array-predicate").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoNegatedArrayPredicate {
    type Query = Semantic<JsUnaryExpression>;
    type State = NegatedPredicateCall;
    type Signals = Option<Self::State>;
    type Options = NoNegatedArrayPredicateOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let unary = ctx.query();
        if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
            return None;
        }

        let call = negated_call(&unary.argument().ok()?)?;
        if call.is_optional_chain() || call.type_arguments().is_some() {
            return None;
        }
        let method_name = method_name(&call)?;
        let method = match method_name.text_trimmed() {
            "some" => PredicateMethod::Some,
            "every" => PredicateMethod::Every,
            _ => return None,
        };

        let arguments = call.arguments().ok()?.args();
        if !(1..=2).contains(&arguments.len()) {
            return None;
        }
        let mut arguments = arguments.iter();
        let Some(Ok(AnyJsCallArgument::AnyJsExpression(callback))) = arguments.next() else {
            return None;
        };
        if arguments.any(|argument| matches!(argument, Ok(AnyJsCallArgument::JsSpread(_)))) {
            return None;
        }

        let callback = AnyJsFunction::cast(callback.omit_parentheses().into_syntax())?;
        if callback.is_async()
            || callback.is_generator()
            || callback.return_type_annotation().is_some()
            || callback.type_parameters().is_some()
        {
            return None;
        }
        let predicate = callback.single_return_expression()?;

        // Resolving the receiver is the least likely check to reject a call, so it runs last.
        let receiver = call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_static_member_expression()?
            .object()
            .ok()?;
        if matches!(
            receiver.clone().omit_type_wrappers()?,
            AnyJsExpression::AnyJsLiteralExpression(_)
        ) || is_known_non_indexed_collection(&receiver, ctx.model(), MAX_RESOLUTION_DEPTH)
        {
            return None;
        }

        Some(NegatedPredicateCall {
            method,
            method_name,
            predicate,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let method = state.method;
        let replacement = method.replacement();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.method_name.text_trimmed_range(),
                markup! {
                    "The result of "<Emphasis>{method}</Emphasis>" is negated with "<Emphasis>"!"</Emphasis>"."
                },
            )
            .note(markup! {
                "Negating the whole call hides the condition that is actually checked. Calling "<Emphasis>{replacement}</Emphasis>" with a negated callback function gives the same result and is easier to read."
            })
            .note(markup! {
                "Remove the "<Emphasis>"!"</Emphasis>" before the call, use "<Emphasis>{replacement}</Emphasis>" instead, and negate the value that the callback function returns."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let unary = ctx.query();

        let (prev_predicate, next_predicate) = negate_predicate(&state.predicate)?;
        let argument = unary
            .argument()
            .ok()?
            .replace_node_discard_trivia(prev_predicate, next_predicate)?;

        let method_name = method_name(&negated_call(&argument)?)?;
        let argument =
            argument.replace_token(method_name, make::ident(state.method.replacement().name()))?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            AnyJsExpression::from(unary.clone()),
            without_negation(unary, argument)?,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>{state.method.replacement()}</Emphasis>" with a negated callback function."
            }
            .to_owned(),
            mutation,
        ))
    }
}

pub struct NegatedPredicateCall {
    method: PredicateMethod,
    /// The `some` or `every` token of the call.
    method_name: JsSyntaxToken,
    /// The expression that the callback function returns.
    predicate: AnyJsExpression,
}

#[derive(Clone, Copy)]
pub enum PredicateMethod {
    Some,
    Every,
}

impl PredicateMethod {
    const fn name(self) -> &'static str {
        match self {
            Self::Some => "some",
            Self::Every => "every",
        }
    }

    /// Returns the method that gives the opposite result for the same callback function.
    const fn replacement(self) -> Self {
        match self {
            Self::Some => Self::Every,
            Self::Every => Self::Some,
        }
    }
}

impl biome_console::fmt::Display for PredicateMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Some => "some()",
            Self::Every => "every()",
        })
    }
}

/// Bounds how many variables a receiver check follows, so that `const a = b; const b = a;`
/// terminates.
const MAX_RESOLUTION_DEPTH: u8 = 8;

/// Tokens that change how a statement or an `export default` declaration is parsed when they
/// start it, or that continue the previous statement when it doesn't end with a semicolon.
const STATEMENT_START_HAZARDS: TokenSet<JsSyntaxKind> = token_set![
    T!['('],
    T!['['],
    T!['{'],
    T!['`'],
    T![+],
    T![-],
    T![/],
    T![/=],
    T![<],
    T![async],
    T![class],
    T![function],
    T![let],
    JsSyntaxKind::JS_REGEX_LITERAL,
];

/// Returns the call that `argument` evaluates to, looking through parentheses and TypeScript
/// type assertions, as in `!(array.some(callback) as boolean)`.
fn negated_call(argument: &AnyJsExpression) -> Option<JsCallExpression> {
    argument
        .clone()
        .omit_type_wrappers()?
        .as_js_call_expression()
        .cloned()
}

/// Returns the method name token of `call` when it calls a method by a static name, as in
/// `array.some(callback)`.
fn method_name(call: &JsCallExpression) -> Option<JsSyntaxToken> {
    call.callee()
        .ok()?
        .omit_parentheses()
        .as_js_static_member_expression()?
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()
}

/// Returns the node to replace and its replacement to negate `predicate`, with the trivia of the
/// replaced node already moved to the replacement.
///
/// A predicate that is already negated, such as `!item.done`, loses its `!`. Any other predicate
/// gets a `!`, with parentheses when the predicate binds more loosely than `!`.
fn negate_predicate(predicate: &AnyJsExpression) -> Option<(AnyJsExpression, AnyJsExpression)> {
    if let AnyJsExpression::JsUnaryExpression(unary) = predicate.clone().omit_parentheses()
        && unary.operator().ok()? == JsUnaryOperator::LogicalNot
    {
        let argument = without_negation(&unary, unary.argument().ok()?)?;
        return Some((unary.into(), argument));
    }

    let needs_parentheses = predicate.precedence().ok()? < OperatorPrecedence::Unary
        || matches!(predicate, AnyJsExpression::JsArrowFunctionExpression(_));
    let leading_trivia = predicate.syntax().first_token()?.leading_trivia();
    let argument = predicate
        .clone()
        .with_leading_trivia_pieces(std::iter::empty())?;
    let argument = if needs_parentheses {
        make::parenthesized(argument).into()
    } else {
        argument
    };
    let negation = make::js_unary_expression(
        make::token(T![!]).with_leading_trivia_pieces(leading_trivia.pieces()),
        argument,
    );
    Some((predicate.clone(), negation.into()))
}

/// Returns `argument` prepared to replace `unary`, which negates the original argument.
///
/// Returns `None` if removing the `!` would remove a comment, or would change how the code around
/// it is parsed, as with `foo()\n![].some(callback)`, which would become `foo()[].every(...)`.
fn without_negation(
    unary: &JsUnaryExpression,
    argument: AnyJsExpression,
) -> Option<AnyJsExpression> {
    let operator = unary.operator_token().ok()?;
    if operator.has_trailing_comments()
        || unary
            .argument()
            .ok()?
            .syntax()
            .first_token()?
            .has_leading_comments()
    {
        return None;
    }

    let first_kind = argument.syntax().first_token()?.kind();
    let previous = operator.prev_token();
    let starts_statement = unary
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)
        .and_then(|statement| statement.syntax().first_token())
        .is_some_and(|first| first == operator);
    // `export default function () {}.a` would be parsed as a function declaration followed by `.a`.
    let follows_export_default = previous.as_ref().is_some_and(|previous| {
        previous.kind() == T![default]
            && previous
                .parent()
                .is_some_and(|parent| JsExportDefaultExpressionClause::can_cast(parent.kind()))
    });
    if (starts_statement || follows_export_default) && STATEMENT_START_HAZARDS.contains(first_kind)
    {
        return None;
    }

    // `() => {}.a` would be parsed as a function body followed by `.a`.
    let argument = if first_kind == T!['{']
        && previous
            .as_ref()
            .is_some_and(|previous| previous.kind() == T![=>])
    {
        make::parenthesized(argument).into()
    } else {
        argument
    };

    let first = argument.syntax().first_token()?;
    // `return!a` must not become `returna`.
    let needs_space = operator.leading_trivia().pieces().len() == 0
        && previous.is_some_and(|previous| {
            previous.kind().is_keyword() && previous.trailing_trivia().pieces().len() == 0
        });
    let new_first = if needs_space {
        first.with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
    } else {
        first.with_leading_trivia_pieces(operator.leading_trivia().pieces())
    };
    argument.replace_token_discard_trivia(first, new_first)
}

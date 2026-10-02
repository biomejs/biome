use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsExpression, AnyJsFunctionBody, AnyJsStatement,
    JsCallExpression, JsFunctionBody, JsLanguage, JsNewTargetExpression, JsReferenceIdentifier,
    JsThisExpression,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet, WalkEvent,
};
use biome_rule_options::no_svelte_useless_derived_by::NoSvelteUselessDerivedByOptions;

use crate::JsRuleAction;
use crate::lint::complexity::use_arrow_function::AnyThisScope;

declare_lint_rule! {
    /// Disallow `$derived.by()` when `$derived()` is sufficient.
    ///
    /// `$derived.by()` is only needed when a derivation requires statements, such as loops,
    /// intermediate variables, or side effects. When the function passed to `$derived.by()` only
    /// returns a single expression, passing that expression to `$derived()` directly is shorter
    /// and behaves the same way.
    ///
    /// The rule reports functions that take no parameters, are neither async nor generators, and
    /// whose body is either a single expression or a single `return` statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// let count = $state(0);
    /// const doubled = $derived.by(() => count * 2);
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// let count = $state(0);
    /// const doubled = $derived.by(() => {
    ///     return count * 2;
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// let count = $state(0);
    /// const doubled = $derived(count * 2);
    /// const doubledPlusOne = $derived.by(() => {
    ///     const doubled = count * 2;
    ///     return doubled + 1;
    /// });
    /// </script>
    /// ```
    ///
    pub NoSvelteUselessDerivedBy {
        version: "next",
        name: "noSvelteUselessDerivedBy",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("prefer-derived-over-derived-by").same()],
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoSvelteUselessDerivedBy {
    type Query = Ast<JsCallExpression>;
    /// The expression returned by the function passed to `$derived.by()`.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = NoSvelteUselessDerivedByOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte()
        {
            return None;
        }

        let call = ctx.query();
        let callee = call.callee().ok()?;
        let member = callee.as_js_static_member_expression()?;
        if !member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            .eq("by")
        {
            return None;
        }
        // Svelte forbids declaring `$`-prefixed bindings, so `$derived` always refers to the rune.
        if !member
            .object()
            .ok()?
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .has_name("$derived")
        {
            return None;
        }

        let args = call.arguments().ok()?.args();
        if args.len() != 1 {
            return None;
        }
        let function = args.first()?.ok()?.as_any_js_expression()?.clone();

        let expression = match &function {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                if arrow.async_token().is_some() {
                    return None;
                }
                match arrow.parameters().ok()? {
                    AnyJsArrowFunctionParameters::JsParameters(params)
                        if params.items().is_empty() => {}
                    _ => return None,
                }
                match arrow.body().ok()? {
                    AnyJsFunctionBody::AnyJsExpression(expression) => expression,
                    AnyJsFunctionBody::JsFunctionBody(body) => single_returned_expression(&body)?,
                }
            }
            AnyJsExpression::JsFunctionExpression(function_expression) => {
                if function_expression.async_token().is_some()
                    || function_expression.star_token().is_some()
                    || !function_expression.parameters().ok()?.items().is_empty()
                {
                    return None;
                }
                let expression = single_returned_expression(&function_expression.body().ok()?)?;
                // Moving the expression out of the function would change what `this`,
                // `arguments`, and `new.target` refer to.
                if uses_function_bindings(&expression) {
                    return None;
                }
                expression
            }
            _ => return None,
        };

        // Keep the parentheses around sequence expressions, otherwise each operand would become
        // a separate argument of `$derived()`.
        let unwrapped = expression.clone().omit_parentheses();
        if matches!(unwrapped, AnyJsExpression::JsSequenceExpression(_)) {
            Some(expression)
        } else {
            Some(unwrapped)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let callee = ctx.query().callee().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                callee.range(),
                markup! {
                    "This "<Emphasis>"$derived.by()"</Emphasis>" function only returns a single expression."
                },
            )
            .note(markup! {
                <Emphasis>"$derived.by()"</Emphasis>" is only needed when the derivation requires statements, such as loops or intermediate variables. Passing the expression to "<Emphasis>"$derived()"</Emphasis>" directly is shorter and behaves the same way."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, expression: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let callee = call.callee().ok()?;
        let member = callee.as_js_static_member_expression()?;
        let function = call
            .arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone();

        if drops_comments(&function, expression) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(callee.clone(), member.object().ok()?);
        mutation.replace_node(function, expression.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Pass the expression to "<Emphasis>"$derived()"</Emphasis>" directly." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Expressions that refer to the bindings of the function that contains them.
const FUNCTION_BINDING_KINDS: SyntaxKindSet<JsLanguage> =
    JsThisExpression::KIND_SET.union(JsNewTargetExpression::KIND_SET);

/// Returns the argument of the `return` statement when it is the only statement of `body`.
fn single_returned_expression(body: &JsFunctionBody) -> Option<AnyJsExpression> {
    if !body.directives().is_empty() {
        return None;
    }
    let mut statements = body.statements().iter();
    let AnyJsStatement::JsReturnStatement(return_statement) = statements.next()? else {
        return None;
    };
    if statements.next().is_some() {
        return None;
    }
    return_statement.argument()
}

/// Returns `true` if `expression` refers to `this`, `arguments`, or `new.target` of the function
/// that contains it.
fn uses_function_bindings(expression: &AnyJsExpression) -> bool {
    let mut preorder = expression.syntax().preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if AnyThisScope::can_cast(node.kind()) {
            // Nested functions bind their own `this`, `arguments`, and `new.target`.
            preorder.skip_subtree();
        } else if FUNCTION_BINDING_KINDS.matches(node.kind())
            || JsReferenceIdentifier::cast_ref(&node)
            .is_some_and(|reference| reference.has_name("arguments"))
        {
            return true;
        }
    }
    false
}

/// Returns `true` if replacing `function` with `expression` would remove a comment.
///
/// The replacement keeps the comments inside `expression` and the outer trivia of `function`.
/// It drops every other token of `function`, as well as the outer trivia of `expression`.
fn drops_comments(function: &AnyJsExpression, expression: &AnyJsExpression) -> bool {
    let function = function.syntax();
    let expression = expression.syntax();
    let expression_range = expression.text_trimmed_range();
    let function_first = function.first_token();
    let function_last = function.last_token();
    let expression_first = expression.first_token();
    let expression_last = expression.last_token();
    function.descendants_tokens(Direction::Next).any(|token| {
        let in_expression = expression_range.contains_range(token.text_trimmed_range());
        let keeps_leading = function_first.as_ref() == Some(&token)
            || (in_expression && expression_first.as_ref() != Some(&token));
        let keeps_trailing = function_last.as_ref() == Some(&token)
            || (in_expression && expression_last.as_ref() != Some(&token));
        (!keeps_leading && token.has_leading_comments())
            || (!keeps_trailing && token.has_trailing_comments())
    })
}

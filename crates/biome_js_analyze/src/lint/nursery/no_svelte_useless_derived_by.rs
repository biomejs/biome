use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsExpression, AnyJsFunctionBody, AnyJsIdentifierReference,
    AnyJsStatement, JsCallExpression, JsComputedMemberName, JsConstructorParameters, JsDecorator,
    JsFormalParameter, JsFunctionBody, JsLanguage, JsNewTargetExpression, JsParameters,
    JsReferenceIdentifier, JsThisExpression, unescape_js_identifier,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxKindSet, WalkEvent,
};
use biome_rule_options::no_svelte_useless_derived_by::NoSvelteUselessDerivedByOptions;

use crate::JsRuleAction;
use crate::lint::complexity::use_arrow_function::AnyThisScope;
use crate::services::{embedded::EmbeddedService, semantic::Semantic};

declare_lint_rule! {
    /// Disallow `$derived.by()` when `$derived()` is sufficient.
    ///
    /// `$derived.by()` is only needed when a derivation requires statements, such as loops,
    /// intermediate variables, or side effects. When the function passed to `$derived.by()` only
    /// returns a single expression, passing that expression to `$derived()` directly is shorter
    /// and behaves the same way.
    ///
    /// The rule reports functions that take no parameters, are neither async nor generators, and
    /// whose body is either a single expression or a single `return` statement. It keeps named
    /// functions when the returned expression refers to the function by name.
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
        version: "2.6.0",
        name: "noSvelteUselessDerivedBy",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("prefer-derived-over-derived-by").same()],
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoSvelteUselessDerivedBy {
    type Query = Semantic<JsCallExpression>;
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
        let reference = member
            .object()
            .ok()?
            .as_js_identifier_expression()?
            .name()
            .ok()?;
        let embedded = ctx
            .get_service::<EmbeddedService>()
            .expect("embedded service");
        if !embedded.is_svelte_rune_reference(ctx.model(), &reference, "$derived") {
            return None;
        }

        let args = call.arguments().ok()?.args();
        if args.len() != 1 {
            return None;
        }
        let function = args.first()?.ok()?.as_any_js_expression()?.clone();

        let expression = match &function {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                if arrow.async_token().is_some()
                    || arrow.type_parameters().is_some()
                    || arrow.return_type_annotation().is_some()
                {
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
                    || function_expression.type_parameters().is_some()
                    || function_expression.return_type_annotation().is_some()
                    || !function_expression.parameters().ok()?.items().is_empty()
                {
                    return None;
                }
                let expression = single_returned_expression(&function_expression.body().ok()?)?;
                if let Some(name) = function_expression.id() {
                    let binding = name.as_js_identifier_binding()?;
                    if binding.all_references(ctx.model()).next().is_some()
                        || method_key_mentions(
                            &expression,
                            &unescape_js_identifier(binding.name_token().ok()?.text_trimmed()),
                        )
                    {
                        return None;
                    }
                }
                // Moving the expression out of the function would change what `this`,
                // `arguments`, and `new.target` refer to.
                if uses_function_bindings(&expression) {
                    return None;
                }
                expression
            }
            _ => return None,
        };

        // Direct eval can observe the callback scope even inside a returned closure.
        if expression
            .syntax()
            .preorder_of_kinds(JsCallExpression::KIND_SET)
            .filter_map(|event| match event {
                WalkEvent::Enter(node) => JsCallExpression::cast(node),
                WalkEvent::Leave(_) => None,
            })
            .any(|call| {
                !call.is_optional()
                    && call
                        .callee()
                        .ok()
                        .and_then(AnyJsExpression::omit_type_wrappers)
                        .and_then(|callee| callee.as_js_reference_identifier())
                        .is_some_and(|reference| has_identifier_name(&reference, "eval"))
            })
        {
            return None;
        }

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

        let object = member.object().ok()?;
        if drops_comments(&callee, &object) || drops_comments(&function, expression) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(callee, object);
        let argument = if matches!(expression, AnyJsExpression::JsSequenceExpression(_)) {
            make::parenthesized(expression.clone().trim_trivia()?).into()
        } else {
            expression.clone()
        };
        mutation.replace_node(function, argument);
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

// A method parameter can mask a reference in the semantic model even though the key
// is evaluated outside the method scope. Keep the callback if the key might use its name.
fn method_key_mentions(expression: &AnyJsExpression, name: &str) -> bool {
    let mut keys = expression
        .syntax()
        .preorder_of_kinds(JsComputedMemberName::KIND_SET);
    while let Some(event) = keys.next() {
        let WalkEvent::Enter(key) = event else {
            continue;
        };
        if key
            .parent()
            .is_some_and(|parent| AnyThisScope::can_cast(parent.kind()))
        {
            if key.preorder_of_kinds(AnyJsIdentifierReference::KIND_SET).any(|event| {
                matches!(event, WalkEvent::Enter(node) if AnyJsIdentifierReference::cast_ref(&node)
                    .and_then(|reference| reference.value_token().ok())
                    .is_some_and(|token| unescape_js_identifier(token.text_trimmed()) == name))
            }) {
                return true;
            }
            keys.skip_subtree();
        }
    }
    false
}

/// Returns `true` if `expression` refers to `this`, `arguments`, or `new.target` of the function
/// that contains it.
fn uses_function_bindings(expression: &AnyJsExpression) -> bool {
    let parameter_kinds = JsParameters::KIND_SET
        .union(JsConstructorParameters::KIND_SET)
        .union(JsFormalParameter::KIND_SET);
    let mut preorder = expression.syntax().preorder_of_kinds(
        JsFunctionBody::KIND_SET
            .union(parameter_kinds)
            .union(FUNCTION_BINDING_KINDS)
            .union(JsReferenceIdentifier::KIND_SET),
    );
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        // Non-arrow parameters and bodies have their own function bindings. Computed
        // names and parameter decorators can still see the surrounding callback.
        if (JsFunctionBody::can_cast(node.kind()) || parameter_kinds.matches(node.kind()))
            && node
                .parent()
                .is_some_and(|parent| AnyThisScope::can_cast(parent.kind()))
            && (JsFunctionBody::can_cast(node.kind())
                || node
                    .preorder_of_kinds(JsDecorator::KIND_SET)
                    .next()
                    .is_none())
        {
            preorder.skip_subtree();
        } else if FUNCTION_BINDING_KINDS.matches(node.kind())
            || JsReferenceIdentifier::cast_ref(&node)
                .is_some_and(|reference| has_identifier_name(&reference, "arguments"))
        {
            return true;
        }
    }
    false
}

fn has_identifier_name(reference: &JsReferenceIdentifier, name: &str) -> bool {
    reference
        .value_token()
        .is_ok_and(|token| unescape_js_identifier(token.text_trimmed()) == name)
}

/// Replacing a node keeps its outer trivia and the replacement's interior trivia.
fn drops_comments(original: &AnyJsExpression, replacement: &AnyJsExpression) -> bool {
    let original_range = original.range();
    let replacement_range = replacement.range();
    original.syntax().borrowed_tokens().any(|token| {
        token
            .leading_trivia_pieces()
            .chain(token.trailing_trivia_pieces())
            .any(|piece| {
                piece.kind().is_comment()
                    && original_range.contains_range(piece.text_range())
                    && !replacement_range.contains_range(piece.text_range())
            })
    })
}

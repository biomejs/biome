use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsLogicalExpression, JsObjectExpression, JsSyntaxKind,
    JsThrowStatement,
};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow throwing non-`Error` values.
    ///
    /// It is considered good practice only to throw the `Error` object itself or an object using the `Error` object
    /// as base objects for user-defined exceptions. The fundamental benefit of `Error` objects is that they automatically
    /// keep track of where they were built and originated.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// throw undefined;
    /// ```
    /// ```js,expect_diagnostic
    /// throw false;
    /// ```
    /// ```js,expect_diagnostic
    /// throw "a" + "b";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// throw new Error();
    /// ```
    /// ```js
    /// throw new TypeError('biome');
    /// ```
    /// ```js
    /// class CustomError extends Error {}
    ///
    /// throw new CustomError();
    /// ```
    ///
    /// ## Caveats
    ///
    /// This rule only covers cases where throwing the value can be known statically.
    /// Complex cases such as object and function access aren't checked.
    /// This will be improved in the future once Biome supports type inference.
    ///
    pub UseThrowOnlyError {
        version: "1.8.0",
        name: "useThrowOnlyError",
        language: "js",
        sources: &[RuleSource::Eslint("no-throw-literal"), RuleSource::EslintTypeScript("only-throw-error")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

impl Rule for UseThrowOnlyError {
    type Query = Ast<JsThrowStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let expr = node.argument().ok()?.omit_parentheses();

        is_invalid_throw_value(&expr).and(Some(()))
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Throwing non-"<Emphasis>"Error"</Emphasis>" values is not allowed."
            },
        ).note(markup! {
            "While Javascript supports throwing any value, handling non-"<Emphasis>"Error"</Emphasis>" values is confusing."
        }))
    }
}

fn is_invalid_throw_value(any_expr: &AnyJsExpression) -> Option<bool> {
    let kind = any_expr.syntax().kind();

    if AnyJsLiteralExpression::can_cast(kind)
        || JsObjectExpression::can_cast(kind)
        || matches!(
            kind,
            JsSyntaxKind::JS_BINARY_EXPRESSION | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
        )
    {
        return Some(true);
    }

    if let Some(logical_expr) = JsLogicalExpression::cast_ref(any_expr.syntax()) {
        let left = &logical_expr.left().ok()?;

        // This will produce some false positives, but having a logical expression
        // as a throw value is not a good practice anyway.
        return is_invalid_throw_value(left).or_else(|| {
            let right = logical_expr.right().ok()?;

            is_invalid_throw_value(&right)
        });
    }

    if let Some(assignment_expr) = any_expr.as_js_assignment_expression() {
        return is_invalid_throw_value(&assignment_expr.right().ok()?.omit_parentheses());
    }

    if let Some(expr) = any_expr.as_js_sequence_expression() {
        return is_invalid_throw_value(&expr.right().ok()?);
    }

    if let Some(expr) = any_expr.as_js_conditional_expression() {
        let consequent = expr.consequent().ok()?;

        return is_invalid_throw_value(&consequent).or_else(|| {
            let alternate = expr.alternate().ok()?;

            is_invalid_throw_value(&alternate)
        });
    }

    if let Some(identifier) = any_expr.as_js_reference_identifier() {
        if identifier.is_undefined() {
            return Some(true);
        }
    }

    None
}

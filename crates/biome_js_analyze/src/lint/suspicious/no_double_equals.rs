use crate::JsRuleAction;
use biome_analyze::RuleSource;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, T};
use biome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use biome_rowan::{BatchMutationExt, SyntaxResult};
use biome_rule_options::no_double_equals::NoDoubleEqualsOptions;

declare_lint_rule! {
    /// Require the use of `===` and `!==`.
    ///
    /// It is generally bad practice to use `==` for comparison instead of
    /// `===`. Double operators will trigger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion)
    /// and are thus not preferred. Using strict equality operators is almost
    /// always best practice.
    ///
    /// For ergonomic reasons, this rule makes by default an exception for `== null` for
    /// comparing to both `null` and `undefined`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo == bar
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo == null
    ///```
    ///
    /// ```js
    /// foo != null
    ///```
    ///
    /// ```js
    /// null == foo
    ///```
    ///
    /// ```js
    /// null != foo
    ///```
    ///
    /// ## Options
    ///
    /// The rule provides the option described below.
    ///
    /// ```json
    /// {
    ///     "//":"...",
    ///     "options": {
    ///         "ignoreNull": true
    ///     }
    /// }
    /// ```
    ///
    /// ### ignoreNull
    ///
    /// When this option is set to `true`, an exception will be made for checking against `null`,
    /// as relying on the double equals operator to compare with `null` is frequently used to check
    /// equality with either `null` or `undefined`.
    ///
    /// When the option is set to `false`, all double equal operators will be forbidden without
    /// exceptions.
    ///
    /// Default: `true`
    ///
    ///
    pub NoDoubleEquals {
        version: "1.0.0",
        name: "noDoubleEquals",
        language: "js",
        sources: &[RuleSource::Eslint("eqeqeq").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoDoubleEquals {
    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoDoubleEqualsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        let op = n.operator_token().ok()?;

        if !matches!(op.kind(), EQ2 | NEQ) {
            return None;
        }

        // TODO: Implement SyntaxResult helpers to make this cleaner
        if ctx.options().ignore_null && (is_null_literal(&n.left()) || is_null_literal(&n.right()))
        {
            return None;
        }

        Some(op)
    }

    fn diagnostic(ctx: &RuleContext<Self>, op: &Self::State) -> Option<RuleDiagnostic> {
        let text_trimmed = op.text_trimmed();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            op.text_trimmed_range(),
            markup! {
                "Using "<Emphasis>{text_trimmed}</Emphasis>" may be unsafe if you are relying on type coercion."
            },
        );

        Some(if ctx.options().ignore_null {
            diagnostic
                .note(markup! {
                    <Emphasis>{text_trimmed}</Emphasis>" is only allowed when comparing against "<Emphasis>"null"</Emphasis>"."
                })
        } else {
            diagnostic
        })
    }

    fn action(ctx: &RuleContext<Self>, op: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let suggestion = if op.kind() == EQ2 { T![===] } else { T![!==] };
        mutation.replace_token(op.clone(), make::token(suggestion));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            // SAFETY: `suggestion` can only be JsSyntaxKind::EQ3 or JsSyntaxKind::NEQ2,
            // the implementation of `to_string` for these two variants always returns Some
            markup! { "Use "<Emphasis>{suggestion.to_string()?}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn is_null_literal(res: &SyntaxResult<AnyJsExpression>) -> bool {
    matches!(
        res,
        Ok(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNullLiteralExpression(_)
        ))
    )
}

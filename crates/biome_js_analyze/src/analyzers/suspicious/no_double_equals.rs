use biome_analyze::RuleSource;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, T};
use biome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use biome_rowan::{BatchMutationExt, SyntaxResult};

use crate::JsRuleAction;

declare_rule! {
    /// Require the use of `===` and `!==`
    ///
    /// It is generally bad practice to use `==` for comparison instead of
    /// `===`. Double operators will triger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion)
    /// and are thus not prefered. Using strict equality operators is almost
    /// always best practice.
    ///
    /// For ergonomic reasons, this rule makes an exception for `== null` for
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
    pub(crate) NoDoubleEquals {
        version: "1.0.0",
        name: "noDoubleEquals",
        source: RuleSource::Eslint("eqeqeq"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoDoubleEquals {
    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        let op = n.operator_token().ok()?;

        if !matches!(op.kind(), EQ2 | NEQ) {
            return None;
        }

        // TODO: Implement SyntaxResult helpers to make this cleaner
        if is_null_literal(&n.left()) || is_null_literal(&n.right()) {
            return None;
        }

        Some(op)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, op: &Self::State) -> Option<RuleDiagnostic> {
        let text_trimmed = op.text_trimmed();
        let suggestion = if op.kind() == EQ2 { "===" } else { "!==" };
        let description = format!(
            "Use {} instead of {}.\n{} is only allowed when comparing against `null`",
            suggestion, text_trimmed, text_trimmed
        );
        Some(RuleDiagnostic::new(
            rule_category!(),
            op.text_trimmed_range(),
            markup! {
                "Use "<Emphasis>{suggestion}</Emphasis>" instead of "<Emphasis>{text_trimmed}</Emphasis>
            },
        )
        .detail(op.text_trimmed_range(), markup! {
            <Emphasis>{text_trimmed}</Emphasis>" is only allowed when comparing against "<Emphasis>"null"</Emphasis>
        }).note(markup! {
            "Using "<Emphasis>{suggestion}</Emphasis>" may be unsafe if you are relying on type coercion"
        })
        .description(description))
    }

    fn action(ctx: &RuleContext<Self>, op: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let suggestion = if op.kind() == EQ2 { T![===] } else { T![!==] };
        mutation.replace_token(op.clone(), make::token(suggestion));

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            // SAFETY: `suggestion` can only be JsSyntaxKind::EQ3 or JsSyntaxKind::NEQ2,
            // the implementation of `to_string` for these two variants always returns Some
            message: markup! { "Use "<Emphasis>{suggestion.to_string().unwrap()}</Emphasis> }
                .to_owned(),
            mutation,
        })
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

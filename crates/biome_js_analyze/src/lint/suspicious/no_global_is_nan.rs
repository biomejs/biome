use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{global_identifier, AnyJsExpression, T};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Use `Number.isNaN` instead of global `isNaN`.
    ///
    /// `Number.isNaN()` and `isNaN()` [do not have the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).
    /// When the argument to `isNaN()` is not a number, the value is first coerced to a number.
    /// `Number.isNaN()` does not perform this coercion.
    /// Therefore, it is a more reliable way to test whether a value is `NaN`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// isNaN({}); // true
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Number.isNaN({}); // false
    /// ```
    ///
    pub NoGlobalIsNan {
        version: "1.0.0",
        name: "noGlobalIsNan",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoGlobalIsNan {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        if name.text() != "isNaN" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    <Emphasis>"isNaN"</Emphasis>" is unsafe. It attempts a type coercion. Use "<Emphasis>"Number.isNaN"</Emphasis>" instead."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description">"the MDN documentation"</Hyperlink>" for more details."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (old, new) = match node {
            AnyJsExpression::JsIdentifierExpression(expression) => (
                node.clone(),
                make::js_static_member_expression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "Number",
                    )))
                    .into(),
                    make::token(T![.]),
                    make::js_name(expression.name().ok()?.value_token().ok()?).into(),
                ),
            ),
            AnyJsExpression::JsStaticMemberExpression(expression) => (
                node.clone(),
                make::js_static_member_expression(
                    make::js_static_member_expression(
                        expression.object().ok()?,
                        make::token(T![.]),
                        make::js_name(make::ident("Number")).into(),
                    )
                    .into(),
                    expression.operator_token().ok()?,
                    expression.member().ok()?,
                ),
            ),
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                let object = expression.object().ok()?;
                (
                    object.clone(),
                    make::js_static_member_expression(
                        object,
                        make::token(T![.]),
                        make::js_name(make::ident("Number")).into(),
                    ),
                )
            }
            _ => return None,
        };
        mutation.replace_node(old, new.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"Number.isNaN"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

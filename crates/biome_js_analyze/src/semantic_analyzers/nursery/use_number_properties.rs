use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{global_identifier, AnyJsExpression, JsSyntaxKind, T};
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Use `Number` properties instead of global ones.
    ///
    /// ECMAScript 2015 moved globals onto the `Number` constructor for consistency and to slightly improve them.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// parseInt("1"); // true
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// parseFloat("1.1"); // true
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// NaN; // true
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Infinity; // true
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// -Infinity; // true
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// Number.parseInt("1"); // false
    /// ```
    ///
    /// ```js
    /// Number.parseFloat("1.1"); // false
    /// ```
    ///
    /// ```js
    /// Number.NaN; // false
    /// ```
    ///
    /// ```js
    /// Number.POSITIVE_INFINITY; // false
    /// ```
    ///
    /// ```js
    /// Number.NEGATIVE_INFINITY; // false
    /// ```
    ///
    pub(crate) UseNumberProperties {
        version: "next",
        name: "useNumberProperties",
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseNumberProperties {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        let names = ["parseInt", "parseFloat", "NaN", "Infinity"];
        if !names.contains(&name.text()) {
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
                    "Use the "<Emphasis>"Number"</Emphasis>" properties instead of the global ones."
                },
            )
            .note(markup! {
                "ECMAScript 2015 moved globals onto the "<Emphasis>"Number"</Emphasis>" constructor for consistency and to slightly improve them."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (old, new) = match node {
            AnyJsExpression::JsIdentifierExpression(expression) => {
                let name = expression.name().ok()?.text();
                let replacement = match name.as_str() {
                    "parseInt" => "parseInt",
                    "parseFloat" => "parseFloat",
                    "NaN" => "NaN",
                    "Infinity" => {
                        let is_negative = expression
                            .syntax()
                            .parent()
                            .and_then(|parent| {
                                parent.children_with_tokens().find_map(|child| match child {
                                    biome_rowan::NodeOrToken::Token(token) => {
                                        if token.kind() == JsSyntaxKind::MINUS {
                                            Some(true)
                                        } else {
                                            None
                                        }
                                    }
                                    biome_rowan::NodeOrToken::Node(_) => None,
                                })
                            })
                            .unwrap_or(false);
                        if is_negative {
                            "NEGATIVE_INFINITY"
                        } else {
                            "POSITIVE_INFINITY"
                        }
                    }
                    _ => return None,
                };
                (
                    if replacement == "NEGATIVE_INFINITY" {
                        node.parent::<AnyJsExpression>()?.clone()
                    } else {
                        node.clone()
                    },
                    make::js_static_member_expression(
                        make::js_identifier_expression(make::js_reference_identifier(make::ident(
                            "Number",
                        )))
                        .into(),
                        make::token(T![.]),
                        make::js_name(make::ident(replacement)).into(),
                    ),
                )
            }
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
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! {
                "Use "<Emphasis>"Number"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        })
    }
}

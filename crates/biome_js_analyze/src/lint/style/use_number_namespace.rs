use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, static_value::StaticValue, AnyJsExpression, JsUnaryExpression,
    JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Use the `Number` properties instead of global ones.
    ///
    /// _ES2015_ moved some globals into the `Number` properties for consistency.
    ///
    /// The rule doesn't report the globals `isFinite` and `isNaN` because they have a slightly different behavior to their corresponding `Number`'s properties `Number.isFinite` and `Number.isNaN`.
    /// You can use the dedicated rules [noGlobalIsFinite](https://biomejs.dev/linter/rules/no-global-is-finite/) and  [noGlobalIsNan](https://biomejs.dev/linter/rules/no-global-is-nan/) to enforce the use of `Number.isFinite` and `Number.isNaN`.
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
    /// ### Valid
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
    pub UseNumberNamespace {
        version: "1.5.0",
        name: "useNumberNamespace",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-number-properties")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

const GLOBAL_NUMBER_PROPERTIES: [&str; 4] = ["parseInt", "parseFloat", "NaN", "Infinity"];

impl Rule for UseNumberNamespace {
    type Query = Semantic<AnyJsExpression>;
    type State = StaticValue;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (reference, global_ident) = global_identifier(node)?;
        if !GLOBAL_NUMBER_PROPERTIES.contains(&global_ident.text()) {
            return None;
        }
        ctx.model()
            .binding(&reference)
            .is_none()
            .then_some(global_ident)
    }

    fn diagnostic(ctx: &RuleContext<Self>, global_ident: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let equivalent_property = match global_ident.text() {
            "Infinity" => {
                if let Some(parent) = node.parent::<JsUnaryExpression>() {
                    match parent.operator().ok()? {
                        JsUnaryOperator::Minus => "NEGATIVE_INFINITY",
                        JsUnaryOperator::Plus => "POSITIVE_INFINITY",
                        _ => return None,
                    }
                } else {
                    "POSITIVE_INFINITY"
                }
            }
            other => other,
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>"Number."{equivalent_property}</Emphasis>" instead of the equivalent global."
                },
            )
            .note(markup! {
                "ES2015 moved some globals into the "<Emphasis>"Number"</Emphasis>" namespace for consistency."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, global_ident: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let (old_node, new_node) = match node {
            AnyJsExpression::JsIdentifierExpression(expression) => {
                let name = expression.name().ok()?.to_trimmed_string();
                if !GLOBAL_NUMBER_PROPERTIES.contains(&name.as_str()) {
                    return None;
                }
                let (old_node, replacement) = match name.as_str() {
                    "Infinity" => {
                        if let Some(parent) = node.parent::<JsUnaryExpression>() {
                            match parent.operator().ok()? {
                                JsUnaryOperator::Minus => (
                                    AnyJsExpression::JsUnaryExpression(parent),
                                    "NEGATIVE_INFINITY",
                                ),
                                JsUnaryOperator::Plus => (
                                    AnyJsExpression::JsUnaryExpression(parent),
                                    "POSITIVE_INFINITY",
                                ),
                                _ => return None,
                            }
                        } else {
                            (node.clone(), "POSITIVE_INFINITY")
                        }
                    }
                    _ => (node.clone(), name.as_str()),
                };
                (
                    old_node,
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
            AnyJsExpression::JsStaticMemberExpression(expression) => {
                let name = expression.member().ok()?.to_trimmed_string();

                if !GLOBAL_NUMBER_PROPERTIES.contains(&name.as_str()) {
                    return None;
                }
                let (old_node, replacement) = match name.as_str() {
                    "Infinity" => {
                        if let Some(parent) = node.parent::<JsUnaryExpression>() {
                            match parent.operator().ok()? {
                                JsUnaryOperator::Minus => (
                                    AnyJsExpression::JsUnaryExpression(parent),
                                    "NEGATIVE_INFINITY",
                                ),
                                JsUnaryOperator::Plus => (
                                    AnyJsExpression::JsUnaryExpression(parent),
                                    "POSITIVE_INFINITY",
                                ),
                                _ => return None,
                            }
                        } else {
                            (node.clone(), "POSITIVE_INFINITY")
                        }
                    }
                    _ => (node.clone(), name.as_str()),
                };
                (
                    old_node,
                    make::js_static_member_expression(
                        make::js_static_member_expression(
                            expression.object().ok()?,
                            make::token(T![.]),
                            make::js_name(make::ident("Number")).into(),
                        )
                        .into(),
                        expression.operator_token().ok()?,
                        make::js_name(make::ident(replacement)).into(),
                    ),
                )
            }
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
        let mut mutation = ctx.root().begin();
        mutation.replace_node(old_node, new_node.into());
        let equivalent_property = match global_ident.text() {
            "Infinity" => {
                if let Some(parent) = node.parent::<JsUnaryExpression>() {
                    match parent.operator().ok()? {
                        JsUnaryOperator::Minus => "NEGATIVE_INFINITY",
                        JsUnaryOperator::Plus => "POSITIVE_INFINITY",
                        _ => return None,
                    }
                } else {
                    "POSITIVE_INFINITY"
                }
            }
            other => other,
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"Number."{equivalent_property}</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

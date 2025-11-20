use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsLiteralExpression, AnyJsxAttributeValue, JsSyntaxKind, JsxAttribute,
    JsxAttributeInitializerClause, T,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_consistent_boolean_props::{
    BooleanPropMode, UseConsistentBooleanPropsOptions,
};

declare_lint_rule! {
    /// Enforces consistent usage of boolean props in JSX attributes.
    ///
    /// ## Options
    ///
    /// `mode` — Controls whether boolean props should be implicit or explicit.
    ///
    /// - `Implicit` (default): enforces `<Foo bar />` style for `true` values.
    /// - `Explicit`: enforces `<Foo bar={true} />` style for `true` values.
    ///
    /// ```json,ignore
    /// // Implicit mode (default)
    /// {
    ///   "useConsistentBooleanProps": { "mode": "implicit" }
    /// }
    ///
    /// // Explicit mode
    /// {
    ///   "useConsistentBooleanProps": { "mode": "explicit" }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid (Implicit mode)
    ///
    /// ```jsx,expect_diagnostic
    /// <input disabled={true} />
    /// ```
    ///
    /// ### Valid (Implicit mode)
    ///
    /// ```jsx
    /// <input disabled />
    /// ```
    ///
    /// ### Invalid (Explicit mode)
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "mode": "explicit"
    ///     }
    /// }
    /// ```
    /// ```jsx,use_options,expect_diagnostic
    /// <input disabled />
    /// ```
    ///
    /// ### Valid (Explicit mode)
    ///
    /// ```jsx,use_options
    /// <input disabled={true} />
    /// ```
    ///
    /// ```jsx,use_options
    /// <input disabled={false} />
    /// ```
    pub UseConsistentBooleanProps {
        version: "next",
        name: "useConsistentBooleanProps",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-boolean-value").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseConsistentBooleanProps {
    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentBooleanPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let jsx_attribute = ctx.query();
        let options = ctx.options();
        let mode = options.mode.clone().unwrap_or_default();

        match (mode, jsx_attribute.initializer()) {
            (BooleanPropMode::Implicit, Some(jsx_attribute_initializer_clause)) => {
                if is_true_literal(&jsx_attribute_initializer_clause) {
                    return Some(());
                }
                None
            }
            (BooleanPropMode::Explicit, None) => Some(()),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let jsx_attribute = ctx.query();
        let mode = ctx.options().mode.clone().unwrap_or_default();

        let prefix = "Boolean JSX prop with value `true` should be ";
        // Determine the proper message based on mode
        let message = match mode {
            BooleanPropMode::Implicit => {
                markup! { {prefix} "implicit (omit "<Emphasis>"={true}"</Emphasis>")." }
            }
            BooleanPropMode::Explicit => {
                markup! { {prefix} "explicit ("<Emphasis>"={true}"</Emphasis>" must be present)." }
            }
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            jsx_attribute.range(),
            message.to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let jsx_attribute = ctx.query();
        let mode = ctx.options().mode.clone().unwrap_or_default();

        let mut mutation = ctx.root().begin();
        let mut mutation_has_changes = false;

        match mode {
            BooleanPropMode::Explicit => {
                // Explicit mode: add `={true}` if missing
                if jsx_attribute.initializer().is_none() {
                    let attr_value = make::jsx_expression_attribute_value(
                        make::token(JsSyntaxKind::L_CURLY),
                        biome_js_syntax::AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsBooleanLiteralExpression(
                                make::js_boolean_literal_expression(make::token(T![true])),
                            ),
                        ),
                        make::token(JsSyntaxKind::R_CURLY),
                    );

                    let initializer = make::jsx_attribute_initializer_clause(
                        make::token(T![=]),
                        AnyJsxAttributeValue::JsxExpressionAttributeValue(attr_value),
                    );

                    let next_attr = jsx_attribute.clone().with_initializer(Some(initializer));
                    mutation_has_changes = true;
                    mutation.replace_node(jsx_attribute.clone(), next_attr);
                }
            }
            BooleanPropMode::Implicit => {
                // Implicit mode: remove initializer if it is `true`
                if let Some(init) = jsx_attribute.initializer()
                    && let Ok(expr) = init.value()
                    && let Some(value) = expr.as_jsx_expression_attribute_value()
                    && let Ok(inner) = value.expression()
                    && let Some(literal_expression) = inner.as_any_js_literal_expression()
                    && literal_expression.as_js_boolean_literal_expression().is_some()
                {
                    // Remove initializer
                    let next_attr = make::jsx_attribute(jsx_attribute.name().ok()?).build();
                    mutation_has_changes = true;
                    mutation.replace_node(jsx_attribute.clone(), next_attr);
                }
            }
        }

        if mutation_has_changes {
            Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                match mode {
                    BooleanPropMode::Explicit => {
                        markup! { "Add explicit `true` literal for this attribute" }.to_owned()
                    }
                    BooleanPropMode::Implicit => {
                        markup! { "Remove explicit `true` literal for this attribute" }.to_owned()
                    }
                },
                mutation,
            ))
        } else {
            None
        }
    }
}

/// Checks whether the given JSX attribute initializer clause represents a `true` literal. e.g. disabled={true},
/// and NOT disabled="true" or disabled={false} etc.
fn is_true_literal(jsx_attribute_initializer_clause: &JsxAttributeInitializerClause) -> bool {
    if let Ok(expr) = jsx_attribute_initializer_clause.value()
        && let Some(lit_expr) = expr.as_jsx_expression_attribute_value()
        && let Ok(expression) = lit_expr.expression()
    {
        return expression.syntax().text_trimmed() == "true";
    }

    false
}

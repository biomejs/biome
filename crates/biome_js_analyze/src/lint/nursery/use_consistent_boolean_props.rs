use crate::JsRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsBooleanLiteralExpression, JsxAttribute, T};
use biome_rowan::{AstNode, AstNodeExt, BatchMutationExt};
use biome_rule_options::use_consistent_boolean_props::{
    BooleanPropMode, UseConsistentBooleanPropsOptions,
};

declare_lint_rule! {
    /// Require a consistent style for JSX props with the value `true`
    ///
    /// In JSX, `<Foo bar />` and `<Foo bar={true} />` are equivalent.
    ///
    /// If `style.noImplicitBoolean` is enabled, turn it off before using `implicit` mode.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo bar />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Foo bar={true} enabled={false} disabled={disabled} />
    /// ```
    ///
    /// ## Options
    ///
    /// ### `mode`
    ///
    /// The default, `explicit`, requires `={true}`. Use `implicit` to require the shorthand.
    ///
    /// ```json,options
    /// { "options": { "mode": "implicit" } }
    /// ```
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// <Foo bar={true} />
    /// ```
    ///
    pub UseConsistentBooleanProps {
        version: "next",
        name: "useConsistentBooleanProps",
        language: "jsx",
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseConsistentBooleanProps {
    type Query = Ast<JsxAttribute>;
    type State = BooleanPropMode;
    type Signals = Option<Self::State>;
    type Options = UseConsistentBooleanPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let mode = ctx.options().mode.unwrap_or_default();
        match mode {
            BooleanPropMode::Implicit => {
                let expression = attribute
                    .initializer()?
                    .value()
                    .ok()?
                    .as_jsx_expression_attribute_value()?
                    .expression()
                    .ok()?;
                let literal = JsBooleanLiteralExpression::cast(expression.omit_parentheses().into_syntax())?;
                // The React JSX transform requires explicit values for these reserved props.
                literal.value_token().ok()?.kind() == T![true]
                    && !attribute.name().ok()?.as_jsx_name().is_some_and(|name| {
                        name.value_token().is_ok_and(|token| {
                            matches!(token.text_trimmed(), "key" | "__self" | "__source")
                        })
                    })
            }
            BooleanPropMode::Explicit => attribute.initializer().is_none(),
        }
        .then_some(mode)
    }

    fn diagnostic(ctx: &RuleContext<Self>, mode: &Self::State) -> Option<RuleDiagnostic> {
        let message = match mode {
            BooleanPropMode::Implicit => {
                "This JSX attribute already implies true when written without a value."
            }
            BooleanPropMode::Explicit => {
                "This JSX attribute omits the required explicit true value."
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            message,
        ))
    }

    fn action(ctx: &RuleContext<Self>, mode: &Self::State) -> Option<JsRuleAction> {
        let attribute = ctx.query();
        let (replacement, message) = match mode {
            BooleanPropMode::Implicit => {
                if attribute.syntax().has_inner_comments() {
                    return None;
                }
                (
                    attribute.clone().with_initializer(None),
                    "Remove the explicit true value.",
                )
            }
            BooleanPropMode::Explicit => (
                with_explicit_true(attribute)?,
                "Add an explicit true value.",
            ),
        };
        let mut mutation = ctx.root().begin();
        if matches!(mode, BooleanPropMode::Implicit)
            && let Some(next) = attribute
                .syntax()
                .next_sibling()
                .and_then(JsxAttribute::cast)
            && attribute.range().end() == next.range().start()
        {
            // Removing the initializer would otherwise join adjacent prop names.
            let last = replacement.syntax().last_token()?;
            let spaced = last.with_trailing_space();
            mutation.replace_node_discard_trivia(
                attribute.clone(),
                replacement.replace_token_discard_trivia(last, spaced)?,
            );
        } else {
            mutation.replace_node(attribute.clone(), replacement);
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

fn with_explicit_true(attribute: &JsxAttribute) -> Option<JsxAttribute> {
    let value = make::jsx_expression_attribute_value(
        make::token(T!['{']),
        AnyJsExpression::AnyJsLiteralExpression(
            make::js_boolean_literal_expression(make::token(T![true])).into(),
        ),
        make::token(T!['}']),
    );
    Some(
        attribute
            .clone()
            .with_trailing_trivia_pieces([])?
            .with_initializer(Some(make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                value.into(),
            ))),
    )
}

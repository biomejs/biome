use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsLiteralMemberName, JsNumberLiteralExpression, JsSyntaxKind};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_octal::NoOctalOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

declare_lint_rule! {
    /// Disallow numbers written with an extra leading zero.
    ///
    /// A leading zero can make a number use base 8 instead of base 10. For example,
    /// `071` represents the decimal value `57`. This notation is deprecated and
    /// forbidden in strict mode. Use the explicit `0o` prefix for octal numbers.
    ///
    /// This rule also reports decimal numbers with leading zeros, such as `08`
    /// and `09.1`, because their notation can be confused with octal numbers.
    /// Strings and octal numbers with the `0o` or `0O` prefix are allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```cjs,expect_diagnostic
    /// const number = 071;
    /// ```
    ///
    /// ```cjs,expect_diagnostic
    /// const number = 08;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const octal = 0o71;
    /// const decimal = 57;
    /// const text = "071";
    /// ```
    pub NoOctal {
        version: "next",
        name: "noOctal",
        language: "js",
        sources: &[RuleSource::Eslint("no-octal").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyJsNumberLiteralLike = JsNumberLiteralExpression | JsLiteralMemberName
}

impl Rule for NoOctal {
    type Query = Ast<AnyJsNumberLiteralLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoOctalOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = match ctx.query() {
            AnyJsNumberLiteralLike::JsNumberLiteralExpression(node) => node.value_token().ok()?,
            AnyJsNumberLiteralLike::JsLiteralMemberName(node) => node.value().ok()?,
        };
        if token.kind() != JsSyntaxKind::JS_NUMBER_LITERAL {
            return None;
        }
        let text = token.text_trimmed();
        if text.starts_with('0')
            && text
                .as_bytes()
                .get(1)
                .is_some_and(|byte| matches!(lookup_byte(*byte), Dispatch::ZER | Dispatch::DIG))
            && !text.ends_with('n')
        {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { "Don't use number literals with leading zeros." },
            )
            .note(markup! {
                "A leading zero can make a number use base 8 instead of base 10, and is forbidden in strict mode."
            })
            .note(markup! {
                "Use the "<Emphasis>"0o"</Emphasis>" prefix for octal numbers, or remove leading zeros from decimal numbers."
            }),
        )
    }
}

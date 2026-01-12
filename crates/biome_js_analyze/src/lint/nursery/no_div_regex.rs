use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_div_regex::NoDivRegexOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow equal signs explicitly at the beginning of regular expressions.
    ///
    /// This rule forbids equal signs (`=`) after the slash (`/`) at the beginning of a regular expression literal,
    /// because the characters `/=` can be confused with a division assignment operator.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function bar() {
    ///   return /=foo/;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function bar() {
    ///   return /[=]foo/;
    /// }
    /// ```
    ///
    pub NoDivRegex {
        version: "next",
        name: "noDivRegex",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-div-regex").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoDivRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDivRegexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let value_token = node.value_token().ok()?;
        let text = value_token.text_trimmed();

        if let Some(first_char) = text.chars().nth(1)
            && first_char == '='
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "A regular expression literal can be confused with '/='."
                },
            )
            .note(markup! {
                "The characters `/=` can be confused with a division assignment operator. Replace with `[=]` to prevent confusion."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let text = node.to_trimmed_string();
        let new_text = format!("/[=]{}", &text[2..]);

        let new_node = node.clone().with_value_token(JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_REGEX_LITERAL,
            new_text.as_str(),
            [],
            [],
        ));

        mutation.replace_node(node.clone(), new_node);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace with `[=]`."
            }
            .to_owned(),
            mutation,
        ))
    }
}

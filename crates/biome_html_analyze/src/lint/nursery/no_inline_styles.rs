use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_inline_styles::NoInlineStylesOptions;
use biome_string_case::StrOnlyExtension;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow the use of inline styles.
    ///
    /// Inline styles via the `style` attribute make code harder to maintain and override,
    /// prevent reusability of styling, and can be a security concern when implementing
    /// a strict Content Security Policy (CSP).
    ///
    /// Instead of inline styles, use CSS classes, CSS modules, or a styling library.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div style="color: red;"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <p style="font-size: 14px;">Hello</p>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="text-red"></div>
    /// ```
    ///
    /// ```html
    /// <p class="body-text">Hello</p>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [html-eslint: no-inline-styles](https://html-eslint.org/docs/rules/no-inline-styles)
    /// - [Content Security Policy: Allowing inline styles](https://content-security-policy.com/examples/allow-inline-style)
    ///
    pub NoInlineStyles {
        version: "next",
        name: "noInlineStyles",
        language: "html",
        recommended: false,
        sources: &[
            RuleSource::HtmlEslint("no-inline-styles").same(),
        ],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInlineStyles {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoInlineStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let value_token = name.value_token().ok()?;
        if value_token.text_trimmed().to_lowercase_cow() == "style" {
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
                    "Avoid using the "<Emphasis>"style"</Emphasis>" attribute. Prefer external CSS classes instead of inline styles."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, reduce reusability, and can prevent effective use of a strict Content Security Policy."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"style"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

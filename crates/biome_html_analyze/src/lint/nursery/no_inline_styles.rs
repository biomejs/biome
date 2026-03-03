use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_inline_styles::NoInlineStylesOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow the use of inline styles on HTML elements.
    ///
    /// Inline styles are specified using the `style` attribute on HTML elements. They are discouraged
    /// because they mix content with presentation, making code harder to read, maintain, and reuse.
    /// Using external CSS files or `<style>` blocks is preferred, as it promotes separation of concerns,
    /// improves cacheability, and enables easier use of a strict Content Security Policy.
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
        sources: &[RuleSource::HtmlEslint("no-inline-styles").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInlineStyles {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoInlineStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if is_style_attribute(node) {
            return Some(());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Avoid using the "<Emphasis>"style"</Emphasis>" attribute. \
                    Prefer external CSS classes instead of inline styles."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, reduce reusability, and can \
                prevent effective use of a strict Content Security Policy."
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

fn is_style_attribute(node: &HtmlAttribute) -> bool {
    node.name().is_ok_and(|name| {
        name.value_token()
            .is_ok_and(|value_token| value_token.text_trimmed() == "style")
    })
}

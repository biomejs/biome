use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::AstNode;
use biome_rule_options::no_inline_styles::NoInlineStylesOptions;

declare_lint_rule! {
    /// Disallow the use of inline styles on elements.
    ///
    /// Inline styles are specified using the `style` attribute directly on an element.
    /// They make code harder to maintain and override, prevent reusability of styling, and
    /// can be a security concern when implementing a strict Content Security Policy (CSP).
    ///
    /// Instead of inline styles, use CSS classes defined in external stylesheets or
    /// `<style>` blocks. This promotes separation of concerns and makes styles easier
    /// to manage, reuse, and override.
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
    /// <button style="background: blue; color: white;">Click</button>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="text-red"></div>
    /// ```
    ///
    /// ```html
    /// <button class="btn btn-primary">Click</button>
    /// ```
    ///
    pub NoInlineStyles {
        version: "next",
        name: "noInlineStyles",
        language: "html",
        sources: &[
            RuleSource::HtmlEslint("no-inline-styles").same(),
        ],
        recommended: false,
    }
}

impl Rule for NoInlineStyles {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoInlineStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?;
        let name_token = name.value_token().ok()?;

        if name_token.text_trimmed() == "style" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let attribute = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                attribute.range(),
                markup! {
                    "Avoid using the "<Emphasis>"style"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, override, and can interfere with Content Security Policy."
            })
            .note(markup! {
                "Use a CSS class instead."
            }),
        )
    }
}

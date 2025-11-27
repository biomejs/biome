use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::BatchMutationExt;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_distracting_elements::NoDistractingElementsOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforces that no distracting elements are used.
    ///
    /// Elements that can be visually distracting can cause accessibility issues with visually impaired users.
    /// Such elements are most likely deprecated, and should be avoided.
    /// By default, the following elements are visually distracting: `<marquee>` and `<blink>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <marquee />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <blink />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div />
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.2.2](https://www.w3.org/WAI/WCAG21/Understanding/pause-stop-hide)
    ///
    pub NoDistractingElements {
        version: "next",
        name: "noDistractingElements",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-distracting-elements").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoDistractingElements {
    type Query = Ast<AnyHtmlElement>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = NoDistractingElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let element_name = element.name()?;
        if is_marquee_or_blink_element(element_name.text()) {
            return Some(element_name);
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            element.range(),
            markup! {"Don't use the '"{name.text()}"' element."}.to_owned(),
        )
        .note(markup! {
            "Visually distracting elements can cause accessibility issues and should be avoided."
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, name: &Self::State) -> Option<HtmlRuleAction> {
        let element = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(element.clone());

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the '"{name.text()}"' element." }.to_owned(),
            mutation,
        ))
    }
}

fn is_marquee_or_blink_element(element_name: &str) -> bool {
    element_name.eq_ignore_ascii_case("marquee") || element_name.eq_ignore_ascii_case("blink")
}

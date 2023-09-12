use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::*;
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
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
    /// ```jsx,expect_diagnostic
    /// <marquee />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <blink />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div />
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.2.2](https://www.w3.org/WAI/WCAG21/Understanding/pause-stop-hide)
    ///
    pub(crate) NoDistractingElements {
        version: "1.0.0",
        name: "noDistractingElements",
        recommended: true,
    }
}

impl Rule for NoDistractingElements {
    type Query = Ast<AnyJsxElement>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name_value_token()?;
        match name.text_trimmed() {
            "marquee" | "blink" => Some(name),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            element.range(),
            markup! {"Don't use the '"{name.text_trimmed()}"' element."}.to_owned(),
        )
        .note(markup! {
            "Visually distracting elements can cause accessibility issues and should be avoided."
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, name: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(element.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the '"{name.text_trimmed()}"' element." }.to_owned(),
            mutation,
        })
    }
}

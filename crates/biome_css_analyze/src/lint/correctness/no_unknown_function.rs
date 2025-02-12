use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::CssFunction;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};

use crate::utils::{is_custom_function, is_function_keyword};

declare_lint_rule! {
    /// Disallow unknown CSS value functions.
    ///
    /// This rule ignores double-dashed custom functions, e.g. `--custom-function()`.
    ///
    /// Data sources of known CSS value functions are:
    /// - MDN reference on [CSS value functions](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions)
    /// - MDN reference on [CSS reference](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference)
    /// - MDN [browser compatibility data for CSS value functions](https://github.com/mdn/browser-compat-data/tree/main/css/types)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { transform: unknown(1); }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { transform: scale(1); }
    /// ```
    ///
    pub NoUnknownFunction {
        version: "1.8.0",
        name: "noUnknownFunction",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("function-no-unknown")],
    }
}

pub struct NoUnknownFunctionState {
    function_name: String,
    span: TextRange,
}

impl Rule for NoUnknownFunction {
    type Query = Ast<CssFunction>;
    type State = NoUnknownFunctionState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let function_name = node.name().ok()?.to_trimmed_string();

        // We don't have a semantic model yet, so we can't determine if functions are defined elsewhere.
        // Therefore, we ignore these custom functions to prevent false detections.
        if is_custom_function(&function_name) {
            return None;
        }

        if is_function_keyword(&function_name) {
            return None;
        }

        Some(NoUnknownFunctionState {
            function_name,
            span: node.name().ok()?.range(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.span,
                markup! {
                    "Unexpected unknown function: "<Emphasis>{state.function_name}</Emphasis>
                },
            )
            .note(markup! {
                "Use a known function instead."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions">"MDN web docs"</Hyperlink>" for more details."
            }),
        )
    }
}

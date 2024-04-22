use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssFunction;
use biome_rowan::{AstNode, TextRange};

use crate::utils::is_function_keyword;

declare_rule! {
    /// Disallow unknown functions.
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
        version: "next",
        name: "noUnknownFunction",
        recommended: true,
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
        let function_name = node.name().ok()?.text();

        if !is_function_keyword(&function_name) {
            return Some(NoUnknownFunctionState {
                function_name,
                span: node.name().ok()?.range(),
            });
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.span,
            markup! {
                "Unexpected unknown function: "<Emphasis>{state.function_name}</Emphasis>
            },
        ))
    }
}

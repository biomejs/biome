use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, global_identifier};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require a description parameter for the `Symbol()`.
    ///
    /// `Symbol` can have an optional description parameter which can be useful for
    /// debugging and making the purpose of the symbol clearer.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Symbol();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Symbol('description');
    /// ```
    ///
    pub UseSymbolDescription {
        version: "next",
        name: "useSymbolDescription",
        language: "js",
        sources: &[
            RuleSource::Eslint("symbol-description"),
        ],
        recommended: false,
    }
}

impl Rule for UseSymbolDescription {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?;
        let model = ctx.model();
        let (reference, name) = global_identifier(&callee)?;

        if name.text() != "Symbol" || model.binding(&reference).is_some() {
            return None;
        }

        let call_args = call_expression.arguments().ok()?;
        if call_args.args().len() > 0 {
            return None;
        }

        Some(call_args.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    <Emphasis>"Symbol()"</Emphasis>" is missing a description parameter."
                },
            )
            .note(markup! {
                "Add explicit description which can be useful in debugging and making the purpose of the symbol clearer."
            }),
        )
    }
}

use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsNewExpression;
use biome_rowan::AstNode;
use biome_rule_options::no_new_require::NoNewRequireOptions;

declare_lint_rule! {
    /// Disallow calling `require` with the `new` operator.
    ///
    /// Code like `new require("app-header")` looks like it loads a module and creates a new
    /// object from what the module exports, all in one step. It doesn't. `new` applies to
    /// `require` itself, not to the value that `require` returns. The result is whatever the
    /// module exports, such as a class, instead of a new object created from that class.
    ///
    /// Store the result of `require` in a variable first, then use `new` on that variable.
    ///
    /// The rule doesn't report `require` when it's a variable, parameter, or function declared
    /// in your own code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const appHeader = new require("app-header");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const AppHeader = require("app-header");
    /// const appHeader = new AppHeader();
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow `require` entirely, see [`noCommonJs`](https://biomejs.dev/linter/rules/no-common-js/).
    ///
    pub NoNewRequire {
        version: "next",
        name: "noNewRequire",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-new-require").same(),
            RuleSource::EslintN("no-new-require").same(),
        ],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoNewRequire {
    type Query = Semantic<JsNewExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNewRequireOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = ctx.query().callee().ok()?;
        let reference = callee
            .omit_parentheses()
            .as_js_identifier_expression()?
            .name()
            .ok()?;
        if reference.value_token().ok()?.text_trimmed() != "require" {
            return None;
        }
        // A local binding named `require` isn't the module loader.
        ctx.model().binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected use of "<Emphasis>"new"</Emphasis>" with "<Emphasis>"require"</Emphasis>"."
                },
            )
            .note(markup! {
                "Here "<Emphasis>"new"</Emphasis>" applies to "<Emphasis>"require"</Emphasis>" itself, so the result is whatever the module exports, not a new object created from it."
            })
            .note(markup! {
                "Store the result of "<Emphasis>"require"</Emphasis>" in a variable first, then use "<Emphasis>"new"</Emphasis>" on that variable."
            }),
        )
    }
}

use crate::globals::is_js_global;

use crate::services::semantic::Semantic;
use biome_analyze::RuleSource;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsIdentifierAssignment, TextRange};
use biome_rule_options::no_global_assign::NoGlobalAssignOptions;

declare_lint_rule! {
    /// Disallow assignments to native objects and read-only global variables.
    ///
    /// JavaScript's environments contain numerous built-in global variables, such as `window` in browsers and `process` in Node.js.
    /// Assigning values to these global variables can be problematic as it can override essential functionality.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object = null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window = {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// undefined = true;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// a = 0;
    /// ```
    ///
    /// ```js
    /// let window;
    /// window = {};
    /// ```
    pub NoGlobalAssign {
        version: "1.5.0",
        name: "noGlobalAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-global-assign").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoGlobalAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoGlobalAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assignment = ctx.query();
        if !ctx.model().is_unresolved_reference(assignment) {
            return None;
        }
        let token = assignment.name_token().ok()?;
        is_js_global(token.text_trimmed()).then(|| token.text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "A global variable should not be reassigned."
                },
            )
            .note(markup! {
                "Assigning to a global variable can override essential functionality."
            }),
        )
    }
}

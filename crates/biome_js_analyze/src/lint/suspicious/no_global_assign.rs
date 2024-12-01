use crate::globals::is_js_global;

use crate::services::semantic::SemanticServices;
use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsSyntaxKind, TextRange};

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
        sources: &[RuleSource::Eslint("no-global-assign")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoGlobalAssign {
    type Query = SemanticServices;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let global_refs = ctx.query().all_unresolved_references();
        let mut result = Vec::new();
        for global_ref in global_refs {
            let is_write = global_ref.syntax().kind() == JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT;
            if is_write {
                let identifier = global_ref.syntax().text_trimmed();
                let text = identifier.to_string();
                let is_global_var = is_js_global(text.as_str());
                if is_global_var {
                    result.push(global_ref.range());
                }
            }
        }
        result.into_boxed_slice()
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

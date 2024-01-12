use crate::semantic_services::SemanticServices;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::TextRange;

declare_rule! {
    /// Disallow the use of `arguments`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///    console.log(arguments);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cjs
    /// function f() {
    ///     let arguments = 1;
    ///     console.log(arguments);
    /// }
    /// ```
    pub(crate) NoArguments {
        version: "1.0.0",
        name: "noArguments",
        source: RuleSource::Eslint("prefer-rest-params"),
        recommended: true,
    }
}

impl Rule for NoArguments {
    type Query = SemanticServices;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.query();

        let mut found_arguments = vec![];

        for unresolved_reference in model.all_unresolved_references() {
            let name = unresolved_reference.syntax().text_trimmed();
            if name == "arguments" {
                let range = unresolved_reference.range();
                found_arguments.push(*range);
            }
        }

        found_arguments
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(),
            range,
            markup! {
                "Use the "<Emphasis>"rest parameters"</Emphasis>" instead of "<Emphasis>"arguments"</Emphasis>"."
            },
        ).note(markup! {<Emphasis>"arguments"</Emphasis>" does not have "<Emphasis>"Array.prototype"</Emphasis>" methods and can be inconvenient to use."}))
    }
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsSwitchStatement;
use biome_rowan::AstNode;
use biome_rule_options::no_nested_switch::NoNestedSwitchOptions;

declare_lint_rule! {
    /// Disallow `switch` statements inside other `switch` statements.
    ///
    /// A `switch` statement inside another `switch` statement is hard to read,
    /// because it is easy to mix up which `case` belongs to which `switch`.
    /// Move the inner `switch` into a separate function, written outside the
    /// outer `switch`, instead.
    ///
    /// A `switch` statement inside a function is still reported when that
    /// function is written inside another `switch` statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo(n, m) {
    ///     switch (n) {
    ///         case 0:
    ///             switch (m) {
    ///                 case 1:
    ///                     break;
    ///             }
    ///             break;
    ///         default:
    ///             break;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (n) {
    ///     case 0: {
    ///         const handle = (m) => {
    ///             switch (m) {
    ///                 case 1:
    ///                     break;
    ///             }
    ///         };
    ///         handle(n);
    ///         break;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo(n, m) {
    ///     switch (n) {
    ///         case 0:
    ///             bar(m);
    ///             break;
    ///         default:
    ///             break;
    ///     }
    /// }
    ///
    /// function bar(m) {
    ///     switch (m) {
    ///         case 1:
    ///             break;
    ///     }
    /// }
    /// ```
    ///
    pub NoNestedSwitch {
        version: "next",
        name: "noNestedSwitch",
        language: "js",
        sources: &[RuleSource::EslintSonarJs("no-nested-switch").same()],
        severity: Severity::Information,
        recommended: false,
    }
}

impl Rule for NoNestedSwitch {
    type Query = Ast<JsSwitchStatement>;
    /// The closest `switch` statement that contains the queried one.
    type State = JsSwitchStatement;
    type Signals = Option<Self::State>;
    type Options = NoNestedSwitchOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query()
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsSwitchStatement::cast)
    }

    fn diagnostic(ctx: &RuleContext<Self>, outer: &Self::State) -> Option<RuleDiagnostic> {
        let switch_token = ctx.query().switch_token().ok()?;
        let outer_switch_token = outer.switch_token().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                switch_token.text_trimmed_range(),
                markup! {
                    "This "<Emphasis>"switch"</Emphasis>" statement is inside another "<Emphasis>"switch"</Emphasis>" statement."
                },
            )
            .detail(
                outer_switch_token.text_trimmed_range(),
                markup! {
                    "The outer "<Emphasis>"switch"</Emphasis>" statement starts here."
                },
            )
            .note(markup! {
                "Nested "<Emphasis>"switch"</Emphasis>" statements are hard to read, because it is easy to mix up which "<Emphasis>"case"</Emphasis>" belongs to which "<Emphasis>"switch"</Emphasis>"."
            })
            .note(markup! {
                "Move the inner "<Emphasis>"switch"</Emphasis>" statement into a separate function written outside the outer "<Emphasis>"switch"</Emphasis>" statement."
            }),
        )
    }
}

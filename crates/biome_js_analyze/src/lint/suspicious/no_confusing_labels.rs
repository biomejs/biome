use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsStatement, JsFileSource, JsLabeledStatement};

declare_lint_rule! {
    /// Disallow labeled statements that are not loops.
    ///
    /// Labeled statements in JavaScript are used in conjunction with `break` and `continue` to control flow around multiple loops.
    /// Their use for other statements is suspicious and unfamiliar.
    ///
    /// The rule ignores reactive Svelte statements in Svelte components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// label: f();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// label: {
    ///     f();
    ///     break label;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// label: if (a) {
    ///     f()
    ///     break label;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// label: switch (a) {
    ///     case 0:
    ///         break label;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// outer: while (a) {
    ///     while(b) {
    ///         break outer;
    ///     }
    /// }
    /// ```
    ///
    /// ```svelte
    /// <script>
    /// $: { /* reactive block */ }
    /// </script>
    /// ```
    pub NoConfusingLabels {
        version: "1.0.0",
        name: "noConfusingLabels",
        language: "js",
        sources: &[RuleSource::Eslint("no-labels")],
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoConfusingLabels {
    type Query = Ast<JsLabeledStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let labeled_stmt = ctx.query();
        let label = labeled_stmt.label_token().ok()?;
        let label = label.text_trimmed();
        if label == "$"
            && ctx
                .source_type::<JsFileSource>()
                .as_embedding_kind()
                .is_svelte()
        {
            return None;
        }
        match labeled_stmt.body().ok()? {
            AnyJsStatement::JsDoWhileStatement(_)
            | AnyJsStatement::JsForInStatement(_)
            | AnyJsStatement::JsForOfStatement(_)
            | AnyJsStatement::JsForStatement(_)
            | AnyJsStatement::JsWhileStatement(_) => None,
            _ => Some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let labeled_stmt = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                labeled_stmt.label_token().ok()?.text_trimmed_range(),
                markup! {
                    "Unexpected "<Emphasis>"label"</Emphasis>"."
                },
            )
            .note("Only loops should be labeled.\nThe use of labels for other statements is suspicious and unfamiliar."),
        )
    }
}

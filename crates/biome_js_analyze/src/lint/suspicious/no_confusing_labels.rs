use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsStatement, JsFileSource, JsLabeledStatement};
use biome_rule_options::no_confusing_labels::NoConfusingLabelsOptions;

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
    /// ## Options
    ///
    /// Use the options to allow specific labels in your code.
    /// Labels can be used to mark code that should be removed under certain conditions,
    /// such as in production builds.
    /// Some bundlers, such as [esbuild](https://esbuild.github.io/api/#drop-labels) and Vite,
    /// can be configured to remove labeled statements.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowedLabels": ["DEV"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// DEV: assertSomeCondition();
    /// ```
    ///
    pub NoConfusingLabels {
        version: "1.0.0",
        name: "noConfusingLabels",
        language: "js",
        sources: &[RuleSource::Eslint("no-labels").inspired()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoConfusingLabels {
    type Query = Ast<JsLabeledStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoConfusingLabelsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let labeled_stmt = ctx.query();
        let label = labeled_stmt.label_token().ok()?;
        let label = label.text_trimmed();

        // Allow $ label which marks reactive statements in Svelte
        if label == "$"
            && ctx
                .source_type::<JsFileSource>()
                .as_embedding_kind()
                .is_svelte()
        {
            return None;
        }

        // Allow custom allowed labels
        if ctx
            .options()
            .allowed_labels
            .iter()
            .any(|s| s.as_ref() == label)
        {
            return None;
        }

        // Allow labels in loops
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
        let allowed_labels = &ctx.options().allowed_labels;

        let message = if allowed_labels.is_empty() {
            "Only loops should be labeled.\nThe use of labels for other statements is suspicious and unfamiliar."
        } else {
            "Some labels are explicitly allowed, but this one is not.\nOtherwise, only loops should be labeled."
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                labeled_stmt.label_token().ok()?.text_trimmed_range(),
                markup! {
                    "Unexpected "<Emphasis>"label"</Emphasis>"."
                },
            )
            .note(message),
        )
    }
}

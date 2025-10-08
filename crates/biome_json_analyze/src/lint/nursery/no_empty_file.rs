use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_json_syntax::JsonRoot;
use biome_rowan::AstNode;
use biome_rule_options::no_empty_file::NoEmptyFileOptions;

declare_lint_rule! {
    /// Disallow empty files.
    ///
    /// A file containing only the following is considered empty:
    ///   - Whitespace (spaces, tabs or newlines)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    ///
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// { }
    /// ```
    ///
    pub NoEmptyFile {
        version: "next",
        name: "noEmptyFile",
        language: "json",
        recommended: false,
    }
}

impl Rule for NoEmptyFile {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptyFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.value().ok().is_none() {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "An empty file is not allowed."
                },
            )
            .note(markup! {
                "A higher amount of files can increase the cognitive load, deleting empty files can help reducing this load."
            }),
        )
    }
}

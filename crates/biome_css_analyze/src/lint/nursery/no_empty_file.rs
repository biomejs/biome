use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssRoot;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_empty_file::NoEmptyFileOptions;

declare_lint_rule! {
    /// Disallow empty files.
    ///
    /// A file containing only the following is considered empty:
    ///   - Whitespace (spaces, tabs or newlines)
    ///   - Comments
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    ///
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// /* Only comments */
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `comments`
    ///
    /// Mark comments as meaningless
    ///
    /// Default `true`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "comments": true
    ///   }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// /* Only comments */
    /// ```
    ///
    pub NoEmptyFile {
        version: "next",
        name: "noEmptyFile",
        language: "css",
        sources: &[RuleSource::Stylelint("no-empty-source").same()],
        recommended: false,
    }
}

impl Rule for NoEmptyFile {
    type Query = Ast<CssRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptyFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.rules().iter().len() > 0 {
            return None;
        }

        if !ctx.options().comments && node.syntax().has_comments_direct() {
            return None;
        }

        Some(())
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

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssRoot;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_empty_source::NoEmptySourceOptions;

declare_lint_rule! {
    /// Disallow empty sources.
    ///
    /// A source containing only the following is considered empty:
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
    /// Whether the comments should be marked as meaningless.
    ///
    /// Default `true`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "comments": false
    ///   }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    ///
    /// ```
    ///
    pub NoEmptySource {
        version: "next",
        name: "noEmptySource",
        language: "css",
        sources: &[RuleSource::Stylelint("no-empty-source").same()],
        recommended: false,
    }
}

impl Rule for NoEmptySource {
    type Query = Ast<CssRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptySourceOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.rules().len() > 0 {
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
                    "An empty source is not allowed."
                },
            )
            .note(markup! {
                "Empty sources can clutter the codebase and increase cognitive load; deleting empty sources can help reduce it."
            }),
        )
    }
}

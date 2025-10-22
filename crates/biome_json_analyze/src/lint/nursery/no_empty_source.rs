use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_json_syntax::JsonRoot;
use biome_rowan::AstNode;
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
    /// ```json,expect_diagnostic
    ///
    /// ```
    ///
    /// ```jsonc,expect_diagnostic
    /// // Only comments
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// { }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowComments`
    ///
    /// Whether the comments should be marked as meaningful.
    /// When this option has been set to `true`, a file with only comments is considered valid.
    ///
    /// Default `false`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowComments": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```jsonc,expect_diagnostic,use_options
    ///
    /// ```
    ///
    /// #### Valid
    ///
    /// ```jsonc,ignore,use_options
    /// // Only comments
    /// ```
    ///
    pub NoEmptySource {
        version: "2.2.7",
        name: "noEmptySource",
        language: "json",
        recommended: false,
    }
}

impl Rule for NoEmptySource {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptySourceOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.value().ok().is_some() {
            return None;
        }

        if ctx.options().allow_comments && node.syntax().has_comments_direct() {
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

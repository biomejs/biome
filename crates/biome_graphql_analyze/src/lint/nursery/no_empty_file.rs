use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_graphql_syntax::GraphqlRoot;
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
    /// ```graphql,expect_diagnostic
    ///
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// # Only comments
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query Member {}
    /// ```
    ///
    /// ```graphql
    /// fragment StrippedMember on Member {}
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
    /// ```graphql,expect_diagnostic
    /// # Only comments
    /// ```
    ///
    pub NoEmptyFile {
        version: "next",
        name: "noEmptyFile",
        language: "graphql",
        recommended: false,
    }
}

impl Rule for NoEmptyFile {
    type Query = Ast<GraphqlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptyFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.definitions().len() > 0 {
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
                "Empty files can clutter the codebase & increase cognitive load; deleting empty files can help reduce it."
            }),
        )
    }
}

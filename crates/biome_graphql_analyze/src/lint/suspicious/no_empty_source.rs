use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_graphql_syntax::GraphqlRoot;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_empty_source::NoEmptySourceOptions;

declare_lint_rule! {
    /// Disallow empty sources.
    ///
    /// A file is empty when it contains no GraphQL definitions. By default, whitespace and
    /// comments do not count as content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    ///
    /// ```
    ///
    /// ```graphql,ignore
    /// # Invalid comment
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
    /// ### `allowComments`
    ///
    /// Treats comments as meaningful content when set to `true`, so a comments-only file is valid.
    /// An entirely empty file remains invalid. Defaults to `false`.
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
    /// ```graphql,expect_diagnostic,use_options
    ///
    /// ```
    ///
    /// #### Valid
    ///
    /// ```graphql,ignore
    /// # Valid comment
    /// ```
    ///
    pub NoEmptySource {
        version: "2.2.7",
        name: "noEmptySource",
        language: "graphql",
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoEmptySource {
    type Query = Ast<GraphqlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptySourceOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.definitions().len() > 0 {
            return None;
        }

        if ctx.options().allow_comments()
            && (node.syntax().has_comments_direct()
                || node.eof_token().ok()?.has_leading_comments())
        {
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

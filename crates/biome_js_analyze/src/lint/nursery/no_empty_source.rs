use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsStatement, JsModule};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_empty_source::NoEmptySourceOptions;

declare_lint_rule! {
    /// Disallow empty sources.
    ///
    /// A source containing only the following is considered empty:
    ///   - Whitespace (spaces, tabs or newlines)
    ///   - Comments
    ///   - Directives
    ///   - Empty statements
    ///   - Empty block statements
    ///   - Hashbang
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    ///
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Only comments
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /* Only comments */
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// 'use strict';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// #!/usr/bin/env node
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = 0;
    /// ```
    ///
    /// ```js
    /// 'use strict';
    /// const x = 0;
    /// ```
    ///
    /// ```js
    /// ;;
    /// const x = 0;
    /// ```
    ///
    /// ```js
    /// {
    ///   const x = 0;
    /// }
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
    /// ```js,expect_diagnostic,use_options
    ///
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// /* Only comments */
    /// ```
    ///
    pub NoEmptySource {
        version: "2.2.7",
        name: "noEmptySource",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-empty-file").same()],
        recommended: false,
    }
}

impl Rule for NoEmptySource {
    type Query = Ast<JsModule>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptySourceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.items().iter().any(|i| {
            let Some(body) = i.as_any_js_statement() else {
                return true;
            };

            match body {
                AnyJsStatement::JsEmptyStatement(_) => false,
                AnyJsStatement::JsBlockStatement(block) => block.statements().len() > 0,
                _ => true,
            }
        }) {
            return None;
        }

        if ctx.options().allow_comments
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

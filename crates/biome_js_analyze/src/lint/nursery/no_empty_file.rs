use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsStatement, JsModule};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_empty_file::NoEmptyFileOptions;

declare_lint_rule! {
/// Disallow empty files.
    ///
    /// A file containing only the following is considered empty:
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
    /// ```js,expect_diagnostic
    /// /* Only comments */
    /// ```
    ///
    pub NoEmptyFile {
        version: "next",
        name: "noEmptyFile",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-empty-file").same()],
        recommended: false,
    }
}

impl Rule for NoEmptyFile {
    type Query = Ast<JsModule>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptyFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.items().iter().any(|i| {
            let Some(body) = i.as_any_js_statement() else {
                return true;
            };

            return match body {
                AnyJsStatement::JsEmptyStatement(_) => false,
                AnyJsStatement::JsBlockStatement(block) => block.statements().len() > 0,
                _ => true,
            };
        }) {
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

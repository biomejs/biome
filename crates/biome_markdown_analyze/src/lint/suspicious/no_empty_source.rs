use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_markdown_syntax::MdRoot;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_empty_source::NoEmptySourceOptions;

declare_lint_rule! {
    /// Disallow empty sources.
    ///
    /// A Markdown file is empty when it contains nothing but blank lines and HTML comments
    /// (such as `<!-- TODO -->`). A file that only has front matter is not empty.
    ///
    /// By default, whitespace and comments do not count as content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```md,expect_diagnostic
    ///
    /// ```
    ///
    /// ```md,expect_diagnostic
    /// <!-- Only comments -->
    /// ```
    ///
    /// ### Valid
    ///
    /// ```md
    /// # Title
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowComments`
    ///
    /// Default: `false`
    ///
    /// Treats comments as meaningful content when set to `true`, so a comments-only file is valid.
    /// An entirely empty file remains invalid.
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
    /// ```md,expect_diagnostic,use_options
    ///
    /// ```
    ///
    /// #### Valid
    ///
    /// ```md,use_options
    /// <!-- Only comments -->
    /// ```
    ///
    pub NoEmptySource {
        version: "2.6.0",
        name: "noEmptySource",
        language: "md",
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoEmptySource {
    type Query = Ast<MdRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptySourceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        if root.frontmatter().is_some() {
            return None;
        }

        let allow_comments = ctx.options().allow_comments();
        let mut has_comments = false;
        for block in root.value().iter() {
            if block.is_newline() || block.is_continuation_indent() {
                continue;
            }
            if !block.is_html_comment() {
                return None;
            }
            has_comments = true;
        }

        if allow_comments && has_comments {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
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

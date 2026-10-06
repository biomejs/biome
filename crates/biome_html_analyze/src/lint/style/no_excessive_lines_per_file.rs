use biome_analyze::{
    Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule, utils::count_lines_in_file,
};
use biome_console::markup;
use biome_html_syntax::{HtmlRoot, HtmlSyntaxKind};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_lines_per_file::NoExcessiveLinesPerFileOptions;

declare_lint_rule! {
    /// Restrict the number of lines in a file.
    ///
    /// Large HTML and component files are harder to navigate and maintain. A line limit can
    /// encourage splitting unrelated markup into focused components or templates.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// This example reports a diagnostic when `maxLines` is `2`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 2
    ///     }
    /// }
    /// ```
    /// ```html,expect_diagnostic,use_options
    /// <div></div>
    /// <span></span>
    /// <p></p>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div></div>
    /// <span></span>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxLines`
    ///
    /// Sets the maximum number of lines allowed in a file. Defaults to `300`.
    ///
    /// This example lowers the limit to `4` and reports a five-line file:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxLines": 4
    ///     }
    /// }
    /// ```
    /// ```html,expect_diagnostic,use_options
    /// <div>Line 1</div>
    /// <div>Line 2</div>
    /// <div>Line 3</div>
    /// <div>Line 4</div>
    /// <div>Line 5</div>
    /// ```
    ///
    /// ### `skipBlankLines`
    ///
    /// Excludes blank lines from the line count when set to `true`. Defaults to `false`.
    ///
    /// Here, the blank lines do not count toward the limit:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxLines": 2,
    ///         "skipBlankLines": true
    ///     }
    /// }
    /// ```
    /// ```html,use_options
    /// <div></div>
    ///
    ///
    /// <span></span>
    /// ```
    ///
    /// ## Suppressions
    ///
    /// If you need to exceed the line limit in a specific file, you can suppress this rule
    /// at the top of the file:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxLines": 2
    ///     }
    /// }
    /// ```
    /// ```html,use_options
    /// <!-- biome-ignore-all lint/style/noExcessiveLinesPerFile: generated file -->
    /// <div></div>
    /// <span></span>
    /// <p></p>
    /// ```
    ///
    pub NoExcessiveLinesPerFile {
        version: "2.5.0",
        name: "noExcessiveLinesPerFile",
        language: "html",
        recommended: false,
    }
}

impl Rule for NoExcessiveLinesPerFile {
    type Query = Ast<HtmlRoot>;
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveLinesPerFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        let file_lines_count = count_lines_in_file(
            node.syntax(),
            |token| token.kind() == HtmlSyntaxKind::EOF,
            options.skip_blank_lines(),
        );

        if file_lines_count > options.max_lines().get().into() {
            return Some(file_lines_count);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let options = ctx.options();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This file has too many lines ("{state}"). Maximum allowed is "{options.max_lines()}"."
                },
            )
            .note(markup! {
                "Consider splitting this file into smaller files."
            }),
        )
    }
}

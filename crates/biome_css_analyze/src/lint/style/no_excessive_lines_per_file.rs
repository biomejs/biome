use biome_analyze::{
    Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule, utils::count_lines_in_file,
};
use biome_console::markup;
use biome_css_syntax::{CssRoot, CssSyntaxKind};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_lines_per_file::NoExcessiveLinesPerFileOptions;

declare_lint_rule! {
    /// Restrict the number of lines in a file.
    ///
    /// Large files tend to do many things and can make it hard to follow what's going on.
    /// This rule can help enforce a limit on the number of lines in a file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following example will show a diagnostic when `maxLines` is set to 2:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 2
    ///     }
    /// }
    /// ```
    /// ```css,expect_diagnostic,use_options
    /// .a { color: red; }
    /// .b { color: blue; }
    /// .c { color: green; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .a { color: red; }
    /// .b { color: blue; }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxLines`
    ///
    /// This option sets the maximum number of lines allowed in a file.
    /// If the file exceeds this limit, a diagnostic will be reported.
    ///
    /// Default: `300`
    ///
    /// #### Examples
    ///
    /// The default value for `maxLines` is `300`. The following example shows how to set the
    /// `maxLines` option to a smaller value. It reports a diagnostic because the file has more
    /// than 4 lines:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxLines": 4
    ///     }
    /// }
    /// ```
    /// ```css,expect_diagnostic,use_options
    /// .a { color: red; }
    /// .b { color: blue; }
    /// .c { color: green; }
    /// .d { color: yellow; }
    /// .e { color: purple; }
    /// ```
    ///
    /// ### `skipBlankLines`
    ///
    /// When this option is set to `true`, blank lines are not counted towards the maximum line limit.
    ///
    /// Default: `false`
    ///
    /// #### Examples
    ///
    /// The following example shows how `skipBlankLines` can prevent a diagnostic by excluding blank
    /// lines from the total count:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxLines": 2,
    ///         "skipBlankLines": true
    ///     }
    /// }
    /// ```
    /// ```css,use_options
    /// .a { color: red; }
    ///
    ///
    /// .b { color: blue; }
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
    /// ```css,use_options
    /// /* biome-ignore-all lint/style/noExcessiveLinesPerFile: generated file */
    /// .a { color: red; }
    /// .b { color: blue; }
    /// .c { color: green; }
    /// ```
    ///
    pub NoExcessiveLinesPerFile {
        version: "2.3.12",
        name: "noExcessiveLinesPerFile",
        language: "css",
        recommended: false,
    }
}

impl Rule for NoExcessiveLinesPerFile {
    type Query = Ast<CssRoot>;
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveLinesPerFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        let file_lines_count = count_lines_in_file(
            node.syntax(),
            |token| token.kind() == CssSyntaxKind::EOF,
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

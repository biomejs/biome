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
    /// ### `skipBlankLines`
    ///
    /// When this option is set to `true`, blank lines are not counted towards the maximum line limit.
    ///
    /// Default: `false`
    ///
    pub NoExcessiveLinesPerFile {
        version: "next",
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
                    "This file has too many lines ("{state}"). Maximum allowed is "{options.max_lines().to_string()}"."
                },
            )
            .note(markup! {
                "Consider splitting this file into smaller files."
            }),
        )
    }
}

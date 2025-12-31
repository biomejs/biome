use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsSyntaxKind};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_lines_per_file::NoExcessiveLinesPerFileOptions;

declare_lint_rule! {
    /// Restrict the number of lines in a file.
    ///
    /// This rule checks the number of lines in a file and reports a diagnostic if it exceeds a specified limit.
    /// Some people consider large files a code smell. Large files tend to do many things and can make it hard to follow what's going on.
    /// Many coding style guides dictate a limit of the number of lines that a file can comprise of. This rule can help enforce that style.
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
    /// ```js,expect_diagnostic,use_options
    /// const a = 1;
    /// const b = 2;
    /// const c = 3;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 1;
    /// const b = 2;
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ```json
    /// {
    ///     "options": {
    ///        "maxLines": 300,
    ///        "skipBlankLines": false
    ///     }
    /// }
    /// ```
    ///
    /// ### maxLines
    ///
    /// This option sets the maximum number of lines allowed in a file.
    /// If the file exceeds this limit, a diagnostic will be reported.
    ///
    /// Default: `300`
    ///
    /// When `maxLines: 2`, the following file will be considered invalid:
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 2
    ///     }
    /// }
    /// ```
    /// ```js,expect_diagnostic,use_options
    /// const a = 1;
    /// const b = 2;
    /// const c = 3;
    /// ```
    ///
    /// ### skipBlankLines
    ///
    /// When this option is set to `true`, blank lines are not counted towards the maximum line limit.
    /// This means that only lines with actual code or comments will be counted.
    ///
    /// Default: `false`
    ///
    /// When `maxLines: 3` and `skipBlankLines: true`, the following file will be considered valid
    /// even though it has 5 total lines, because only 3 lines contain code:
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 3,
    ///        "skipBlankLines": true
    ///     }
    /// }
    /// ```
    /// ```js,use_options
    /// const a = 1;
    ///
    /// const b = 2;
    ///
    /// const c = 3;
    /// ```
    ///
    pub NoExcessiveLinesPerFile {
        version: "next",
        name: "noExcessiveLinesPerFile",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("max-lines").inspired()],
        issue_number: Some("5557"),
    }
}

impl Rule for NoExcessiveLinesPerFile {
    type Query = Ast<AnyJsRoot>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveLinesPerFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        let file_lines_count = node
            .syntax()
            .descendants()
            .flat_map(|descendant| descendant.tokens().collect::<Vec<_>>())
            .filter(|token| token.kind() != JsSyntaxKind::EOF)
            .fold(0, |acc, token| {
                if options.skip_blank_lines() {
                    return acc + token.has_leading_newline() as usize;
                };

                acc + token
                    .trim_trailing_trivia()
                    .leading_trivia()
                    .pieces()
                    .filter(|piece| piece.is_newline())
                    .count()
            })
            + 1; // Add 1 for the first line

        if file_lines_count > options.max_lines().get().into() {
            return Some(State { file_lines_count });
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
                    "This file has too many lines ("{state.file_lines_count}"). Maximum allowed is "{options.max_lines().to_string()}"."
                },
            )
            .note(markup! {
                "Consider splitting this file into smaller files."
            }),
        )
    }
}

pub struct State {
    file_lines_count: usize,
}

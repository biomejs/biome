use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsFunction, JsCallExpression, JsParenthesizedExpression,
};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_lines_per_function::NoExcessiveLinesPerFunctionOptions;

declare_lint_rule! {
    /// Restrict the number of lines of code in a function.
    ///
    /// This rule checks the number of lines in a function body and reports a diagnostic if it exceeds a specified limit. Remember that this rule only counts the lines of code in the function body, not the entire function declaration.
    /// Some people consider large functions a code smell. Large functions tend to do a lot of things and can make it hard following what’s going on. Many coding style guides dictate a limit of the number of lines that a function can comprise of. This rule can help enforce that style.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following example will show diagnostic when you set the maxLines limit to 3, however the default value is 50.
    ///
    /// ```js
    /// function foo () {
    ///   const x = 0;
    ///   const y = 1;
    ///   const z = 2;
    ///   return x + y + z;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    ///  function foo () {
    ///     const x = 0;
    ///     const y = 1;
    /// };
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ```json
    /// {
    ///     "options": {
    ///        "maxLines": 50,
    ///        "skipBlankLines": false,
    ///        "skipIifes": false
    ///     }
    /// }
    /// ```
    ///
    /// ### maxLines
    ///
    /// This option sets the maximum number of lines allowed in a function body.
    /// If the function body exceeds this limit, a diagnostic will be reported.
    ///
    /// Default: `50`
    ///
    /// When `maxLines: 2`, the following function will be considered invalid:
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 2
    ///     }
    /// }
    /// ```
    /// ```js,expect_diagnostic,use_options
    /// function example() {
    ///  const a = 1; // 1
    ///  const b = 2; // 2
    ///  const c = 3; // 3
    /// };
    /// ```
    ///
    /// ### skipBlankLines
    ///
    /// When this options is set to `true`, blank lines in the function body are not counted towards the maximum line limit.
    /// This means that only lines with actual code or comments will be counted.
    ///
    /// Default: `false`
    ///
    /// When `maxLines: 2` and `skipBlankLines: true`, the following function will be considered valid:
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 2,
    ///        "skipBlankLines": true
    ///     }
    /// }
    /// ```
    /// ```js,use_options
    /// function example() {
    ///  const a = 1; // 1
    ///  // not counted
    ///  const b = 2; // 2
    ///  // not counted
    /// };
    /// ```
    ///
    /// ### skipIifes
    ///
    /// When this option is set to `true`, Immediately Invoked Function Expressions (IIFEs) are not checked for the maximum line limit.
    ///
    /// Default: `false`
    ///
    /// When `maxLines: 2` and `skipIifes: true`, the following IIFE will be considered valid even though its body has 3 lines:
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxLines": 2,
    ///        "skipIifes": true
    ///     }
    /// }
    /// ```
    /// ```js,use_options
    /// (() => {
    ///  const a = 1; // 1
    ///  const b = 2; // 2
    ///  const c = 3; // 3
    /// })();
    /// ```
    ///
    pub NoExcessiveLinesPerFunction {
        version: "2.0.0",
        name: "noExcessiveLinesPerFunction",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("max-lines-per-function").inspired()],
    }
}

impl Rule for NoExcessiveLinesPerFunction {
    type Query = Semantic<AnyFunctionLike>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveLinesPerFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let options = ctx.options();

        if let AnyFunctionLike::AnyJsFunction(func) = binding
            && is_iife(func)
            && options.skip_iifes
        {
            return None;
        };

        let Ok(func_body) = binding.body() else {
            return None;
        };

        let function_lines_count = func_body
            .syntax()
            .descendants()
            .flat_map(|descendant| descendant.tokens().collect::<Vec<_>>())
            .filter(|token| {
                !matches!(
                    token.kind(),
                    biome_js_syntax::JsSyntaxKind::L_CURLY | biome_js_syntax::JsSyntaxKind::R_CURLY
                )
            })
            .fold(0, |acc, token| {
                if options.skip_blank_lines {
                    return acc + token.has_leading_newline() as usize;
                };

                acc + token
                    .trim_trailing_trivia()
                    .leading_trivia()
                    .pieces()
                    .filter(|piece| piece.is_newline())
                    .count()
            });

        if function_lines_count > options.max_lines.get().into() {
            return Some(State {
                function_lines_count,
            });
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
                    "This function has too many lines ("{state.function_lines_count}"). Maximum allowed is "{options.max_lines.to_string()}"."
                },
            )
            .note(markup! {
                "Consider refactoring this function to split it into smaller functions."
            }),
        )
    }
}

fn is_iife(func: &AnyJsFunction) -> bool {
    func.parent::<JsParenthesizedExpression>()
        .and_then(|expr| expr.parent::<JsCallExpression>())
        .is_some()
}

pub struct State {
    function_lines_count: usize,
}

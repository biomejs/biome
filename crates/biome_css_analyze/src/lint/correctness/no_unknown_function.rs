use crate::fonts::is_function_keyword;
use crate::utils::is_custom_function;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssFunction;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_unknown_function::NoUnknownFunctionOptions;

declare_lint_rule! {
    /// Disallow unrecognized CSS value functions.
    ///
    /// CSS value functions use a name followed by parentheses, such as `scale()` or `calc()`.
    /// Custom functions whose names begin with `--`, such as `--custom-function()`, are allowed.
    /// Known functions come from the
    /// [MDN CSS reference](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions) and
    /// [browser compatibility data](https://github.com/mdn/browser-compat-data/tree/main/css/types).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { transform: unknown(1); }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { transform: scale(1); }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignore`
    ///
    /// Lists additional function names to allow, without regard to letter case. Defaults to an
    /// empty list.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": [
    ///       "custom-function"
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a { transform: custom-function(1); }
    /// ```
    ///
    pub NoUnknownFunction {
        version: "1.8.0",
        name: "noUnknownFunction",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("function-no-unknown").same()],
    }
}

pub struct NoUnknownFunctionState {
    function_name: Box<str>,
    span: TextRange,
}

impl Rule for NoUnknownFunction {
    type Query = Ast<CssFunction>;
    type State = NoUnknownFunctionState;
    type Signals = Option<Self::State>;
    type Options = NoUnknownFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let binding = node.name().ok().and_then(|name| {
            name.as_css_identifier()
                .and_then(|name| name.value_token().ok())
        })?;
        let function_name = binding.text_trimmed();

        // We don't have a semantic model yet, so we can't determine if functions are defined elsewhere.
        // Therefore, we ignore these custom functions to prevent false detections.
        if is_custom_function(function_name) {
            return None;
        }

        if is_function_keyword(function_name) {
            return None;
        }

        if should_ignore(function_name, ctx.options()) {
            return None;
        }

        Some(NoUnknownFunctionState {
            function_name: function_name.into(),
            span: node.name().ok()?.range(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.span,
                markup! {
                    "Unexpected unknown function: "<Emphasis>{state.function_name}</Emphasis>
                },
            )
            .note(markup! {
                "Use a known function instead."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions">"MDN web docs"</Hyperlink>" for more details."
            }),
        )
    }
}

fn should_ignore(name: &str, options: &NoUnknownFunctionOptions) -> bool {
    for ignore_pattern in &options.ignore {
        if name.eq_ignore_ascii_case(ignore_pattern) {
            return true;
        }
    }
    false
}

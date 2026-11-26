use crate::fonts::is_function_keyword;
use crate::services::semantic::Semantic;
use crate::utils::{is_custom_function, is_scss_built_in_function};
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::CssFunction;
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_unknown_function::NoUnknownFunctionOptions;

declare_lint_rule! {
    /// Disallow unrecognized CSS value functions.
    ///
    /// This rule ignores double-dashed custom functions, e.g. `--custom-function()`.
    /// In SCSS files, it also ignores SCSS built-in functions and functions declared in the same file.
    ///
    /// ## SCSS limitations
    ///
    /// The rule does not resolve functions made globally available by legacy SCSS `@import`.
    /// Configure the `ignore` option for imported functions that cannot be resolved locally.
    ///
    /// Data sources of known CSS value functions are:
    /// - MDN reference on [CSS value functions](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions)
    /// - MDN reference on [CSS reference](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference)
    /// - MDN [browser compatibility data for CSS value functions](https://github.com/mdn/browser-compat-data/tree/main/css/types)
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
    type Query = Semantic<CssFunction>;
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

        // The semantic model doesn't track custom function definitions, which may also come
        // from other files, so we ignore custom functions to prevent false detections.
        if is_custom_function(function_name) {
            return None;
        }

        if is_function_keyword(function_name) {
            return None;
        }

        if ctx.source_type::<CssFileSource>().is_scss()
            && (is_scss_built_in_function(function_name)
                || ctx.model().has_scss_function(function_name))
        {
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
                "Browsers ignore a declaration that contains an unknown function, so the style is not applied."
            })
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

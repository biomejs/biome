use crate::fonts::is_function_keyword;
use crate::utils::is_custom_function;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssFunction, ScssFunctionAtRule};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_unknown_function::NoUnknownFunctionOptions;

declare_lint_rule! {
    /// Disallow unrecognized CSS value functions.
    ///
    /// This rule ignores double-dashed custom functions, e.g. `--custom-function()`.
    /// In SCSS files, it also ignores Sass built-in functions and functions declared in the same file.
    ///
    /// ## Sass limitations
    ///
    /// The rule does not resolve functions made globally available by legacy Sass `@import`.
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

const SCSS_BUILT_IN_FUNCTIONS: &[&str] = &[
    "abs",
    "adjust-color",
    "adjust-hue",
    "alpha",
    "append",
    "blue",
    "call",
    "ceil",
    "change-color",
    "color",
    "comparable",
    "complement",
    "content-exists",
    "darken",
    "desaturate",
    "fade-in",
    "fade-out",
    "feature-exists",
    "floor",
    "function-exists",
    "get-function",
    "global-variable-exists",
    "grayscale",
    "green",
    "hsl",
    "hsla",
    "hue",
    "hwb",
    "ie-hex-str",
    "if",
    "index",
    "inspect",
    "invert",
    "is-bracketed",
    "is-superselector",
    "join",
    "keywords",
    "lab",
    "lch",
    "length",
    "lighten",
    "lightness",
    "list-separator",
    "map-get",
    "map-has-key",
    "map-keys",
    "map-merge",
    "map-remove",
    "map-values",
    "max",
    "min",
    "mix",
    "mixin-exists",
    "nth",
    "oklab",
    "oklch",
    "opacify",
    "opacity",
    "percentage",
    "quote",
    "random",
    "red",
    "rgb",
    "rgba",
    "round",
    "saturate",
    "saturation",
    "scale-color",
    "selector-append",
    "selector-extend",
    "selector-nest",
    "selector-parse",
    "selector-replace",
    "selector-unify",
    "set-nth",
    "simple-selectors",
    "str-index",
    "str-insert",
    "str-length",
    "str-slice",
    "to-lower-case",
    "to-upper-case",
    "transparentize",
    "type-of",
    "unit",
    "unitless",
    "unique-id",
    "unquote",
    "variable-exists",
    "zip",
];

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

        if ctx.source_type::<CssFileSource>().is_scss()
            && (is_scss_built_in_function(function_name)
                || ctx
                    .root()
                    .syntax()
                    .descendants()
                    .filter_map(ScssFunctionAtRule::cast)
                    .any(|function| {
                        function
                            .name()
                            .ok()
                            .and_then(|name| name.value_token().ok())
                            .is_some_and(|name| {
                                scss_function_names_equal(name.text_trimmed(), function_name)
                            })
                    }))
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
                "Use a known function instead."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions">"MDN web docs"</Hyperlink>" for more details."
            }),
        )
    }
}

fn is_scss_built_in_function(name: &str) -> bool {
    SCSS_BUILT_IN_FUNCTIONS
        .iter()
        .any(|built_in| scss_function_names_equal(name, built_in))
}

fn scss_function_names_equal(left: &str, right: &str) -> bool {
    left.bytes()
        .map(|byte| if byte == b'_' { b'-' } else { byte })
        .eq(right
            .bytes()
            .map(|byte| if byte == b'_' { b'-' } else { byte }))
}

fn should_ignore(name: &str, options: &NoUnknownFunctionOptions) -> bool {
    for ignore_pattern in &options.ignore {
        if name.eq_ignore_ascii_case(ignore_pattern) {
            return true;
        }
    }
    false
}

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::TextRange;

declare_lint_rule! {
    /// Enforces the use of a recommended `display` strategy with Google Fonts.
    ///
    /// The `display` property controls how a font is displayed while it is loading. When using Google Fonts,
    /// it's important to specify an appropriate value for this property to ensure good user experience and prevent layout shifts.
    ///
    /// This rule flags the absence of the `display` parameter, or the usage of less optimal values such as `auto`, `block`, or `fallback`.
    /// Using `&display=optional` is generally recommended as it minimizes the risk of invisible text or layout shifts.
    /// In cases where swapping to the custom font after it has loaded is important, consider using `&display=swap`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <link href="https://fonts.googleapis.com/css2?family=Krona+One" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <link href="https://fonts.googleapis.com/css2?family=Krona+One&display=auto" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <link href="https://fonts.googleapis.com/css2?family=Krona+One&display=block" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <link href="https://fonts.googleapis.com/css2?family=Krona+One&display=fallback" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <link href="https://fonts.googleapis.com/css2?family=Krona+One&display=optional" rel="stylesheet" />
    /// ```
    ///
    /// ```jsx
    /// <link href="https://fonts.googleapis.com/css2?display=unknown" rel="stylesheet" />
    /// ```
    ///
    /// ```jsx
    /// <link rel="stylesheet" />
    /// ```
    pub UseGoogleFontDisplay {
        version: "1.9.4",
        name: "useGoogleFontDisplay",
        language: "jsx",
        sources: &[RuleSource::EslintNext("google-font-display")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        domains: &[RuleDomain::Next],
    }
}

const FORBIDDEN_VALUES: [&str; 3] = ["auto", "block", "fallback"];

pub enum FontDisplayIssue {
    MissingDisplayParam,
    ForbiddenValue,
}

impl Rule for UseGoogleFontDisplay {
    type Query = Ast<AnyJsxElement>;
    type State = (FontDisplayIssue, TextRange);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if element.name().ok()?.name_value_token().ok()?.text_trimmed() != "link" {
            return None;
        }

        let href = element.find_attribute_by_name("href")?;
        let initializer = href.initializer()?.value().ok()?.as_static_value()?;
        let href_text = initializer.as_string_constant()?;

        if !href_text.starts_with("https://fonts.googleapis.com/css") {
            return None;
        }

        let display_param = href_text
            .split('?')
            .last()?
            .split('&')
            .find(|p| p.starts_with("display="));
        let range = initializer.range();

        if let Some(display_param) = display_param {
            for forbidden_value in FORBIDDEN_VALUES {
                if display_param.ends_with(forbidden_value) {
                    return Some((FontDisplayIssue::ForbiddenValue, range));
                }
            }
        } else {
            return Some((FontDisplayIssue::MissingDisplayParam, range));
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, (issue, range): &Self::State) -> Option<RuleDiagnostic> {
        let title = match issue {
            FontDisplayIssue::MissingDisplayParam => markup! {
                "The Google Font link is missing the "<Emphasis>"display"</Emphasis>" parameter."
            },
            FontDisplayIssue::ForbiddenValue => markup! {
                "The Google Font link has a non-recommended "<Emphasis>"display"</Emphasis>" value."
            },
        };

        Some(RuleDiagnostic::new(rule_category!(), range, title).note(
            markup! {
                "Use "<Emphasis>"&display=optional"</Emphasis>" to prevent invisible text and layout shifts. If font swapping is important, use "<Emphasis>"&display=swap"</Emphasis>"."
            }
        ))
    }
}

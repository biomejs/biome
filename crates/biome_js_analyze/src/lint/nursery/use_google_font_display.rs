use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::TextRange;

declare_lint_rule! {
    /// Enforces the use of a recommended `font-display` strategy with Google Fonts.
    ///
    /// The `font-display` property controls how a font is displayed while it is loading. When using Google Fonts,
    /// it's important to specify an appropriate value for this property to ensure good user experience and prevent layout shifts.
    ///
    /// This rule flags the absence of the `font-display` parameter, or the usage of less optimal values such as `auto`, `block`, or `fallback`.
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
        version: "next",
        name: "useGoogleFontDisplay",
        language: "jsx",
        sources: &[RuleSource::EslintNext("google-font-display")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

impl Rule for UseGoogleFontDisplay {
    type Query = Ast<AnyJsxElement>;
    type State = (bool, TextRange);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if element.name().ok()?.name_value_token()?.text_trimmed() != "link" {
            return None;
        }

        let href = element.find_attribute_by_name("href");
        let initializer = href?.initializer()?.value().ok()?;
        let jsx_string = initializer.as_jsx_string();
        let href_text = jsx_string?.inner_string_text().ok()?;

        if !href_text.starts_with("https://fonts.googleapis.com/css") {
            return None;
        }

        let display_param = href_text
            .text()
            .split('?')
            .last()?
            .split('&')
            .find(|p| p.starts_with("display="));
        let range = jsx_string?.value_token().ok()?.text_trimmed_range();

        if let Some(display_param) = display_param {
            if matches!(
                &*display_param.replace("display=", ""),
                "auto" | "block" | "fallback"
            ) {
                return Some((false, range));
            }
        } else {
            return Some((true, range));
        }

        None
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        (is_missing_param, range): &Self::State,
    ) -> Option<RuleDiagnostic> {
        if *is_missing_param {
            return Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The Google Font link is missing the "<Emphasis>"font-display"</Emphasis>" parameter."
                    },
                )
                .note(markup!{
                    "Add "<Emphasis>"&display=optional"</Emphasis>" to prevent invisible text and layout shifts. If font swapping is important, use "<Emphasis>"&display=swap"</Emphasis>"."
                })
            );
        };

        return Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The Google Font link has a non-recommended "<Emphasis>"font-display"</Emphasis>" value."
                },
            )
            .note(markup!{
                "Use "<Emphasis>"&display=optional"</Emphasis>" to improve performance and prevent layout shifts, or "<Emphasis>"&display=swap"</Emphasis>" if font swapping is necessary after loading."
            })
        );
    }
}

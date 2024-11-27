use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_aria_metadata::{is_valid_country, is_valid_language};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
declare_lint_rule! {
    /// Ensure that the attribute passed to the `lang` attribute is a correct ISO language and/or country.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang="lorem" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang="en-babab" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang="en-GB-typo" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Html lang="en-babab" />
    /// ```
    pub UseValidLang {
        version: "1.0.0",
        name: "useValidLang",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("lang")],
        recommended: true,
        severity: Severity::Error,
    }
}

enum InvalidKind {
    Language,
    Country,
    Value,
}

pub struct UseValidLangState {
    invalid_kind: InvalidKind,
    attribute_range: TextRange,
}

impl Rule for UseValidLang {
    type Query = Ast<AnyJsxElement>;
    type State = UseValidLangState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let element_text = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        if element_text.text_trimmed() == "html" {
            let attribute = node.find_attribute_by_name("lang")?;
            let attribute_value = attribute.initializer()?.value().ok()?;
            let attribute_static_value = attribute_value.as_static_value()?;
            let attribute_text = attribute_static_value.text();
            let mut split_value = attribute_text.split('-');
            match (split_value.next(), split_value.next()) {
                (Some(language), Some(country)) => {
                    if !is_valid_language(language) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            invalid_kind: InvalidKind::Language,
                        });
                    } else if !is_valid_country(country) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            invalid_kind: InvalidKind::Country,
                        });
                    } else if split_value.next().is_some() {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            invalid_kind: InvalidKind::Value,
                        });
                    }
                }

                (Some(language), None) => {
                    if !is_valid_language(language) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            invalid_kind: InvalidKind::Language,
                        });
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.attribute_range,
            markup! {
                "Provide a valid value for the "<Emphasis>"lang"</Emphasis>" attribute."
            },
        );
        diagnostic = match state.invalid_kind {
            InvalidKind::Language => {
                let languages = biome_aria_metadata::languages();
                let languages = if languages.len() > 15 {
                    &languages[..15]
                } else {
                    languages
                };

                diagnostic.footer_list("Some of valid languages:", languages)
            }
            InvalidKind::Country => {
                let countries = biome_aria_metadata::countries();
                let countries = if countries.len() > 15 {
                    &countries[..15]
                } else {
                    countries
                };

                diagnostic.footer_list("Some of valid countries:", countries)
            }
            InvalidKind::Value => diagnostic,
        };
        Some(diagnostic)
    }
}

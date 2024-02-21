use crate::aria_services::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
declare_rule! {
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
        source: RuleSource::EslintJsxA11y("lang"),
        recommended: true,
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
    type Query = Aria<AnyJsxElement>;
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
                    if !ctx.is_valid_iso_language(language) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            invalid_kind: InvalidKind::Language,
                        });
                    } else if !ctx.is_valid_iso_country(country) {
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
                    if !ctx.is_valid_iso_language(language) {
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

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.attribute_range,
            markup! {
                "Provide a valid value for the "<Emphasis>"lang"</Emphasis>" attribute."
            },
        );
        diagnostic = match state.invalid_kind {
            InvalidKind::Language => {
                let languages = ctx.iso_language_list();
                let languages = if languages.len() > 15 {
                    &languages[..15]
                } else {
                    languages
                };

                diagnostic.footer_list("Some of valid languages:", languages)
            }
            InvalidKind::Country => {
                let countries = ctx.iso_country_list();
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

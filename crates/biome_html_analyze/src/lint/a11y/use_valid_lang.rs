use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_aria_metadata::{is_valid_country, is_valid_language, is_valid_script};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlFileSource;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_valid_lang::UseValidLangOptions;

declare_lint_rule! {
    /// Ensure that the attribute passed to the `lang` attribute is a correct ISO language and/or country.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <html lang="lorem" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <html lang="en-babab" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <html lang="en-GB-typo" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <html lang="en-GB" />
    /// ```
    pub UseValidLang {
        version: "2.4.0",
        name: "useValidLang",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("lang").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

enum InvalidKind {
    Language,
    Country,
    Script,
    Value,
}

pub struct UseValidLangState {
    invalid_kind: InvalidKind,
    attribute_range: TextRange,
}

impl Rule for UseValidLang {
    type Query = Ast<AnyHtmlTagElement>;
    type State = UseValidLangState;
    type Signals = Option<Self::State>;
    type Options = UseValidLangOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let element_text = node.name().ok()?.token_text_trimmed()?;
        let source_type = ctx.source_type::<HtmlFileSource>();
        let matches_tag = if source_type.is_html() {
            element_text.eq_ignore_ascii_case("html")
        } else {
            element_text == "html"
        };
        if !matches_tag {
            return None;
        }

        let attribute = node.find_attribute_by_name("lang")?;
        let attribute_value = attribute.initializer()?.value().ok()?;
        let attribute_static_value = attribute_value.as_static_value()?;
        let attribute_text = attribute_static_value.text();
        let mut split_value = attribute_text.split('-');
        match (split_value.next(), split_value.next(), split_value.next()) {
            (Some(language), Some(script), Some(country)) => {
                if split_value.next().is_some() {
                    return Some(UseValidLangState {
                        attribute_range: attribute_value.range(),
                        invalid_kind: InvalidKind::Value,
                    });
                } else if !is_valid_language(language) {
                    return Some(UseValidLangState {
                        attribute_range: attribute_value.range(),
                        invalid_kind: InvalidKind::Language,
                    });
                } else if !is_valid_script(script) {
                    return Some(UseValidLangState {
                        attribute_range: attribute_value.range(),
                        invalid_kind: InvalidKind::Script,
                    });
                } else if !is_valid_country(country) {
                    return Some(UseValidLangState {
                        attribute_range: attribute_value.range(),
                        invalid_kind: InvalidKind::Country,
                    });
                }
            }

            (Some(language), Some(script_or_country), None) => {
                if !is_valid_language(language) {
                    return Some(UseValidLangState {
                        attribute_range: attribute_value.range(),
                        invalid_kind: InvalidKind::Language,
                    });
                } else if !is_valid_script(script_or_country)
                    && !is_valid_country(script_or_country)
                {
                    match script_or_country.len() {
                        4 => {
                            return Some(UseValidLangState {
                                attribute_range: attribute_value.range(),
                                invalid_kind: InvalidKind::Script,
                            });
                        }
                        2 | 3 => {
                            return Some(UseValidLangState {
                                attribute_range: attribute_value.range(),
                                invalid_kind: InvalidKind::Country,
                            });
                        }
                        _ => {
                            return Some(UseValidLangState {
                                attribute_range: attribute_value.range(),
                                invalid_kind: InvalidKind::Value,
                            });
                        }
                    }
                }
            }

            (Some(language), None, None) => {
                if !is_valid_language(language) {
                    return Some(UseValidLangState {
                        attribute_range: attribute_value.range(),
                        invalid_kind: InvalidKind::Language,
                    });
                }
            }
            _ => {}
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
            InvalidKind::Script => {
                let scripts = biome_aria_metadata::scripts();
                let scripts = if scripts.len() > 15 {
                    &scripts[..15]
                } else {
                    scripts
                };

                diagnostic.footer_list("Some of valid scripts:", scripts)
            }
            InvalidKind::Value => diagnostic,
        };
        Some(diagnostic)
    }
}

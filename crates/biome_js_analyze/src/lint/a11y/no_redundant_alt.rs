use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, AnyJsxAttributeValue,
};
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Enforce `img` alt prop does not contain the word "image", "picture", or "photo".
    ///
    /// The rule will first check if `aria-hidden` is truthy to determine whether to enforce the rule. If the image is
    /// hidden, then the rule will always succeed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="src" alt="photo content" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img alt={`picture doing ${things}`} {...this.props} />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img alt="picture of cool person" aria-hidden={false} />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    /// 	<img src="src" alt="alt" />
    /// 	<img src="src" alt={photo} />
    /// 	<img src="bar" aria-hidden alt="Picture of me taking a photo of an image" />
    /// </>
    /// ```
    ///
    pub NoRedundantAlt {
        version: "1.0.0",
        name: "noRedundantAlt",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("img-redundant-alt")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoRedundantAlt {
    type Query = Ast<AnyJsxElement>;
    type State = AnyJsxAttributeValue;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.name_value_token().ok()?.text_trimmed() != "img" {
            return None;
        }
        let aria_hidden_attribute = node.find_attribute_by_name("aria-hidden");
        if let Some(aria_hidden) = aria_hidden_attribute {
            let is_false = match aria_hidden.initializer()?.value().ok()? {
                AnyJsxAttributeValue::AnyJsxTag(_) => false,
                AnyJsxAttributeValue::JsxExpressionAttributeValue(aria_hidden) => {
                    aria_hidden
                        .expression()
                        .ok()?
                        .as_any_js_literal_expression()?
                        .as_js_boolean_literal_expression()?
                        .value_token()
                        .ok()?
                        .text_trimmed()
                        == "false"
                }
                AnyJsxAttributeValue::JsxString(aria_hidden) => {
                    aria_hidden.inner_string_text().ok()?.text() == "false"
                }
            };

            if !is_false {
                return None;
            }
        }

        let alt = node
            .find_attribute_by_name("alt")?
            .initializer()?
            .value()
            .ok()?;

        match alt {
            AnyJsxAttributeValue::AnyJsxTag(_) => None,
            AnyJsxAttributeValue::JsxExpressionAttributeValue(ref value) => {
                match value.expression().ok()? {
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(expr),
                    ) => is_redundant_alt(expr.inner_string_text().ok()?.text()).then_some(alt),
                    AnyJsExpression::JsTemplateExpression(expr) => {
                        let contain_redundant_alt =
                            expr.elements().into_iter().any(|template_element| {
                                match template_element {
                                    AnyJsTemplateElement::JsTemplateChunkElement(node) => {
                                        node.template_chunk_token().ok().is_some_and(|token| {
                                            is_redundant_alt(token.text_trimmed())
                                        })
                                    }
                                    AnyJsTemplateElement::JsTemplateElement(_) => false,
                                }
                            });

                        contain_redundant_alt.then_some(alt)
                    }

                    _ => None,
                }
            }
            AnyJsxAttributeValue::JsxString(ref value) => {
                let inner_string_text = value.inner_string_text().ok()?;
                is_redundant_alt(inner_string_text.text()).then_some(alt)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Avoid the words \"image\", \"picture\", or \"photo\" in " <Emphasis>"img"</Emphasis>" element alt text."
                },
            )
            .note(markup! {
                "Screen readers announce img elements as \"images\", so it is not necessary to redeclare this in alternative text."
            }),
        )
    }
}

const REDUNDANT_WORDS: [&str; 3] = ["image", "photo", "picture"];

fn is_redundant_alt(alt: &str) -> bool {
    REDUNDANT_WORDS.into_iter().any(|word| {
        alt.split_whitespace()
            .any(|x| x.to_ascii_lowercase_cow() == word)
    })
}

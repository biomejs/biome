use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, AnyVueDirective};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_vue_valid_v_html::UseVueValidVHtmlOptions;

declare_lint_rule! {
    /// Enforce valid `v-html` directives.
    ///
    /// This rule reports v-html directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-html:aaa></div>`
    /// - The directive has a modifier. E.g. `<div v-html.bbb></div>`
    /// - The directive does not have an attribute value. E.g. `<div v-html></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-html:aaa="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-html.bbb="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-html></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-html="htmlContent"></div>
    /// ```
    ///
    pub UseVueValidVHtml {
        version: "2.3.6",
        name: "useVueValidVHtml",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-html").same()],
    }
}

pub enum ViolationKind {
    UnexpectedArgument(TextRange),
    UnexpectedModifier(TextRange),
    MissingValue,
}

impl Rule for UseVueValidVHtml {
    type Query = Ast<AnyVueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVHtmlOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            AnyVueDirective::VueDirective(vue_directive) => {
                if vue_directive.name_token().ok()?.text_trimmed() != "v-html" {
                    return None;
                }

                if let Some(arg) = vue_directive.arg() {
                    return Some(ViolationKind::UnexpectedArgument(arg.range()));
                }

                if !vue_directive.modifiers().is_empty() {
                    let first_modifier = vue_directive.modifiers().iter().next()?;
                    return Some(ViolationKind::UnexpectedModifier(first_modifier.range()));
                }

                if let Some(initializer) = vue_directive.initializer() {
                    // Check if value is empty
                    if let Ok(value) = initializer.value() {
                        if let AnyHtmlAttributeInitializer::HtmlString(html_string) = value {
                            // Check if the string value is empty
                            if let Ok(token) = html_string.value_token() {
                                if token.text().is_empty() {
                                    return Some(ViolationKind::MissingValue);
                                }
                            } else {
                                // No value token means empty string
                                return Some(ViolationKind::MissingValue);
                            }
                        }
                        // Non-string values (like expressions) are valid
                    } else {
                        // No value at all
                        return Some(ViolationKind::MissingValue);
                    }
                } else {
                    return Some(ViolationKind::MissingValue);
                }

                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            match state {
                ViolationKind::UnexpectedArgument(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The v-html directive does not accept an argument."
                    },
                )
                .note(markup! {
                    "v-html directives should be used without arguments, like " <Emphasis>"v-html=\"content\""</Emphasis>"."
                }),
                ViolationKind::UnexpectedModifier(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The v-html directive does not support modifiers."
                    },
                )
                .note(markup! {
                    "v-html directives do not support any modifiers. Remove the modifier."
                }),
                ViolationKind::MissingValue => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "The v-html directive is missing a value."
                    },
                )
                .note(markup! {
                    "v-html directives require a value containing the HTML content to render."
                }).note(markup! {
                    "For example, use " <Emphasis>"v-html=\"htmlContent\""</Emphasis> " to render the content of the " <Emphasis>"htmlContent"</Emphasis> " variable."
                }),
            }
        )
    }
}

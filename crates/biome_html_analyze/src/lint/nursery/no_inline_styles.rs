use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute, HtmlAttributeList, HtmlFileSource};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TokenText};
use biome_rule_options::no_inline_styles::NoInlineStylesOptions;
use biome_string_case::StrOnlyExtension;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow the use of inline styles.
    ///
    /// Inline styles via the `style` attribute make code harder to maintain and override,
    /// prevent reusability of styling, and can be a security concern when implementing
    /// a strict Content Security Policy (CSP).
    ///
    /// Instead of inline styles, use CSS classes, CSS modules, or a styling library.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div style="color: red;"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <p style="font-size: 14px;">Hello</p>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="text-red"></div>
    /// ```
    ///
    /// ```html
    /// <p class="body-text">Hello</p>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [Content Security Policy: Allowing inline styles](https://content-security-policy.com/examples/allow-inline-style)
    ///
    pub NoInlineStyles {
        version: "2.4.9",
        name: "noInlineStyles",
        language: "html",
        recommended: false,
        sources: &[
            RuleSource::HtmlEslint("no-inline-styles").same(),
        ],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInlineStyles {
    type Query = Ast<AnyHtmlElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoInlineStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyHtmlElement::HtmlElement(element) => {
                let opening = element.opening_element().ok()?;

                if let Some(name) = opening.tag_name()
                    && is_custom_component(name, ctx)
                {
                    return None;
                }

                find_style_attribute(opening.attributes()).map(|attribute| attribute.range())
            }
            AnyHtmlElement::HtmlSelfClosingElement(self_closing_element) => {
                if let Some(name) = self_closing_element.tag_name()
                    && is_custom_component(name, ctx)
                {
                    return None;
                }

                find_style_attribute(self_closing_element.attributes())
                    .map(|attribute| attribute.range())
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Unexpected "<Emphasis>"style"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, reduce reusability, and can prevent effective use of a strict Content Security Policy. Use external CSS classes or stylesheets instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            AnyHtmlElement::HtmlElement(element) => {
                let opening = element.opening_element().ok()?;
                mutation.remove_node(find_style_attribute(opening.attributes()).unwrap())
            }
            AnyHtmlElement::HtmlSelfClosingElement(self_closing_element) => mutation
                .remove_node(find_style_attribute(self_closing_element.attributes()).unwrap()),
            _ => {}
        }

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"style"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

fn find_style_attribute(attributes: HtmlAttributeList) -> Option<HtmlAttribute> {
    for attribute in attributes {
        if let Some(attribute) = attribute.as_html_attribute()
            && let Some(name) = attribute.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.text_trimmed().to_lowercase_cow() == "style"
        {
            return Some(attribute.clone());
        }
    }

    None
}

fn is_custom_component(element_name: TokenText, ctx: &RuleContext<NoInlineStyles>) -> bool {
    let source_type = ctx.source_type::<HtmlFileSource>();

    if source_type.is_html() {
        return false;
    }

    element_name.text() != element_name.to_lowercase_cow()
}

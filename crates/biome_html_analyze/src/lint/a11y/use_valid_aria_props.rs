use std::{borrow::Cow, str::FromStr};

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaAttribute;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlAttribute, HtmlFileSource};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};
use biome_rule_options::use_valid_aria_props::UseValidAriaPropsOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Ensures that ARIA properties `aria-*` are all valid.
    ///
    /// :::note
    /// In `.html` files, this rule treats `aria-*` attribute names case-insensitively.
    ///
    /// In Vue templates, this rule also checks static `v-bind:aria-*` and `:aria-*` arguments
    /// because their generated ARIA attribute names are known statically.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input aria-labell="" />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div :aria-labell="label"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div aria-label="Search"></div>
    /// ```
    ///
    /// ```vue
    /// <div :aria-label="label"></div>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    pub UseValidAriaProps {
        version: "2.4.0",
        name: "useValidAriaProps",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("aria-props").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseValidAriaProps {
    type Query = Ast<AnyHtmlAttribute>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = UseValidAriaPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<HtmlFileSource>();
        let attribute_name = extract_attribute_name(ctx.query())?;
        let normalized_name = normalize_attribute_name(attribute_name.text(), *source_type);

        if !normalized_name.starts_with("aria-") {
            return None;
        }

        if AriaAttribute::from_str(normalized_name.as_ref()).is_ok() {
            return None;
        }

        Some(attribute_name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The element contains an invalid ARIA attribute."
                },
            )
            .note(markup! {
                <Emphasis>{state.text()}</Emphasis>" is not a valid ARIA attribute."
            })
            .note(markup! {
                "Check the list of all "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes#aria_attribute_types">"valid"</Hyperlink>" aria-* attributes."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the invalid "<Emphasis>{state.text()}</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        ))
    }
}

fn extract_attribute_name(attribute: &AnyHtmlAttribute) -> Option<TokenText> {
    if let Some(html_attribute) = attribute.as_html_attribute()
        && let Ok(name) = html_attribute.name()
        && let Ok(token) = name.value_token()
    {
        return Some(token.token_text_trimmed());
    }

    let vue_directive = attribute.as_any_vue_directive()?;

    if let Some(directive) = vue_directive.as_vue_directive() {
        let name = directive.name_token().ok()?.text_trimmed();
        if name != "v-bind" {
            return None;
        }

        let static_argument = directive.arg()?.arg().ok()?.as_vue_static_argument()?;
        return Some(static_argument.name_token().ok()?.token_text_trimmed());
    }

    if let Some(shorthand_bind) = vue_directive.as_vue_v_bind_shorthand_directive() {
        let static_argument = shorthand_bind
            .arg()
            .ok()?
            .arg()
            .ok()?
            .as_vue_static_argument()?;
        return Some(static_argument.name_token().ok()?.token_text_trimmed());
    }

    None
}

fn normalize_attribute_name<'a>(name: &'a str, source_type: HtmlFileSource) -> Cow<'a, str> {
    if source_type.is_html() {
        Cow::Owned(name.to_ascii_lowercase())
    } else {
        Cow::Borrowed(name)
    }
}

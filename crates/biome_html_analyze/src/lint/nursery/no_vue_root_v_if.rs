use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlElement, HtmlRoot, T, VueDirective};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_vue_root_v_if::NoVueRootVIfOptions;

declare_lint_rule! {
    /// Disallow `v-if` on the root element of a Vue component template.
    ///
    /// When the only root element of a template has a `v-if` and no `v-else`,
    /// the component renders nothing whenever the condition is false. Making
    /// the whole component conditional is clearer in the parent component, by
    /// putting the `v-if` where the component is used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="visible">Content</div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-if="visible">Content</div>
    ///   <div v-else>Fallback</div>
    /// </template>
    /// ```
    ///
    pub NoVueRootVIf {
        version: "next",
        name: "noVueRootVIf",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-root-v-if").same()],
    }
}

impl Rule for NoVueRootVIf {
    type Query = Ast<HtmlRoot>;
    type State = VueDirective;
    type Signals = Option<Self::State>;
    type Options = NoVueRootVIfOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let template = ctx.query().html().iter().find_map(|element| match element {
            AnyHtmlElement::HtmlElement(element)
                if element
                    .opening_element()
                    .ok()
                    .and_then(|opening| opening.name().ok())
                    .and_then(|name| name.tag_name_kind())
                    == Some(T![template]) =>
            {
                Some(element)
            }
            _ => None,
        })?;

        // Text, interpolations, and comments don't count as root elements.
        let mut root_elements = template.children().iter().filter(|child| {
            matches!(
                child,
                AnyHtmlElement::HtmlElement(_) | AnyHtmlElement::HtmlSelfClosingElement(_)
            )
        });
        let root_element = root_elements.next()?;
        if root_elements.next().is_some() {
            return None;
        }

        root_element.attributes()?.iter().find_map(|attribute| {
            let directive = attribute.as_any_vue_directive()?.as_vue_directive()?;
            directive.is_if().then(|| directive.clone())
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "The root element of this template is conditionally rendered with "<Emphasis>"v-if"</Emphasis>"."
                },
            )
            .note(markup! {
                "When the condition is false, the component renders nothing, which hides the condition from the parent component."
            })
            .note(markup! {
                "Move the "<Emphasis>"v-if"</Emphasis>" to the parent component where this component is used, or add a "<Emphasis>"v-else"</Emphasis>" branch."
            }),
        )
    }
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    HtmlAttribute, HtmlAttributeList, VueDirective, VueVBindShorthandDirective,
    element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_vue_deprecated_html_element_is::NoVueDeprecatedHtmlElementIsOptions;

use crate::vue::is_native_element_name;

declare_lint_rule! {
    /// Disallow the `is` attribute on HTML elements in Vue templates.
    ///
    /// In Vue 2, you could add `is` to an HTML element, such as `<tr is="my-row">`, to render a
    /// component in its place. Vue 3 removed this. On an HTML, SVG, or MathML element, Vue 3
    /// passes `is` to the browser as a regular attribute, so the component isn't rendered.
    ///
    /// To render a component in place of an HTML element, prefix the value with `vue:`, as in
    /// `<tr is="vue:my-row">`. To choose the component while the app is running, use the
    /// built-in `<component>` element, as in `<component :is="current">`.
    ///
    /// This rule doesn't report `is` on components, such as `<MyComponent>` or `<component>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div is="my-component" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :is="currentComponent" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div />
    ///   <div is="vue:my-component" />
    ///   <component is="my-component" />
    ///   <component :is="currentComponent" />
    ///   <MyComponent is="my-component" />
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue 3 Migration Guide: Custom Elements Interop](https://v3-migration.vuejs.org/breaking-changes/custom-elements-interop.html#customized-built-in-elements)
    /// - [Vue `is` attribute](https://vuejs.org/api/built-in-special-attributes.html#is)
    pub NoVueDeprecatedHtmlElementIs {
        version: "2.6.5",
        name: "noVueDeprecatedHtmlElementIs",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-html-element-is").same()],
    }
}

declare_node_union! {
    pub AnyVueIsAttribute = HtmlAttribute | VueDirective | VueVBindShorthandDirective
}

impl Rule for NoVueDeprecatedHtmlElementIs {
    type Query = Ast<AnyVueIsAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedHtmlElementIsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let attribute = ctx.query();
        if !is_reported_is_attribute(attribute) {
            return None;
        }

        let element = attribute
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        is_native_element_name(&element.name().ok()?).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let attribute = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attribute.range(),
            markup! {
                "The "<Emphasis>"is"</Emphasis>" attribute is used on a built-in element."
            },
        )
        .note(markup! {
            "Vue 3 no longer renders a component in place of a built-in HTML, SVG, or MathML element with "<Emphasis>"is"</Emphasis>". It passes the attribute to the browser instead."
        });
        Some(match attribute {
            AnyVueIsAttribute::HtmlAttribute(_) => diagnostic.note(markup! {
                "Prefix the value with "<Emphasis>"vue:"</Emphasis>", or use the "<Emphasis>"<component>"</Emphasis>" element to render a component."
            }),
            AnyVueIsAttribute::VueDirective(_)
            | AnyVueIsAttribute::VueVBindShorthandDirective(_) => diagnostic.note(markup! {
                "Use the "<Emphasis>"<component>"</Emphasis>" element to choose which component to render."
            }),
        })
    }
}

/// Returns `true` for `is`, `:is`, and `v-bind:is`, except for a static `is` whose value starts
/// with `vue:`, which renders a component.
fn is_reported_is_attribute(attribute: &AnyVueIsAttribute) -> bool {
    match attribute {
        AnyVueIsAttribute::HtmlAttribute(attribute) => {
            attribute
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .is_some_and(|name| name.text_trimmed() == "is")
                && !attribute
                    .as_static_value()
                    .is_some_and(|value| value.text().starts_with("vue:"))
        }
        AnyVueIsAttribute::VueDirective(directive) => {
            directive.is_binding()
                && directive
                    .arg()
                    .and_then(|arg| arg.arg())
                    .and_then(|arg| arg.as_vue_static_argument()?.name_token().ok())
                    .is_some_and(|name| name.text_trimmed() == "is")
        }
        AnyVueIsAttribute::VueVBindShorthandDirective(directive) => directive
            .arg()
            .ok()
            .and_then(|arg| arg.arg())
            .and_then(|arg| arg.as_vue_static_argument()?.name_token().ok())
            .is_some_and(|name| name.text_trimmed() == "is"),
    }
}

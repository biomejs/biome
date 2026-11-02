use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttribute, AnyHtmlElement, HtmlRoot, T};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNodeList;
use biome_rule_options::no_vue_deprecated_functional_template::NoVueDeprecatedFunctionalTemplateOptions;

declare_lint_rule! {
    /// Disallow the deprecated `functional` attribute on the `<template>` of a Vue component.
    ///
    /// Vue 2 let you write `<template functional>` to turn a component into a
    /// functional component (a component without its own state). Vue 3 no
    /// longer supports this attribute, because functional components are no
    /// faster than regular components anymore. Remove the attribute and use a
    /// regular `<template>` instead.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/functional-components.html) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template functional>
    ///   <div>{{ props.message }}</div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div>{{ message }}</div>
    /// </template>
    /// ```
    ///
    pub NoVueDeprecatedFunctionalTemplate {
        version: "next",
        name: "noVueDeprecatedFunctionalTemplate",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-functional-template").same()],
    }
}

impl Rule for NoVueDeprecatedFunctionalTemplate {
    type Query = Ast<HtmlRoot>;
    type State = AnyHtmlAttribute;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedFunctionalTemplateOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        // Only the first top-level `<template>` is the component's template.
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

        template.find_attribute_by_name("functional")
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "The "<Emphasis>"functional"</Emphasis>" attribute on "<Emphasis>"<template>"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 no longer supports functional templates, because functional components are no faster than regular components anymore."
            })
            .note(markup! {
                "Remove the "<Emphasis>"functional"</Emphasis>" attribute and use a regular "<Emphasis>"<template>"</Emphasis>"."
            }),
        )
    }
}

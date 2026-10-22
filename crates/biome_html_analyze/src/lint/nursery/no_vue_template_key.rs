use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_template_key::NoVueTemplateKeyOptions;
use biome_vue_semantic::{Attr, DirectiveKind};

declare_lint_rule! {
    /// Disallow the `key` attribute on `<template>` elements in Vue.
    ///
    /// A `<template>` element groups other elements without adding anything to the page.
    /// Vue uses `key` to tell elements apart when it updates the page, but a `<template>`
    /// isn't an element on the page, so a `key` written on it does nothing.
    /// Put the `key` on the elements inside the `<template>` instead.
    ///
    /// A `key` on a `<template>` that also has `v-for` is allowed, because Vue uses that `key`
    /// to keep track of each item in the list.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <template key="foo"><div /></template>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <template :key="item.id"><div /></template>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div key="foo"></div>
    ///   <template v-for="item in list" :key="item.id"><div /></template>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `key` attribute](https://vuejs.org/api/built-in-special-attributes.html#key)
    pub NoVueTemplateKey {
        version: "next",
        name: "noVueTemplateKey",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-template-key").same()],
    }
}

impl Rule for NoVueTemplateKey {
    type Query = VueSemantic<HtmlRoot>;
    /// The range of the `key` attribute.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueTemplateKeyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.model()
            .elements()
            .filter(|element| {
                element.tag() == "template"
                    && !element.is_component()
                    && element.directive(DirectiveKind::For).is_none()
            })
            .filter_map(|element| element.attrs().find(is_key_attribute))
            .map(|attr| attr.range())
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"key"</Emphasis>" attribute isn't allowed on a "<Emphasis>"<template>"</Emphasis>" element."
                },
            )
            .note(markup! {
                "A "<Emphasis>"<template>"</Emphasis>" isn't an element on the page, so Vue can't use its "<Emphasis>"key"</Emphasis>" to tell it apart from other elements."
            })
            .note(markup! {
                "Put the "<Emphasis>"key"</Emphasis>" on the elements inside the "<Emphasis>"<template>"</Emphasis>" instead."
            }),
        )
    }
}

/// Returns `true` for `key`, `:key` and `v-bind:key`.
fn is_key_attribute(attr: &Attr) -> bool {
    attr.plain_name() == Some("key")
        || (attr.directive_kind() == Some(DirectiveKind::Bind) && attr.static_arg() == Some("key"))
}

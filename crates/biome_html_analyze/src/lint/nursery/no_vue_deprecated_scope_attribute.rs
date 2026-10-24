use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{HtmlAttribute, HtmlAttributeList, T, element_ext::AnyHtmlTagElement};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_deprecated_scope_attribute::NoVueDeprecatedScopeAttributeOptions;

declare_lint_rule! {
    /// Disallow the `scope` attribute on `<template>` elements in Vue templates.
    ///
    /// Old versions of Vue used the `scope` attribute to receive the values that a component
    /// passes to its slot, also called slot props.
    /// Current versions of Vue don't support this attribute: they ignore it, so the names it
    /// declares are never defined inside the `<template>`.
    ///
    /// Use the `v-slot` directive, or its `#` shorthand, to receive slot props instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <ListComponent>
    ///     <template scope="props">
    ///       {{ props.title }}
    ///     </template>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <ListComponent>
    ///     <template v-slot="props">
    ///       {{ props.title }}
    ///     </template>
    ///   </ListComponent>
    ///   <ListComponent>
    ///     <template #item="{ title }">
    ///       {{ title }}
    ///     </template>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue scoped slots](https://vuejs.org/guide/components/slots.html#scoped-slots)
    pub NoVueDeprecatedScopeAttribute {
        version: "next",
        name: "noVueDeprecatedScopeAttribute",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-scope-attribute").same()],
    }
}

impl Rule for NoVueDeprecatedScopeAttribute {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedScopeAttributeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }
        let attribute = ctx.query();
        let name = attribute.name().ok()?.value_token().ok()?;
        if name.text_trimmed() != "scope" {
            return None;
        }
        let element = attribute
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        (element.tag_name_kind() == Some(T![template])).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().name().ok()?.range(),
                markup! {
                    "Unexpected "<Emphasis>"scope"</Emphasis>" attribute on a "<Emphasis>"<template>"</Emphasis>" element."
                },
            )
            .note(markup! {
                "Vue no longer supports the "<Emphasis>"scope"</Emphasis>" attribute, so the slot props it names are never defined."
            })
            .note(markup! {
                "Use the "<Emphasis>"v-slot"</Emphasis>" directive instead, like "<Emphasis>"<template v-slot=\"props\">"</Emphasis>"."
            }),
        )
    }
}

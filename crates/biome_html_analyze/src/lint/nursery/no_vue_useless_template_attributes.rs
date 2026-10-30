use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rule_options::no_vue_useless_template_attributes::NoVueUselessTemplateAttributesOptions;
use biome_vue_semantic::{Attr, DirectiveKind};

declare_lint_rule! {
    /// Disallow attributes and directives that have no effect on `<template>` elements.
    ///
    /// Inside a Vue template, a `<template>` element with `v-if`, `v-else-if`, `v-else`, `v-for`,
    /// or `v-slot` (or its `#` shorthand) is only a wrapper: Vue renders its content, but not the
    /// `<template>` element itself. Any other attribute or directive on it, such as `class`,
    /// `ref`, `:title`, or `@click`, has nowhere to go, so Vue ignores it.
    ///
    /// Move the attribute or directive to an element inside the `<template>`, or remove it.
    ///
    /// This rule doesn't report:
    /// - `key`, `:key`, or `v-bind:key`, which Vue uses to tell the rendered items apart.
    /// - `v-memo` on a `<template>` that has `v-for`, which Vue uses to skip updating unchanged items.
    /// - `v-once` on a `<template>` that has `v-if`, `v-else-if`, `v-else`, or `v-for`, which makes
    ///   Vue render the content only once.
    /// - Any attribute on a `<template>` that has none of the directives listed above. Vue keeps
    ///   such an element as a regular HTML `<template>` element, so its attributes aren't ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <template v-if="visible" class="heading">Title</template>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <template v-for="item in items" :title="item.name">{{ item.name }}</template>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyList>
    ///     <template #item="{ name }" @click="select">{{ name }}</template>
    ///   </MyList>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <template v-if="visible">
    ///     <h1 class="heading">Title</h1>
    ///   </template>
    ///   <template v-else-if="loading">Loading</template>
    ///   <template v-else v-once>Hidden</template>
    ///   <template v-for="item in items" :key="item.id" v-memo="[item.name]">
    ///     <span :title="item.name">{{ item.name }}</span>
    ///   </template>
    ///   <MyList>
    ///     <template #item="{ name }">{{ name }}</template>
    ///   </MyList>
    ///   <template class="heading" ref="content">Content</template>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `<template>` element](https://vuejs.org/api/built-in-special-elements.html#template)
    pub NoVueUselessTemplateAttributes {
        version: "next",
        name: "noVueUselessTemplateAttributes",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-useless-template-attributes").same()],
    }
}

impl Rule for NoVueUselessTemplateAttributes {
    type Query = VueSemantic<HtmlRoot>;
    /// An attribute or directive that Vue ignores on a `<template>` element.
    type State = Attr;
    type Signals = Box<[Self::State]>;
    type Options = NoVueUselessTemplateAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals = Vec::new();
        // The top-level `<template>` block has no parent and is never a wrapper.
        for element in ctx
            .model()
            .elements()
            .filter(|element| element.tag() == "template" && element.parent().is_some())
        {
            let mut has_slot = false;
            let mut has_for = false;
            let mut has_condition = false;
            for kind in element.attrs().filter_map(|attr| attr.directive_kind()) {
                match kind {
                    DirectiveKind::Slot => has_slot = true,
                    DirectiveKind::For => has_for = true,
                    DirectiveKind::If | DirectiveKind::ElseIf | DirectiveKind::Else => {
                        has_condition = true;
                    }
                    _ => {}
                }
            }
            if !(has_slot || has_for || has_condition) {
                continue;
            }
            signals.extend(element.attrs().filter(|attr| {
                if is_wrapper_directive(attr) || is_key(attr) {
                    return false;
                }
                match attr.directive_kind() {
                    Some(DirectiveKind::Memo) => !has_for,
                    // Vue caches the whole conditional or list block, so it renders only once.
                    Some(DirectiveKind::Once) => !(has_for || has_condition),
                    _ => true,
                }
            }));
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                attr.range(),
                if attr.is_directive() {
                    markup! {
                        "This directive has no effect on a "<Emphasis>"<template>"</Emphasis>" element."
                    }
                } else {
                    markup! {
                        "This attribute has no effect on a "<Emphasis>"<template>"</Emphasis>" element."
                    }
                },
            )
            .note(markup! {
                "Vue doesn't render a "<Emphasis>"<template>"</Emphasis>" element that has "<Emphasis>"v-if"</Emphasis>", "<Emphasis>"v-else-if"</Emphasis>", "<Emphasis>"v-else"</Emphasis>", "<Emphasis>"v-for"</Emphasis>", or "<Emphasis>"v-slot"</Emphasis>", only its content, so it ignores this."
            })
            .note(markup! {
                "Move it to an element inside the "<Emphasis>"<template>"</Emphasis>", or remove it."
            }),
        )
    }
}

/// Returns `true` for the directives that make Vue render the content of a `<template>`
/// element without the element itself.
fn is_wrapper_directive(attr: &Attr) -> bool {
    matches!(
        attr.directive_kind(),
        Some(
            DirectiveKind::If
                | DirectiveKind::ElseIf
                | DirectiveKind::Else
                | DirectiveKind::For
                | DirectiveKind::Slot
        )
    )
}

/// Returns `true` for a plain `key` attribute and for `v-bind:key` or `:key`.
fn is_key(attr: &Attr) -> bool {
    match attr.directive_kind() {
        None => attr.plain_name() == Some("key"),
        Some(DirectiveKind::Bind) => attr.static_arg() == Some("key"),
        Some(_) => false,
    }
}

use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_v_for_template_key_on_child::NoVueVForTemplateKeyOnChildOptions;
use biome_vue_semantic::{Attr, DirectiveKind, Element, ScopeKind, SymbolKind};

declare_lint_rule! {
    /// Disallow placing the `key` of a `<template v-for>` on its child elements.
    ///
    /// When a `<template>` tag repeats its content with `v-for`, Vue expects the `key`
    /// on the `<template>` tag itself. A `key` on a child element that uses the loop
    /// variables belongs on the `<template>` tag instead.
    ///
    /// Child elements that have their own `v-if`, `v-else-if`, `v-else`, or `v-for`
    /// are not reported, and neither are keys that don't use any variable from the
    /// `<template>`'s `v-for`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <template v-for="todo in todos">
    ///     <TodoItem :key="todo.id" />
    ///   </template>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <template v-for="todo in todos" :key="todo.id">
    ///     <TodoItem />
    ///   </template>
    ///   <template v-for="todo in todos" :key="todo.id">
    ///     <TodoItem v-if="todo.visible" :key="todo.id" />
    ///   </template>
    /// </template>
    /// ```
    ///
    pub NoVueVForTemplateKeyOnChild {
        version: "2.6.5",
        name: "noVueVForTemplateKeyOnChild",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-v-for-template-key-on-child").same()],
    }
}

impl Rule for NoVueVForTemplateKeyOnChild {
    type Query = VueSemantic<HtmlRoot>;
    /// The range of a misplaced `key`.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueVForTemplateKeyOnChildOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.model()
            .elements()
            .filter_map(|child| misplaced_key(&child))
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The key of this "<Emphasis>"<template v-for>"</Emphasis>" is placed on a child element."
                },
            )
            .note(markup! {
                "Vue tracks each repeated block by the key of the "<Emphasis>"<template>"</Emphasis>" tag, not by the keys of its children."
            })
            .note(markup! {
                "Move this key to the "<Emphasis>"<template v-for>"</Emphasis>" tag."
            }),
        )
    }
}

/// Returns the range of the `key` of `child` when it belongs on the
/// `<template v-for>` that directly contains `child`.
fn misplaced_key(child: &Element) -> Option<TextRange> {
    let key = find_key(child)?;
    if CONTROL_FLOW_DIRECTIVES
        .iter()
        .any(|kind| child.directive(*kind).is_some())
    {
        return None;
    }

    let template = child.parent()?;
    if !template.tag().eq_ignore_ascii_case("template")
        || template.directive(DirectiveKind::For).is_none()
    {
        return None;
    }

    if find_key(&template).is_some_and(|template_key| uses_v_for_variable(&template_key, &template))
    {
        return None;
    }

    uses_v_for_variable(&key, &template).then(|| key.range())
}

/// Returns the `:key` or `v-bind:key` directive of `element`.
fn find_key(element: &Element) -> Option<Attr> {
    element.attrs().find(|attr| {
        attr.directive_kind() == Some(DirectiveKind::Bind) && attr.static_arg() == Some("key")
    })
}

/// Returns `true` if the value of `key` uses a variable that the `v-for` of
/// `element` declares.
fn uses_v_for_variable(key: &Attr, element: &Element) -> bool {
    key.value_snippet().is_some_and(|snippet| {
        snippet.references().any(|reference| {
            reference.symbol().is_some_and(|symbol| {
                symbol.kind() == SymbolKind::VForAlias
                    && symbol.scope().kind() == ScopeKind::Element(element.id())
            })
        })
    })
}

const CONTROL_FLOW_DIRECTIVES: [DirectiveKind; 4] = [
    DirectiveKind::If,
    DirectiveKind::ElseIf,
    DirectiveKind::Else,
    DirectiveKind::For,
];

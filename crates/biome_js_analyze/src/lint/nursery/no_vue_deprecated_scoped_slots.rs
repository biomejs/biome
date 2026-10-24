use crate::JsRuleAction;
use crate::frameworks::vue::vue_instance::{
    is_in_vue_component_options, is_this_or_direct_const_alias,
};
use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsName, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxToken,
};
use biome_languages::JsFileSource;
use biome_rowan::{BatchMutationExt, declare_node_union};
use biome_rule_options::no_vue_deprecated_scoped_slots::NoVueDeprecatedScopedSlotsOptions;

declare_lint_rule! {
    /// Disallow the deprecated Vue `$scopedSlots` API.
    ///
    /// Vue 3 unifies normal and scoped slots under `$slots`. Replace `$scopedSlots` with `$slots` when migrating a component from Vue 2.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/slots-unification.html) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   render() {
    ///     return this.$scopedSlots.default;
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   render() {
    ///     return this.$slots.default;
    ///   }
    /// };
    /// </script>
    /// ```
    pub NoVueDeprecatedScopedSlots {
        version: "2.5.12",
        name: "noVueDeprecatedScopedSlots",
        language: "js",
        sources: &[RuleSource::EslintVueJs("no-deprecated-dollar-scopedslots-api").same()],
        recommended: true,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Vue],
    }
}

declare_node_union! {
    pub AnyScopedSlotsReference = AnyJsIdentifierReference | JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Rule for NoVueDeprecatedScopedSlots {
    type Query = Semantic<AnyScopedSlotsReference>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedScopedSlotsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyScopedSlotsReference::AnyJsIdentifierReference(reference) => {
                template_reference(ctx, reference)
            }
            AnyScopedSlotsReference::JsStaticMemberExpression(member) => {
                component_member_reference(ctx, member.object().ok()?, &member.member().ok()?)
            }
            AnyScopedSlotsReference::JsStaticMemberAssignment(member) => {
                component_member_reference(ctx, member.object().ok()?, &member.member().ok()?)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, token: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                token.text_trimmed_range(),
                markup! {
                    "The Vue "<Emphasis>"$scopedSlots"</Emphasis>" API is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 exposes all slots through "<Emphasis>"$slots"</Emphasis>". See the "<Hyperlink href="https://v3-migration.vuejs.org/breaking-changes/slots-unification.html">"Vue 3 migration guide"</Hyperlink>" for more information."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, token: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), make::ident("$slots"));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>"$scopedSlots"</Emphasis>" with "<Emphasis>"$slots"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Handle usages in embedded snippets (the `<template>` section)
fn template_reference(
    ctx: &RuleContext<NoVueDeprecatedScopedSlots>,
    reference: &AnyJsIdentifierReference,
) -> Option<JsSyntaxToken> {
    let source = ctx.source_type::<JsFileSource>();
    if !source.as_embedding_kind().is_vue() || source.is_embedded_source() {
        return None;
    }

    let token = reference.value_token().ok()?;
    if token.text_trimmed() != "$scopedSlots"
        || !ctx.model().is_unresolved_reference(reference)
    {
        return None;
    }

    let embedded = ctx.get_service::<EmbeddedService>()?;
    if embedded.contains_binding(token.token_text_trimmed()) {
        return None;
    }

    Some(token)
}

fn component_member_reference(
    ctx: &RuleContext<NoVueDeprecatedScopedSlots>,
    object: AnyJsExpression,
    member: &AnyJsName,
) -> Option<JsSyntaxToken> {
    let token = member.as_js_name()?.value_token().ok()?;
    if token.text_trimmed() != "$scopedSlots"
        || !is_this_or_direct_const_alias(object, ctx.model())
        || !is_in_vue_component_options(
            member,
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        )
    {
        return None;
    }

    Some(token)
}

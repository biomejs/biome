use crate::frameworks::vue::vue_call::is_vue_api_reference;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsStaticMemberAssignment, JsStaticMemberExpression,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_vue_deprecated_config_key_codes::NoVueDeprecatedConfigKeyCodesOptions;

declare_lint_rule! {
    /// Disallow the deprecated `Vue.config.keyCodes` option.
    ///
    /// In Vue 2, `Vue.config.keyCodes` defined custom names for keyboard keys, so that templates could use modifiers such as `@keyup.f1`.
    /// Vue 3 removed this option, because it relies on the deprecated `KeyboardEvent.keyCode` browser property.
    /// Setting or reading it has no effect in Vue 3.
    ///
    /// Use the key name in kebab-case as the modifier instead, such as `@keyup.page-down` for the `PageDown` key.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/keycode-modifiers.html) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Vue.config.keyCodes = {
    ///   f1: 112,
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import Vue from "vue";
    ///
    /// const keyCodes = Vue.config.keyCodes;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Vue.config.silent = true;
    /// config.keyCodes = {};
    /// ```
    ///
    pub NoVueDeprecatedConfigKeyCodes {
        version: "2.6.5",
        name: "noVueDeprecatedConfigKeyCodes",
        language: "js",
        sources: &[RuleSource::EslintVueJs("no-deprecated-vue-config-keycodes").same()],
        recommended: true,
        domains: &[RuleDomain::Vue],
    }
}

declare_node_union! {
    pub AnyStaticMemberAccess = JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Rule for NoVueDeprecatedConfigKeyCodes {
    type Query = Semantic<AnyStaticMemberAccess>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedConfigKeyCodesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let (object, member) = match ctx.query() {
            AnyStaticMemberAccess::JsStaticMemberExpression(member) => {
                (member.object().ok()?, member.member().ok()?)
            }
            AnyStaticMemberAccess::JsStaticMemberAssignment(member) => {
                (member.object().ok()?, member.member().ok()?)
            }
        };

        let member = member.as_js_name()?.value_token().ok()?;
        (member.text_trimmed() == "keyCodes"
            && (is_named_vue_config(&object)
                || is_vue_api_reference(&object, ctx.model(), "config")))
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"Vue.config.keyCodes"</Emphasis>" option is deprecated and has no effect in Vue 3."
                },
            )
            .note(markup! {
                "Vue 3 no longer supports custom key names, because they rely on the deprecated "<Emphasis>"KeyboardEvent.keyCode"</Emphasis>" browser property."
            })
            .note(markup! {
                "Remove this option, and use the key name in kebab-case as the modifier instead, such as "<Emphasis>"@keyup.page-down"</Emphasis>". See the "<Hyperlink href="https://v3-migration.vuejs.org/breaking-changes/keycode-modifiers.html">"Vue 3 migration guide"</Hyperlink>" for more information."
            }),
        )
    }
}

/// Returns whether `object` is `config` accessed on a variable named `Vue`, such as `Vue.config`.
///
/// The variable doesn't have to be the global `Vue` object: Vue 2 plugins receive the Vue
/// constructor as the `Vue` parameter of their `install` function, and CommonJS files bind it
/// with `require("vue")`.
fn is_named_vue_config(object: &AnyJsExpression) -> bool {
    object
        .inner_expression()
        .and_then(|object| AnyJsMemberExpression::cast(object.into_syntax()))
        .is_some_and(|config| {
            config
                .member_name()
                .is_some_and(|name| name.text() == "config")
                && config.object().is_ok_and(|object| {
                    object
                        .omit_parentheses()
                        .as_js_reference_identifier()
                        .is_some_and(|reference| reference.has_name("Vue"))
                })
        })
}

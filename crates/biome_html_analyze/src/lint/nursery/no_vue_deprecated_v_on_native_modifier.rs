use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyVueDirective, VueModifier, VueModifierList};
use biome_rowan::AstNode;
use biome_rule_options::no_vue_deprecated_v_on_native_modifier::NoVueDeprecatedVOnNativeModifierOptions;

declare_lint_rule! {
    /// Disallow the `.native` modifier on Vue's `v-on` directive.
    ///
    /// In Vue 2, the `.native` modifier made `v-on` listen to an event on the
    /// root element of a component, instead of an event sent by the component itself.
    /// Vue 3 removed this modifier.
    /// A listener for an event that the component does not declare in its `emits`
    /// option is now added to the component's root element automatically,
    /// so the modifier is no longer needed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <MyComponent v-on:click.native="onClick" />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <MyComponent @click.native="onClick" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <MyComponent v-on:click="onClick" />
    /// <MyComponent @click="onClick" />
    /// <input @keyup.enter="submit" />
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue 3 migration guide: `v-on.native` modifier removed](https://v3-migration.vuejs.org/breaking-changes/v-on-native-modifier-removed.html)
    pub NoVueDeprecatedVOnNativeModifier {
        version: "next",
        name: "noVueDeprecatedVOnNativeModifier",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-v-on-native-modifier").same()],
    }
}

impl Rule for NoVueDeprecatedVOnNativeModifier {
    type Query = Ast<VueModifier>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedVOnNativeModifierOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let modifier = ctx.query();
        if modifier.modifier_token().ok()?.text_trimmed() != "native" {
            return None;
        }

        let directive = modifier
            .parent::<VueModifierList>()?
            .parent::<AnyVueDirective>()?;
        let is_v_on = match directive {
            AnyVueDirective::VueDirective(directive) => directive.is_event_listener(),
            AnyVueDirective::VueVOnShorthandDirective(_) => true,
            _ => false,
        };
        is_v_on.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>".native"</Emphasis>" modifier on "<Emphasis>"v-on"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 removed this modifier. Listeners for events that a component does not declare in its "<Emphasis>"emits"</Emphasis>" option are added to its root element automatically."
            })
            .note(markup! {
                "Remove the "<Emphasis>".native"</Emphasis>" modifier."
            }),
        )
    }
}

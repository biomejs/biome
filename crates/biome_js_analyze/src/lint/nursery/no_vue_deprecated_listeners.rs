use crate::frameworks::vue::vue_instance::{
    component_instance_property_member, template_instance_property_reference,
};
use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsIdentifierReference, JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxToken,
};
use biome_languages::JsFileSource;
use biome_rowan::declare_node_union;
use biome_rule_options::no_vue_deprecated_listeners::NoVueDeprecatedListenersOptions;

declare_lint_rule! {
    /// Disallow the deprecated Vue `$listeners` API.
    ///
    /// Vue 3 removed `$listeners`. Event listeners passed to a component are now part of `$attrs`,
    /// so code that still reads `$listeners` no longer receives them. Use `$attrs` instead when
    /// migrating a component from Vue 2.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/listeners-removed.html) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   mounted() {
    ///     console.log(this.$listeners);
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
    ///   mounted() {
    ///     console.log(this.$attrs);
    ///   }
    /// };
    /// </script>
    /// ```
    pub NoVueDeprecatedListeners {
        version: "next",
        name: "noVueDeprecatedListeners",
        language: "js",
        sources: &[RuleSource::EslintVueJs("no-deprecated-dollar-listeners-api").same()],
        recommended: true,
        domains: &[RuleDomain::Vue],
    }
}

declare_node_union! {
    pub AnyListenersReference = AnyJsIdentifierReference | JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Rule for NoVueDeprecatedListeners {
    type Query = Semantic<AnyListenersReference>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedListenersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let source = ctx.source_type::<JsFileSource>();
        let (object, member) = match ctx.query() {
            AnyListenersReference::AnyJsIdentifierReference(reference) => {
                return template_instance_property_reference(
                    reference,
                    "$listeners",
                    model,
                    source,
                    ctx.get_service::<EmbeddedService>(),
                );
            }
            AnyListenersReference::JsStaticMemberExpression(member) => {
                (member.object().ok()?, member.member().ok()?)
            }
            AnyListenersReference::JsStaticMemberAssignment(member) => {
                (member.object().ok()?, member.member().ok()?)
            }
        };
        component_instance_property_member(
            object,
            member.as_js_name()?,
            "$listeners",
            model,
            source,
            ctx.file_path(),
        )
    }

    fn diagnostic(_ctx: &RuleContext<Self>, token: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                token.text_trimmed_range(),
                markup! {
                    "The Vue "<Emphasis>"$listeners"</Emphasis>" API is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 removed "<Emphasis>"$listeners"</Emphasis>" and passes event listeners to the component through "<Emphasis>"$attrs"</Emphasis>" instead."
            })
            .note(markup! {
                "Use "<Emphasis>"$attrs"</Emphasis>". See the "<Hyperlink href="https://v3-migration.vuejs.org/breaking-changes/listeners-removed.html">"Vue 3 migration guide"</Hyperlink>" for more information."
            }),
        )
    }
}

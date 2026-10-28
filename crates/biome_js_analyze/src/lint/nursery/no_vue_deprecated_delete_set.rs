use crate::frameworks::vue::vue_call::vue_api_name;
use crate::frameworks::vue::vue_instance::{
    is_in_component_definition_object, is_this_or_direct_const_alias,
};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, JsCallExpression};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_deprecated_delete_set::NoVueDeprecatedDeleteSetOptions;

declare_lint_rule! {
    /// Disallow the `$set` and `$delete` functions that Vue 3 removed.
    ///
    /// Vue 2 could not detect when a property was added to or deleted from a reactive object, so it provided `$set` and `$delete` (also available as `Vue.set` and `Vue.delete`, or as `set` and `del` imported from `vue`) to make these changes visible.
    /// Vue 3 detects these changes on its own and no longer provides these functions, so calling them throws an error.
    ///
    /// Assign the property directly to add it, and use the `delete` operator to remove it.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/#removed-apis) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   methods: {
    ///     addName() {
    ///       this.$set(this.user, "name", "Alice");
    ///     }
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   methods: {
    ///     removeName() {
    ///       this.$delete(this.user, "name");
    ///     }
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import Vue from "vue";
    ///
    /// Vue.set(user, "name", "Alice");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { del } from "vue";
    ///
    /// del(user, "name");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   methods: {
    ///     addName() {
    ///       this.user.name = "Alice";
    ///     },
    ///     removeName() {
    ///       delete this.user.name;
    ///     }
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueDeprecatedDeleteSet {
        version: "next",
        name: "noVueDeprecatedDeleteSet",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-delete-set").same()],
    }
}

impl Rule for NoVueDeprecatedDeleteSet {
    type Query = Semantic<JsCallExpression>;
    type State = RemovedApi;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedDeleteSetOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = ctx.query().callee().ok()?;
        component_instance_api(ctx, &callee).or_else(|| vue_global_api(ctx, &callee))
    }

    fn diagnostic(ctx: &RuleContext<Self>, api: &Self::State) -> Option<RuleDiagnostic> {
        let callee = ctx.query().callee().ok()?;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            callee.range(),
            markup! {
                "This calls the Vue "<Emphasis>{api}</Emphasis>" function, which Vue 3 removed."
            },
        )
        .note(markup! {
            "Vue 3 detects added and deleted properties on its own, so this function no longer exists and calling it throws an error."
        });

        let diagnostic = match api {
            RemovedApi::Set => diagnostic.note(markup! {
                "Assign the property directly instead, for example "<Emphasis>"object.key = value"</Emphasis>"."
            }),
            RemovedApi::Delete => diagnostic.note(markup! {
                "Use the "<Emphasis>"delete"</Emphasis>" operator instead, for example "<Emphasis>"delete object.key"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }
}

/// The Vue 2 function that a call uses.
#[derive(Clone, Copy, Debug)]
pub enum RemovedApi {
    /// `$set`, `Vue.set`, or `set` imported from `vue`.
    Set,
    /// `$delete`, `Vue.delete`, or `del` imported from `vue`.
    Delete,
}

impl biome_console::fmt::Display for RemovedApi {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Set => "set",
            Self::Delete => "delete",
        })
    }
}

/// Matches `this.$set(...)` and `this.$delete(...)` inside the options of a
/// component, also when `this` is first stored in a `const` variable.
fn component_instance_api(
    ctx: &RuleContext<NoVueDeprecatedDeleteSet>,
    callee: &AnyJsExpression,
) -> Option<RemovedApi> {
    let member = AnyJsMemberExpression::cast(callee.inner_expression()?.into_syntax())?;
    let api = match member.member_name()?.text() {
        "$set" => RemovedApi::Set,
        "$delete" => RemovedApi::Delete,
        _ => return None,
    };
    let object = member.object().ok()?;
    let is_component_instance = is_this_or_direct_const_alias(object.clone(), ctx.model())
        && is_in_component_definition_object(
            &object,
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        );
    is_component_instance.then_some(api)
}

/// Matches `Vue.set(...)`, `Vue.delete(...)`, and calls of `set` and `del`
/// imported from `vue`.
fn vue_global_api(
    ctx: &RuleContext<NoVueDeprecatedDeleteSet>,
    callee: &AnyJsExpression,
) -> Option<RemovedApi> {
    match vue_api_name(callee, ctx.model())?.text() {
        "set" => Some(RemovedApi::Set),
        "del" | "delete" => Some(RemovedApi::Delete),
        _ => None,
    }
}

use crate::services::vue::VueSemantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rule_options::no_vue_deprecated_router_link_tag_prop::NoVueDeprecatedRouterLinkTagPropOptions;
use biome_vue_semantic::{Attr, DirectiveKind};

declare_lint_rule! {
    /// Disallow the `tag` prop on `RouterLink`.
    ///
    /// Vue Router 4, the version of Vue Router for Vue 3, doesn't support the `tag` prop on
    /// `<RouterLink>`. Writing `<RouterLink tag="li">` doesn't render an `<li>` element: the
    /// prop is ignored, and the link still renders as an `<a>` element.
    ///
    /// To render a different element, add the `custom` prop and put your own element in the
    /// link's default slot. The slot gives you a `navigate` function to call when the element
    /// is clicked:
    ///
    /// ```vue
    /// <template>
    ///   <RouterLink to="/" custom v-slot="{ navigate }">
    ///     <li role="link" @click="navigate">Home</li>
    ///   </RouterLink>
    /// </template>
    /// ```
    ///
    /// This rule checks `<RouterLink>` and `<router-link>`. Use the `components` option to
    /// check other link components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <RouterLink tag="li" to="/">Home</RouterLink>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <router-link :tag="element" to="/">Home</router-link>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <RouterLink to="/">Home</RouterLink>
    ///   <router-link to="/">
    ///     <div>Home</div>
    ///   </router-link>
    ///   <NuxtLink tag="li" to="/">Home</NuxtLink>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `components`
    ///
    /// Default: `["RouterLink"]`
    ///
    /// The names of the link components to check. Each name matches both its PascalCase and
    /// kebab-case spelling, so `"NuxtLink"` matches `<NuxtLink>` and `<nuxt-link>`. Setting
    /// this option replaces the default list, so include `"RouterLink"` to keep checking it.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "components": ["RouterLink", "NuxtLink"]
    ///     }
    /// }
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <nuxt-link tag="li" to="/">Home</nuxt-link>
    /// </template>
    /// ```
    ///
    /// ```vue,use_options
    /// <template>
    ///   <NuxtLink to="/">Home</NuxtLink>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue Router migration guide: removal of the `tag` prop](https://router.vuejs.org/guide/migration/#Removal-of-event-and-tag-props-in-router-link-)
    /// - [Extending `RouterLink`](https://router.vuejs.org/guide/advanced/extending-router-link.html)
    pub NoVueDeprecatedRouterLinkTagProp {
        version: "next",
        name: "noVueDeprecatedRouterLinkTagProp",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-router-link-tag-prop").same()],
    }
}

impl Rule for NoVueDeprecatedRouterLinkTagProp {
    type Query = VueSemantic<HtmlRoot>;
    /// The `tag` attribute or `v-bind:tag` directive of a checked link component.
    type State = Attr;
    type Signals = Box<[Self::State]>;
    type Options = NoVueDeprecatedRouterLinkTagPropOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let names: Vec<_> = ctx
            .options()
            .components()
            .flat_map(|name| [to_pascal_case(name), to_kebab_case(name)])
            .collect();

        ctx.model()
            .elements()
            .filter(|element| names.iter().any(|name| name == element.tag()))
            .filter_map(|element| element.attrs().find(is_tag_prop))
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        let element = attr.element();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                attr.range(),
                markup! {
                    "The "<Emphasis>"tag"</Emphasis>" prop is used on "<Emphasis>{element.tag()}</Emphasis>"."
                },
            )
            .note(markup! {
                "Vue Router 4 doesn't support this prop, so it's ignored and the link still renders as an "<Emphasis>"<a>"</Emphasis>" element."
            })
            .note(markup! {
                "To render a different element, add the "<Emphasis>"custom"</Emphasis>" prop and put the element in the link's default slot."
            }),
        )
    }
}

/// Returns `true` for a plain `tag` attribute and for `v-bind:tag` or `:tag`.
fn is_tag_prop(attr: &Attr) -> bool {
    match attr.directive_kind() {
        None => attr.plain_name() == Some("tag"),
        Some(DirectiveKind::Bind) => attr.static_arg() == Some("tag"),
        Some(_) => false,
    }
}

// `to_pascal_case` and `to_kebab_case` follow `casing.pascalCase` and `casing.kebabCase` in
// eslint-plugin-vue rather than `biome_string_case::Case`. `Case` lowercases letters inside a
// word, so `UILink` would become `UiLink`, which Vue doesn't resolve to the `UILink` component.

/// Removes `-` and `_` and uppercases the letter after each, and the first letter.
/// For example, `router-link` becomes `RouterLink` and `UILink` stays `UILink`.
fn to_pascal_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut uppercase_next = true;
    for c in name.chars() {
        if matches!(c, '-' | '_') {
            uppercase_next = true;
        } else if uppercase_next {
            result.extend(c.to_uppercase());
            uppercase_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Lowercases `name`, replaces `_` with `-`, and adds `-` before each uppercase letter that
/// follows a letter or digit.
/// For example, `RouterLink` becomes `router-link` and `UILink` becomes `u-i-link`.
fn to_kebab_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    let mut previous_is_alphanumeric = false;
    for c in name.chars() {
        if c == '_' {
            result.push('-');
        } else {
            if c.is_uppercase() && previous_is_alphanumeric {
                result.push('-');
            }
            result.extend(c.to_lowercase());
        }
        previous_is_alphanumeric = c.is_alphanumeric();
    }
    result
}

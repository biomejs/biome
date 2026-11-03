use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttribute, HtmlElement, HtmlElementList,
    element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_deprecated_slot_attribute::NoVueDeprecatedSlotAttributeOptions;

declare_lint_rule! {
    /// Disallow the deprecated `slot` attribute in Vue templates.
    ///
    /// Vue 3 removed the `slot` attribute, which Vue 2 used to send content to a
    /// named slot of a component. Vue 3 treats it as a regular attribute, so the
    /// content ends up in the component's default slot instead of the named slot.
    ///
    /// Use the `v-slot` directive on a `<template>` element instead, for example
    /// `<template v-slot:header>` or its shorter form `<template #header>`.
    ///
    /// The rule checks `slot="..."`, `:slot="..."`, and `v-bind:slot="..."`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <template slot="header">Title</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <div slot="header">Title</div>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <template :slot="slotName">Title</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <MyComponent>
    ///     <template v-slot:header>Title</template>
    ///   </MyComponent>
    ///   <MyComponent>
    ///     <template #header>Title</template>
    ///   </MyComponent>
    ///   <MyComponent>
    ///     <template #[slotName]>Title</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignore`
    ///
    /// Type: `string[]`
    ///
    /// Default: `[]`
    ///
    /// Names of elements that can use the `slot` attribute. Case and hyphens are
    /// ignored, so `my-component` also matches `MyComponent`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": ["my-component"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <ListComponent>
    ///     <MyComponent slot="header">Title</MyComponent>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <ListComponent>
    ///     <OtherComponent slot="header">Title</OtherComponent>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// ### `ignoreParents`
    ///
    /// Type: `string[]`
    ///
    /// Default: `[]`
    ///
    /// Names of elements whose direct children can use the `slot` attribute. Case
    /// and hyphens are ignored, so `my-web-component` also matches `MyWebComponent`.
    ///
    /// This is useful for [web components](https://developer.mozilla.org/en-US/docs/Web/API/Web_components),
    /// where `slot` is a standard HTML attribute that places a child into a named slot
    /// of the web component.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreParents": ["my-web-component"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <my-web-component>
    ///     <span slot="header">Title</span>
    ///   </my-web-component>
    /// </template>
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <OtherComponent>
    ///     <span slot="header">Title</span>
    ///   </OtherComponent>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue named slots](https://vuejs.org/guide/components/slots.html#named-slots)
    /// - [Vue `v-slot` directive](https://vuejs.org/api/built-in-directives.html#v-slot)
    pub NoVueDeprecatedSlotAttribute {
        version: "next",
        name: "noVueDeprecatedSlotAttribute",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-slot-attribute").inspired()],
    }
}

impl Rule for NoVueDeprecatedSlotAttribute {
    type Query = Ast<AnyHtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedSlotAttributeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let attribute = ctx.query();
        // Matches `slot`, `:slot`, and `v-bind:slot`.
        if attribute.name()?.text() != "slot" {
            return None;
        }

        let element = AnyHtmlTagElement::cast(attribute.syntax().grand_parent()?)?;
        let options = ctx.options();
        if is_ignored(options.ignore(), &element) {
            return None;
        }
        if !options.ignore_parents().is_empty()
            && parent_element(&element)
                .is_some_and(|parent| is_ignored(options.ignore_parents(), &parent))
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"slot"</Emphasis>" attribute is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 treats it as a regular attribute, so the content goes to the component's default slot instead of the named slot."
            })
            .note(markup! {
                "Use the "<Emphasis>"v-slot"</Emphasis>" directive on a "<Emphasis>"<template>"</Emphasis>" element instead, for example "<Emphasis>"<template v-slot:name>"</Emphasis>" or "<Emphasis>"<template #name>"</Emphasis>"."
            }),
        )
    }
}

/// Returns the opening tag of the element that directly contains `element`.
fn parent_element(element: &AnyHtmlTagElement) -> Option<AnyHtmlTagElement> {
    let list = match element {
        AnyHtmlTagElement::HtmlOpeningElement(opening) => {
            HtmlElementList::cast(opening.syntax().grand_parent()?)?
        }
        AnyHtmlTagElement::HtmlSelfClosingElement(element) => {
            element.parent::<HtmlElementList>()?
        }
    };
    let parent = list.parent::<HtmlElement>()?.opening_element().ok()?;
    Some(parent.into())
}

/// Returns `true` if the tag name of `element` is in `names`.
fn is_ignored(names: &[Box<str>], element: &AnyHtmlTagElement) -> bool {
    if names.is_empty() {
        return false;
    }
    element
        .tag_name()
        .is_some_and(|tag_name| names.iter().any(|name| is_same_name(name, tag_name.text())))
}

/// Compares two element names, ignoring ASCII case and hyphens, so that
/// `my-component`, `MyComponent`, and `myComponent` are the same name.
fn is_same_name(a: &str, b: &str) -> bool {
    normalized_bytes(a).eq(normalized_bytes(b))
}

fn normalized_bytes(name: &str) -> impl Iterator<Item = u8> + '_ {
    name.bytes()
        .filter(|byte| *byte != b'-')
        .map(|byte| byte.to_ascii_lowercase())
}

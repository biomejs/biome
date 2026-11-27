use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyVueDirective, HtmlElement, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_vue_lone_template::NoVueLoneTemplateOptions;

declare_lint_rule! {
    /// Disallow `<template>` elements in a Vue template that don't use `v-if`, `v-else-if`, `v-else`, `v-for`, or `v-slot`.
    ///
    /// Inside a Vue template, a `<template>` element is an invisible wrapper only
    /// when it has one of these directives. Without one, Vue renders it as a
    /// regular HTML `<template>` element, and the browser doesn't show its
    /// content on the page.
    ///
    /// The `<template>` at the top level of a `.vue` file isn't checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <template>
    ///     <p>Hello</p>
    ///   </template>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <template class="greeting">
    ///     <p>Hello</p>
    ///   </template>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <template v-if="visible">
    ///     <p>Hello</p>
    ///   </template>
    ///   <template v-else>
    ///     <p>Goodbye</p>
    ///   </template>
    ///   <template v-for="item in items">
    ///     <p>{{ item }}</p>
    ///   </template>
    ///   <MyList>
    ///     <template #item>
    ///       <p>Item</p>
    ///     </template>
    ///   </MyList>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreAccessible`
    ///
    /// Default: `false`
    ///
    /// When `true`, a `<template>` without one of these directives is allowed
    /// if it has an `id` or `ref` attribute, written plainly or bound with
    /// `v-bind`. Use this when your code reads the `<template>` element on purpose,
    /// for example to copy its content.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreAccessible": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <template class="row">
    ///     <p>Cell</p>
    ///   </template>
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <template id="row">
    ///     <p>Cell</p>
    ///   </template>
    ///   <template ref="rowTemplate">
    ///     <p>Cell</p>
    ///   </template>
    /// </template>
    /// ```
    ///
    pub NoVueLoneTemplate {
        version: "2.6.5",
        name: "noVueLoneTemplate",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-lone-template").same()],
    }
}

impl Rule for NoVueLoneTemplate {
    type Query = Ast<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueLoneTemplateOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let tag = ctx.query();
        if tag.tag_name_kind()? != T![template] {
            return None;
        }

        // The top-level `<template>` of the file holds the component's markup,
        // so only `<template>` elements nested inside another element count.
        let element = match tag {
            AnyHtmlTagElement::HtmlOpeningElement(opening) => opening.syntax().parent()?,
            AnyHtmlTagElement::HtmlSelfClosingElement(element) => element.syntax().clone(),
        };
        if !element
            .ancestors()
            .skip(1)
            .any(|ancestor| HtmlElement::can_cast(ancestor.kind()))
        {
            return None;
        }

        let ignore_accessible = ctx.options().ignore_accessible();
        let is_allowed = tag.attributes().iter().any(|attribute| {
            is_template_directive(&attribute)
                || (ignore_accessible
                    && attribute.name().is_some_and(|name| {
                        // HTML attribute names ignore case, but Vue only
                        // recognizes `ref` in lowercase.
                        name.text().eq_ignore_ascii_case("id") || name.text() == "ref"
                    }))
        });
        if is_allowed {
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
                    "This "<Emphasis>"<template>"</Emphasis>" has no "<Emphasis>"v-if"</Emphasis>", "<Emphasis>"v-else-if"</Emphasis>", "<Emphasis>"v-else"</Emphasis>", "<Emphasis>"v-for"</Emphasis>", or "<Emphasis>"v-slot"</Emphasis>" directive."
                },
            )
            .note(markup! {
                "Without one of these directives, Vue renders a regular HTML "<Emphasis>"<template>"</Emphasis>" element, and the browser doesn't show its content."
            })
            .note(markup! {
                "Add the directive you meant to use, or remove the "<Emphasis>"<template>"</Emphasis>" tags and keep their content."
            }),
        )
    }
}

/// Returns `true` if `attribute` is a directive that makes Vue treat a
/// `<template>` as an invisible wrapper: `v-if`, `v-else-if`, `v-else`, `v-for`,
/// or `v-slot`, including its `#name` shorthand.
fn is_template_directive(attribute: &AnyHtmlAttribute) -> bool {
    match attribute.as_any_vue_directive() {
        Some(AnyVueDirective::VueVSlotShorthandDirective(_)) => true,
        Some(AnyVueDirective::VueDirective(directive)) => {
            directive.name_token().is_ok_and(|name| {
                matches!(
                    name.text_trimmed(),
                    "v-if" | "v-else-if" | "v-else" | "v-for" | "v-slot"
                )
            })
        }
        _ => false,
    }
}

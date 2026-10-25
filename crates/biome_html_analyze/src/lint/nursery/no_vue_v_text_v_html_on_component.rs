use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    HtmlAttributeList, HtmlSyntaxKind, VueDirective, element_ext::AnyHtmlTagElement,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::AstNode;
use biome_rule_options::no_vue_v_text_v_html_on_component::NoVueVTextVHtmlOnComponentOptions;

declare_lint_rule! {
    /// Disallow `v-text` and `v-html` on components.
    ///
    /// When `v-text` or `v-html` is used on a component, Vue doesn't put the value in the
    /// component's slot. Instead, it replaces everything inside the component's root element.
    /// Anything the component renders inside that element, such as icons, wrappers, or other
    /// components, is lost. If the component has more than one root element, Vue drops the
    /// value and shows a warning.
    ///
    /// To pass content to a component, put it between the component's opening and closing
    /// tags. To use `v-text` or `v-html`, put the directive on an HTML element inside them.
    ///
    /// This rule treats an element as a component when its name isn't a known HTML, SVG, or
    /// MathML element, for example `<MyComponent>`, `<my-component>`, or `<component>`. An
    /// element with an `is` attribute that starts with `vue:`, such as
    /// `<tr is="vue:my-row">`, is also a component.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-text="content"></MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-html="html"></MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <component :is="tag" v-text="content"></component>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <tr is="vue:my-row" v-text="content"></tr>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-text="content"></div>
    ///   <div v-html="html"></div>
    ///   <svg><text v-text="content" /></svg>
    ///   <math><mi v-text="content" /></math>
    ///   <MyComponent>{{ content }}</MyComponent>
    ///   <MyComponent><span v-html="html"></span></MyComponent>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `v-text` directive](https://vuejs.org/api/built-in-directives.html#v-text)
    /// - [Vue `v-html` directive](https://vuejs.org/api/built-in-directives.html#v-html)
    /// - [Vue fallthrough attributes](https://vuejs.org/guide/components/attrs.html)
    pub NoVueVTextVHtmlOnComponent {
        version: "2.6.3",
        name: "noVueVTextVHtmlOnComponent",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-v-text-v-html-on-component").same()],
    }
}

impl Rule for NoVueVTextVHtmlOnComponent {
    type Query = Ast<VueDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueVTextVHtmlOnComponentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        let name = directive.name_token().ok()?;
        if !matches!(name.text_trimmed(), "v-text" | "v-html") {
            return None;
        }

        let element = directive
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        is_component(&element).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        let name = directive.name_token().ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>{name}</Emphasis>" directive is used on a component."
                },
            )
            .note(markup! {
                "Vue doesn't put the value in the component's slot. It replaces everything inside the component's root element, so the component's own content is lost."
            })
            .note(markup! {
                "Put the content between the component's opening and closing tags, or move "<Emphasis>{name}</Emphasis>" to an HTML element inside them."
            }),
        )
    }
}

/// Tag names that Vue renders as components even though the lexer gives them a plain tag name.
const COMPONENT_TAG_NAMES: TokenSet<HtmlSyntaxKind> = token_set!(
    HtmlSyntaxKind::COMPONENT_KW,
    HtmlSyntaxKind::HTML_UNKNOWN_TAG
);

/// MathML element names that Vue renders as native elements. They have no tag-name keyword,
/// so they lex as `HTML_UNKNOWN_TAG` (or as a component name, for `annotation-xml`).
const MATHML_ELEMENTS: &[&str] = &[
    "annotation",
    "annotation-xml",
    "maction",
    "maligngroup",
    "malignmark",
    "math",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mfraction",
    "mglyph",
    "mi",
    "mlabeledtr",
    "mlongdiv",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mscarries",
    "mscarry",
    "msgroup",
    "msline",
    "mspace",
    "msqrt",
    "msrow",
    "mstack",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "none",
    "semantics",
];

/// Returns `true` if Vue renders `element` as a component instead of a native element.
///
/// An element is a component if its name isn't a known HTML, SVG, or MathML element, such as
/// `<MyComponent>`, `<my-element>`, or `<component>`, or if it has an `is` attribute whose
/// value starts with `vue:`, such as `<tr is="vue:my-row">`.
fn is_component(element: &AnyHtmlTagElement) -> bool {
    let Ok(name) = element.name() else {
        return false;
    };
    let is_native = match name.tag_name_kind() {
        Some(kind) if !COMPONENT_TAG_NAMES.contains(kind) => true,
        _ => name
            .token_text_trimmed()
            .is_some_and(|name| MATHML_ELEMENTS.binary_search(&name.text()).is_ok()),
    };
    !is_native || has_vue_is_attribute(element)
}

fn has_vue_is_attribute(element: &AnyHtmlTagElement) -> bool {
    element
        .find_attribute_by_name("is")
        .and_then(|attribute| {
            attribute
                .as_html_attribute()?
                .initializer()?
                .value()
                .ok()?
                .as_static_value()
        })
        .is_some_and(|value| value.text().starts_with("vue:"))
}

#[cfg(test)]
mod tests {
    use super::MATHML_ELEMENTS;

    #[test]
    fn mathml_elements_are_sorted() {
        for pair in MATHML_ELEMENTS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "`{}` should come after `{}`",
                pair[0],
                pair[1]
            );
        }
    }
}

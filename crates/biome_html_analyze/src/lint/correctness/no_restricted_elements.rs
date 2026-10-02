use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlTagName, element_ext::AnyHtmlTagElement};
use biome_rowan::AstNode;
use biome_rule_options::no_restricted_elements::NoRestrictedElementsOptions;

declare_lint_rule! {
    /// Disallow the use of configured elements.
    ///
    /// This rule disallows the use of configured elements in HTML, Vue, Svelte, and Astro files.
    /// Without elements configured, this rule doesn't do anything.
    ///
    /// This rule is useful when a project wants to enforce a component or a different
    /// element instead of certain HTML elements. For example, a project might restrict
    /// `<a>` in favor of a router link component that handles client-side navigation.
    ///
    /// Native HTML, SVG, and MathML element names are matched case-insensitively, so
    /// restricting `marquee` also reports `<MARQUEE>`.
    /// Component names, such as `<RouterLink>` in Vue, Svelte, and Astro files, are matched exactly.
    ///
    /// ## Options
    ///
    /// ### `elements`
    ///
    /// A map of element names to the message to show when the element is used.
    /// Defaults to no restricted elements.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "elements": {
    ///             "marquee": "Use CSS animations instead.",
    ///             "RouterLink": "Use NuxtLink instead."
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic,use_options
    /// <marquee>Breaking news</marquee>
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///     <RouterLink to="/">Home</RouterLink>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///     <p>Breaking news</p>
    ///     <NuxtLink to="/">Home</NuxtLink>
    /// </template>
    /// ```
    ///
    pub NoRestrictedElements {
        version: "next",
        name: "noRestrictedElements",
        language: "html",
        recommended: false,
        sources: &[RuleSource::EslintVueJs("no-restricted-html-elements").inspired()],
    }
}

impl Rule for NoRestrictedElements {
    type Query = Ast<AnyHtmlTagElement>;
    /// The index of the matching entry in the configured `elements`.
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query().name().ok()?;
        find_restricted_element(ctx.options(), &name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, index: &Self::State) -> Option<RuleDiagnostic> {
        let name = ctx.query().name().ok()?;
        let (_, message) = ctx.options().elements.as_ref()?.get_index(*index)?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name.range(),
                markup! {
                    "The "<Emphasis>{format_args!("{}", name.syntax().text_trimmed())}</Emphasis>" element is restricted in this project."
                },
            )
            .note(markup! { {message} }),
        )
    }
}

/// Returns the index of the configured entry that restricts the element `name`.
///
/// Native element names are case-insensitive, while component names must match exactly.
fn find_restricted_element(
    options: &NoRestrictedElementsOptions,
    name: &AnyHtmlTagName,
) -> Option<usize> {
    let elements = options.elements.as_ref()?;
    match name {
        AnyHtmlTagName::HtmlTagName(tag) => {
            let name = tag.value_token().ok()?;
            let name = name.text_trimmed();
            elements.get_index_of(name).or_else(|| {
                elements
                    .keys()
                    .position(|element| element.eq_ignore_ascii_case(name))
            })
        }
        AnyHtmlTagName::HtmlComponentName(component) => {
            elements.get_index_of(component.value_token().ok()?.text_trimmed())
        }
        AnyHtmlTagName::HtmlMemberName(member) => {
            let name = member.syntax().text_trimmed();
            elements.keys().position(|element| name == element.as_ref())
        }
    }
}

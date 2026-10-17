use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlTagName, T};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_vue_component_is::UseVueComponentIsOptions;

declare_lint_rule! {
    /// Require `<component>` elements to bind the `is` attribute.
    ///
    /// Vue's built-in `<component>` element renders the component passed to its
    /// `is` attribute, which lets you switch between components in one place.
    /// The value must be bound with `:is` or `v-bind:is`. Without it, Vue has
    /// no component to render.
    ///
    /// A plain `is="..."` attribute is also reported, because it always renders
    /// the same component. Use that component directly instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <component />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <component is="MyComponent" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <component :is="currentView" />
    ///   <component v-bind:is="currentView" />
    /// </template>
    /// ```
    ///
    pub UseVueComponentIs {
        version: "next",
        name: "useVueComponentIs",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-component-is").same()],
    }
}

impl Rule for UseVueComponentIs {
    type Query = Ast<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueComponentIsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let is_component_element = match element.name().ok()? {
            AnyHtmlTagName::HtmlTagName(name) => name.value_token().ok()?.kind() == T![component],
            AnyHtmlTagName::HtmlComponentName(name) => {
                name.value_token().ok()?.text_trimmed() == "Component"
            }
            AnyHtmlTagName::HtmlMemberName(_) => false,
        };
        if !is_component_element || !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        element.find_vue_binding("is").is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"<component>"</Emphasis>" element doesn't bind the "<Emphasis>"is"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Vue renders the component passed to "<Emphasis>":is"</Emphasis>". Without it, there is no component to render."
            })
            .note(markup! {
                "Bind the component with "<Emphasis>":is"</Emphasis>", or use the component directly if it never changes."
            }),
        )
    }
}

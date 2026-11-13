use crate::services::embedded::EmbeddedService;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, HtmlAttributeList, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_astro_unserializable_client_props::NoAstroUnserializableClientPropsOptions;

declare_lint_rule! {
    /// Disallow function props on client-hydrated components in Astro.
    ///
    /// Astro serializes props to send them from the server to a hydrated framework
    /// component in the browser. Functions cannot be serialized, so event handlers
    /// passed from frontmatter cannot provide browser interactivity.
    /// Define these handlers inside the framework component instead.
    ///
    /// This rule checks direct function expressions, frontmatter function declarations,
    /// and `const` variables initialized with functions. It skips reassigned functions,
    /// imported values, calls, nested object values, and elements with spread attributes.
    /// Components without a client directive can receive functions for server rendering.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// ---
    /// import Button from "./Button.svelte";
    /// const onClick = () => console.log("clicked");
    /// ---
    /// <Button client:load onClick={onClick} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// import Button from "./Button.svelte";
    /// ---
    /// <Button client:load label="Save" />
    /// ```
    ///
    /// ## References
    ///
    /// - [Passing props to framework components](https://docs.astro.build/en/guides/framework-components/#passing-props-to-framework-components)
    pub NoAstroUnserializableClientProps {
        version: "next",
        name: "noAstroUnserializableClientProps",
        language: "html",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroUnserializableClientProps {
    type Query = Ast<AnyHtmlAttribute>;
    type State = (TextRange, TextRange);
    type Signals = Option<Self::State>;
    type Options = NoAstroUnserializableClientPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }
        let attribute = ctx.query();
        let expression = match attribute {
            AnyHtmlAttribute::HtmlAttribute(attribute) => attribute
                .initializer()?
                .value()
                .ok()?
                .as_html_attribute_single_text_expression()?
                .clone(),
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(expression) => {
                expression.parent::<HtmlAttributeList>()?;
                expression.clone()
            }
            _ => return None,
        };
        let tag = attribute
            .syntax()
            .ancestors()
            .find_map(AnyHtmlTagElement::cast)?;
        if !tag.is_custom_component() {
            return None;
        }
        let mut client = None;
        for attribute in tag.attributes() {
            match attribute {
                AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroClientDirective(
                    directive,
                )) => {
                    directive.value().ok()?.name().ok()?.value_token().ok()?;
                    client = Some(directive.range());
                }
                AnyHtmlAttribute::HtmlSpreadAttribute(_)
                | AnyHtmlAttribute::HtmlBogusAttribute(_) => return None,
                _ => {}
            }
        }
        let client = client?;
        expression.r_curly_token().ok()?;
        let function = ctx
            .get_service::<EmbeddedService>()?
            .astro()?
            .function_value(expression.range())?;
        Some((function, client))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(
            ctx.query()
                .syntax()
                .ancestors()
                .find_map(AnyHtmlTagElement::cast)?
                .range(),
        )
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), state.0, markup! {
            "This function cannot be serialized as a client prop."
        }).detail(state.1, markup! { "This directive sends the component's props to the browser." })
        .note(markup! { "Functions passed from the server cannot run as browser event handlers." })
        .note(markup! { "Define the handler inside the framework component and pass serializable data instead." }))
    }
}

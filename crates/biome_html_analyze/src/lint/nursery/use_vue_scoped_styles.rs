use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, HtmlFileSource, HtmlOpeningElement, HtmlSyntaxKind, HtmlSyntaxToken,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_vue_scoped_styles::UseVueScopedStylesOptions;

declare_lint_rule! {
    /// Enforce that `<style>` blocks in Vue SFCs have the `scoped` attribute.
    ///
    /// Vue's `scoped` attribute automatically scopes CSS to the component,
    /// preventing style leakage and conflicts.
    ///
    /// Style blocks with the `module` attribute are exempt, as CSS Modules
    /// is an alternative scoping mechanism.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <style>
    /// .foo { color: red; }
    /// </style>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <style scoped>
    /// .foo { color: red; }
    /// </style>
    /// ```
    ///
    /// ```vue
    /// <style module>
    /// .foo { color: red; }
    /// </style>
    /// ```
    ///
    /// ## References:
    ///
    /// - [Vue Documentation](https://vuejs.org/api/sfc-css-features.html#scoped-css)
    pub UseVueScopedStyles {
        version: "next",
        name: "useVueScopedStyles",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("enforce-style-attribute").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueScopedStyles {
    type Query = Ast<HtmlOpeningElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueScopedStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let opening = ctx.query();

        let name = opening.name().ok()?;
        let name_text = name.token_text_trimmed()?;
        if !name_text.eq_ignore_ascii_case("style") {
            return None;
        }

        let attributes = opening.attributes();
        let has_scoped = attributes.find_by_name("scoped").is_some();
        let has_module = attributes.find_by_name("module").is_some();

        if has_scoped || has_module {
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
                    "This "<Emphasis>"<style>"</Emphasis>" block is missing the "<Emphasis>"scoped"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "In Vue, unscoped styles become global across the entire project. This can lead to unintended side effects and maintenance challenges. Adding the "<Emphasis>"scoped"</Emphasis>" attribute ensures that styles are scoped to this component, preventing style leakage and conflicts."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<crate::HtmlRuleAction> {
        let opening = ctx.query();
        let old_attributes = opening.attributes();

        let token = HtmlSyntaxToken::new_detached(HtmlSyntaxKind::HTML_LITERAL, " scoped", [], []);

        let attr = AnyHtmlAttribute::HtmlAttribute(
            make::html_attribute(make::html_attribute_name(token)).build(),
        );
        let mut items: Vec<AnyHtmlAttribute> = old_attributes.iter().collect();
        items.push(attr);
        let new_attributes = make::html_attribute_list(items);

        let mut mutation = BatchMutationExt::begin(ctx.root());
        mutation.replace_node(old_attributes, new_attributes);

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"scoped"</Emphasis>" attribute so the styles in will only apply to this component." }.to_owned(),
            mutation,
        ))
    }
}

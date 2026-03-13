use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AstroIsDirective, HtmlFileSource, HtmlOpeningElement, HtmlSyntaxKind,
    HtmlSyntaxToken,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, SyntaxNodeCast};
use biome_rule_options::use_scoped_styles::UseScopedStylesOptions;

declare_lint_rule! {
    /// Enforce that `<style>` blocks in Vue SFCs have the `scoped` attribute and that `<style>` blocks in Astro components do not have the `is:global` directive.
    ///
    /// Vue's `scoped` attribute automatically scopes CSS to the component,
    /// preventing style leakage and conflicts. Astro's `is:global` attribute
    /// allows for global styles, but without it, styles are scoped to the component by default.
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
    /// ```astro,expect_diagnostic
    /// <style is:global>
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
    /// - [Astro Documentation](https://docs.astro.build/en/guides/styling/#global-styles)
    pub UseScopedStyles {
        version: "2.4.5",
        name: "useScopedStyles",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("enforce-style-attribute").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

pub enum GlobalStylesKind {
    Vue,
    Astro { directive: AstroIsDirective },
}

impl Rule for UseScopedStyles {
    type Query = Ast<HtmlOpeningElement>;
    type State = GlobalStylesKind;
    type Signals = Option<Self::State>;
    type Options = UseScopedStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue()
            && !ctx.source_type::<HtmlFileSource>().is_astro()
        {
            return None;
        }

        let opening = ctx.query();

        let name = opening.name().ok()?;
        let name_text = name.token_text_trimmed()?;
        if !name_text.eq_ignore_ascii_case("style") {
            return None;
        }

        let attributes = opening.attributes();
        if ctx.source_type::<HtmlFileSource>().is_vue() {
            let has_scoped = attributes.find_by_name("scoped").is_some();
            let has_module = attributes.find_by_name("module").is_some();

            if has_scoped || has_module {
                return None;
            } else {
                return Some(GlobalStylesKind::Vue);
            }
        } else if ctx.source_type::<HtmlFileSource>().is_astro() {
            let is_directives = attributes
                .iter()
                .filter_map(|attr| attr.syntax().clone().cast::<AstroIsDirective>());
            for directive in is_directives {
                let name = directive.value().ok()?.name().ok()?;
                let name_text = name.token_text_trimmed()?;
                if name_text.eq_ignore_ascii_case("global") {
                    return Some(GlobalStylesKind::Astro { directive });
                }
            }
            return None;
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            GlobalStylesKind::Vue => {
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
            },
            GlobalStylesKind::Astro { directive } => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        directive.range(),
                        markup! {
                            "This "<Emphasis>"is:global"</Emphasis>" directive is making the styles in this block global."
                        },
                    )
                    .note(markup! {
                        "In Astro, styles are scoped to the component by default. The "<Emphasis>"is:global"</Emphasis>" directive allows for global styles, but it can lead to unintended side effects and maintenance challenges."
                    }),
                )
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<crate::HtmlRuleAction> {
        match state {
            GlobalStylesKind::Vue => {
                let opening = ctx.query();
                let old_attributes = opening.attributes();

                let token =
                    HtmlSyntaxToken::new_detached(HtmlSyntaxKind::HTML_LITERAL, " scoped", [], []);

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
                    markup! { "Add the "<Emphasis>"scoped"</Emphasis>" attribute so the styles will only apply to this component." }.to_owned(),
                    mutation,
                ))
            }
            GlobalStylesKind::Astro { directive } => {
                let mut mutation = BatchMutationExt::begin(ctx.root());
                mutation.remove_node(directive.clone());

                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the "<Emphasis>"is:global"</Emphasis>" directive so the styles in this block will be scoped to this component." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

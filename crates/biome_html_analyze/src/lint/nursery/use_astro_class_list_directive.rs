use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlAttribute};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_astro_class_list_directive::UseAstroClassListDirectiveOptions;

declare_lint_rule! {
    /// Promotes Astro's `class:list` directive for expression-based classes.
    ///
    /// `class:list` accepts strings, arrays, and objects and normalizes them with Astro's class-list handling.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <div class={classes}></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <div class="card"></div>
    /// <div class:list={classes}></div>
    /// ```
    pub UseAstroClassListDirective {
        version: "next",
        name: "useAstroClassListDirective",
        language: "html",
        sources: &[RuleSource::EslintAstro("prefer-class-list-directive").inspired()],
        recommended: false,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for UseAstroClassListDirective {
    type Query = Ast<HtmlAttribute>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseAstroClassListDirectiveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }

        let attribute = ctx.query();
        let name = attribute.name().ok()?.value_token().ok()?;
        if name.text_trimmed() != "class" {
            return None;
        }
        let initializer = attribute.initializer()?.value().ok()?;
        let AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) =
            initializer
        else {
            return None;
        };
        expression.expression().ok()?.html_literal_token().ok()?;

        Some(name.text_trimmed_range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _class_name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Use "<Emphasis>"class:list"</Emphasis>" for expression-based classes."
                },
            )
            .note(markup! {
                "The "<Emphasis>"class"</Emphasis>" attribute does not apply Astro's class-list normalization to arrays and objects."
            })
            .note(markup! {
                "Replace the attribute with "<Emphasis>"class:list"</Emphasis>" after verifying that the expression has the intended class-string behavior."
            }),
        )
    }
}

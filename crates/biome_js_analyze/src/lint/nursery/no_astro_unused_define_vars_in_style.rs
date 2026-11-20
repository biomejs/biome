use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::AnyJsObjectMember;
use biome_languages::JsFileSource;
use biome_module_graph::AstroStyleVariable;
use biome_rowan::AstNode;
use biome_rule_options::no_astro_unused_define_vars_in_style::NoAstroUnusedDefineVarsInStyleOptions;

declare_lint_rule! {
    /// Disallows unused variables passed to an Astro style block with `define:vars`.
    ///
    /// The rule compares statically known object keys with custom properties referenced by `var()` in the same style block.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic,ignore
    /// ---
    /// const color = "red";
    /// ---
    /// <style define:vars={{ color, unused: "blue" }}>
    /// div { color: var(--color); }
    /// </style>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,ignore
    /// ---
    /// const color = "red";
    /// ---
    /// <style define:vars={{ color }}>
    /// div { color: var(--color); }
    /// </style>
    /// ```
    pub NoAstroUnusedDefineVarsInStyle {
        version: "next",
        name: "noAstroUnusedDefineVarsInStyle",
        language: "js",
        sources: &[RuleSource::EslintAstro("no-unused-define-vars-in-style").inspired()],
        recommended: false,
        domains: &[RuleDomain::Astro, RuleDomain::Project],
    }
}

impl Rule for NoAstroUnusedDefineVarsInStyle {
    type Query = ResolvedImports<AnyJsObjectMember>;
    type State = AstroStyleVariable;
    type Signals = Option<Self::State>;
    type Options = NoAstroUnusedDefineVarsInStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let content_offset = ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .astro_content_offset()?;
        let info = ctx.db().html_module_info_for_path(ctx.file_path())?;
        let query_range = ctx.query().range() + content_offset;
        let style = info
            .astro_styles
            .iter()
            .find(|style| style.define_vars_range.contains_range(query_range))?;

        style
            .definitions
            .iter()
            .find(|definition| {
                query_range.contains_range(definition.range)
                    && !style.references.contains(definition.name.text())
            })
            .map(|definition| AstroStyleVariable {
                name: definition.name.clone(),
                range: definition.range - content_offset,
            })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, variable: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                variable.range,
                markup! { "This style variable is defined but never used." },
            )
            .note(markup! {
                "The variable "<Emphasis>{variable.name.text()}</Emphasis>" is not referenced by a matching CSS var() call in this style block."
            })
            .note(markup! {
                "Reference "<Emphasis>"--"{variable.name.text()}</Emphasis>" from this style block or remove the definition."
            }),
        )
    }
}

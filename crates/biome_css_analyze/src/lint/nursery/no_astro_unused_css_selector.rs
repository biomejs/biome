use crate::services::module_graph::CssModuleGraph;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssSelector, AnyCssSubSelector, CssClassSelector, CssCompoundSelector, CssQualifiedRule,
    CssSelectorList,
};
use biome_languages::CssFileSource;
use biome_languages::css::{CssEmbeddingKind, EmbeddingHtmlKind, EmbeddingStyleApplicability};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};
use biome_rule_options::no_astro_unused_css_selector::NoAstroUnusedCssSelectorOptions;

declare_lint_rule! {
    /// Disallows unused standalone class selectors in local Astro styles.
    ///
    /// The rule checks selectors whose complete selector-list entry is one class. It skips the document when a class-producing expression cannot be resolved statically.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic,ignore
    /// <div class="used"></div>
    /// <style>
    /// .unused { color: red; }
    /// </style>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,ignore
    /// <div class="used"></div>
    /// <style>
    /// .used { color: green; }
    /// .card.active { color: blue; }
    /// </style>
    /// ```
    pub NoAstroUnusedCssSelector {
        version: "next",
        name: "noAstroUnusedCssSelector",
        language: "css",
        sources: &[RuleSource::EslintAstro("no-unused-css-selector").inspired()],
        recommended: false,
        domains: &[RuleDomain::Astro, RuleDomain::Project],
    }
}

impl Rule for NoAstroUnusedCssSelector {
    type Query = CssModuleGraph<CssClassSelector>;
    type State = biome_rowan::TokenText;
    type Signals = Option<Self::State>;
    type Options = NoAstroUnusedCssSelectorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source = ctx.source_type::<CssFileSource>();
        if !source.is_css()
            || !matches!(
                source.as_embedding_kind(),
                CssEmbeddingKind::Html(EmbeddingHtmlKind::Astro {
                    applicability: EmbeddingStyleApplicability::Local
                })
            )
            || !is_standalone_class_selector(ctx.query())
        {
            return None;
        }

        let class_name = ctx
            .query()
            .name()
            .ok()?
            .as_css_custom_identifier()?
            .value_token()
            .ok()?
            .token_text_trimmed();
        if class_name.text().contains('\\') {
            return None;
        }

        let info = ctx.db().html_module_info_for_path(ctx.file_path())?;
        if info.has_unknown_astro_class_reference
            || info
                .astro_class_references
                .contains(class_name.text())
        {
            return None;
        }

        Some(class_name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, class_name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { "This CSS selector is not used by this Astro component." },
            )
            .note(markup! {
                "No static class reference matches "<Emphasis>"."{class_name.text()}</Emphasis>" in this component."
            })
            .note(markup! {
                "Use this class in the component or remove the selector."
            }),
        )
    }
}

fn is_standalone_class_selector(class: &CssClassSelector) -> bool {
    let Some(compound) = class
        .syntax()
        .parent()
        .and_then(|list| list.parent())
        .and_then(CssCompoundSelector::cast)
    else {
        return false;
    };
    if !compound.nesting_selectors().is_empty()
        || compound.simple_selector().is_some()
        || compound.sub_selectors().len() != 1
        || !compound.sub_selectors().iter().any(|selector| {
            matches!(selector, AnyCssSubSelector::CssClassSelector(candidate) if candidate == *class)
        })
    {
        return false;
    }

    compound
        .syntax()
        .parent()
        .and_then(CssSelectorList::cast)
        .filter(|list| list.len() >= 1)
        .and_then(|list| {
            list.iter().find_map(|selector| match selector.ok()? {
                AnyCssSelector::CssCompoundSelector(candidate) if candidate == compound => Some(()),
                _ => None,
            })?;
            list.syntax().parent()
        })
        .and_then(CssQualifiedRule::cast)
        .is_some()
}

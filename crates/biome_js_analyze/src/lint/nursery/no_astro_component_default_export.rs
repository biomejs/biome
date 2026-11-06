use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleExecutionScope, RuleSource,
    context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExportClause, AnyJsExportDefaultDeclaration};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_astro_component_default_export::NoAstroComponentDefaultExportOptions;

declare_lint_rule! {
    /// Disallow runtime default exports in Astro component frontmatter.
    ///
    /// Astro generates the runtime default export for each component. Defining another
    /// runtime default export in the component frontmatter conflicts with that generated export.
    /// Named exports and type-only default exports are allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// ---
    /// export default function render() {}
    /// ---
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// export function helper() {}
    /// export default interface Props {}
    /// ---
    /// ```
    ///
    pub NoAstroComponentDefaultExport {
        version: "next",
        name: "noAstroComponentDefaultExport",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("no-exports-from-components").inspired()],
        execution_scope: RuleExecutionScope::Program,
    }
}

impl Rule for NoAstroComponentDefaultExport {
    type Query = Ast<AnyJsExportClause>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoAstroComponentDefaultExportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_frontmatter()
        {
            return None;
        }

        runtime_default_export_range(ctx.query())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Astro components cannot define a runtime default export."
                },
            )
            .note(markup! {
                "Astro generates the component's runtime default export."
            })
            .note(markup! {
                "Remove this export or use a named export instead."
            }),
        )
    }
}

fn runtime_default_export_range(export_clause: &AnyJsExportClause) -> Option<TextRange> {
    match export_clause {
        AnyJsExportClause::JsExportDefaultExpressionClause(clause) => {
            Some(clause.default_token().ok()?.text_trimmed_range())
        }
        AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => match clause
            .declaration()
            .ok()?
        {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(_)
            | AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(_) => {
                Some(clause.default_token().ok()?.text_trimmed_range())
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
            | AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_) => None,
        },
        AnyJsExportClause::JsExportNamedClause(clause) if clause.type_token().is_none() => clause
            .specifiers()
            .iter()
            .find_map(|specifier| {
                let specifier = specifier.ok()?;
                let specifier = specifier.as_js_export_named_specifier()?;
                if specifier.type_token().is_some() {
                    return None;
                }
                let name = specifier.exported_name().ok()?;
                name.is_default().then(|| name.range())
            }),
        AnyJsExportClause::JsExportNamedFromClause(clause) if clause.type_token().is_none() => clause
            .specifiers()
            .iter()
            .find_map(|specifier| {
                let specifier = specifier.ok()?;
                if specifier.type_token().is_some() {
                    return None;
                }
                match specifier.export_as() {
                    Some(export_as) => {
                        let name = export_as.exported_name().ok()?;
                        name.is_default().then(|| name.range())
                    }
                    None => {
                        let name = specifier.source_name().ok()?;
                        name.is_default().then(|| name.range())
                    }
                }
            }),
        _ => None,
    }
}

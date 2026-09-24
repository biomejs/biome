use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExportClause, AnyJsExportDefaultDeclaration, JsExport,
};
use biome_languages::JsFileSource;
use biome_rowan::AstSeparatedList;
use biome_rule_options::no_astro_exports_from_components::NoAstroExportsFromComponentsOptions;

declare_lint_rule! {
    /// Disallows runtime default exports from Astro component frontmatter.
    ///
    /// Astro reserves the component's default export for the compiled component. Type-only and named exports remain valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic,ignore
    /// ---
    /// export default function helper() {}
    /// ---
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,ignore
    /// ---
    /// export const helper = () => {};
    /// export default interface Props {}
    /// ---
    /// ```
    pub NoAstroExportsFromComponents {
        version: "next",
        name: "noAstroExportsFromComponents",
        language: "js",
        sources: &[RuleSource::EslintAstro("no-exports-from-components").inspired()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroExportsFromComponents {
    type Query = Ast<JsExport>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = NoAstroExportsFromComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_frontmatter()
        {
            return None;
        }

        let default_token = match ctx.query().export_clause().ok()? {
            AnyJsExportClause::JsExportDefaultExpressionClause(clause) => {
                clause.default_token().ok()?
            }
            AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => {
                match clause.declaration().ok()? {
                    AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(_)
                    | AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(_) => {}
                    AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
                    | AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_) => return None,
                }
                clause.default_token().ok()?
            }
            AnyJsExportClause::JsExportFromClause(clause) => {
                if clause.type_token().is_some() {
                    return None;
                }
                let exported_name = clause.export_as()?.exported_name().ok()?;
                if !exported_name.is_default() {
                    return None;
                }
                exported_name.value().ok()?
            }
            AnyJsExportClause::JsExportNamedClause(clause) => {
                if clause.type_token().is_some() {
                    return None;
                }
                clause.specifiers().iter().find_map(|specifier| {
                    let specifier = specifier.ok()?.as_js_export_named_specifier()?.clone();
                    if specifier.type_token().is_some() {
                        return None;
                    }
                    let exported_name = specifier.exported_name().ok()?;
                    exported_name
                        .is_default()
                        .then(|| exported_name.value().ok())?
                })?
            }
            AnyJsExportClause::JsExportNamedFromClause(clause) => {
                if clause.type_token().is_some() {
                    return None;
                }
                clause.specifiers().iter().find_map(|specifier| {
                    let specifier = specifier.ok()?;
                    if specifier.type_token().is_some() {
                        return None;
                    }
                    let exported_name = if let Some(export_as) = specifier.export_as() {
                        export_as.exported_name().ok()?
                    } else {
                        specifier.source_name().ok()?
                    };
                    exported_name
                        .is_default()
                        .then(|| exported_name.value().ok())?
                })?
            }
            AnyJsExportClause::AnyJsDeclarationClause(_)
            | AnyJsExportClause::TsExportAsNamespaceClause(_)
            | AnyJsExportClause::TsExportAssignmentClause(_)
            | AnyJsExportClause::TsExportDeclareClause(_) => return None,
        };

        Some(default_token.text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Astro components cannot define a "<Emphasis>"runtime default export"</Emphasis>"."
                },
            )
            .note(markup! {
                "Astro generates the component's default export from the component template."
            })
            .note(markup! {
                "Remove this export or expose the value through a named export."
            }),
        )
    }
}

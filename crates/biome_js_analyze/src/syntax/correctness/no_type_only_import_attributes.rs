use biome_analyze::{context::RuleContext, declare_syntax_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExportClause, AnyJsImportClause, AnyJsModuleItem, JsNamedImportSpecifiers, JsSyntaxToken,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_syntax_rule! {
    /// Disallow type-only imports and exports with import attributes.
    ///
    /// There is one exception: TypeScript 5.3 and above allow this in CommonJS files, e.g. files ending with the `.cts` extension.
    /// See the [TypeScript docs](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-3.html#stable-support-resolution-mode-in-import-types).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts
    /// import type { A } from "./a.json" with { type: "json" };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cts
    /// import type { A } from "./a.json" with { "resolution-mode": "require" };
    /// ```
    ///
    pub NoTypeOnlyImportAttributes {
        version: "1.5.0",
        name: "noTypeOnlyImportAttributes",
        language: "js",
    }
}

impl Rule for NoTypeOnlyImportAttributes {
    type Query = Ast<AnyJsModuleItem>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let extension = ctx.file_path().extension()?;
        if extension.as_bytes() == b"cts" {
            // Ignore `*.cts`
            return None;
        }
        let module_item = ctx.query();
        match module_item {
            AnyJsModuleItem::AnyJsStatement(_) => None,
            AnyJsModuleItem::JsExport(export) => match export.export_clause().ok()? {
                AnyJsExportClause::AnyJsDeclarationClause(_)
                | AnyJsExportClause::JsExportDefaultDeclarationClause(_)
                | AnyJsExportClause::JsExportDefaultExpressionClause(_)
                | AnyJsExportClause::JsExportNamedClause(_)
                | AnyJsExportClause::TsExportAsNamespaceClause(_)
                | AnyJsExportClause::TsExportAssignmentClause(_)
                | AnyJsExportClause::TsExportDeclareClause(_) => None,
                AnyJsExportClause::JsExportFromClause(clause) => Some(RuleState {
                    assertion_range: clause.assertion()?.range(),
                    type_token_range: clause.type_token()?.text_trimmed_range(),
                }),
                AnyJsExportClause::JsExportNamedFromClause(clause) => {
                    let assertion_range = clause.assertion()?.range();
                    let type_token = clause.type_token().or_else(|| {
                        clause
                            .specifiers()
                            .iter()
                            .filter_map(|specifier| specifier.ok())
                            .find_map(|specifier| specifier.type_token())
                    })?;
                    Some(RuleState {
                        assertion_range,
                        type_token_range: type_token.text_trimmed_range(),
                    })
                }
            },
            AnyJsModuleItem::JsImport(import) => match import.import_clause().ok()? {
                AnyJsImportClause::JsImportBareClause(_) => None,
                AnyJsImportClause::JsImportCombinedClause(clause) => {
                    let assertion_range = clause.assertion()?.range();
                    let type_token = find_first_type_token(
                        clause.specifier().ok()?.as_js_named_import_specifiers()?,
                    )?;
                    Some(RuleState {
                        assertion_range,
                        type_token_range: type_token.text_trimmed_range(),
                    })
                }
                AnyJsImportClause::JsImportDefaultClause(clause) => Some(RuleState {
                    assertion_range: clause.assertion()?.range(),
                    type_token_range: clause.type_token()?.text_trimmed_range(),
                }),
                AnyJsImportClause::JsImportNamedClause(clause) => {
                    let assertion_range = clause.assertion()?.range();
                    let type_token = clause
                        .type_token()
                        .or_else(|| find_first_type_token(&clause.named_specifiers().ok()?))?;
                    Some(RuleState {
                        assertion_range,
                        type_token_range: type_token.text_trimmed_range(),
                    })
                }
                AnyJsImportClause::JsImportNamespaceClause(clause) => Some(RuleState {
                    assertion_range: clause.assertion()?.range(),
                    type_token_range: clause.type_token()?.text_trimmed_range(),
                }),
            },
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let import_or_export = if matches!(node, AnyJsModuleItem::JsImport(_)) {
            "import"
        } else {
            "export"
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.assertion_range,
                markup! {
                    "Import attributes cannot be used with a type-only "{import_or_export}"."
                },
            )
            .detail(
                state.type_token_range,
                markup! { "The type-only "{import_or_export}" is defined here." },
            ),
        )
    }
}

#[derive(Debug)]
pub struct RuleState {
    /// Range of the first found type token
    type_token_range: TextRange,
    /// Range of import attributes
    assertion_range: TextRange,
}

fn find_first_type_token(named_specifiers: &JsNamedImportSpecifiers) -> Option<JsSyntaxToken> {
    named_specifiers
        .specifiers()
        .iter()
        .filter_map(|specifier| specifier.ok())
        .find_map(|specifier| specifier.type_token())
}

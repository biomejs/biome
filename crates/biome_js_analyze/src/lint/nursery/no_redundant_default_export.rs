use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExportClause, AnyJsExpression, AnyJsModuleItem, JsModule,
};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_redundant_default_export::NoRedundantDefaultExportOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Checks if a default export exports the same symbol as a named export.
    ///
    /// This rule warns when a `default` export references the same identifier as a named export.
    /// Re-exports are out of scope.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const foo = 42;
    /// export default foo;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const foo = 42;
    /// export default 42;
    /// ```
    ///
    pub NoRedundantDefaultExport {
        version: "next",
        name: "noRedundantDefaultExport",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
    }
}

fn collect_exports(module: &JsModule) -> (FxHashSet<TokenText>, Option<(TokenText, TextRange)>) {
    let mut named_export_names: FxHashSet<TokenText> = FxHashSet::default();
    let mut default_export: Option<(TokenText, TextRange)> = None;
    
    for item in module.items() {
        if let AnyJsModuleItem::JsExport(export) = item {
            let export_clause = match export.export_clause() {
                Ok(clause) => clause,
                Err(_) => continue,
            };
            
            if matches!(
                export_clause,
                AnyJsExportClause::JsExportNamedFromClause(_)
                    | AnyJsExportClause::JsExportFromClause(_)
            ) {
                continue;
            }
            
            if !matches!(
                export_clause,
                AnyJsExportClause::JsExportDefaultDeclarationClause(_)
                    | AnyJsExportClause::JsExportDefaultExpressionClause(_)
            ) {
                for exported_item in export.get_exported_items() {
                    if !exported_item.is_default {
                        if let Some(identifier) = exported_item.identifier {
                            if let Some(name_token) = identifier.name_token() {
                                named_export_names.insert(name_token.token_text_trimmed());
                            }
                        }
                    }
                }
            }
            
            if let AnyJsExportClause::JsExportDefaultExpressionClause(default_clause) = export_clause {
                if let Ok(expression) = default_clause.expression() {
                    if let AnyJsExpression::JsIdentifierExpression(identifier_expr) = expression {
                        if let Ok(reference_id) = identifier_expr.name() {
                            if let Ok(name_token) = reference_id.value_token() {
                                let name = name_token.token_text_trimmed();
                                default_export = Some((name, default_clause.range()));
                            }
                        }
                    }
                }
            }
        }
    }
    
    (named_export_names, default_export)
}

impl Rule for NoRedundantDefaultExport {
    type Query = Ast<JsModule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoRedundantDefaultExportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();
        let (named_export_names, default_export) = collect_exports(module);
        
        if let Some((name, range)) = default_export {
            if named_export_names.contains(&name) {
                return Some(range);
            }
        }
        
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Default export exports the same symbol as a named export."
                },
            )
            .note(markup! {
                "Exporting the same identifier as both a named export and a default export is redundant."
            }),
        )
    }
}

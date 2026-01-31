use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    export_ext::{AnyIdentifier, AnyJsExported}, AnyJsBinding, AnyJsBindingPattern, AnyJsExportClause,
    AnyJsExpression, AnyJsModuleItem, JsModule,
};
use biome_rowan::TextRange;
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

fn get_binding_from_identifier(
    identifier: &AnyIdentifier,
    model: &SemanticModel,
) -> Option<Binding> {
    match identifier {
        AnyIdentifier::AnyJsBindingPattern(binding_pattern) => {
            let AnyJsBindingPattern::AnyJsBinding(binding) = binding_pattern else {
                return None;
            };
            let AnyJsBinding::JsIdentifierBinding(id_binding) = binding else {
                return None;
            };
            Some(model.as_binding(id_binding))
        }
        AnyIdentifier::AnyTsIdentifierBinding(ts_binding) => {
            ts_binding
                .as_ts_identifier_binding()
                .map(|binding| model.as_binding(binding))
        }
        AnyIdentifier::JsReferenceIdentifier(ref_id) => model.binding(ref_id),
        AnyIdentifier::JsIdentifierExpression(id_expr) => {
            id_expr.name().ok().and_then(|ref_id| model.binding(&ref_id))
        }
        AnyIdentifier::JsLiteralExportName(_) => None,
    }
}

fn is_re_export(export_clause: &AnyJsExportClause) -> bool {
    matches!(
        export_clause,
        AnyJsExportClause::JsExportNamedFromClause(_) | AnyJsExportClause::JsExportFromClause(_)
    )
}

fn resolve_binding_and_range_from_exported_item(
    exported_item: &biome_js_syntax::export_ext::ExportedItem,
    model: &SemanticModel,
) -> Option<(Binding, Option<TextRange>)> {
    if exported_item.is_default {
        if let Some(AnyJsExported::JsFunctionExportDefaultDeclaration(_) | AnyJsExported::JsClassExportDefaultDeclaration(_)) =
            exported_item.exported.as_ref()
        {
            return None;
        }
    }

    if let Some(identifier) = exported_item.identifier.as_ref() {
        if let Some(binding) = get_binding_from_identifier(identifier, model) {
            let range = identifier.name_token().map(|token| token.text_range());
            return Some((binding, range));
        }
    }

    if let Some(AnyJsExported::AnyIdentifier(identifier)) = exported_item.exported.as_ref() {
        if let Some(binding) = get_binding_from_identifier(identifier, model) {
            let range = identifier.name_token().map(|token| token.text_range());
            return Some((binding, range));
        }
    }

    None
}

fn extract_binding_from_default_expression_clause(
    default_clause: &biome_js_syntax::JsExportDefaultExpressionClause,
    model: &SemanticModel,
) -> Option<(Binding, TextRange)> {
    let expression = default_clause.expression().ok()?;
    let AnyJsExpression::JsIdentifierExpression(identifier_expr) = expression else {
        return None;
    };
    let reference_id = identifier_expr.name().ok()?;
    let name_token = reference_id.value_token().ok()?;
    let binding = model.binding(&reference_id)?;

    Some((binding, name_token.text_range()))
}

fn collect_exports(
    module: &JsModule,
    model: &SemanticModel,
) -> (FxHashSet<Binding>, Option<(Binding, TextRange)>) {
    let mut named_export_bindings = FxHashSet::default();
    let mut default_export: Option<(Binding, TextRange)> = None;

    for item in module.items() {
        let AnyJsModuleItem::JsExport(export) = item else {
            continue;
        };

        let Ok(export_clause) = export.export_clause() else {
            continue;
        };

        if is_re_export(&export_clause) {
            continue;
        }

        if let AnyJsExportClause::JsExportDefaultExpressionClause(default_clause) = &export_clause {
            if let Some((binding, range)) =
                extract_binding_from_default_expression_clause(default_clause, model)
            {
                default_export = Some((binding, range));
            }
            continue;
        }

        for exported_item in export.get_exported_items() {
            let Some((binding, range)) =
                resolve_binding_and_range_from_exported_item(&exported_item, model)
            else {
                continue;
            };

            if exported_item.is_default {
                if let Some(range) = range {
                    default_export = Some((binding, range));
                }
            } else {
                named_export_bindings.insert(binding);
            }
        }
    }

    (named_export_bindings, default_export)
}

impl Rule for NoRedundantDefaultExport {
    type Query = Semantic<JsModule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoRedundantDefaultExportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();
        let model = ctx.model();
        let (named_export_bindings, default_export) = collect_exports(module, model);
        
        if let Some((binding, range)) = default_export {
            if named_export_bindings.contains(&binding) {
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

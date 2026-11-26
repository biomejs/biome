use crate::services::database::ResolvedImports;
use crate::utils::module_exports::{has_checkable_exports, is_es_module};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyJsExportClause, AnyJsImportLike, AnyJsModuleItem, AnyJsRoot,
    JsExportFromClause, JsModuleSource,
};
use biome_module_graph::{JsModuleInfo, ModuleDb, ModuleInfo, ModuleInfoKind};
use biome_rowan::AstNode;
use biome_rule_options::no_useless_re_export_all::NoUselessReExportAllOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Disallow `export * from` a module that has no named exports.
    ///
    /// `export * from` re-exports the named exports of a module, but not its
    /// default export. When the module has no named exports, the statement
    /// exports nothing.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,file=foo.js
    /// export default function foo() {}
    /// ```
    ///
    /// ```js,expect_diagnostic,file=index.js
    /// export * from "./foo.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=foo.js
    /// export default function foo() {}
    /// export function bar() {}
    /// ```
    ///
    /// ```js,file=index.js
    /// export * from "./foo.js";
    /// export { default } from "./foo.js";
    /// ```
    ///
    /// ## See Also
    ///
    /// - To report names that more than one `export * from` statement exports,
    ///   see [`noDuplicateExports`](https://biomejs.dev/linter/rules/no-duplicate-exports/).
    pub NoUselessReExportAll {
        version: "next",
        name: "noUselessReExportAll",
        language: "js",
        sources: &[RuleSource::EslintImport("export").inspired()],
        domains: &[RuleDomain::Project],
        severity: Severity::Warning,
        recommended: true,
    }
}

impl Rule for NoUselessReExportAll {
    type Query = ResolvedImports<JsExportFromClause>;
    type State = JsModuleSource;
    type Signals = Option<Self::State>;
    type Options = NoUselessReExportAllOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let clause = ctx.query();
        // `export * as ns from` exports the namespace object, even if it's
        // empty.
        if clause.export_as().is_some() {
            return None;
        }
        let source = clause.source().ok()?.as_js_module_source()?.clone();
        let owner = ctx.module_info_for_path(ctx.file_path())?;
        let ModuleInfoKind::Js(owner_info) = owner.kind(ctx.db()) else {
            return None;
        };
        let target = owner_info
            .get_import_path_by_js_node(&AnyJsImportLike::JsModuleSource(source.clone()))?
            .resolve_js(ctx.db(), owner)
            .path()
            .as_path()
            .and_then(|path| ctx.module_info_for_path(path))?;
        (has_named_exports(ctx.db(), target) == Some(false)).then_some(source)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, source: &Self::State) -> Option<RuleDiagnostic> {
        let specifier = source.inner_string_text().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                source.range(),
                markup! {
                    "The module "<Emphasis>{specifier.text()}</Emphasis>" has no named exports, so this statement exports nothing."
                },
            )
            .note(markup! {
                <Emphasis>"export * from"</Emphasis>" doesn't re-export the default export."
            })
            .note(markup! {
                "Re-export the default export with "<Emphasis>"export { default } from"</Emphasis>", or remove this statement."
            }),
        )
    }
}

/// Returns whether `module` has a named export, including the names from its
/// own `export * from` statements.
///
/// Returns `None` when a module whose exports Biome can't check is reached.
fn has_named_exports(db: &dyn ModuleDb, module: ModuleInfo) -> Option<bool> {
    let mut seen_modules = FxHashSet::default();
    let mut stack = vec![module];
    while let Some(module) = stack.pop() {
        if !seen_modules.insert(module) {
            continue;
        }
        if !has_checkable_exports(module.path(db)) {
            return None;
        }
        let ModuleInfoKind::Js(info) = module.kind(db) else {
            return None;
        };
        if !is_es_module(info) {
            return None;
        }
        if info.exports.keys().any(|name| name.text() != "default")
            || has_unrecorded_named_exports(info)
        {
            return Some(true);
        }
        for reexport in &info.blanket_reexports {
            let target = reexport
                .import
                .resolve_js(db, module)
                .path()
                .as_path()
                .and_then(|path| db.module_for_path(path))?;
            stack.push(target);
        }
    }
    Some(false)
}

/// Returns whether the module has a named export that the module graph doesn't
/// record, such as `export import a = b.c` or `export namespace A.B {}`.
fn has_unrecorded_named_exports(info: &JsModuleInfo) -> bool {
    let AnyJsRoot::JsModule(module) = info.semantic_model.root() else {
        return false;
    };
    module.items().into_iter().any(|item| {
        let AnyJsModuleItem::JsExport(export) = item else {
            return false;
        };
        let declaration = match export.export_clause() {
            Ok(AnyJsExportClause::AnyJsDeclarationClause(declaration)) => declaration,
            Ok(AnyJsExportClause::TsExportDeclareClause(clause)) => {
                let Ok(declaration) = clause.declaration() else {
                    return false;
                };
                declaration
            }
            _ => return false,
        };
        matches!(
            declaration,
            AnyJsDeclarationClause::TsImportEqualsDeclaration(_)
                | AnyJsDeclarationClause::TsModuleDeclaration(_)
        )
    })
}

use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_languages::JsFileSource;
use biome_js_syntax::{
    AnyJsImportLike, JsDefaultImportSpecifier, JsImportCombinedClause, JsImportDefaultClause,
    JsModuleSource,
};
use biome_module_graph::{
    JsExportedSymbolLookup, JsModuleInfo, ModuleInfoKind, SymbolFromModuleInfo,
    find_js_exported_symbol,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_unresolved_default_import::NoUnresolvedDefaultImportOptions;
use camino::Utf8Path;

declare_lint_rule! {
    /// Disallow default imports from modules that have no default export.
    ///
    /// A default import such as `import foo from "./foo.js"` only works when
    /// `./foo.js` has a default export. Otherwise, the import fails when the
    /// code runs, or `foo` ends up `undefined`, depending on the tool that
    /// loads the code. This usually happens when the module only has named
    /// exports, for example after a refactor that removed its default export.
    ///
    /// This rule only checks modules that use `import` or `export` syntax.
    /// Imports of CommonJS modules, such as files that assign
    /// `module.exports`, aren't reported, because those modules have no
    /// default export that Biome can check.
    ///
    /// Imports of TypeScript declaration files, such as `.d.ts` files, aren't
    /// reported either. A declaration file often describes a CommonJS module,
    /// so its default export can exist even if the file doesn't declare one.
    ///
    /// Imports that Biome can't resolve to a file aren't reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,file=foo.js
    /// export function foo() {}
    /// ```
    ///
    /// ```js,expect_diagnostic,file=bar.js
    /// import foo from "./foo.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=foo.js
    /// export default function foo() {}
    /// export function bar() {}
    /// ```
    ///
    /// ```js,file=baz.js
    /// import foo, { bar } from "./foo.js";
    /// ```
    ///
    /// ## See Also
    ///
    /// - To report imports of names that a module doesn't export, and imports
    ///   that can't be resolved, see
    ///   [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/).
    pub NoUnresolvedDefaultImport {
        version: "2.6.7",
        name: "noUnresolvedDefaultImport",
        language: "js",
        sources: &[RuleSource::EslintImport("default").same()],
        domains: &[RuleDomain::Project],
        severity: Severity::Error,
        recommended: true,
    }
}

declare_node_union! {
    pub AnyJsDefaultImportClause = JsImportDefaultClause | JsImportCombinedClause
}

impl Rule for NoUnresolvedDefaultImport {
    type Query = ResolvedImports<AnyJsDefaultImportClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnresolvedDefaultImportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let clause = ctx.query();
        // `import source foo from "..."` imports the module source, not its
        // default export.
        if let AnyJsDefaultImportClause::JsImportDefaultClause(clause) = clause
            && clause.phase_token().is_some()
        {
            return None;
        }

        let owner = ctx.module_info_for_path(ctx.file_path())?;
        let ModuleInfoKind::Js(module_info) = owner.kind(ctx.db()) else {
            return None;
        };
        let source = AnyJsImportLike::JsModuleSource(clause.source()?);
        let resolved = module_info
            .get_import_path_by_js_node(&source)?
            .resolve_js(ctx.db(), owner);
        let target_path = resolved.path().as_path()?;
        if !has_checkable_exports(target_path) {
            return None;
        }
        let target = ctx.module_info_for_path(target_path)?;
        // Other kinds of files, such as CSS modules, get their default export
        // from the tool that loads them.
        let ModuleInfoKind::Js(target_info) = target.kind(ctx.db()) else {
            return None;
        };
        if !is_es_module(target_info) {
            return None;
        }

        let lookup = find_js_exported_symbol(
            ctx.db(),
            SymbolFromModuleInfo::new(ctx.db(), "default", target),
        );
        // `Unknown` means that a re-export couldn't be followed, so the
        // default export may still exist.
        matches!(lookup, JsExportedSymbolLookup::Missing).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let clause = ctx.query();
        let specifier = clause.source()?.inner_string_text().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                clause.default_specifier()?.range(),
                markup! {
                    "The module "<Emphasis>{specifier.text()}</Emphasis>" has no default export."
                },
            )
            .note(if clause.is_type_only() {
                markup! {
                    "TypeScript reports this import as an error."
                }
            } else {
                markup! {
                    "This import fails when the code runs, or the imported value is "<Emphasis>"undefined"</Emphasis>", depending on the tool that loads the code."
                }
            })
            .note(markup! {
                "Import one of the named exports of the module instead, or add a default export to the module."
            }),
        )
    }
}

impl AnyJsDefaultImportClause {
    fn default_specifier(&self) -> Option<JsDefaultImportSpecifier> {
        match self {
            Self::JsImportDefaultClause(clause) => clause.default_specifier().ok(),
            Self::JsImportCombinedClause(clause) => clause.default_specifier().ok(),
        }
    }

    fn is_type_only(&self) -> bool {
        match self {
            Self::JsImportDefaultClause(clause) => clause.type_token().is_some(),
            Self::JsImportCombinedClause(_) => false,
        }
    }

    fn source(&self) -> Option<JsModuleSource> {
        let source = match self {
            Self::JsImportDefaultClause(clause) => clause.source(),
            Self::JsImportCombinedClause(clause) => clause.source(),
        };
        source.ok()?.as_js_module_source().cloned()
    }
}

/// Returns whether the exports of the file at `path` describe its default
/// export.
///
/// Declaration files often describe CommonJS modules, whose default export
/// isn't declared. Vue, Svelte and Astro components get their default export
/// from the framework compiler.
fn has_checkable_exports(path: &Utf8Path) -> bool {
    JsFileSource::try_from(path)
        .is_ok_and(|source| !source.is_embedded() && !source.language().is_definition_file())
}

/// Returns whether the module uses `import` or `export` syntax.
///
/// Modules without it, such as CommonJS modules, have no default export that
/// can be looked up.
fn is_es_module(module_info: &JsModuleInfo) -> bool {
    !module_info.exports.is_empty()
        || !module_info.blanket_reexports.is_empty()
        || module_info
            .import_paths
            .iter()
            .any(|import_path| import_path.kind.is_static())
}

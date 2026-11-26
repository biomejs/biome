use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_fs::is_node_modules_path;
use biome_js_syntax::{
    AnyJsImportClause, JsExportFromClause, JsExportNamedFromClause, JsModuleSource,
};
use biome_module_graph::ModuleInfoKind;
use biome_rowan::{AstNode, TokenText, declare_node_union};
use biome_rule_options::no_convex_wrong_runtime_imports::NoConvexWrongRuntimeImportsOptions;
use camino::Utf8Path;

declare_lint_rule! {
    /// Disallow importing Node.js runtime files from Convex runtime files.
    ///
    /// Files in your `convex` folder that start with `"use node"` run in Node.js. All other
    /// files run in the Convex runtime, which doesn't support Node.js APIs. If a Convex runtime
    /// file imports a `"use node"` file, that code runs in the Convex runtime too, and can fail.
    ///
    /// Move the shared code into a file without `"use node"`, or add `"use node"` to the
    /// importing file if it only defines actions.
    ///
    /// Type-only imports are allowed. Files that Convex doesn't deploy as functions, such as
    /// `schema.ts`, test files, and files in `_generated`, are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,file=convex/stripe.ts
    /// "use node";
    ///
    /// export function formatAmount(amount: number) {
    ///     return `$${amount.toFixed(2)}`;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=convex/payments.ts
    /// import { formatAmount } from "./stripe";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=convex/format.ts
    /// export function formatAmount(amount: number) {
    ///     return `$${amount.toFixed(2)}`;
    /// }
    /// ```
    ///
    /// ```ts,file=convex/payments.ts
    /// import { formatAmount } from "./format";
    /// ```
    ///
    pub NoConvexWrongRuntimeImports {
        version: "next",
        name: "noConvexWrongRuntimeImports",
        language: "js",
        sources: &[RuleSource::EslintConvex("import-wrong-runtime").inspired()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex, RuleDomain::Project],
    }
}

declare_node_union! {
    pub AnyJsModuleSourceClause = AnyJsImportClause | JsExportFromClause | JsExportNamedFromClause
}

impl Rule for NoConvexWrongRuntimeImports {
    type Query = ResolvedImports<AnyJsModuleSourceClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoConvexWrongRuntimeImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source = ctx.query().source()?;
        if source.imports_only_types() {
            return None;
        }

        let self_path = ctx.file_path();
        if !is_convex_function_file(self_path) {
            return None;
        }

        let owner = ctx.module_info_for_path(self_path)?;
        let ModuleInfoKind::Js(module_info) = owner.kind(ctx.db()) else {
            return None;
        };
        if has_use_node_directive(module_info.directives.as_ref()) {
            return None;
        }

        let specifier = source.inner_string_text().ok()?;
        let resolved = module_info
            .import_paths
            .get(specifier.text())?
            .resolve_js(ctx.db(), owner);
        let target_path = resolved
            .path()
            .as_path()
            .filter(|path| !is_node_modules_path(path) && convex_folder(path).is_some())?;

        let target = ctx.module_info_for_path(target_path)?;
        let ModuleInfoKind::Js(target_info) = target.kind(ctx.db()) else {
            return None;
        };
        has_use_node_directive(target_info.directives.as_ref()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let source = ctx.query().source()?;
        let specifier = source.inner_string_text().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                source.range(),
                markup! {
                    "This file runs in the Convex runtime, but it imports "<Emphasis>{specifier.text()}</Emphasis>", which runs in Node.js."
                },
            )
            .note(markup! {
                "Files with the "<Emphasis>"\"use node\""</Emphasis>" directive run in Node.js. When you import one from a file without the directive, its code runs in the Convex runtime instead, where Node.js APIs aren't available."
            })
            .note(markup! {
                "Move the code you need into a file without the "<Emphasis>"\"use node\""</Emphasis>" directive, or add the directive to this file if it only defines actions."
            }),
        )
    }
}

impl AnyJsModuleSourceClause {
    fn source(&self) -> Option<JsModuleSource> {
        match self {
            Self::AnyJsImportClause(clause) => clause.source().ok(),
            Self::JsExportFromClause(clause) => clause.source().ok()?.as_js_module_source().cloned(),
            Self::JsExportNamedFromClause(clause) => {
                clause.source().ok()?.as_js_module_source().cloned()
            }
        }
    }
}

fn has_use_node_directive(directives: &[TokenText]) -> bool {
    directives.iter().any(|directive| directive.text() == "use node")
}

/// Returns the closest ancestor folder of `path` named `convex`.
fn convex_folder(path: &Utf8Path) -> Option<&Utf8Path> {
    path.ancestors()
        .skip(1)
        .find(|ancestor| ancestor.file_name() == Some("convex"))
}

/// Returns whether Convex deploys the file at `path` as a module of
/// functions, which runs in the runtime that its directives select.
///
/// The checks mirror the entry point selection of the Convex CLI bundler.
fn is_convex_function_file(path: &Utf8Path) -> bool {
    let Some(relative) = convex_folder(path).and_then(|folder| path.strip_prefix(folder).ok())
    else {
        return false;
    };
    let Some(name) = relative.file_name() else {
        return false;
    };

    relative
        .extension()
        .is_some_and(|extension| FUNCTION_FILE_EXTENSIONS.binary_search(&extension).is_ok())
        && !relative.starts_with("_generated")
        && !name.starts_with(['.', '#'])
        && name != "schema.ts"
        && name != "schema.js"
        && name.bytes().filter(|byte| *byte == b'.').count() == 1
        && !relative.as_str().contains(' ')
}

/// The extensions of the files that Convex can deploy as modules of
/// functions, sorted for binary search.
const FUNCTION_FILE_EXTENSIONS: &[&str] = &["cjs", "cts", "js", "jsx", "mjs", "mts", "ts", "tsx"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_convex_function_files() {
        for path in [
            "/app/convex/messages.ts",
            "/app/convex/lib/helpers.js",
            "/app/convex/component.tsx",
            "/convex/app/convex/messages.ts",
        ] {
            assert!(is_convex_function_file(Utf8Path::new(path)), "{path}");
        }
        for path in [
            "/app/src/messages.ts",
            "/app/convex/_generated/api.ts",
            "/app/convex/schema.ts",
            "/app/convex/schema.js",
            "/app/convex/auth.config.ts",
            "/app/convex/messages.test.ts",
            "/app/convex/.hidden.ts",
            "/app/convex/#messages.ts#",
            "/app/convex/#messages.ts",
            "/app/convex/my folder/messages.ts",
            "/app/convex/my messages.ts",
            "/app/convex/data.json",
            "/app/convex/App.vue",
        ] {
            assert!(!is_convex_function_file(Utf8Path::new(path)), "{path}");
        }
    }

    #[test]
    fn function_file_extensions_are_sorted() {
        for pair in FUNCTION_FILE_EXTENSIONS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come before {}",
                pair[1],
                pair[0]
            );
        }
    }
}

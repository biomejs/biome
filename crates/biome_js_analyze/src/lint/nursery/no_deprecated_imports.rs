use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike, JsModuleSource};
use biome_module_graph::{JsImportPath, JsModuleInfo, ModuleGraph};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_deprecated_imports::NoDeprecatedImportsOptions;
use camino::Utf8Path;

use crate::services::module_graph::ResolvedImports;

declare_lint_rule! {
    /// Restrict imports of deprecated exports.
    ///
    /// This rule flags any imports for symbols (such as types, functions, or
    /// anything else that can be imported), that are documented with a JSDoc
    /// comment that contains an "@deprecated" annotation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,file=foo.js
    /// import { oldUtility } from "./utils.js";
    /// ```
    ///
    /// ```js,file=utils.js
    /// /**
    ///  * @deprecated
    ///  */
    /// export function oldUtility() {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=foo.js
    /// import { newUtility, oldUtility } from "./utils.js";
    /// ```
    ///
    /// ```js,file=utils.js
    /// export function newUtility() {}
    ///
    /// // @deprecated (this is not a JSDoc comment)
    /// export function oldUtility() {}
    /// ```
    ///
    pub NoDeprecatedImports {
        version: "2.2.5",
        name: "noDeprecatedImports",
        language: "js",
        sources: &[
            RuleSource::EslintTypeScript("no-deprecated").inspired(),
            RuleSource::EslintImport("no-deprecated").inspired()
        ],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Project],
    }
}

#[derive(Debug)]
pub struct NoDeprecatedImportsState {
    /// Range where the deprecated symbol is found.
    range: TextRange,

    /// The documented deprecation message, if any.
    message: Option<String>,
}

impl Rule for NoDeprecatedImports {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = NoDeprecatedImportsState;
    type Signals = Vec<Self::State>;
    type Options = NoDeprecatedImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(module_info) = ctx.module_info_for_path(ctx.file_path()) else {
            return Vec::new();
        };

        let node = ctx.query();
        let Some(target_path) = module_info
            .get_import_path_by_js_node(node)
            .and_then(JsImportPath::as_path)
        else {
            return Vec::new();
        };

        match node {
            AnyJsImportLike::JsModuleSource(node) => {
                get_deprecated_imports_from_module_source(node, target_path, ctx.module_graph())
            }

            // TODO: require() and import() calls should also be handled here, but tracking the
            //       bindings to get the used symbol names is not easy. I think we can leave it
            //       for future opportunities.
            _ => Vec::new(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = if let Some(message) = &state.message {
            markup! { "Deprecated import: "{message.to_string()} }.to_owned()
        } else {
            markup! { "Deprecated import." }.to_owned()
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.range, message)
                .note(markup! {
                    "An "<Emphasis>"@deprecated"</Emphasis>" annotation "
                    "indicates the author doesn't want you to rely on this "
                    "import anymore."
                })
                .note(markup! {
                    "You should probably import a different symbol instead."
                }),
        )
    }
}

fn get_deprecated_imports_from_module_source(
    node: &JsModuleSource,
    target_path: &Utf8Path,
    module_graph: &ModuleGraph,
) -> Vec<NoDeprecatedImportsState> {
    let Some(module_info) = module_graph.module_info_for_path(target_path) else {
        return Vec::new();
    };

    let Some(import_clause) = node.syntax().parent().and_then(AnyJsImportClause::cast) else {
        return Vec::new();
    };

    import_clause.filter_map_all_imported_symbols(|name, range| {
        find_deprecation(&module_info, module_graph, &name)
            .map(|message| NoDeprecatedImportsState { range, message })
    })
}

/// Looks up the symbol with the given `name` inside the module with the given
/// `module_info`, and returns whether the symbol is deprecated.
///
/// - Returns `Some(Some(message))` if the symbol is deprecated and has a
///   deprecation message.
/// - Returns `Some(None)` if the symbol is deprecated but has no message.
/// - Returns `None` if the symbol is not deprecated or cannot be found.
fn find_deprecation(
    module_info: &JsModuleInfo,
    module_graph: &ModuleGraph,
    name: &str,
) -> Option<Option<String>> {
    module_info
        .find_jsdoc_for_exported_symbol(module_graph, name)
        .and_then(|jsdoc| {
            let mut is_deprecated = false;
            let mut message = String::new();
            for line in jsdoc.lines() {
                let line = line.trim();
                if is_deprecated {
                    if line.is_empty() {
                        break;
                    }

                    if !message.is_empty() {
                        message.push(' ');
                    }

                    message.push_str(line);
                } else if let Some((_before, after)) = line.split_once("@deprecated") {
                    is_deprecated = true;
                    message.push_str(after.trim_start());
                }
            }

            is_deprecated.then_some(if message.is_empty() {
                None
            } else {
                Some(message)
            })
        })
}

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike, JsModuleSource};
use biome_module_graph::{JsImportPath, JsModuleInfo, ModuleGraph, SUPPORTED_EXTENSIONS};
use biome_resolver::ResolveError;
use biome_rowan::{AstNode, Text, TextRange, TokenText};
use biome_rule_options::no_unresolved_imports::NoUnresolvedImportsOptions;
use camino::{Utf8Path, Utf8PathBuf};

use crate::services::module_graph::ResolvedImports;

declare_lint_rule! {
    /// Warn when importing non-existing exports.
    ///
    /// Importing a non-existing export is an error at runtime or build time.
    /// Biome can detect such incorrect imports and report errors for them.
    ///
    /// Note that if you use TypeScript, you probably don't want to use this
    /// rule, since TypeScript already performs such checks for you.
    ///
    /// ## Known Limitations
    ///
    /// * This rule does not validate imports through dynamic `import()`
    ///   expressions or CommonJS `require()` calls.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,file=foo.js
    /// export function foo() {};
    /// ```
    ///
    /// ```js,expect_diagnostic,file=bar.js
    /// // Attempt to import symbol with a typo:
    /// import { fooo } from "./foo.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=foo.js
    /// export function foo() {};
    /// ```
    ///
    /// ```js,file=bar.js
    /// // Fixed typo:
    /// import { foo } from "./foo.js";
    /// ```
    pub NoUnresolvedImports {
        version: "2.0.0",
        name: "noUnresolvedImports",
        language: "js",
        sources: &[RuleSource::EslintImport("named").inspired()],
        domains: &[RuleDomain::Project],
    }
}

pub enum NoUnresolvedImportsState {
    UnresolvedPath {
        range: TextRange,
        specifier: Box<str>,
        resolve_error: ResolveError,
    },
    UnresolvedSymbol {
        range: TextRange,
        specifier: Box<str>,
        export_name: Box<str>,
    },
}

impl NoUnresolvedImportsState {
    fn range(&self) -> TextRange {
        match self {
            Self::UnresolvedPath { range, .. } => *range,
            Self::UnresolvedSymbol { range, .. } => *range,
        }
    }

    fn specifier(&self) -> &str {
        match self {
            Self::UnresolvedPath { specifier, .. } => specifier,
            Self::UnresolvedSymbol { specifier, .. } => specifier,
        }
    }
}

impl Rule for NoUnresolvedImports {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = NoUnresolvedImportsState;
    type Signals = Vec<Self::State>;
    type Options = NoUnresolvedImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(module_info) = ctx.module_info_for_path(ctx.file_path()) else {
            return Vec::new();
        };

        let node = ctx.query();
        let Some(JsImportPath { resolved_path, .. }) = module_info.get_import_path_by_js_node(node)
        else {
            return Vec::new();
        };

        let Some(specifier) = node.inner_string_text() else {
            return Vec::new();
        };

        let resolved_path = match resolved_path.as_deref() {
            Ok(resolved_path) => resolved_path,
            Err(resolve_error) => {
                if Utf8Path::new(&specifier)
                    .extension()
                    .is_some_and(|extension| !SUPPORTED_EXTENSIONS.contains(&extension))
                {
                    return Vec::new();
                }

                return vec![NoUnresolvedImportsState::UnresolvedPath {
                    range: node.syntax().text_trimmed_range(),
                    specifier: specifier.as_ref().into(),
                    resolve_error: *resolve_error,
                }];
            }
        };

        let Some(target_info) = ctx.module_info_for_path(resolved_path) else {
            return Vec::new();
        };

        let options = GetUnresolvedImportsOptions {
            module_graph: ctx.module_graph(),
            specifier,
            target_info,
        };

        match node {
            AnyJsImportLike::JsModuleSource(node) => {
                get_unresolved_imports_from_module_source(node, &options)
            }

            // TODO: require() and import() calls should also be handled here, but tracking the
            //       bindings to get the used symbol names is not easy. I think we can leave it
            //       for future opportunities.
            _ => Vec::new(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let cwd = Utf8PathBuf::from(
            std::env::current_dir()
                .map(|cwd| cwd.to_string_lossy().to_string())
                .unwrap_or_default(),
        );

        // Use the relative path if possible.
        let specifier = Utf8Path::new(state.specifier());
        let specifier = specifier.strip_prefix(&cwd).unwrap_or(specifier).as_str();

        let range = state.range();

        let diagnostic = match state {
            NoUnresolvedImportsState::UnresolvedPath { resolve_error, .. } => {
                let specifier_kind = if specifier.starts_with('.') {
                    "path"
                } else {
                    "import specifier"
                };
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The "{specifier_kind}" "<Emphasis>{specifier}</Emphasis>
                        " cannot be resolved: "<Emphasis>{resolve_error.to_string()}</Emphasis>
                    },
                )
                .note(if specifier_kind == "path" {
                    markup! {
                        "Make sure that the path exists and is readable."
                    }
                } else {
                    markup! {
                        "Make sure the specifier is correct and your project is set up correctly."
                    }
                })
            }
            NoUnresolvedImportsState::UnresolvedSymbol { export_name, .. }
                if export_name.as_ref() == "default" =>
            {
                let specifier_kind = if specifier.starts_with('.') {
                    "path"
                } else {
                    "module"
                };
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The "{specifier_kind}" "<Emphasis>{specifier}</Emphasis>" has no default export."
                    },
                )
                .note(markup! {
                    "Make sure that the "{specifier_kind}" is correct and that you're importing the right symbol."
                })
            }
            NoUnresolvedImportsState::UnresolvedSymbol { export_name, .. } => {
                let specifier_kind = if specifier.starts_with('.') {
                    "path"
                } else {
                    "module"
                };
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The "{specifier_kind}" "<Emphasis>{specifier}</Emphasis>" has no export named "<Emphasis>{export_name}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Make sure that the "{specifier_kind}" is correct and that you're importing the right symbol."
                })
            }
        };

        Some(diagnostic)
    }
}

struct GetUnresolvedImportsOptions<'a> {
    /// The module graph to use for further lookups.
    module_graph: &'a ModuleGraph,

    /// The path of the module we're importing from.
    specifier: TokenText,

    /// Module info of the module we're importing from.
    target_info: JsModuleInfo,
}

fn get_unresolved_imports_from_module_source(
    node: &JsModuleSource,
    options: &GetUnresolvedImportsOptions,
) -> Vec<NoUnresolvedImportsState> {
    let Some(import_clause) = node.syntax().parent().and_then(AnyJsImportClause::cast) else {
        return Vec::new();
    };

    import_clause.filter_map_all_imported_symbols(|imported_name, range| {
        (!has_exported_symbol(&imported_name, options)).then(|| {
            NoUnresolvedImportsState::UnresolvedSymbol {
                range,
                specifier: options.specifier.as_ref().into(),
                export_name: imported_name.into(),
            }
        })
    })
}

fn has_exported_symbol(import_name: &Text, options: &GetUnresolvedImportsOptions) -> bool {
    options
        .target_info
        .find_exported_symbol(options.module_graph, import_name.text())
        .is_some()
}

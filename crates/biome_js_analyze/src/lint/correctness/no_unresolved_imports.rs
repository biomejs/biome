use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::JsDeclarationKind;
use biome_js_syntax::function_ext::AnyFunctionLike;
use biome_js_syntax::{
    AnyJsImportClause, AnyJsImportLike, JsAwaitExpression, JsExport, JsExportNamedFromClause,
    JsForOfStatement, JsImport, JsImportMetaExpression, JsModuleSource, JsVariableDeclaration,
    TsImportEqualsDeclaration,
};
use biome_module_graph::{
    JsExportedSymbolLookup, JsModuleInfo, ModuleDb, ModuleInfo, ModuleInfoKind, ResolutionMode,
    SUPPORTED_EXTENSIONS, SymbolFromModuleInfo, find_js_exported_symbol, resolve_module_import,
};
use biome_package::PackageType;
use biome_resolver::ResolveError;
use biome_rowan::{AstNode, AstSeparatedList, Text, TextRange, TokenText};
use biome_rule_options::no_unresolved_imports::NoUnresolvedImportsOptions;
use camino::{Utf8Path, Utf8PathBuf};

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
        severity: Severity::Error,
    }
}

pub enum NoUnresolvedImportsState {
    UnresolvedPath {
        range: TextRange,
        specifier: TokenText,
        resolve_error: ResolveError,
    },
    UnresolvedSymbol {
        range: TextRange,
        specifier: TokenText,
        export_name: Text,
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
            Self::UnresolvedPath { specifier, .. } => specifier.text(),
            Self::UnresolvedSymbol { specifier, .. } => specifier.text(),
        }
    }
}

impl Rule for NoUnresolvedImports {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = NoUnresolvedImportsState;
    type Signals = Vec<Self::State>;
    type Options = NoUnresolvedImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(owner) = ctx.module_info_for_path(ctx.file_path()) else {
            return Vec::new();
        };
        let ModuleInfoKind::Js(module_info) = owner.kind(ctx.db()) else {
            return Vec::new();
        };

        let node = ctx.query();
        let Some(import_path) = module_info.get_import_path_by_js_node(node) else {
            return Vec::new();
        };
        let resolved = import_path.resolve_js(ctx.db(), owner);

        let Some(specifier) = node.inner_string_text() else {
            return Vec::new();
        };

        let (resolved_path, runtime_fallback) = match resolved.path().as_deref() {
            Ok(resolved_path) => (resolved_path.to_path_buf(), false),
            Err(resolve_error) => {
                // Runtime built-ins (e.g. `node:fs`, `bun:sqlite`) are valid
                // imports — they simply cannot be resolved to a file path.
                if *resolve_error == ResolveError::NodeBuiltIn
                    || *resolve_error == ResolveError::BunBuiltIn
                {
                    return Vec::new();
                }

                if Utf8Path::new(&specifier)
                    .extension()
                    .is_some_and(|extension| !SUPPORTED_EXTENSIONS.contains(&extension))
                {
                    return Vec::new();
                }

                let imports_types = match node {
                    AnyJsImportLike::JsModuleSource(source) => {
                        source.imports_only_types()
                            || source
                                .syntax()
                                .grand_parent()
                                .and_then(TsImportEqualsDeclaration::cast)
                                .is_some_and(|declaration| declaration.type_token().is_some())
                            || source.parent::<AnyJsImportClause>().is_some_and(|clause| {
                                clause.named_specifiers().is_some_and(|specifiers| {
                                    specifiers.specifiers().iter().any(|specifier| {
                                        specifier
                                            .is_ok_and(|specifier| specifier.imports_only_types())
                                    })
                                })
                            })
                            || source
                                .parent::<JsExportNamedFromClause>()
                                .is_some_and(|clause| {
                                    clause.specifiers().iter().any(|specifier| {
                                        specifier
                                            .is_ok_and(|specifier| specifier.type_token().is_some())
                                    })
                                })
                    }
                    _ => false,
                };
                let runtime_path = (!imports_types)
                    .then(|| {
                        resolve_module_import(
                            ctx.db(),
                            owner,
                            &specifier,
                            ResolutionMode::JavaScriptRuntime,
                        )
                    })
                    .and_then(|resolved| resolved.path().as_ref().ok().cloned());
                match runtime_path {
                    Some(path) => (path, true),
                    None => {
                        return vec![NoUnresolvedImportsState::UnresolvedPath {
                            range: node.syntax().text_trimmed_range(),
                            specifier,
                            resolve_error: *resolve_error,
                        }];
                    }
                }
            }
        };

        let Some(target_info) = ctx.module_info_for_path(&resolved_path) else {
            return Vec::new();
        };

        if runtime_fallback {
            let ModuleInfoKind::Js(info) = target_info.kind(ctx.db()) else {
                return Vec::new();
            };
            let esm = has_runtime_module_syntax(info)
                || resolved_path.extension() == Some("mjs")
                || (resolved_path.extension() == Some("js")
                    && ctx
                        .project_layout()
                        .find_node_manifest_for_path(&resolved_path)
                        .is_some_and(|(_, manifest)| manifest.r#type == Some(PackageType::Module)));
            if !esm {
                // CommonJS exports cannot be validated without declarations.
                return Vec::new();
            }
        }

        let options = GetUnresolvedImportsOptions {
            module_db: ctx.db(),
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
                if export_name.text() == "default" =>
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
                        "The "{specifier_kind}" "<Emphasis>{specifier}</Emphasis>" has no export named "<Emphasis>{format_args!("{}", export_name)}</Emphasis>"."
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

fn has_runtime_module_syntax(module: &JsModuleInfo) -> bool {
    let model = &module.semantic_model;
    if model.root().syntax().descendants().any(|node| {
        if JsImport::can_cast(node.kind())
            || JsExport::can_cast(node.kind())
            || JsImportMetaExpression::can_cast(node.kind())
        {
            return true;
        }
        let awaits = JsAwaitExpression::can_cast(node.kind())
            || JsForOfStatement::cast_ref(&node)
                .is_some_and(|statement| statement.await_token().is_some())
            || JsVariableDeclaration::cast_ref(&node)
                .is_some_and(|declaration| declaration.await_token().is_some());
        awaits
            && !node
                .ancestors()
                .any(|ancestor| AnyFunctionLike::can_cast(ancestor.kind()))
    }) {
        return true;
    }
    ["require", "module", "exports", "__dirname", "__filename"]
        .into_iter()
        .any(|name| {
            model
                .global_scope()
                .get_binding(name)
                .is_some_and(|binding| {
                    matches!(
                        binding.declaration_kind(),
                        JsDeclarationKind::Value
                            | JsDeclarationKind::Class
                            | JsDeclarationKind::Using
                    )
                })
        })
}

struct GetUnresolvedImportsOptions<'a> {
    /// The module database to use for further lookups.
    module_db: &'a dyn ModuleDb,

    /// The path of the module we're importing from.
    specifier: TokenText,

    /// Module info of the module we're importing from.
    target_info: ModuleInfo,
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
                specifier: options.specifier.clone(),
                export_name: imported_name,
            }
        })
    })
}

fn has_exported_symbol(import_name: &Text, options: &GetUnresolvedImportsOptions) -> bool {
    let lookup = find_js_exported_symbol(
        options.module_db,
        SymbolFromModuleInfo::new(options.module_db, import_name.text(), options.target_info),
    );
    // `Unknown` means a re-export target could not be resolved, so the symbol
    // may exist. Only report symbols that are missing for certain.
    !matches!(lookup, JsExportedSymbolLookup::Missing)
}

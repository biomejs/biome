use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::{fmt::Display, markup};
use biome_dependency_graph::{DependencyGraph, Import, ModuleDependencyData};
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyJsImportClause, AnyJsImportLike, JsDefaultImportSpecifier, JsLanguage, JsModuleSource,
    inner_string_text,
};
use biome_rowan::{AstNode, SyntaxNode, SyntaxResult, TextRange, TokenText};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

use crate::services::dependency_graph::ResolvedImports;

declare_lint_rule! {
    /// Restricts imports of private exports.
    ///
    /// In JavaScript and TypeScript, as soon as you `export` a symbol, such as
    /// a type, function, or anything else that can be exported, it is
    /// considered public and can be imported from anywhere else. Unfortunately,
    /// this makes it hard to enforce module boundaries, or to prevent importing
    /// things that were only exported for test purposes, for instance.
    ///
    /// This rule recognizes the JSDoc annotations `@public`, `@package`, and
    /// `@private` so that you are free to set the visibility of exports.
    /// Exports without annotation have a default visibility of **public**, but
    /// this can be configured.
    ///
    /// By enabling this rule, all exported symbols, such as types, functions
    /// or other things that may be exported, are considered to be "package
    /// private". This means that modules that reside in the same directory, as
    /// well as submodules of those "sibling" modules, are allowed to import
    /// them, while any other modules that are further away in the file system
    /// are restricted from importing them. A symbol's visibility may be
    /// extended by re-exporting from an index file.
    ///
    /// Notes:
    ///
    /// * This rule only applies to imports for JavaScript and TypeScript
    ///   files. Imports for resources such as images or CSS files are exempted
    ///   regardless of the default visibility setting.
    ///
    /// Source: https://github.com/uhyo/eslint-plugin-import-access
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// **`sub/foo.js`**
    /// ```js
    /// /**
    ///  * @package
    ///  */
    /// export const fooPackageVariable = 1;
    /// ```
    ///
    /// **`bar.js`**
    /// ```js
    /// // Attempt to import package private variable from `sub/foo.js` from
    /// // outside its `sub` module:
    /// import { fooPackageVariable } from "./sub/foo.js";
    ///
    /// /**
    ///  * @private For test purposes only!
    ///  */
    /// export function getTestStuff() {}
    /// ```
    ///
    /// **`bar.test.js`**
    /// // Attempt to import a private export. To allow this, you probably want
    /// // to configure an `override` to disable this rule in test files.
    /// // See: https://biomejs.dev/reference/configuration/#overrides
    /// ```js
    /// import { getTestStuff } from "./bar.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// **`sub/index.js`**
    /// ```js
    /// // Package-private exports can be imported from inside the same module.
    /// import { fooPackageVariable } from "./foo.js";
    ///
    /// // Resources (anything other than JS/TS files) are always exempt.
    /// import { barResource } from "../resources/bar.png";
    ///
    /// /** @private */
    /// export const subPrivateVariable = 2;
    /// ```
    ///
    /// **`sub/deep/index.js`**
    /// ```js
    /// // Private exports are accessible within the same module only, but
    /// // modules can be nested. So the following works because you can always
    /// // import from the index file of a parent module:
    /// import { subPrivateVariable } from "../index.js";
    /// ```
    ///
    pub NoPrivateImports {
        version: "next",
        name: "noPrivateImports",
        language: "js",
        sources: &[
            RuleSource::EslintImportAccess("eslint-plugin-import-access")
        ],
        recommended: true,
    }
}

/// Options for the rule `noPrivateImports`.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoPrivateImportsOptions {
    /// The default visibility to assume for symbols without annotation.
    ///
    /// Default: **public**.
    pub default_visibility: Visibility,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
enum Visibility {
    #[default]
    Public,
    Package,
    Private,
}

impl Display for Visibility {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            Visibility::Public => fmt.write_str("public"),
            Visibility::Package => fmt.write_str("package"),
            Visibility::Private => fmt.write_str("private"),
        }
    }
}

pub struct NoPrivateImportsState {
    range: TextRange,

    /// The path where the visibility of the imported symbol is defined.
    path: String,

    /// The visibility of the offending symbol.
    visibility: Visibility,
}

impl Rule for NoPrivateImports {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = NoPrivateImportsState;
    type Signals = Vec<Self::State>;
    type Options = NoPrivateImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(file_imports) = ctx.imports_for_path(ctx.file_path()) else {
            return Vec::new();
        };

        let node = ctx.query();
        let Some(target_data) = file_imports
            .get_import_by_node(node)
            .and_then(|import| import.resolved_path.as_ref().ok())
            .and_then(|target_path| ctx.imports_for_path(&target_path))
        else {
            return Vec::new();
        };

        let options = GetRestrictedImportOptions {
            dependency_graph: ctx
                .get_service()
                .expect("Dependency graph must be initialised"),
            target_data,
            default_visibility: ctx.options().default_visibility,
        };

        let result = match node {
            AnyJsImportLike::JsModuleSource(node) => {
                get_restricted_imports_from_module_source(node, options)
            }
            AnyJsImportLike::JsCallExpression(node) => todo!(),
            AnyJsImportLike::JsImportCallExpression(node) => todo!(),
        };

        result.unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "You may not import a symbol with "<Emphasis>{state.visibility}</Emphasis>" visibility from here."
            },
        )
        .note(markup! {
            "You may need to import an alternative symbol, or relax the visibility of this symbol."
        })
        .note(markup! {
            "The visibility for this symbol is defined in "<Emphasis>{state.path}</Emphasis>"."
        });

        Some(diagnostic)
    }
}

struct GetRestrictedImportOptions<'a> {
    /// Reference to the dependency graph for looking up additional imports.
    dependency_graph: &'a DependencyGraph,

    /// Dependency data of the target module we're importing from.
    target_data: ModuleDependencyData,

    /// The visibility to assume for symbols without explicit visibility
    /// annotation.
    default_visibility: Visibility,
}

fn get_restricted_imports_from_module_source(
    node: &JsModuleSource,
    options: GetRestrictedImportOptions,
) -> SyntaxResult<Vec<NoPrivateImportsState>> {
    let results = match node.syntax().parent().and_then(AnyJsImportClause::cast) {
        Some(AnyJsImportClause::JsImportCombinedClause(node)) => todo!(),
        Some(AnyJsImportClause::JsImportDefaultClause(node)) => get_restricted_import(
            node.default_specifier()
                .map(JsDefaultImportSpecifier::into_syntax)?,
            &options,
        )?
        .into_iter()
        .collect(),
        Some(AnyJsImportClause::JsImportNamedClause(node)) => todo!(),
        Some(
            AnyJsImportClause::JsImportBareClause(_)
            | AnyJsImportClause::JsImportNamespaceClause(_),
        )
        | None => Vec::new(),
    };

    Ok(results)
}

/// Returns `Some` signal if the given `specifier_node` references an import
/// that is more private than allowed.
fn get_restricted_import(
    specifier_node: SyntaxNode<JsLanguage>,
    options: &GetRestrictedImportOptions,
) -> SyntaxResult<Option<NoPrivateImportsState>> {
    let symbol_name = specifier_node.text_trimmed();

    if !module_path.starts_with('.') {
        return None;
    }

    let mut path_parts: Vec<_> = module_path.text().split('/').collect();
    let mut index_filename = None;

    // TODO. The implementation could be optimized further by using
    // `Path::new(module_path.text())` for further inspiration see `use_import_extensions` rule.
    if let Some(extension) = get_extension(&path_parts) {
        if !SOURCE_EXTENSIONS.contains(&extension) {
            return None; // Resource files are exempt.
        }

        if let Some(basename) = get_basename(&path_parts) {
            if INDEX_BASENAMES.contains(&basename) {
                // We pop the index file because it shouldn't count as a path,
                // component, but we store the file name so we can add it to
                // both the reported path and the suggestion.
                index_filename = path_parts.last().copied();
                path_parts.pop();
            }
        }
    }

    let is_restricted = path_parts
        .iter()
        .filter(|&&part| part != "." && part != "..")
        .count()
        > 1;
    if !is_restricted {
        return None;
    }

    let mut suggestion_parts = path_parts[..path_parts.len() - 1].to_vec();

    // Push the index file if it exists. This makes sure the reported path
    // matches the import path exactly.
    if let Some(index_filename) = index_filename {
        path_parts.push(index_filename);

        // Assumes the user probably wants to use an index file that has the
        // same name as the original.
        suggestion_parts.push(index_filename);
    }

    Some(NoPrivateImportsState {
        range,
        path: path_parts.join("/"),
        suggestion: suggestion_parts.join("/"),
    })
}

fn get_basename<'a>(path_parts: &'_ [&'a str]) -> Option<&'a str> {
    path_parts.last().map(|&part| match part.find('.') {
        Some(dot_index) if dot_index > 0 && dot_index < part.len() - 1 => &part[..dot_index],
        _ => part,
    })
}

fn get_extension<'a>(path_parts: &'_ [&'a str]) -> Option<&'a str> {
    path_parts.last().and_then(|part| match part.find('.') {
        Some(dot_index) if dot_index > 0 && dot_index < part.len() - 1 => {
            Some(&part[dot_index + 1..])
        }
        _ => None,
    })
}

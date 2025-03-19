use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::{fmt::Display, markup};
use biome_dependency_graph::{Export, ModuleDependencyData};
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyJsImportClause, AnyJsImportLike, AnyJsNamedImportSpecifier, JsModuleSource, JsSyntaxToken,
};
use biome_rowan::{AstNode, SyntaxResult, Text, TextRange};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
pub enum Visibility {
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

impl FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Visibility::Public),
            "package" => Ok(Visibility::Package),
            "private" => Ok(Visibility::Private),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
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
        let self_path = ctx.file_path();
        let Some(file_imports) = ctx.imports_for_path(ctx.file_path()) else {
            return Vec::new();
        };

        let node = ctx.query();
        let Some(target_path) = file_imports
            .get_import_by_node(node)
            .and_then(|import| import.resolved_path.as_ref().ok())
        else {
            return Vec::new();
        };

        let Some(target_data) = ctx.imports_for_path(target_path) else {
            return Vec::new();
        };

        let options = GetRestrictedImportOptions {
            self_path,
            target_path,
            target_data,
            default_visibility: ctx.options().default_visibility,
        };

        let result = match node {
            AnyJsImportLike::JsModuleSource(node) => {
                get_restricted_imports_from_module_source(node, &options)
            }

            // TODO: require() and import() calls should also be handled here, but tracking the
            //       bindings to get the used symbol names is not easy. I think we can leave it
            //       for future opportunities.
            _ => Ok(Vec::new()),
        };

        result.unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let cwd = Utf8PathBuf::from(
            std::env::current_dir()
                .map(|cwd| cwd.to_string_lossy().to_string())
                .unwrap_or_default(),
        );

        // Use the relative path if possible.
        let path = Utf8PathBuf::from(&state.path);
        let path = path.strip_prefix(&cwd).unwrap_or(&path).to_string();

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
            "The visibility for this symbol is defined in "<Emphasis>{path}</Emphasis>"."
        });

        Some(diagnostic)
    }
}

struct GetRestrictedImportOptions<'a> {
    /// The self module path we're importing to.
    self_path: &'a Utf8Path,

    /// The target module path we're importing from.
    target_path: &'a Utf8Path,

    /// Dependency data of the target module we're importing from.
    target_data: ModuleDependencyData,

    /// The visibility to assume for symbols without explicit visibility
    /// annotation.
    default_visibility: Visibility,
}

fn get_restricted_imports_from_module_source(
    node: &JsModuleSource,
    options: &GetRestrictedImportOptions,
) -> SyntaxResult<Vec<NoPrivateImportsState>> {
    let path = options.target_path.to_string();

    let results = match node.syntax().parent().and_then(AnyJsImportClause::cast) {
        Some(AnyJsImportClause::JsImportCombinedClause(node)) => {
            let range = node.default_specifier()?.range();
            get_restricted_import(&Text::Static("default"), options)
                .map(|visibility| NoPrivateImportsState {
                    range,
                    path: path.clone(),
                    visibility,
                })
                .into_iter()
                .chain(
                    node.specifier()?
                        .as_js_named_import_specifiers()
                        .map(|specifiers| specifiers.specifiers())
                        .into_iter()
                        .flatten()
                        .flatten()
                        .filter_map(get_named_specifier_import_name)
                        .filter_map(|name| {
                            get_restricted_import(
                                &Text::Borrowed(name.token_text_trimmed()),
                                options,
                            )
                            .map(|visibility| NoPrivateImportsState {
                                range: name.text_trimmed_range(),
                                path: path.clone(),
                                visibility,
                            })
                        }),
                )
                .collect()
        }
        Some(AnyJsImportClause::JsImportDefaultClause(node)) => {
            let range = node.default_specifier()?.range();
            get_restricted_import(&Text::Static("default"), options)
                .map(|visibility| NoPrivateImportsState {
                    range,
                    path,
                    visibility,
                })
                .into_iter()
                .collect()
        }
        Some(AnyJsImportClause::JsImportNamedClause(node)) => node
            .named_specifiers()?
            .specifiers()
            .into_iter()
            .flatten()
            .filter_map(get_named_specifier_import_name)
            .filter_map(|name| {
                get_restricted_import(&Text::Borrowed(name.token_text_trimmed()), options).map(
                    |visibility| NoPrivateImportsState {
                        range: name.text_trimmed_range(),
                        path: path.clone(),
                        visibility,
                    },
                )
            })
            .collect(),
        Some(
            AnyJsImportClause::JsImportBareClause(_)
            | AnyJsImportClause::JsImportNamespaceClause(_),
        )
        | None => Vec::new(),
    };

    Ok(results)
}

fn get_named_specifier_import_name(specifier: AnyJsNamedImportSpecifier) -> Option<JsSyntaxToken> {
    match specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
            specifier.name().ok().and_then(|name| name.value().ok())
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => specifier
            .local_name()
            .ok()
            .and_then(|binding| binding.as_js_identifier_binding()?.name_token().ok()),
        _ => None,
    }
}

/// Returns `Some` signal if the given `import_name` references an import
/// that is more private than allowed.
fn get_restricted_import(
    import_name: &Text,
    options: &GetRestrictedImportOptions,
) -> Option<Visibility> {
    let visibility = options
        .target_data
        .exports
        .get(import_name)
        .and_then(|export| match export {
            Export::Own(export) => export.jsdoc_comment.as_deref().and_then(parse_visibility),

            // TODO: Should we follow re-exports here? I think re-exports don't inherit the
            //       visibility from where the name is declared; e.g. package-private symbols can be
            //       re-exported from index.js to make it public. Thus we can fallback to the
            //       default visibility if they're re-exported and not added any visibility there.
            _ => None,
        })
        .unwrap_or(options.default_visibility);

    let is_restricted = match visibility {
        Visibility::Public => false,
        Visibility::Private => true,
        Visibility::Package => options.target_path.parent() != options.self_path.parent(),
    };

    is_restricted.then_some(visibility)
}

/// Parses a JSDoc comment to find the first `@public`, `@package`, or `@private` tag.
fn parse_visibility(jsdoc_comment: &str) -> Option<Visibility> {
    jsdoc_comment
        .lines()
        .find_map(|line| {
            line.strip_prefix("@")
                .and_then(|tag| tag.split_whitespace().next())
        })
        .and_then(|tag| Visibility::from_str(tag).ok())
}

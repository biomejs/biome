use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::{fmt::Display, markup};
use biome_dependency_graph::{DependencyGraph, ModuleDependencyData};
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

const INDEX_BASENAMES: &[&str] = &["index", "mod"];

declare_lint_rule! {
    /// Restricts imports of private exports.
    ///
    /// In JavaScript and TypeScript, as soon as you `export` a symbol, such as
    /// a type, function, or anything else that can be exported, it is
    /// considered public and can be imported from anywhere else. Unfortunately,
    /// this makes it hard to enforce module boundaries, or to prevent importing
    /// things that were only exported for test purposes, for instance.
    ///
    /// This rule recognizes the JSDoc tags `@public`, `@package`, and
    /// `@private` so that you are free to set the visibility of exports.
    /// Exports without tag have a default visibility of **public**, but  this
    /// can be configured.
    ///
    /// The `@access` tag is also supported if it's used with one of the values
    /// `public`, `package`, or `private`.
    ///
    /// ## Public visibility
    ///
    /// Public visibility is the default and means there are no restrictions for
    /// importing a given symbol. In other words, without this rule, all
    /// exported symbols are implicitly public.
    ///
    /// ## Package visibility
    ///
    /// Within the context of this rule, _package visibility_ means that a
    /// symbol is visible within the same "package", which means that any module
    /// that resides in the same folder, or one of its subfolders, is allowed to
    /// import the symbol. Modules that only share a common folder higher up in
    /// the hierarchy are not allowed to import the symbol.
    ///
    /// For a visual explanation, see
    /// [this illustration](https://github.com/uhyo/eslint-plugin-import-access?tab=readme-ov-file#what).
    ///
    /// ## Private visibility
    ///
    /// Private visibility means that a symbol may not be imported from other
    /// modules.
    ///
    /// The key thing to understanding the usefulness of `@private` is that
    /// this rule doesn't treat modules and files as one and the same thing.
    /// While files are indeed modules, folders are considered modules too, with
    /// their files and subfolders being submodules. Therefore, symbols exported
    /// as `@private` from an index file, such as `index.js`, can _still_ be
    /// imported from other submodules in that same module.
    ///
    /// :::note
    /// For the sake of compatibility with conventions used with Deno, modules
    /// named `mod.js`/`mod.ts` are considered index files too.
    /// :::
    ///
    /// Another reason why private visibility may still be useful is that it
    /// allows you to choose specific exceptions. For example, using
    /// [overrides](https://biomejs.dev/reference/configuration/#overrides), you
    /// may want to disable this rule in all files with a `.test.js` extension.
    /// This way, symbols marked private cannot be imported from outside
    /// modules, with the exception of test files.
    ///
    /// ## Known Limitations
    ///
    /// * This rule currently only looks at the JSDoc comments that are attached
    ///   to the _`export` statement_ nearest to the symbol's definition. If the
    ///   symbol isn't exported in the same statement as in which it is defined,
    ///   the visibility as specified in the `export` statement is used, not
    ///   that of the symbol definition. Re-exports cannot (currently) override
    ///   the visibility from the original `export`.
    /// * This rule only applies to imports from JavaScript and TypeScript
    ///   files. Imports for resources such as images or CSS files are exempted
    ///   regardless of the default visibility setting.
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
    /// The default visibility to assume for symbols without visibility tag.
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
            dependency_graph: ctx.dependency_graph(),
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
        let path = Utf8Path::new(&state.path);
        let path = path.strip_prefix(&cwd).unwrap_or(path).as_str();

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
            "This symbol was imported from "<Emphasis>{path}</Emphasis>"."
        });

        Some(diagnostic)
    }
}

struct GetRestrictedImportOptions<'a> {
    /// The dependency graph to use for further lookups.
    dependency_graph: &'a DependencyGraph,

    /// The self module path we're importing to.
    self_path: &'a Utf8Path,

    /// The target module path we're importing from.
    target_path: &'a Utf8Path,

    /// Dependency data of the target module we're importing from.
    target_data: ModuleDependencyData,

    /// The visibility to assume for symbols without explicit visibility tag.
    default_visibility: Visibility,
}

impl GetRestrictedImportOptions<'_> {
    /// Returns whether [Self::target_path] is within the same module as
    /// [Self::self_path].
    fn target_path_is_in_same_module(&self) -> bool {
        target_path_is_in_same_module_as_self_path(self.target_path, self.self_path)
    }

    /// Returns whether [Self::target_path] is within the same package as
    /// [Self::self_path].
    fn target_path_is_in_same_package(&self) -> bool {
        target_path_is_in_same_package_as_self_path(self.target_path, self.self_path)
    }
}

fn get_restricted_imports_from_module_source(
    node: &JsModuleSource,
    options: &GetRestrictedImportOptions,
) -> SyntaxResult<Vec<NoPrivateImportsState>> {
    let path = options.target_path.to_string();

    let results = match node.syntax().parent().and_then(AnyJsImportClause::cast) {
        Some(AnyJsImportClause::JsImportCombinedClause(node)) => {
            let range = node.default_specifier()?.range();
            get_restricted_import_visibility(&Text::Static("default"), options)
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
                            get_restricted_import_visibility(
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
            get_restricted_import_visibility(&Text::Static("default"), options)
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
                get_restricted_import_visibility(
                    &Text::Borrowed(name.token_text_trimmed()),
                    options,
                )
                .map(|visibility| NoPrivateImportsState {
                    range: name.text_trimmed_range(),
                    path: path.clone(),
                    visibility,
                })
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

/// Returns the visibility of the symbol exported as the given `import_name`,
/// if (and only if) that symbol has a stricter visibility than allowed.
fn get_restricted_import_visibility(
    import_name: &Text,
    options: &GetRestrictedImportOptions,
) -> Option<Visibility> {
    let visibility = options
        .target_data
        .find_exported_symbol(options.dependency_graph, import_name.text())
        .and_then(|export| export.jsdoc_comment.as_deref().and_then(parse_visibility))
        .unwrap_or(options.default_visibility);

    let is_restricted = match visibility {
        Visibility::Public => false,
        Visibility::Private => !options.target_path_is_in_same_module(),
        Visibility::Package => !options.target_path_is_in_same_package(),
    };

    is_restricted.then_some(visibility)
}

/// Searches JSDoc comments to find the first `@public`, `@package`, `@private`,
/// or `@access` tag, and maps it to one of the supported [Visibility] values,
/// if possible.
fn parse_visibility(jsdoc_comment: &str) -> Option<Visibility> {
    jsdoc_comment.lines().find_map(|line| {
        line.strip_prefix('@')
            .map(|tag| tag.strip_prefix("access ").unwrap_or(tag))
            .and_then(|tag| tag.split_whitespace().next())
            .and_then(|tag| Visibility::from_str(tag).ok())
    })
}

/// Returns whether `target_path` is within the same module as `self_path`.
#[inline]
fn target_path_is_in_same_module_as_self_path(
    target_path: &Utf8Path,
    self_path: &Utf8Path,
) -> bool {
    if !target_path
        .file_stem()
        .is_some_and(|stem| INDEX_BASENAMES.contains(&stem))
    {
        return false;
    }

    let Some(target_parent) = target_path.parent() else {
        // If we cannot navigate further up from the target path, it means the
        // target is in the root, which means everything else is in the same
        // module as it.
        return true;
    };

    self_path
        .ancestors()
        .any(|ancestor| ancestor == target_parent)
}

/// Returns whether `target_path` is within the same package as `self_path`.
#[inline]
fn target_path_is_in_same_package_as_self_path(
    target_path: &Utf8Path,
    self_path: &Utf8Path,
) -> bool {
    let target_path = if target_path
        .file_stem()
        .is_some_and(|stem| INDEX_BASENAMES.contains(&stem))
    {
        target_path.parent().unwrap_or(Utf8Path::new("."))
    } else {
        target_path
    };

    let Some(target_parent) = target_path.parent() else {
        // If we cannot navigate further up from the target path, it means the
        // target is in the root, which means everything else is in the same
        // package as it.
        return true;
    };

    self_path
        .ancestors()
        .any(|ancestor| ancestor == target_parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_path_is_in_same_module_as_self_path() {
        assert!(target_path_is_in_same_module_as_self_path(
            Utf8Path::new("index.js"),
            Utf8Path::new("self.js")
        ));
        assert!(target_path_is_in_same_module_as_self_path(
            Utf8Path::new("index.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(target_path_is_in_same_module_as_self_path(
            Utf8Path::new("./index.js"),
            Utf8Path::new("./nested/self.js")
        ));
        assert!(target_path_is_in_same_module_as_self_path(
            Utf8Path::new("./nested/index.js"),
            Utf8Path::new("./nested/nested/self.js")
        ));

        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("target.js"),
            Utf8Path::new("self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("target.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("./target.js"),
            Utf8Path::new("./nested/self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("target/index.js"),
            Utf8Path::new("self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("target/index.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("target/private.js"),
            Utf8Path::new("self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("target/private.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(!target_path_is_in_same_module_as_self_path(
            Utf8Path::new("./target/private.js"),
            Utf8Path::new("./self.js")
        ));
    }

    #[test]
    fn test_target_path_is_in_same_package_as_self_path() {
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("index.js"),
            Utf8Path::new("self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("index.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("./index.js"),
            Utf8Path::new("./nested/self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("./nested/index.js"),
            Utf8Path::new("./nested/nested/self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("target.js"),
            Utf8Path::new("self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("target.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("./target.js"),
            Utf8Path::new("./nested/self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("target/index.js"),
            Utf8Path::new("self.js")
        ));
        assert!(target_path_is_in_same_package_as_self_path(
            Utf8Path::new("target/index.js"),
            Utf8Path::new("nested/self.js")
        ));

        assert!(!target_path_is_in_same_package_as_self_path(
            Utf8Path::new("target/private.js"),
            Utf8Path::new("self.js")
        ));
        assert!(!target_path_is_in_same_package_as_self_path(
            Utf8Path::new("target/private.js"),
            Utf8Path::new("nested/self.js")
        ));
        assert!(!target_path_is_in_same_package_as_self_path(
            Utf8Path::new("./target/private.js"),
            Utf8Path::new("./self.js")
        ));
    }
}

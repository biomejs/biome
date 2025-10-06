use crate::services::module_graph::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_fs::BiomePath;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike, JsModuleSource};
use biome_jsdoc_comment::JsdocComment;
use biome_module_graph::{JsImportPath, JsModuleInfo, ModuleGraph};
use biome_rowan::{AstNode, Text, TextRange};
use biome_rule_options::no_private_imports::{NoPrivateImportsOptions, Visibility};
use camino::{Utf8Path, Utf8PathBuf};
use std::str::FromStr;

const INDEX_BASENAMES: &[&str] = &["index", "mod"];

declare_lint_rule! {
    /// Restrict imports of private exports.
    ///
    /// In JavaScript and TypeScript, as soon as you `export` a symbol, such as
    /// a type, function, or anything else that can be exported, it is
    /// considered public and can be imported from anywhere else. Unfortunately,
    /// this makes it hard to enforce module boundaries, or to prevent importing
    /// things that were only exported for test purposes, for instance.
    ///
    /// This rule recognizes the JSDoc tags `@public`, `@package`, and
    /// `@private` so that you are free to set the visibility of exports.
    /// Exports without tag have a default visibility of **public**, but this
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
    /// The key thing to understanding the usefulness of `@private` is that this
    /// rule doesn't treat modules and files as one and the same thing. While
    /// files are indeed modules, folders are considered modules too, with their
    /// files and subfolders being submodules. Therefore, symbols exported as
    /// `@private` from an index file, such as `index.js`, can _still_ be
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
    /// * This rule only applies to imports from JavaScript and TypeScript
    ///   files. Imports for resources such as images or CSS files are exempted
    ///   regardless of the default visibility setting.
    /// * This rule does not validate imports through dynamic `import()`
    ///   expressions or CommonJS `require()` calls.
    /// * Imports from dependencies under `node_modules` are considered out of
    ///   scope.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,file=sub/foo.js
    /// /**
    ///  * @package
    ///  */
    /// export const fooPackageVariable = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic,file=bar.js
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
    /// ```js,expect_diagnostic,file=bar.test.js
    /// // Attempt to import a private export. To allow this, you probably want
    /// // to configure an `override` to disable this rule in test files.
    /// // See: https://biomejs.dev/reference/configuration/#overrides
    /// import { getTestStuff } from "./bar.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=sub/index.js
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
    /// ```js,file=sub/deep/index.js
    /// // Private exports are accessible within the same module only, but
    /// // modules can be nested. So the following works because you can always
    /// // import from the index file of a parent module:
    /// import { subPrivateVariable } from "../index.js";
    /// ```
    pub NoPrivateImports {
        version: "2.0.0",
        name: "noPrivateImports",
        language: "js",
        sources: &[
            RuleSource::EslintImportAccess("eslint-plugin-import-access").same()
        ],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Project],
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
        let Some(module_info) = ctx.module_info_for_path(ctx.file_path()) else {
            return Vec::new();
        };

        let node = ctx.query();
        let Some(target_path) = node
            .is_static_import()
            .then(|| node.inner_string_text())
            .flatten()
            .and_then(|specifier| module_info.static_import_paths.get(specifier.text()))
            .and_then(JsImportPath::as_path)
            .filter(|path| !BiomePath::new(path).is_dependency())
        else {
            return Vec::new();
        };

        let Some(target_info) = ctx.module_info_for_path(target_path) else {
            return Vec::new();
        };

        let options = GetRestrictedImportOptions {
            module_graph: ctx.module_graph(),
            self_path,
            target_path,
            target_info,
            default_visibility: ctx.options().default_visibility,
        };

        match node {
            AnyJsImportLike::JsModuleSource(node) => {
                get_restricted_imports_from_module_source(node, &options)
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
    /// The module graph to use for further lookups.
    module_graph: &'a ModuleGraph,

    /// The self module path we're importing to.
    self_path: &'a Utf8Path,

    /// The target module path we're importing from.
    target_path: &'a Utf8Path,

    /// Module info of the target module we're importing from.
    target_info: JsModuleInfo,

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
) -> Vec<NoPrivateImportsState> {
    let Some(import_clause) = node.syntax().parent().and_then(AnyJsImportClause::cast) else {
        return Vec::new();
    };

    import_clause.filter_map_all_imported_symbols(|imported_name, range| {
        get_restricted_import_visibility(&imported_name, options).map(|visibility| {
            NoPrivateImportsState {
                range,
                path: options.target_path.to_string(),
                visibility,
            }
        })
    })
}

/// Returns the visibility of the symbol exported as the given `import_name`,
/// if (and only if) that symbol has a stricter visibility than allowed.
fn get_restricted_import_visibility(
    import_name: &Text,
    options: &GetRestrictedImportOptions,
) -> Option<Visibility> {
    let visibility = options
        .target_info
        .find_jsdoc_for_exported_symbol(options.module_graph, import_name.text())
        .as_ref()
        .and_then(parse_visibility)
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
fn parse_visibility(jsdoc_comment: &JsdocComment) -> Option<Visibility> {
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

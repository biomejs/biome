use crate::{
    services::manifest::Manifest,
    utils::{parse_package_name, path_alias_helper::is_path_alias_prefix},
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike};
use biome_resolver::is_builtin_node_module;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_undeclared_dependencies::NoUndeclaredDependenciesOptions;
use camino::Utf8PathBuf;

declare_lint_rule! {
    /// Disallow the use of dependencies that aren't specified in the `package.json`.
    ///
    /// Indirect dependencies will trigger the rule because they aren't declared in the `package.json`.
    /// This means that if the package `@org/foo` has a dependency on `lodash`, and then you use
    /// `import "lodash"` somewhere in your project, the rule will trigger a diagnostic for this import.
    ///
    /// The rule is meant to catch those dependencies that aren't declared inside the closest `package.json`, and
    /// isn't meant to detect dependencies declared in other manifest files, e.g. the root `package.json` in a monorepo setting.
    ///
    /// The rule ignores imports that are not valid package names.
    /// This includes internal imports that start with `#` and `@/` and imports with a protocol such as `node:`, `bun:`, `jsr:`, `https:`.
    ///
    /// Path aliases configured in `tsconfig.json` (e.g., `@components/*`) are also ignored, as they are not npm packages.
    ///
    /// To ensure that Visual Studio Code uses relative imports when it automatically imports a variable,
    /// you may set [`javascript.preferences.importModuleSpecifier` and `typescript.preferences.importModuleSpecifier`](https://code.visualstudio.com/docs/getstarted/settings) to `relative`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,file=index.js
    /// import "vite";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": {
    ///     "vite": "*"
    ///   }
    /// }
    /// ```
    ///
    /// ```js,file=index.js
    /// import "vite"; // package is correctly declared
    ///
    /// import assert from "node:assert"; // Node imports don't need declaration
    ///
    /// import { A } from "./local.js"; // relative imports don't trigger the rule
    /// import { B } from "#alias"; // same goes for aliases
    ///
    /// import { C } from "@components/Button"; // path alias in tsconfig.json
    /// ```
    ///
    /// ## Options
    ///
    /// This rule supports the following options:
    /// - `devDependencies`: If set to `false`, then the rule will show an error when `devDependencies` are imported. Defaults to `true`.
    /// - `peerDependencies`: If set to `false`, then the rule will show an error when `peerDependencies` are imported. Defaults to `true`.
    /// - `optionalDependencies`: If set to `false`, then the rule will show an error when `optionalDependencies` are imported. Defaults to `true`.
    /// - `bundleDependencies`: If set to `false`, then the rule will show an error when `bundleDependencies` are imported. Defaults to `true`.
    ///
    /// You can set the options like this:
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "devDependencies": false,
    ///     "peerDependencies": false,
    ///     "optionalDependencies": false,
    ///     "bundleDependencies": false
    ///   }
    /// }
    /// ```
    ///
    /// You can also use an array of globs instead of literal booleans.
    /// When using an array of globs, the setting will be set to `true` (no errors reported)
    /// if the name of the file being linted (i.e. not the imported file/module) matches a single glob
    /// in the array, and `false` otherwise.
    ///
    /// ### Example using the `devDependencies` option
    ///
    /// In this example, only test files can use dependencies in the `devDependencies` section.
    /// `dependencies`, `peerDependencies`, `optionalDependencies` and `bundleDependencies` are always available.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "devDependencies": ["**/tests/*.test.js", "**/tests/*.spec.js"]
    ///   }
    /// }
    /// ```
    ///
    /// ```json,file=package.json
    /// {
    ///   "devDependencies": {
    ///     "vite": "*"
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options,file=src/index.js
    /// // cannot import from a non-test file
    /// import "vite";
    /// ```
    ///
    /// ```js,use_options,file=tests/foo.test.js
    /// // this works, because the file matches a glob from the options
    /// import "vite";
    /// ```
    pub NoUndeclaredDependencies {
        version: "1.6.0",
        name: "noUndeclaredDependencies",
        language: "js",
        sources: &[RuleSource::EslintImport("no-extraneous-dependencies").same()],
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Project],
    }
}

pub struct RuleState {
    import_text: TokenText,
    is_dev_dependency_available: bool,
    is_peer_dependency_available: bool,
    is_optional_dependency_available: bool,
    is_bundle_dependency_available: bool,
}

impl Rule for NoUndeclaredDependencies {
    type Query = Manifest<AnyJsImportLike>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoUndeclaredDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }

        let path = ctx.file_path();
        let is_dev_dependency_available =
            // Type-only imports are always considered as dev dependencies.
            is_type_import(node)
                || ctx
                    .options()
                    .dev_dependencies
                    .as_ref()
                    .is_none_or(|dep| dep.is_available(path));
        let is_peer_dependency_available = ctx
            .options()
            .peer_dependencies
            .as_ref()
            .is_none_or(|dep| dep.is_available(path));
        let is_optional_dependency_available = ctx
            .options()
            .optional_dependencies
            .as_ref()
            .is_none_or(|dep| dep.is_available(path));
        let is_bundle_dependency_available = ctx
            .options()
            .bundle_dependencies
            .as_ref()
            .is_none_or(|dep| dep.is_available(path));

        let is_available = |package_name| {
            ctx.is_dependency(package_name)
                || (is_dev_dependency_available && ctx.is_dev_dependency(package_name))
                || (is_peer_dependency_available && ctx.is_peer_dependency(package_name))
                || (is_optional_dependency_available && ctx.is_optional_dependency(package_name))
                || (is_bundle_dependency_available && ctx.is_bundle_dependency(package_name))
        };

        let import_text = node.inner_string_text()?;
        let package_name = parse_package_name(import_text.text())?;

        // Check if it's a TypeScript path alias configured in tsconfig.json
        // This fixes Issue #10607: @components/Button should not trigger error if @components is a path alias
        if package_name.starts_with('@') {
            // Extract alias prefix: "@components/Button" -> "@components"
            let alias_prefix = package_name
                .split('/')
                .next()
                .unwrap_or(package_name);
            if is_path_alias_prefix(alias_prefix, &path.to_path_buf()) {
                return None; // Valid path alias, ignore this import
            }
        }

        if is_available(package_name)
            // Self package imports
            // TODO: we should also check that an `.` exports exists.
            // See https://nodejs.org/api/packages.html#self-referencing-a-package-using-its-name
            || ctx.name() == Some(package_name)
            // ignore Node.js builtin modules
            || is_builtin_node_module(package_name)
            // Ignore `bun` import
            || package_name == "bun"
        {
            return None;
        }

        if !package_name.starts_with('@') {
            // Handle DefinitelyTyped imports https://github.com/DefinitelyTyped/DefinitelyTyped
            // e.g. `lodash` can import types from `@types/lodash`.
            if let Some(import_clause) = node.parent::<AnyJsImportClause>()
                && import_clause.type_token().is_some()
            {
                let package_name = format!("@types/{package_name}");
                if is_available(&package_name) {
                    return None;
                }
            }
        }

        Some(RuleState {
            import_text,
            is_dev_dependency_available,
            is_peer_dependency_available,
            is_optional_dependency_available,
            is_bundle_dependency_available,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let RuleState {
            import_text,
            is_dev_dependency_available,
            is_peer_dependency_available,
            is_optional_dependency_available,
            is_bundle_dependency_available,
        } = state;
        let package_name = parse_package_name(import_text.text())?;

        let Some(package_path) = ctx.package_path.as_ref() else {
            return Some(RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Dependency "<Emphasis>{package_name}</Emphasis>" cannot be verified because no package.json file was found."
                },
            ));
        };

        let cwd = Utf8PathBuf::from(
            std::env::current_dir()
                .map(|cwd| cwd.to_string_lossy().to_string())
                .unwrap_or_default(),
        );

        let manifest_path = package_path
            .strip_prefix(&cwd)
            .unwrap_or(package_path)
            .join("package.json");

        let diag = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Dependency "<Emphasis>{package_name}</Emphasis>" isn't specified in "<Emphasis>{manifest_path.as_str()}</Emphasis>"."
            },
        );

        let available_in = if ctx.is_dev_dependency(package_name) && !is_dev_dependency_available {
            Some("devDependencies")
        } else if ctx.is_peer_dependency(package_name) && !is_peer_dependency_available {
            Some("peerDependencies")
        } else if ctx.is_optional_dependency(package_name) && !is_optional_dependency_available {
            Some("optionalDependencies")
        } else if ctx.is_bundle_dependency(package_name) && !is_bundle_dependency_available {
            Some("bundleDependencies")
        } else {
            None
        };

        if let Some(section) = available_in {
            Some(diag.note(markup! {
                <Emphasis>{package_name}</Emphasis>" is part of your "<Emphasis>{section}</Emphasis>", but it's not intended to be used in this file."
            }).note(markup! {
                "You may want to consider moving it to the "<Emphasis>"dependencies"</Emphasis>" section."
            }))
        } else {
            Some(
                diag.note(markup! { "This could lead to errors." })
                    .note(markup! { "Add the dependency in your manifest." }),
            )
        }
    }
}

fn is_type_import(import: &AnyJsImportLike) -> bool {
    match import.parent::<AnyJsImportClause>() {
        Some(clause) => clause.type_token().is_some(),
        _ => false,
    }
}
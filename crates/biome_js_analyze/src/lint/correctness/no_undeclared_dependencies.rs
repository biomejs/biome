use crate::services::manifest::Manifest;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of dependencies that aren't specified in the `package.json`.
    ///
    /// Indirect dependencies will trigger the rule because they aren't declared in the `package.json`.
    /// This means that if the package `@org/foo` has a dependency on `lodash`, and then you use
    /// `import "lodash"` somewhere in your project, the rule will trigger a diagnostic for this import.
    ///
    /// The rule ignores imports that are not valid package names.
    /// This includes internal imports that start with `#` and `@/` and imports with a protocol such as `node:`, `bun:`, `jsr:`, `https:`.
    ///
    /// To ensure that Visual Studio Code uses relative imports when it automatically imports a variable,
    /// you may set [`javascript.preferences.importModuleSpecifier` and `typescript.preferences.importModuleSpecifier`](https://code.visualstudio.com/docs/getstarted/settings) to `relative`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// import "vite";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// import { A } from "./local.js";
    /// ```
    ///
    /// ```js,ignore
    /// import assert from "node:assert";
    /// ```
    pub NoUndeclaredDependencies {
        version: "1.6.0",
        name: "noUndeclaredDependencies",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoUndeclaredDependencies {
    type Query = Manifest<AnyJsImportLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }

        let token_text = node.inner_string_text()?;
        let package_name = parse_package_name(token_text.text())?;
        if ctx.is_dependency(package_name)
            || ctx.is_dev_dependency(package_name)
            || ctx.is_peer_dependency(package_name)
            || ctx.is_optional_dependency(package_name)
            // Self package imports
            // TODO: we should also check that an `.` exports exists.
            // See https://nodejs.org/api/packages.html#self-referencing-a-package-using-its-name
            || ctx.name() == Some(package_name)
            // Ignore `bun` import
            || package_name == "bun"
        {
            return None;
        } else if !package_name.starts_with('@') {
            // Handle DefinitelyTyped imports https://github.com/DefinitelyTyped/DefinitelyTyped
            // e.g. `lodash` can import typ[es from `@types/lodash`.
            if let Some(import_clause) = node.parent::<AnyJsImportClause>() {
                if import_clause.type_token().is_some() {
                    let package_name = format!("@types/{package_name}");
                    if ctx.is_dependency(&package_name)
                        || ctx.is_dev_dependency(&package_name)
                        || ctx.is_peer_dependency(&package_name)
                        || ctx.is_optional_dependency(&package_name)
                    {
                        return None;
                    }
                }
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The current dependency isn't specified in your package.json."
                },
            )
            .note(markup! {
                "This could lead to errors."
            })
            .note(markup! {
                "Add the dependency in your manifest."
            }),
        )
    }
}

fn parse_package_name(path: &str) -> Option<&str> {
    let mut in_scope = false;
    for (i, c) in path.bytes().enumerate() {
        match c {
            b'@' if i == 0 => {
                in_scope = true;
            }
            // uppercase characters are not allowed in package name
            // and a package name cannot start with an underscore.
            // Here we are more tolerant and accept them.
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' => {}
            b'/' => {
                if in_scope {
                    if i == 1 {
                        // Invalid empty scope
                        // `@/`
                        return None;
                    } else {
                        // We consumed the scope.
                        // `@scope/`
                        in_scope = false;
                    }
                } else if i == 0 {
                    // absolute path
                    return None;
                } else {
                    // We consumed the package name
                    return Some(&path[..i]);
                }
            }
            _ => {
                return None;
            }
        }
    }
    // Handle cases where only the scope is given. e.g. `@scope/`
    (!path.ends_with('/')).then_some(path)
}

#[test]
fn test() {
    assert_eq!(
        parse_package_name("@scope/package-name"),
        Some("@scope/package-name")
    );
    assert_eq!(
        parse_package_name("@scope/package-name/path"),
        Some("@scope/package-name")
    );
    assert_eq!(parse_package_name("package_"), Some("package_"));
    assert_eq!(parse_package_name("package/path"), Some("package"));
    assert_eq!(parse_package_name("0"), Some("0"));
    assert_eq!(parse_package_name("0/path"), Some("0"));
    assert_eq!(parse_package_name("-"), Some("-"));
    assert_eq!(parse_package_name("-/path"), Some("-"));

    // Invalid package names that we accept
    assert_eq!(parse_package_name("PACKAGE"), Some("PACKAGE"));
    assert_eq!(parse_package_name("_"), Some("_"));

    // Invalid package names that we reject
    assert_eq!(parse_package_name("@/path"), None);
    assert_eq!(parse_package_name("."), None);
    assert_eq!(parse_package_name("./path"), None);
    assert_eq!(parse_package_name("#path"), None);
    assert_eq!(parse_package_name("/path"), None);
    assert_eq!(parse_package_name("p@ckage/name"), None);
}

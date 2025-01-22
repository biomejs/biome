use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike};
use biome_rowan::AstNode;
use camino::Utf8Path;

use crate::{globals::is_node_builtin_module, services::manifest::Manifest};

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
    ///
    /// ## Options
    ///
    /// **Since v2.0.0**
    ///
    /// This rule supports the following options:
    /// - `devDependencies`: If set to `false`, then the rule will show an error when `devDependencies` are imported. Defaults to `true`.
    /// - `peerDependencies`: If set to `false`, then the rule will show an error when `peerDependencies` are imported. Defaults to `true`.
    /// - `optionalDependencies`: If set to `false`, then the rule will show an error when `optionalDependencies` are imported. Defaults to `true`.
    ///
    /// You can set the options like this:
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "devDependencies": false,
    ///     "peerDependencies": false,
    ///     "optionalDependencies": false
    ///   }
    /// }
    /// ```
    ///
    /// You can also use an array of globs instead of literal booleans.
    /// When using an array of globs, the setting will be set to `true` (no errors reported)
    /// if the name of the file being linted (i.e. not the imported file/module) matches a single glob
    /// in the array, and `false` otherwise.
    ///
    /// In the following example, only test files can use dependencies in `devDependencies` section.
    /// `dependencies`, `peerDependencies`, and `optionalDependencies` are always available.
    ///
    /// ```json
    /// {
    ///   "options": {
    ///     "devDependencies": ["tests/*.test.js", "tests/*.spec.js"]
    ///   }
    /// }
    /// ```
    pub NoUndeclaredDependencies {
        version: "1.6.0",
        name: "noUndeclaredDependencies",
        language: "js",
        sources: &[
            RuleSource::EslintImport("no-extraneous-dependencies"),
        ],
        recommended: false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
enum DependencyAvailability {
    /// Dependencies are always available or unavailable.
    Bool(bool),

    /// Dependencies are available in files that matches any of the globs.
    Patterns(Box<[biome_glob::Glob]>),
}

impl Default for DependencyAvailability {
    fn default() -> Self {
        Self::Bool(true)
    }
}

impl Deserializable for DependencyAvailability {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type()? == DeserializableType::Bool {
            Self::Bool(bool::deserialize(ctx, value, name)?)
        } else {
            Self::Patterns(Deserializable::deserialize(ctx, value, name)?)
        })
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for DependencyAvailability {
    fn schema_name() -> String {
        "DependencyAvailability".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::*;

        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Boolean.into()),
                        metadata: Some(Box::new(Metadata {
                            description: Some("This type of dependency will be always available or unavailable.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(SingleOrVec::Single(Box::new(Schema::Object(SchemaObject {
                                instance_type: Some(InstanceType::String.into()),
                                ..Default::default()
                            })))),
                            min_items: Some(1),
                            ..Default::default()
                        })),
                        metadata: Some(Box::new(Metadata {
                            description: Some("This type of dependency will be available only if the linted file matches any of the globs.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl DependencyAvailability {
    fn is_available(&self, path: &Utf8Path) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Patterns(globs) => {
                biome_glob::CandidatePath::new(&path).matches_with_exceptions(globs)
            }
        }
    }
}

/// Rule's options
#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoUndeclaredDependenciesOptions {
    /// If set to `false`, then the rule will show an error when `devDependencies` are imported. Defaults to `true`.
    #[serde(default)]
    dev_dependencies: DependencyAvailability,

    /// If set to `false`, then the rule will show an error when `peerDependencies` are imported. Defaults to `true`.
    #[serde(default)]
    peer_dependencies: DependencyAvailability,

    /// If set to `false`, then the rule will show an error when `optionalDependencies` are imported. Defaults to `true`.
    #[serde(default)]
    optional_dependencies: DependencyAvailability,
}

pub struct RuleState {
    package_name: String,
    is_dev_dependency_available: bool,
    is_peer_dependency_available: bool,
    is_optional_dependency_available: bool,
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
        let is_dev_dependency_available = ctx.options().dev_dependencies.is_available(path);
        let is_peer_dependency_available = ctx.options().peer_dependencies.is_available(path);
        let is_optional_dependency_available =
            ctx.options().optional_dependencies.is_available(path);

        let is_available = |package_name| {
            ctx.is_dependency(package_name)
                || (is_dev_dependency_available && ctx.is_dev_dependency(package_name))
                || (is_peer_dependency_available && ctx.is_peer_dependency(package_name))
                || (is_optional_dependency_available && ctx.is_optional_dependency(package_name))
        };

        let token_text = node.inner_string_text()?;
        let package_name = parse_package_name(token_text.text())?;
        if is_available(package_name)
            // Self package imports
            // TODO: we should also check that an `.` exports exists.
            // See https://nodejs.org/api/packages.html#self-referencing-a-package-using-its-name
            || ctx.name() == Some(package_name)
            // ignore Node.js builtin modules
            || is_node_builtin_module(package_name)
            // Ignore `bun` import
            || package_name == "bun"
        {
            return None;
        }

        if !package_name.starts_with('@') {
            // Handle DefinitelyTyped imports https://github.com/DefinitelyTyped/DefinitelyTyped
            // e.g. `lodash` can import types from `@types/lodash`.
            if let Some(import_clause) = node.parent::<AnyJsImportClause>() {
                if import_clause.type_token().is_some() {
                    let package_name = format!("@types/{package_name}");
                    if is_available(&package_name) {
                        return None;
                    }
                }
            }
        }

        Some(RuleState {
            package_name: package_name.to_string(),
            is_dev_dependency_available,
            is_peer_dependency_available,
            is_optional_dependency_available,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let RuleState {
            package_name,
            is_dev_dependency_available,
            is_peer_dependency_available,
            is_optional_dependency_available,
        } = state;

        let Some(package_path) = ctx.package_path.as_ref() else {
            return Some(RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Dependency "<Emphasis>{package_name}</Emphasis>" cannot be verified because no package.json file was found."
                },
            ));
        };

        let manifest_path = package_path.clone();

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

fn parse_package_name(path: &str) -> Option<&str> {
    let mut in_scope = false;
    for (i, c) in path.bytes().enumerate() {
        match c {
            b'@' if i == 0 => {
                in_scope = true;
            }
            // uppercase characters are not allowed in package name
            // Here we are more tolerant and accept them.
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' => {}
            b'.' if i != 0 => {}
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
    assert_eq!(parse_package_name("a.js"), Some("a.js"));
    assert_eq!(parse_package_name("@././file"), Some("@./."));

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

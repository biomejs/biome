use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_json_syntax::{JsonRoot, JsonSyntaxKind, JsonSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_duplicate_dependencies::NoDuplicateDependenciesOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Prevent the listing of duplicate dependencies.
    /// The rule supports the following dependency groups: "bundledDependencies", "bundleDependencies", "dependencies", "devDependencies", "overrides", "optionalDependencies", and "peerDependencies".
    ///
    /// Dependencies are not allowed to be listed twice under the same dependency group.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json
    /// {
    ///     "dependencies": {
    ///         "foo": "1.0.0",
    ///         "foo": "2.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// ```json
    /// {
    ///     "bundleDependencies": ["foo", "foo"]
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///     "dependencies": {
    ///         "foo": "2.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// ```json
    /// {
    ///     "bundleDependencies": ["foo"]
    /// }
    /// ```
    ///
    /// Some dependency group dependencies are checked against other dependency groups;
    ///  - Dependencies listed in "dependencies" cannot be listed under "devDependencies", "optionalDependencies" or "peerDependencies".
    ///  - Dependencies listed in "optionalDependencies" cannot be listed under "peerDependencies" (and vice versa).
    ///
    /// Dependencies listed in "devDependencies" are allowed to be listed in "optionalDependencies" or "peerDependencies".
    /// And dependencies listed in "overrides" & "bundleDependencies" are not checked against other dependency groups.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json
    /// {
    ///     "dependencies": {
    ///         "foo": "1.0.0"
    ///     },
    ///     "devDependencies": {
    ///         "foo": "1.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///     "dependencies": {
    ///         "foo": "1.0.0"
    ///     }
    /// }
    /// ```
    ///
    pub NoDuplicateDependencies {
        version: "2.2.4",
        name: "noDuplicateDependencies",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintPackageJson("unique-dependencies").same(), RuleSource::EslintPackageJsonDependencies("duplicate-dependencies").same()],
    }
}

const PACKAGE_JSON: &str = "package.json";

// dependencies <-> devDependencies / optionalDependencies / peerDependencies
// peerDependencies <-> optionalDependencies
const UNIQUE_PROPERTY_KEYS: [(&str, &[&str]); 2] = [
    (
        "dependencies",
        &[
            "devDependencies",
            "optionalDependencies",
            "peerDependencies",
        ],
    ),
    ("peerDependencies", &["optionalDependencies"]),
];

const DUPLICATE_PROPERTY_KEYS: &[&str; 7] = &[
    "bundledDependencies",
    "bundleDependencies",
    "dependencies",
    "devDependencies",
    "optionalDependencies",
    "overrides",
    "peerDependencies",
];

pub struct Dependency {
    pub group: String,
    pub token: JsonSyntaxToken,
}

impl Rule for NoDuplicateDependencies {
    type Query = Ast<JsonRoot>;
    type State = (Dependency, Dependency);
    type Signals = Option<Self::State>;
    type Options = NoDuplicateDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let path = ctx.file_path();

        let value = query.value().ok()?;
        let object_value = value.as_json_object_value()?;

        if !path.ends_with(PACKAGE_JSON) {
            return None;
        }

        let mut dependencies = FxHashMap::<String, FxHashMap<String, JsonSyntaxToken>>::default();

        // Filter JsonMembers matching the valid dependency group keys
        let groups = object_value
            .json_member_list()
            .iter()
            .flatten()
            .filter(|member| {
                let name = member.name();
                if let Ok(name) = name {
                    let text = name.inner_string_text();
                    if let Ok(text) = text {
                        return DUPLICATE_PROPERTY_KEYS.contains(&text.text());
                    }
                }
                false
            });

        // Loop through all the dependency groups to map all dependencies & check for duplicates within the same dependency group
        for dependency_group in groups {
            let dependency_group_name = dependency_group.name().ok()?;
            let dependency_group_text = dependency_group_name.inner_string_text().ok()?;
            let dependency_group_value = dependency_group.value().ok()?;

            let mut deps = FxHashMap::<String, JsonSyntaxToken>::default();

            match dependency_group_value.syntax().kind() {
                JsonSyntaxKind::JSON_OBJECT_VALUE => {
                    let object_value = dependency_group_value.as_json_object_value()?;

                    for dependency in object_value.json_member_list().iter().flatten() {
                        let dependency_name = dependency.name().ok()?;
                        let dependency_text = dependency_name.inner_string_text().ok()?;

                        // Add dependencies to deps if not exists else to duplicates
                        if let Some(original_member) = deps.get(&dependency_text.to_string()) {
                            let original = Dependency {
                                group: dependency_group_text.to_string(),
                                token: original_member.clone(),
                            };
                            let dupe = Dependency {
                                group: dependency_group_text.to_string(),
                                token: dependency_name.value_token().ok()?,
                            };

                            return Some((original, dupe));
                        } else {
                            deps.insert(
                                dependency_text.to_string(),
                                dependency_name.value_token().ok()?,
                            );
                        }
                    }
                }
                // bundledDependencies / bundleDependencies are an array of strings
                JsonSyntaxKind::JSON_ARRAY_VALUE => {
                    let array_value = dependency_group_value.as_json_array_value()?;

                    for dependency in array_value.elements().iter().flatten() {
                        if dependency.syntax().kind() != JsonSyntaxKind::JSON_STRING_VALUE {
                            continue;
                        }

                        let dependency_name = dependency.as_json_string_value()?;
                        let dependency_text = dependency_name.inner_string_text().ok()?;

                        if let Some(original_member) = deps.get(&dependency_text.to_string()) {
                            let original = Dependency {
                                group: dependency_group_text.to_string(),
                                token: original_member.clone(),
                            };
                            let dupe = Dependency {
                                group: dependency_group_text.to_string(),
                                token: dependency_name.value_token().ok()?,
                            };

                            return Some((original, dupe));
                        } else {
                            deps.insert(
                                dependency_text.to_string(),
                                dependency_name.value_token().ok()?,
                            );
                        }
                    }
                }
                _ => {}
            }

            dependencies.insert(dependency_group_text.to_string(), deps);
        }

        if dependencies.is_empty() {
            return None;
        }

        // Check for duplicate dependencies between dependency groups
        for (key, properties) in UNIQUE_PROPERTY_KEYS {
            let key_deps = dependencies.get(key);

            if key_deps.is_none() {
                continue;
            }

            for property in properties {
                let deps = dependencies.get(*property);

                if deps.is_none() {
                    continue;
                }

                for (dependency_name, original_member) in key_deps? {
                    if let Some(member) = deps?.get(dependency_name) {
                        let original = Dependency {
                            group: key.to_string(),
                            token: original_member.clone(),
                        };
                        let dupe = Dependency {
                            group: (*property).to_string(),
                            token: member.clone(),
                        };

                        return Some((original, dupe));
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (original, dupe) = state;
        let name = original.token.text_trimmed();

        let mut diagnostic = if original.group == dupe.group {
            RuleDiagnostic::new(
                rule_category!(),
                dupe.token.text_trimmed_range(),
                markup!("The dependency "<Emphasis>{name}</Emphasis>" is listed twice under "<Emphasis>{dupe.group.to_string()}</Emphasis>"."),
            )
        } else {
            RuleDiagnostic::new(
                rule_category!(),
                dupe.token.text_trimmed_range(),
                markup!("The dependency "<Emphasis>{name}</Emphasis>" is also listed under "<Emphasis>{dupe.group.to_string()}</Emphasis>"."),
            )
        };
        diagnostic = diagnostic.detail(
            original.token.text_trimmed_range(),
            markup! {
                "The dependency is also specified here."
            },
        );
        Some(diagnostic.note(markup! {
            "This can lead to package manager issues and confusion for other developers. To resolve this, remove one of the listings."
        }))
    }
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_json_syntax::{JsonMemberName, JsonObjectValue, TextRange};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_duplicate_dependencies::NoDuplicateDependenciesOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Checks a dependency isn't specified more than once (i.e. in `dependencies` and `devDependencies`)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
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
        version: "next",
        name: "noDuplicateDependencies",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintPackageJson("unique-dependencies").same(), RuleSource::EslintPackageJsonDependencies("duplicate-dependencies").same()],
    }
}

const PACKAGE_JSON: &str = "package.json";

const PROPERTY_KEYS: &[&str; 4] = &[
    "dependencies",
    "devDependencies",
    "optionalDependencies",
    "peerDependencies",
];

impl Rule for NoDuplicateDependencies {
    type Query = Ast<JsonObjectValue>;
    type State = (JsonMemberName, Vec<TextRange>);
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let path = ctx.file_path();

        let mut duplicates = FxHashMap::<JsonMemberName, Vec<TextRange>>::default();
        let mut seen = FxHashMap::<String, JsonMemberName>::default();

        if path.ends_with(PACKAGE_JSON) {
            for member in query.json_member_list().iter().flatten() {
                let name = member.name();

                if let Ok(name) = name {
                    let text = name.inner_string_text();
                    if let Ok(text) = text {
                        if PROPERTY_KEYS.contains(&text.text()) {
                            let value = member.value().ok();

                            for dependency in value
                                .unwrap()
                                .as_json_object_value()
                                .unwrap()
                                .json_member_list()
                                .iter()
                                .flatten()
                            {
                                let dependency_name = dependency.name();

                                if let Ok(dependency_name) = dependency_name {
                                    let dependency_text = dependency_name.inner_string_text();
                                    if let Ok(dependency_text) = dependency_text {
                                        if let Some(original_member) =
                                            seen.get(dependency_text.text())
                                        {
                                            if let Some(ranges) =
                                                duplicates.get_mut(original_member)
                                            {
                                                ranges.push(dependency_name.range());
                                            } else {
                                                duplicates.insert(
                                                    original_member.clone(),
                                                    vec![dependency_name.range()],
                                                );
                                            }
                                        } else {
                                            seen.insert(
                                                dependency_text.to_string(),
                                                dependency_name,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let duplicated_keys: Vec<_> = duplicates.into_iter().collect();
        return duplicated_keys.into_boxed_slice();
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (original, ranges) = state;
        let name = original.inner_string_text().ok()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            original.range(),
            markup! {
                "The dependency "<Emphasis>{name.text()}</Emphasis>" is also listed later."
            },
        );
        for range in ranges {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "This dependency has already been listed before"
                },
            );
        }
        Some(diagnostic)
    }
}

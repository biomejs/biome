use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::DeserializableValue;
use biome_json_syntax::JsonRoot;
use biome_module_replacements::{
    ModuleReplacement, find_mapping, find_replacement, resolve_doc_url,
};
use biome_rowan::{TextRange, TokenText};
use biome_rule_options::no_restricted_dependencies::NoRestrictedDependenciesOptions;

use crate::utils::is_package_json;

declare_lint_rule! {
    /// Disallow dependencies that are known to have better alternatives.
    ///
    /// This rule checks `dependencies` and `devDependencies` in `package.json`
    /// against [e18e](https://e18e.dev/)'s replacement data and suggests modern, native, or more
    /// maintainable alternatives.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,file=package.json,expect_diagnostic
    /// {
    ///   "dependencies": {
    ///     "globby": "x.x.x"
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": {
    ///     "tinyglobby": "x.x.x"
    ///   }
    /// }
    /// ```
    ///
    /// See https://e18e.dev/docs/replacements/ for the full list of replacements.
    ///
    pub NoRestrictedDependencies {
        version: "next",
        name: "noRestrictedDependencies",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintE18e("ban-dependencies").same()],
    }
}

impl Rule for NoRestrictedDependencies {
    type Query = Ast<JsonRoot>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = NoRestrictedDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut found = Vec::new();

        let path = ctx.file_path();
        if !is_package_json(path) {
            return found;
        }

        let root = ctx.query();
        let Some(value) = root.value().ok() else {
            return found;
        };
        let Some(object) = value.as_json_object_value() else {
            return found;
        };

        for member in object.json_member_list() {
            let Some(member) = member.ok() else {
                continue;
            };
            let Some(name) = member.name().ok() else {
                continue;
            };
            let Some(name_text) = name.inner_string_text() else {
                continue;
            };
            let Some(name_text) = name_text.ok() else {
                continue;
            };
            if !DEPENDENCY_KEYS.contains(&name_text.text()) {
                continue;
            }

            let Some(dep_value) = member.value().ok() else {
                continue;
            };
            let Some(dep_object) = dep_value.as_json_object_value() else {
                continue;
            };

            for member in dep_object.json_member_list() {
                let Some(member) = member.ok() else {
                    continue;
                };
                let Some(name) = member.name().ok() else {
                    continue;
                };
                let Some(name) = name.as_json_member_name() else {
                    continue;
                };
                let Some(name_text) = name.inner_string_text().ok() else {
                    continue;
                };

                if find_mapping(name_text.text()).is_some() {
                    found.push(RuleState {
                        name: name_text,
                        range: name.range(),
                    });
                }
            }
        }

        found
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mapping = find_mapping(state.name.text())?;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "The dependency "<Emphasis>{mapping.module_name}</Emphasis>" can be replaced."
            },
        );

        let mut replacement_text = if mapping.replacements.len() > 1 {
            markup!("The following replacements are suggested:").to_owned()
        } else {
            markup!("The following replacement is suggested:").to_owned()
        };

        for replacement_id in mapping.replacements {
            let Some(replacement) = find_replacement(replacement_id) else {
                continue;
            };

            match replacement {
                ModuleReplacement::Native(replacement) => {
                    replacement_text.extend_with(markup! {
                        "\n- Remove the dependency in favour of a native implementation "<Emphasis>{replacement.common.id}</Emphasis>"."
                    });
                }
                ModuleReplacement::Documented(replacement) => {
                    replacement_text.extend_with(markup! {
                        "\n- Replace with the alternative: "<Emphasis>{replacement.replacement_module}</Emphasis>"."
                    });
                }
                ModuleReplacement::Simple(replacement) => {
                    replacement_text.extend_with(markup! {
                        "\n- Replace with inline or local logic. "{replacement.description}""
                    });
                }
                ModuleReplacement::Removal(replacement) => {
                    replacement_text.extend_with(markup! {
                        "\n- Remove the dependency as it's no longer needed. "{replacement.description}""
                    });
                }
            };
        }

        diagnostic = diagnostic.note(replacement_text);

        if let Some(url) = resolve_doc_url(mapping.url) {
            diagnostic = diagnostic.note(markup! {
                "Read more: "<Hyperlink href={url.as_str()}>{url.as_str()}</Hyperlink>
            });
        }

        Some(diagnostic)
    }
}

const DEPENDENCY_KEYS: &[&str] = &["dependencies", "devDependencies"];

pub struct RuleState {
    name: TokenText,
    range: TextRange,
}

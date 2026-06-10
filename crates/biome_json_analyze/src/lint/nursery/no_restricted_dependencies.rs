use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_json_syntax::JsonMember;
use biome_module_replacements::{
    ModuleReplacement, find_mapping, find_replacement, resolve_doc_url,
};
use biome_rowan::AstNode;
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
    /// See [the e18e docs](https://e18e.dev/docs/replacements/) for the full list of replacements.
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
    type Query = Ast<JsonMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoRestrictedDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let path = ctx.file_path();
        if !is_package_json(path) {
            return None;
        }

        let node = ctx.query();

        let parent_member = node
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsonMember::cast)?;
        let parent_member_name = parent_member.name().ok()?;
        let parent_member_name_text = parent_member_name.inner_string_text()?;
        if !DEPENDENCY_KEYS.contains(&parent_member_name_text.text()) {
            return None;
        }

        let name = node.name().ok()?;
        let name_text = name.inner_string_text()?;
        if find_mapping(name_text.text()).is_some() {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = node.name().ok()?;
        let name_text = name.inner_string_text()?;
        let mapping = find_mapping(name_text.text())?;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            name.range(),
            markup! {
                "Use of the restricted dependency "<Emphasis>{mapping.module_name}</Emphasis>" detected."
            },
        ).note(markup! {
            "The dependency might be old, not actively maintained, or there's a native alternative."
        });

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
                        "\n- Remove the dependency in favor of a native implementation "<Emphasis>{replacement.common.id}</Emphasis>"."
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

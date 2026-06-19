use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::AnyJsImportLike;
use biome_module_replacements::{
    ModuleReplacement, find_mapping, find_replacement, resolve_doc_url,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_restricted_dependencies::NoRestrictedDependenciesOptions;

use crate::{services::manifest::Manifest, utils::parse_package_name};

declare_lint_rule! {
    /// Disallow dependencies that are known to have better alternatives.
    ///
    /// This rule checks static imports, dynamic `import()`, and `require()` calls
    /// and suggests modern, native, or more maintainable alternatives based on
    /// [e18e](https://e18e.dev/)'s replacement data.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import glob from "globby";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const glob = require("globby");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const glob = await import("globby");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import glob from "tinyglobby";
    /// ```
    ///
    /// ```js
    /// const glob = require("tinyglobby");
    /// ```
    ///
    /// ```js
    /// const glob = await import("tinyglobby");
    /// ```
    ///
    /// See [the e18e docs](https://e18e.dev/docs/replacements/) for the full list of replacements.
    ///
    pub NoRestrictedDependencies {
        version: "2.5.0",
        name: "noRestrictedDependencies",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintE18e("ban-dependencies").same()],
    }
}

impl Rule for NoRestrictedDependencies {
    type Query = Manifest<AnyJsImportLike>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }

        let source = node.inner_string_text()?;
        let dep_name = parse_package_name(source.text())?;

        if find_mapping(dep_name).is_some() {
            let range = node
                .module_name_token()
                .map_or_else(|| node.range(), |token| token.text_trimmed_range());

            return Some(RuleState {
                name: dep_name.into(),
                range,
            });
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mapping = find_mapping(&state.name)?;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
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

pub struct RuleState {
    name: Box<str>,
    range: TextRange,
}

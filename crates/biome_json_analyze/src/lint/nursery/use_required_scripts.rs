use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_json_syntax::JsonRoot;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_required_scripts::UseRequiredScriptsOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Enforce the presence of required scripts in package.json.
    ///
    /// This rule ensures that specified scripts are defined in the `scripts` section of a `package.json` file.
    /// It's particularly useful in monorepo environments where consistency across workspaces is important.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json
    /// {
    ///     "scripts": {
    ///         "test": "vitest"
    ///     }
    /// }
    /// ```
    ///
    /// With options `{ "requiredScripts": ["test", "build"] }`, this is invalid because `build` is missing.
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///     "scripts": {
    ///         "test": "vitest",
    ///         "build": "tsc"
    ///     }
    /// }
    /// ```
    ///
    /// With options `{ "requiredScripts": ["test", "build"] }`, this is valid because all required scripts are present.
    ///
    /// ## Options
    ///
    /// The rule accepts an array of script names that must be present:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "requiredScripts": ["test", "build", "lint"]
    ///     }
    /// }
    /// ```
    ///
    pub UseRequiredScripts {
        version: "next",
        name: "useRequiredScripts",
        language: "json",
        recommended: false,
    }
}

const PACKAGE_JSON: &str = "package.json";

impl Rule for UseRequiredScripts {
    type Query = Ast<JsonRoot>;
    type State = Vec<String>;
    type Signals = Option<Self::State>;
    type Options = UseRequiredScriptsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let path = ctx.file_path();
        let options = ctx.options();

        if !path.ends_with(PACKAGE_JSON) {
            return None;
        }
        if options.required_scripts.is_empty() {
            return None;
        }

        let value = query.value().ok()?;
        let object_value = value.as_json_object_value()?;

        let scripts_member = object_value
            .json_member_list()
            .iter()
            .flatten()
            .find(|member| {
                if let Ok(name) = member.name() {
                    if let Ok(text) = name.inner_string_text() {
                        return text.text() == "scripts";
                    }
                }
                false
            })?;

        let scripts_value = scripts_member.value().ok()?;
        let scripts_object = scripts_value.as_json_object_value()?;

        let existing_scripts: FxHashSet<String> = scripts_object
            .json_member_list()
            .iter()
            .flatten()
            .filter_map(|member| {
                let name = member.name().ok()?;
                let text = name.inner_string_text().ok()?;
                Some(text.to_string())
            })
            .collect();

        let missing_scripts: Vec<String> = options
            .required_scripts
            .iter()
            .filter(|script| !existing_scripts.contains(*script))
            .cloned()
            .collect();

        if missing_scripts.is_empty() {
            None
        } else {
            Some(missing_scripts)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let missing_count = state.len();
        let missing_list = state.join(", ");

        let message = if missing_count == 1 {
            markup! {
                "The required script "<Emphasis>{missing_list}</Emphasis>" is missing from package.json."
            }
        } else {
            markup! {
                "The required scripts "<Emphasis>{missing_list}</Emphasis>" are missing from package.json."
            }
        };

        Some(
            RuleDiagnostic::new(rule_category!(), ctx.query().range(), message).note(markup! {
                "Add the missing script"{{if missing_count > 1 { "s" } else { "" }}}" to your package.json."
            }),
        )
    }
}

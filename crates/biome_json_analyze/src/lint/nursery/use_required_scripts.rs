use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_json_syntax::{JsonRoot, TextRange};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_required_scripts::UseRequiredScriptsOptions;

use crate::utils::is_package_json;

/// State containing the missing scripts and the range to highlight
pub struct UseRequiredScriptsState {
    /// The list of missing script names
    pub missing_scripts: Vec<String>,
    /// The range to highlight in the diagnostic (scripts object or root object)
    pub range: TextRange,
}

declare_lint_rule! {
    /// Enforce the presence of required scripts in package.json.
    ///
    /// This rule ensures that specified scripts are defined in the `scripts` section of a `package.json` file.
    /// It's particularly useful in monorepo environments where consistency across workspaces is important.
    ///
    /// Without required scripts configured, this rule doesn't do anything.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "requiredScripts": ["test", "build"]
    ///     }
    /// }
    /// ```
    ///
    /// ```json,use_options
    /// {
    ///     "scripts": {
    ///         "test": "vitest"
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,use_options
    /// {
    ///     "scripts": {
    ///         "test": "vitest",
    ///         "build": "tsc"
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `requiredScripts`
    ///
    /// An array of script names that must be present in the `scripts` section of `package.json`.
    /// Default: `[]` (no scripts required)
    ///
    pub UseRequiredScripts {
        version: "next",
        name: "useRequiredScripts",
        language: "json",
        recommended: false,
    }
}

impl Rule for UseRequiredScripts {
    type Query = Ast<JsonRoot>;
    type State = UseRequiredScriptsState;
    type Signals = Option<Self::State>;
    type Options = UseRequiredScriptsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let path = ctx.file_path();
        let options = ctx.options();

        if !is_package_json(path) {
            return None;
        }
        if options.required_scripts.is_empty() {
            return None;
        }

        let value = query.value().ok()?;
        let object_value = value.as_json_object_value()?;

        let scripts_member = object_value.find_member("scripts");

        // If there's no scripts section, all required scripts are missing
        // Point to the root object in this case
        let Some(scripts_member) = scripts_member else {
            return Some(UseRequiredScriptsState {
                missing_scripts: options.required_scripts.clone(),
                range: object_value.range(),
            });
        };

        let scripts_value = scripts_member.value().ok()?;
        let scripts_object = scripts_value.as_json_object_value()?;

        let existing_scripts: Vec<String> = scripts_object
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
            .filter(|script| !existing_scripts.iter().any(|s| s == *script))
            .cloned()
            .collect();

        if missing_scripts.is_empty() {
            None
        } else {
            // Point to the scripts member when scripts exist but some are missing
            Some(UseRequiredScriptsState {
                missing_scripts,
                range: scripts_member.range(),
            })
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let missing_count = state.missing_scripts.len();
        let missing_list = state.missing_scripts.join(", ");

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
            RuleDiagnostic::new(rule_category!(), state.range, message).note(markup! {
                "Consistent scripts across packages ensure that each can be run reliably from the root of our project. Add the missing script"{{if missing_count > 1 { "s" } else { "" }}}" to your package.json."
            }),
        )
    }
}

use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
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
    /// Require configured scripts in `package.json`.
    ///
    /// A repository can contain several packages that share commands such as `test` or `build`.
    /// Tools run from the repository's top-level directory may expect every package to provide the
    /// same script names. The rule does nothing when `requiredScripts` is empty.
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
    /// ```json,use_options,expect_diagnostic,file=package.json
    /// {
    ///     "scripts": {
    ///         "test": "vitest"
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,use_options,file=package.json
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
    /// Lists script names that must appear in the `scripts` object. Defaults to an empty list, which
    /// disables the rule.
    ///
    pub UseRequiredScripts {
        version: "2.3.9",
        name: "useRequiredScripts",
        language: "json",
        recommended: false,
        severity: Severity::Warning,
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

        let members = scripts_object.json_member_list();
        let missing_scripts: Vec<String> = options
            .required_scripts
            .iter()
            .filter(|script| {
                !members.iter().flatten().any(|member| {
                    member
                        .name()
                        .ok()
                        .and_then(|n| n.inner_string_text())
                        .is_some_and(|text| text.text() == script.as_str())
                })
            })
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

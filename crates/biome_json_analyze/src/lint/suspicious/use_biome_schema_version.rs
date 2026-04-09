use crate::JsonRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_factory::make::{json_string_literal, json_string_value};
use biome_json_syntax::{JsonMember, JsonStringValue};
use biome_rowan::{AstNode, BatchMutationExt};
use std::cmp::Ordering;
const BIOME_SCHEMA_SUFFIX: &str = "/schema.json";
const BIOME_SCHEMA_PREFIX: &str = "https://biomejs.dev/schemas/";
const BIOME_VERSION: &str = match option_env!("BIOME_VERSION") {
    Some(version) => version,
    None => "0.0.0",
};

#[derive(Clone, Debug)]
pub struct SchemaVersionState {
    schema_value: JsonStringValue,
    found_version: String,
    expected_schema_url: String,
}

declare_lint_rule! {
    /// Ensures that Biome configuration files use the current schema version.
    ///
    /// When a configuration contains a `$schema` URL that points to a different
    /// Biome version, editor tooling can become inconsistent with the currently
    /// running Biome CLI version.
    ///
    /// This rule applies only to top-level `$schema` fields in `biome.json` and
    /// `biome.jsonc`, and only when the schema URL matches the official Biome
    /// schema format: `https://biomejs.dev/schemas/{version}/schema.json`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,ignore
    /// {
    ///     "$schema": "https://biomejs.dev/schemas/1.9.0/schema.json"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,ignore
    /// {
    ///     "$schema": "https://biomejs.dev/schemas/2.4.10/schema.json"
    /// }
    /// ```
    ///
    pub UseBiomeSchemaVersion {
        version: "2.4.10",
        name: "useBiomeSchemaVersion",
        language: "json",
        recommended: true,
        fix_kind: FixKind::Safe,
        severity: Severity::Information,
    }
}

impl Rule for UseBiomeSchemaVersion {
    type Query = Ast<JsonMember>;
    type State = SchemaVersionState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !is_biome_configuration_file(ctx.file_path()) || !is_top_level_schema_member(node) {
            return None;
        }

        let schema_value = node.value().ok()?.as_json_string_value()?.clone();
        let schema_text = schema_value.inner_string_text().ok()?;
        let schema_url = schema_text.text();
        let mismatch = parse_biome_schema_version(schema_url)?;
        let expected_version = Version::new(BIOME_VERSION);

        if mismatch.cmp(&expected_version) == Ordering::Equal {
            return None;
        }

        Some(SchemaVersionState {
            schema_value,
            found_version: mismatch.0.to_string(),
            expected_schema_url: expected_schema_url(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.schema_value.range(),
                markup! {
                    "The configuration schema version does not match the CLI version "{BIOME_VERSION}
                },
            )
            .note(markup! {
                "Expected schema URL: "<Emphasis>{state.expected_schema_url.as_str()}</Emphasis>
            })
            .note(markup! {
                "Found version: "<Emphasis>{state.found_version.as_str()}</Emphasis>
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsonRuleAction> {
        let mut mutation = ctx.root().begin();
        let new_value = json_string_value(json_string_literal(state.expected_schema_url.as_str()));
        mutation.replace_node(state.schema_value.clone(), new_value);

        Some(JsonRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use the schema URL that matches the running Biome version."
            },
            mutation,
        ))
    }
}

fn is_biome_configuration_file(path: &camino::Utf8Path) -> bool {
    path.file_name().is_some_and(|file_name| {
        file_name.ends_with("biome.json") || file_name.ends_with("biome.jsonc")
    })
}

fn is_top_level_schema_member(node: &JsonMember) -> bool {
    let Ok(name) = node.name() else {
        return false;
    };
    if name
        .inner_string_text()
        .is_none_or(|text| text.text() != "$schema")
    {
        return false;
    }

    node.syntax()
        .ancestors()
        .skip(1)
        .find_map(JsonMember::cast)
        .is_none()
}

fn parse_biome_schema_version(schema_url: &str) -> Option<Version<'_>> {
    let version = schema_url
        .strip_prefix(BIOME_SCHEMA_PREFIX)?
        .strip_suffix(BIOME_SCHEMA_SUFFIX)?;

    if version.is_empty() || version.split('.').any(|part| part.parse::<u32>().is_err()) {
        return None;
    }

    Some(Version::new(version))
}

fn expected_schema_url() -> String {
    format!("{BIOME_SCHEMA_PREFIX}{BIOME_VERSION}{BIOME_SCHEMA_SUFFIX}")
}

#[derive(Clone, Copy)]
struct Version<'a>(&'a str);

impl<'a> Version<'a> {
    fn new(version: &'a str) -> Self {
        Self(version)
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let left_parts: Vec<_> = self
            .0
            .split('.')
            .filter_map(|part| part.parse::<u32>().ok())
            .collect();
        let right_parts: Vec<_> = other
            .0
            .split('.')
            .filter_map(|part| part.parse::<u32>().ok())
            .collect();

        for (left, right) in left_parts.iter().zip(right_parts.iter()) {
            match left.cmp(right) {
                Ordering::Equal => {}
                non_equal => return non_equal,
            }
        }

        left_parts.len().cmp(&right_parts.len())
    }
}

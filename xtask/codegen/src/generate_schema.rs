use biome_configuration::Configuration;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{JsonParserOptions, parse_json};
use schemars::{Schema, schema_for};
use serde_json::{Value, to_string};
use xtask::{Mode, Result, project_root};
use xtask_codegen::update;

pub(crate) fn generate_configuration_schema(mode: Mode) -> Result<()> {
    let schema_path_npm = project_root().join("packages/@biomejs/biome/configuration_schema.json");

    let schema = rename_partial_references_in_schema(schema_for!(Configuration));

    let json_schema = to_string(&schema)?;
    let parsed = parse_json(&json_schema, JsonParserOptions::default());
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())
            .unwrap()
            .print()
            .unwrap();

    update(&schema_path_npm, formatted.as_code(), &mode)?;

    Ok(())
}

/// Strips all "Partial" prefixes from type names in the schema.
///
/// We do this to avoid leaking our `Partial` derive macro to the outside world,
/// since it should be just an implementation detail.
fn rename_partial_references_in_schema(mut schema: Schema) -> Schema {
    if let Some(title) = schema.get_mut("title").and_then(|title| match title {
        Value::String(title) => Some(title),
        _ => None,
    }) {
        if title == "RuleWithOptions_for_Null" {
            *title = "RuleWithNoOptions".to_string();
        } else if title == "RuleWithFixOptions_for_Null" {
            *title = "RuleWithFixNoOptions".to_string();
        } else if title == "RuleConfiguration_for_Null" {
            *title = "RuleConfiguration".to_string();
        } else if title == "RuleFixConfiguration_for_Null" {
            *title = "RuleFixConfiguration".to_string();
        } else if let Some(stripped) = title.strip_prefix("RuleWithOptions_for_") {
            *title = format!("RuleWith{stripped}");
        } else if let Some(stripped) = title.strip_prefix("RuleWithFixOptions_for_") {
            *title = format!("RuleWith{stripped}");
        } else if let Some(stripped) = title
            .strip_prefix("RuleConfiguration_for_")
            .map(|x| x.strip_suffix("Options").unwrap_or(x))
        {
            *title = format!("{stripped}Configuration");
        } else if let Some(stripped) = title
            .strip_prefix("RuleFixConfiguration_for_")
            .map(|x| x.strip_suffix("Options").unwrap_or(x))
        {
            *title = format!("{stripped}Configuration");
        }
    }

    if let Some(object) = schema.as_object_mut()
        && let Some(definitions) = object.get_mut("definitions").and_then(Value::as_object_mut)
    {
        *definitions = definitions
            .into_iter()
            .map(|(key, schema)| {
                let mut key = key.clone();
                let mut schema = schema.clone();

                if key == "RuleWithOptions_for_Null" || key == "RuleWithFixOptions_for_Null" {
                    key = if key == "RuleWithOptions_for_Null" {
                        "RuleWithNoOptions".to_string()
                    } else {
                        "RuleWithFixNoOptions".to_string()
                    };
                    if let Some(object) = schema.as_object_mut() {
                        if let Some(required) =
                            object.get_mut("required").and_then(Value::as_array_mut)
                        {
                            required.retain(|name| name != "options");
                        }
                        if let Some(properties) =
                            object.get_mut("properties").and_then(Value::as_object_mut)
                        {
                            properties.remove("options");
                        }
                    }
                } else if key == "RuleConfiguration_for_Null" {
                    key = "RuleConfiguration".to_string();
                } else if key == "RuleFixConfiguration_for_Null" {
                    key = "RuleFixConfiguration".to_string();
                } else if let Some(stripped) = key.strip_prefix("RuleWithOptions_for_") {
                    key = format!("RuleWith{stripped}");
                    if let Some(object) = schema.as_object_mut() {
                        if let Some(required) =
                            object.get_mut("required").and_then(Value::as_array_mut)
                        {
                            required.retain(|name| name != "options");
                        }
                    }
                } else if let Some(stripped) = key.strip_prefix("RuleWithFixOptions_for_") {
                    key = format!("RuleWith{stripped}");
                    if let Some(object) = schema.as_object_mut() {
                        if let Some(required) =
                            object.get_mut("required").and_then(Value::as_array_mut)
                        {
                            required.retain(|name| name != "options");
                        }
                    }
                } else if let Some(stripped) = key
                    .strip_prefix("RuleConfiguration_for_")
                    .map(|x| x.strip_suffix("Options").unwrap_or(x))
                {
                    key = format!("{stripped}Configuration");
                } else if let Some(stripped) = key
                    .strip_prefix("RuleFixConfiguration_for_")
                    .map(|x| x.strip_suffix("Options").unwrap_or(x))
                {
                    key = format!("{stripped}Configuration");
                }

                (key, schema)
            })
            .collect();
    }

    schema
}

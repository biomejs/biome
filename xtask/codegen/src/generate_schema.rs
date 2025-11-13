use crate::update;
use biome_configuration::Configuration;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{JsonParserOptions, parse_json};
use schemars::{Schema, schema_for};
use serde_json::{Map, Value, to_string};
use xtask_glue::*;

/// Returns the configuration schema as a string
pub fn generate_schema_as_string() -> Result<String> {
    let schema = rename_references_in_schema(schema_for!(Configuration));

    let json_schema = to_string(&schema)?;
    let parsed = parse_json(&json_schema, JsonParserOptions::default());
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())?
            .print()?;

    Ok(formatted.into_code())
}

/// Generate the schema and saves it at `packages/@biomejs/biome/configuration_schema.json`
pub fn generate_configuration_schema(mode: Mode) -> Result<()> {
    let schema_path_npm = project_root().join("packages/@biomejs/biome/configuration_schema.json");

    let schema = generate_schema_as_string()?;

    update(&schema_path_npm, schema.as_str(), &mode)?;

    Ok(())
}

/// Rename complex type names with simpler ones.
///
/// Complex names are generated from generic types.
fn rename_type(name: &str) -> Option<String> {
    if let Some(stripped) = name.strip_prefix("RuleWithOptions_for_") {
        Some(format!("RuleWith{stripped}"))
    } else if let Some(stripped) = name.strip_prefix("RuleWithFixOptions_for_") {
        Some(format!("RuleWith{stripped}"))
    } else if let Some(stripped) = name.strip_prefix("RuleAssistWithOptions_for_") {
        Some(format!("RuleAssistWith{stripped}"))
    } else if let Some(stripped) = name
        .strip_prefix("RuleConfiguration_for_")
        .map(|x| x.strip_suffix("Options").unwrap_or(x))
    {
        Some(format!("{stripped}Configuration"))
    } else if let Some(stripped) = name
        .strip_prefix("RuleFixConfiguration_for_")
        .map(|x| x.strip_suffix("Options").unwrap_or(x))
    {
        Some(format!("{stripped}Configuration"))
    } else if let Some(stripped) = name
        .strip_prefix("RuleAssistConfiguration_for_")
        .map(|x| x.strip_suffix("Options").unwrap_or(x))
    {
        Some(format!("{stripped}Configuration"))
    } else {
        name.strip_prefix("SeverityOrGroup_for_")
            .map(|stripped| format!("SeverityOr{stripped}"))
    }
}

/// Rename complex names with simpler ones.
///
/// Complex names are generated from generic types.
fn rename_references_in_schema(mut schema: Schema) -> Schema {
    // Rename the root schema title if needed
    if let Some(title) = schema.get("title").and_then(|v| v.as_str())
        && let Some(renamed_title) = rename_type(title)
    {
        schema.insert("title".to_string(), Value::String(renamed_title));
    }

    // Rename references in the root schema value
    if let Some(obj) = schema.as_object_mut() {
        rename_references_in_object(obj);
    }

    // Process definitions (try both $defs and definitions for backwards compatibility)
    for defs_key in ["$defs", "definitions"] {
        if let Some(Value::Object(defs)) = schema.get_mut(defs_key) {
            let keys_to_rename: Vec<_> = defs
                .keys()
                .filter_map(|key| rename_type(key).map(|new_key| (key.clone(), new_key)))
                .collect();

            // Rename definition keys
            for (old_key, new_key) in keys_to_rename {
                if let Some(value) = defs.remove(&old_key) {
                    defs.insert(new_key, value);
                }
            }

            // Recursively process each definition
            for value in defs.values_mut() {
                if let Value::Object(obj) = value {
                    rename_references_in_object(obj);
                }
            }
            break; // Only process the first one found
        }
    }

    schema
}

/// Recursively rename references in a schema object
fn rename_references_in_object(obj: &mut Map<String, Value>) {
    // Rename $ref if it references a definition that should be renamed
    if let Some(Value::String(reference)) = obj.get_mut("$ref") {
        if let Some(stripped_ref) = reference.strip_prefix("#/definitions/")
            && let Some(renamed_ref) = rename_type(stripped_ref)
        {
            *reference = format!("#/definitions/{renamed_ref}");
        } else if let Some(stripped_ref) = reference.strip_prefix("#/$defs/")
            && let Some(renamed_ref) = rename_type(stripped_ref)
        {
            *reference = format!("#/$defs/{renamed_ref}");
        }
    }

    // Process properties
    if let Some(Value::Object(properties)) = obj.get_mut("properties") {
        for prop_value in properties.values_mut() {
            if let Value::Object(prop_obj) = prop_value {
                rename_references_in_object(prop_obj);
            }
        }
    }

    // Process items
    if let Some(items) = obj.get_mut("items") {
        match items {
            Value::Object(items_obj) => rename_references_in_object(items_obj),
            Value::Array(arr) => {
                for item in arr {
                    if let Value::Object(item_obj) = item {
                        rename_references_in_object(item_obj);
                    }
                }
            }
            _ => {}
        }
    }

    // Process subschemas (allOf, anyOf, oneOf)
    for subschema_key in ["allOf", "anyOf", "oneOf"] {
        if let Some(Value::Array(schemas)) = obj.get_mut(subschema_key) {
            for schema in schemas {
                if let Value::Object(schema_obj) = schema {
                    rename_references_in_object(schema_obj);
                }
            }
        }
    }

    // Process single subschemas (not, if, then, else)
    for key in ["not", "if", "then", "else"] {
        if let Some(Value::Object(schema_obj)) = obj.get_mut(key) {
            rename_references_in_object(schema_obj);
        }
    }
}

use crate::update;
use biome_configuration::Configuration;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{JsonParserOptions, parse_json};
use schemars::{Schema, schema_for};
use serde_json::{Map, Value, to_string};
use xtask_glue::*;

/// Returns the configuration schema as a string
pub fn generate_schema_as_string() -> Result<String> {
    let schema = strip_markdown_links_from_descriptions(rename_references_in_schema(schema_for!(
        Configuration
    )));

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

fn strip_markdown_links_from_description(description: &str) -> String {
    let mut result = String::new();
    let mut remaining = description;

    while let Some(open_bracket) = remaining.find('[') {
        let (before_link, after_open_bracket) = remaining.split_at(open_bracket);
        let after_open_bracket = &after_open_bracket['['.len_utf8()..];

        let Some(close_bracket) = after_open_bracket.find(']') else {
            break;
        };

        let label = &after_open_bracket[..close_bracket];
        let after_label = &after_open_bracket[close_bracket + ']'.len_utf8()..];

        let Some(after_open_paren) = after_label.strip_prefix('(') else {
            result.push_str(before_link);
            result.push('[');
            remaining = after_open_bracket;
            continue;
        };

        let Some(close_paren) = after_open_paren.find(')') else {
            break;
        };

        let url = &after_open_paren[..close_paren];
        result.push_str(before_link);
        result.push_str(label);
        result.push_str(" (");
        result.push_str(url);
        result.push(')');
        remaining = &after_open_paren[close_paren + ')'.len_utf8()..];
    }

    result.push_str(remaining);
    result
}

fn strip_markdown_links_from_descriptions(mut schema: Schema) -> Schema {
    if let Some(obj) = schema.as_object_mut() {
        strip_markdown_links_from_descriptions_in_object(obj);
    }
    schema
}

fn strip_markdown_links_from_descriptions_in_value(value: &mut Value) {
    match value {
        Value::Object(obj) => strip_markdown_links_from_descriptions_in_object(obj),
        Value::Array(values) => {
            for value in values {
                strip_markdown_links_from_descriptions_in_value(value);
            }
        }
        _ => {}
    }
}

fn strip_markdown_links_from_descriptions_in_object(obj: &mut Map<String, Value>) {
    if let Some(Value::String(description)) = obj.get_mut("description") {
        let stripped_description = strip_markdown_links_from_description(description);
        if stripped_description != *description {
            *description = stripped_description;
        }
    }

    for value in obj.values_mut() {
        strip_markdown_links_from_descriptions_in_value(value);
    }
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

#[cfg(test)]
mod tests {
    use super::strip_markdown_links_from_description;

    #[test]
    fn strips_markdown_links_from_schema_descriptions() {
        assert_eq!(
            strip_markdown_links_from_description(
                "A field for the [JSON schema](https://json-schema.org/) specification",
            ),
            "A field for the JSON schema (https://json-schema.org/) specification",
        );
        assert_eq!(
            strip_markdown_links_from_description(
                "Uses [React Fast Refresh](https://github.com/facebook/react/tree/main/packages/react-refresh), such as [`meta` in Remix](https://remix.run/docs/en/main/route/meta)",
            ),
            "Uses React Fast Refresh (https://github.com/facebook/react/tree/main/packages/react-refresh), such as `meta` in Remix (https://remix.run/docs/en/main/route/meta)",
        );
    }
}

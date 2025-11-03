use biome_configuration::Configuration;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{JsonParserOptions, parse_json};
use schemars::schema::{RootSchema, Schema, SchemaObject};
use schemars::schema_for;
use serde_json::to_string;
use xtask::{Mode, Result, project_root};
use xtask_codegen::update;

pub(crate) fn generate_configuration_schema(mode: Mode) -> Result<()> {
    let schema_path_npm = project_root().join("packages/@biomejs/biome/configuration_schema.json");

    let schema = rename_references_in_schema(schema_for!(Configuration));

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

/// Rename complex type names with simpler ones.
///
/// Complex names are generated from generic types.
fn rename_type(name: &str) -> Option<String> {
    if let Some(stripped) = name.strip_prefix("RuleWithOptions_for_") {
        Some(format!("RuleWith{stripped}"))
    } else if let Some(stripped) = name.strip_prefix("RuleWithFixOptions_for_") {
        Some(format!("RuleWith{stripped}"))
    } else if let Some(stripped) = name
        .strip_prefix("RuleConfiguration_for_")
        .map(|x| x.strip_suffix("Options").unwrap_or(x))
    {
        Some(format!("{stripped}Configuration"))
    } else {
        name.strip_prefix("RuleFixConfiguration_for_")
            .map(|x| x.strip_suffix("Options").unwrap_or(x))
            .map(|stripped| format!("{stripped}Configuration"))
    }
}

/// Rename complex names with simpler ones.
///
/// Complex names are generated from generic types.
fn rename_references_in_schema(mut schema: RootSchema) -> RootSchema {
    if let Some(meta) = schema.schema.metadata.as_mut()
        && let Some(title) = meta.title.as_ref()
        && let Some(renamed_title) = rename_type(title)
    {
        meta.title = Some(renamed_title);
    }

    rename_references_in_schema_object(&mut schema.schema);

    schema.definitions = schema
        .definitions
        .into_iter()
        .map(|(mut key, mut schema)| {
            if let Some(renamed_key) = rename_type(&key) {
                key = renamed_key;
            }

            if let Schema::Object(object) = &mut schema {
                rename_references_in_schema_object(object);
            }

            (key, schema)
        })
        .collect();

    schema
}

fn rename_references_in_schema_object(object: &mut SchemaObject) {
    if let Some(object) = &mut object.object {
        for prop_schema in object.properties.values_mut() {
            if let Schema::Object(object) = prop_schema {
                rename_references_in_schema_object(object);
            }
        }
    }

    if let Some(reference) = &mut object.reference
        && let Some(stripped_ref) = reference.strip_prefix("#/definitions/")
        && let Some(renamed_ref) = rename_type(stripped_ref)
    {
        *reference = format!("#/definitions/{renamed_ref}");
    }

    if let Some(subschemas) = &mut object.subschemas {
        rename_references_in_optional_schema_vec(&mut subschemas.all_of);
        rename_references_in_optional_schema_vec(&mut subschemas.any_of);
        rename_references_in_optional_schema_vec(&mut subschemas.one_of);

        rename_references_in_optional_schema_box(&mut subschemas.not);
        rename_references_in_optional_schema_box(&mut subschemas.if_schema);
        rename_references_in_optional_schema_box(&mut subschemas.then_schema);
        rename_references_in_optional_schema_box(&mut subschemas.else_schema);
    }
}

fn rename_references_in_optional_schema_box(schema: &mut Option<Box<Schema>>) {
    if let Some(schema) = schema
        && let Schema::Object(object) = schema.as_mut()
    {
        rename_references_in_schema_object(object);
    }
}

fn rename_references_in_optional_schema_vec(schemas: &mut Option<Vec<Schema>>) {
    if let Some(schemas) = schemas {
        rename_references_in_schema_slice(schemas);
    }
}

fn rename_references_in_schema_slice(schemas: &mut [Schema]) {
    for schema in schemas {
        if let Schema::Object(object) = schema {
            rename_references_in_schema_object(object);
        }
    }
}

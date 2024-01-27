use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_service::PartialConfiguration;
use schemars::schema::{RootSchema, Schema, SchemaObject};
use schemars::schema_for;
use serde_json::to_string;
use xtask::{project_root, Mode, Result};
use xtask_codegen::update;

pub(crate) fn generate_configuration_schema(mode: Mode) -> Result<()> {
    let schema_path_npm = project_root().join("packages/@biomejs/biome/configuration_schema.json");

    let schema = rename_partial_references_in_schema(schema_for!(PartialConfiguration));

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
fn rename_partial_references_in_schema(mut schema: RootSchema) -> RootSchema {
    if let Some(meta) = schema.schema.metadata.as_mut() {
        if let Some(stripped_title) = meta
            .title
            .as_mut()
            .and_then(|title| title.strip_prefix("Partial"))
        {
            meta.title = Some(stripped_title.to_owned());
        }
    }

    rename_partial_references_in_schema_object(&mut schema.schema);

    schema.definitions = schema
        .definitions
        .into_iter()
        .map(|(mut key, mut schema)| {
            if let Some(stripped_key) = key.strip_prefix("Partial") {
                key = stripped_key.to_owned();
            }

            if let Schema::Object(object) = &mut schema {
                rename_partial_references_in_schema_object(object);
            }

            (key, schema)
        })
        .collect();

    schema
}

fn rename_partial_references_in_schema_object(object: &mut SchemaObject) {
    if let Some(object) = &mut object.object {
        for prop_schema in object.properties.values_mut() {
            if let Schema::Object(object) = prop_schema {
                rename_partial_references_in_schema_object(object);
            }
        }
    }

    if let Some(reference) = &mut object.reference {
        if let Some(stripped_ref) = reference.strip_prefix("#/definitions/Partial") {
            *reference = format!("#/definitions/{stripped_ref}");
        }
    }

    if let Some(subschemas) = &mut object.subschemas {
        rename_partial_references_in_optional_schema_vec(&mut subschemas.all_of);
        rename_partial_references_in_optional_schema_vec(&mut subschemas.any_of);
        rename_partial_references_in_optional_schema_vec(&mut subschemas.one_of);

        rename_partial_references_in_optional_schema_box(&mut subschemas.not);
        rename_partial_references_in_optional_schema_box(&mut subschemas.if_schema);
        rename_partial_references_in_optional_schema_box(&mut subschemas.then_schema);
        rename_partial_references_in_optional_schema_box(&mut subschemas.else_schema);
    }
}

fn rename_partial_references_in_optional_schema_box(schema: &mut Option<Box<Schema>>) {
    if let Some(schema) = schema {
        if let Schema::Object(object) = schema.as_mut() {
            rename_partial_references_in_schema_object(object);
        }
    }
}

fn rename_partial_references_in_optional_schema_vec(schemas: &mut Option<Vec<Schema>>) {
    if let Some(schemas) = schemas {
        rename_partial_references_in_schema_slice(schemas);
    }
}

fn rename_partial_references_in_schema_slice(schemas: &mut [Schema]) {
    for schema in schemas {
        if let Schema::Object(object) = schema {
            rename_partial_references_in_schema_object(object);
        }
    }
}

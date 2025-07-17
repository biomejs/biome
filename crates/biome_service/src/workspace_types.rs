//! Utility functions to help with generating bindings for the [Workspace] API

use std::collections::VecDeque;

use biome_analyze::{RuleCategories, RuleDomain};
use biome_configuration::{
    Configuration,
    analyzer::{RuleDomainValue, RuleSelector},
};
use biome_diagnostics::serde::Diagnostic;
use biome_formatter::SourceMarker;
use biome_fs::BiomePath;
use biome_grit_patterns::GritTargetLanguage;
use biome_js_syntax::{AnyJsDeclaration, AnyTsTupleTypeElement};
use rustc_hash::FxHashSet;
use schemars::{JsonSchema, Schema, SchemaGenerator, generate::SchemaSettings};
use serde_json::{Map, Value};

use crate::{WorkspaceError, projects::ProjectKey, workspace::*};
use biome_js_factory::{
    make,
    syntax::{AnyJsObjectMemberName, AnyTsName, AnyTsType, AnyTsTypeMember, T},
};
use biome_rowan::{AstSeparatedList, TriviaPieceKind};

/// Manages a queue of type definitions that need to be generated
#[derive(Default)]
pub struct ModuleQueue<'a> {
    /// Set of type names that have already been emitted
    visited: FxHashSet<&'a str>,
    /// Queue of type names and definitions that need to be generated
    queue: VecDeque<(&'a str, &'a Map<String, Value>)>,
}

impl<'a> ModuleQueue<'a> {
    /// Add a type definition to the queue if it hasn't been emitted already
    fn push_back(&mut self, item: (&'a str, &'a Map<String, Value>)) {
        if self.visited.insert(item.0) {
            self.queue.push_back(item);
        }
    }

    /// Pull a type name and definition from the queue
    fn pop_front(&mut self) -> Option<(&'a str, &'a Map<String, Value>)> {
        self.queue.pop_front()
    }

    pub fn visited(&self) -> &FxHashSet<&'a str> {
        &self.visited
    }
}

/// Generate an [AnyTsType] node from an individual instance type definition.
fn instance_type<'a>(
    generator: &'a SchemaGenerator,
    queue: &mut ModuleQueue<'a>,
    schema: &'a Map<String, Value>,
    instance_type: &'a str,
) -> AnyTsType {
    match instance_type {
        // If the instance type is an object, generate a TS object type with the corresponding properties
        "object" => {
            let properties = schema
                .get("properties")
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|properties| properties.iter())
                .map(|(property, schema)| {
                    let (ts_type, optional, description) = schema_type(generator, queue, schema);
                    assert!(!optional, "optional nested types are not supported");

                    let mut property = make::ident(property);
                    if let Some(description) = description {
                        let comment = format!("/**\n\t* {description} \n\t */");
                        let trivia = vec![
                            (TriviaPieceKind::Newline, "\n"),
                            (TriviaPieceKind::MultiLineComment, comment.as_str()),
                            (TriviaPieceKind::Newline, "\n"),
                        ];
                        property = property.with_leading_trivia(trivia);
                    }

                    AnyTsTypeMember::from(
                        make::ts_property_signature_type_member(AnyJsObjectMemberName::from(
                            make::js_literal_member_name(property),
                        ))
                        .with_type_annotation(make::ts_type_annotation(make::token(T![:]), ts_type))
                        .build(),
                    )
                })
                .collect::<Vec<_>>();

            let properties_type = (!properties.is_empty()).then(|| {
                make::ts_object_type(
                    make::token(T!['{']),
                    make::ts_type_member_list(properties),
                    make::token(T!['}']),
                )
                .into()
            });

            // Don't use `additionalProperties: false` here.
            let additional_properties =
                schema
                    .get("additionalProperties")
                    .and_then(|schema| match schema {
                        Value::Bool(false) => None,
                        _ => Some(schema),
                    });

            // If `additionalProperties` is not empty, add a mapped or record type.
            let additional_properties_type =
                additional_properties.map(|additional_properties_schema| {
                    // If `propertyNames` is not empty, use it as the key type.
                    let key_type = schema.get("propertyNames").map(|schema| {
                        let (ts_type, optional, _) = schema_type(generator, queue, schema);
                        assert!(!optional, "optional nested types are not supported");
                        ts_type
                    });

                    let value_type = {
                        let (ts_type, optional, _) =
                            schema_type(generator, queue, additional_properties_schema);
                        assert!(!optional, "optional nested types are not supported");
                        ts_type
                    };

                    if let Some(key_type) = key_type {
                        // Use a mapped type for the key type and the value type. All keys are optional.
                        // e.g. `{ [K in Key]?: Value }`.
                        // TODO: Support `required` keys here when needed.
                        make::ts_mapped_type(
                            make::token(T!['{']),
                            make::token(T!['[']),
                            make::ts_type_parameter_name(make::ident("K")),
                            make::token(T![in]),
                            key_type,
                            make::token(T![']']),
                            make::token(T!['}']),
                        )
                        .with_optional_modifier(
                            make::ts_mapped_type_optional_modifier_clause(make::token(T![?]))
                                .build(),
                        )
                        .with_mapped_type(make::ts_type_annotation(make::token(T![:]), value_type))
                        .build()
                        .into()
                    } else {
                        // Use `Record<string, Value>` otherwise.
                        make::ts_reference_type(
                            make::js_reference_identifier(make::ident("Record")).into(),
                        )
                        .with_type_arguments(make::ts_type_arguments(
                            make::token(T![<]),
                            make::ts_type_argument_list(
                                [
                                    make::ts_reference_type(
                                        make::js_reference_identifier(make::ident("string")).into(),
                                    )
                                    .build()
                                    .into(),
                                    value_type,
                                ],
                                [make::token(T![,])],
                            ),
                            make::token(T![>]),
                        ))
                        .build()
                        .into()
                    }
                });

            // If both `properties` and `additionalProperties` are provided, turn into an
            // intersection type. Pick one for the final type otherwise.
            let result = [properties_type, additional_properties_type]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            let separators = (0..result.len().saturating_sub(1)).map(|_| make::token(T![&]));

            make::ts_intersection_type(make::ts_intersection_type_element_list(result, separators))
                .build()
                .into()
        }
        // If the instance type is an array, generate a TS array type with the corresponding item type
        "array" => match schema.get("items") {
            Some(Value::Array(items)) => AnyTsType::from(make::ts_tuple_type(
                make::token(T!['[']),
                make::ts_tuple_type_element_list(
                    items.iter().map(|schema| {
                        let (ts_type, optional, _) = schema_type(generator, queue, schema);
                        assert!(!optional, "optional nested types are not supported");
                        AnyTsTupleTypeElement::AnyTsType(ts_type)
                    }),
                    items.iter().map(|_| make::token(T![,])),
                ),
                make::token(T![']']),
            )),
            Some(schema) => {
                let (ts_type, optional, _) = schema_type(generator, queue, schema);
                assert!(!optional, "optional nested types are not supported");

                AnyTsType::from(make::ts_array_type(
                    ts_type,
                    make::token(T!['[']),
                    make::token(T![']']),
                ))
            }
            None => AnyTsType::TsAnyType(make::ts_any_type(make::token(T![any]))),
        },

        // Map native types to the corresponding TS type
        "null" => AnyTsType::from(make::ts_null_literal_type(make::token(T![null]))),
        "boolean" => AnyTsType::from(make::ts_boolean_type(make::token(T![boolean]))),
        "string" => AnyTsType::from(make::ts_string_type(make::token(T![string]))),
        "number" | "integer" => AnyTsType::from(make::ts_number_type(make::token(T![number]))),
        other => panic!("unexpected instance type: {other}"),
    }
}

/// Generate a literal [TsType] from a `serde_json` [Value]
fn value_type(value: &Value) -> AnyTsType {
    match value {
        Value::Null => AnyTsType::from(make::ts_null_literal_type(make::token(T![null]))),
        Value::Bool(true) => AnyTsType::from(make::ts_boolean_literal_type(make::token(T![true]))),
        Value::Bool(false) => {
            AnyTsType::from(make::ts_boolean_literal_type(make::token(T![false])))
        }
        Value::Number(value) => AnyTsType::from(
            make::ts_number_literal_type(make::js_number_literal(value.as_f64().unwrap())).build(),
        ),
        Value::String(value) => {
            AnyTsType::from(make::ts_string_literal_type(make::js_string_literal(value)))
        }
        Value::Array(_) => unimplemented!(),
        Value::Object(_) => unimplemented!(),
    }
}

/// Generate a union [TsType] node from a list of [TsType]s,
/// flattening any nested union type the iterator may emit
fn make_union_type(items: impl IntoIterator<Item = AnyTsType>) -> AnyTsType {
    let mut result = Vec::new();

    for item in items {
        if let AnyTsType::TsUnionType(union_type) = item {
            for item in union_type.types().iter() {
                result.push(item.unwrap());
            }
        } else {
            result.push(item);
        }
    }

    let separators = (0..result.len().saturating_sub(1)).map(|_| make::token(T![|]));
    AnyTsType::from(
        make::ts_union_type(make::ts_union_type_variant_list(result, separators)).build(),
    )
}

/// Generate a [TsType] node from a [SchemaObject], returning the generated
/// TypeScript type along with a boolean flag indicating whether the type is
/// considered "optional" in the schema
fn schema_object_type<'a>(
    generator: &'a SchemaGenerator,
    queue: &mut ModuleQueue<'a>,
    schema: &'a Map<String, Value>,
) -> (AnyTsType, bool, Option<&'a str>) {
    // Start by detecting enum types by inspecting the `enum_values` field, i
    // the field is set return a union type generated from the literal enum values
    let description = schema.get("description").and_then(Value::as_str);
    let ts_type = schema
        .get("enumValues")
        .and_then(Value::as_array)
        .map(|enum_values| make_union_type(enum_values.iter().map(value_type)))
        // If the type isn't an enum, inspect its `type` field, if the field is
        // a set return a type annotation for the corresponding type
        .or_else(|| {
            Some(match schema.get("type")? {
                Value::String(ty) => instance_type(generator, queue, schema, ty),
                Value::Array(types) => make_union_type(
                    types
                        .iter()
                        .filter_map(Value::as_object)
                        .filter_map(|object| object.get("type").and_then(Value::as_str))
                        .map(|ty| instance_type(generator, queue, schema, ty)),
                ),
                other => panic!("unexpected instance type: {other}"),
            })
        })
        // Otherwise inspect the `$ref` field of the schema, if it's set return
        // a TS reference type and add the corresponding type to the queue
        .or_else(|| {
            let reference = schema
                .get("$ref")
                .and_then(|reference| reference.as_str())?;
            let key = reference
                .trim_start_matches("#/components/schemas/")
                .trim_start_matches("#/definitions/");
            match generator.definitions().get(key) {
                Some(Value::Object(schema)) => queue.push_back((key, schema)),
                Some(other) => unimplemented!("referenced definition of type {other:?}"),
                None => panic!("definition for type {key:?} not found"),
            }

            Some(AnyTsType::from(
                make::ts_reference_type(AnyTsName::from(make::js_reference_identifier(
                    make::ident(key),
                )))
                .build(),
            ))
        })
        // Finally try to inspect the subschemas for this type
        .or_else(|| {
            // First try to inspect the `allOf` list of subschemas, if it's
            // set generate an intersection type from it
            schema
                .get("allOf")
                .and_then(Value::as_array)
                .map(|all_of| {
                    AnyTsType::from(
                        make::ts_intersection_type(make::ts_intersection_type_element_list(
                            all_of.iter().map(|ty| {
                                let (ts_type, optional, _) = schema_type(generator, queue, ty);
                                assert!(!optional, "optional nested types are not supported");
                                ts_type
                            }),
                            (0..all_of.len().saturating_sub(1)).map(|_| make::token(T![&])),
                        ))
                        .build(),
                    )
                })
                // Otherwise try to inspect the `anyOf`/`oneOf` list of
                // subschemas, and generate the corresponding union type for it
                .or_else(|| {
                    let any_of = schema
                        .get("anyOf")
                        .or(schema.get("oneOf"))
                        .and_then(Value::as_array)?;

                    Some(make_union_type(any_of.iter().map(|ty| {
                        let (ts_type, ..) = schema_type(generator, queue, ty);
                        ts_type
                    })))
                })
        })
        .unwrap_or_else(|| {
            // this is temporary workaround to fix the `options` field, which is not used at the moment
            AnyTsType::from(make::ts_any_type(make::token(T![any])))
        });

    // Types are considered "optional" in the serialization protocol if they
    // have the `nullable` OpenAPI extension property, or if they have a default value
    let is_nullable = matches!(schema.get("nullable"), Some(Value::Bool(true)));
    let has_defaults = schema.get("default").is_some();

    (ts_type, is_nullable || has_defaults, description)
}

/// Generate a [TsType] node from a [Schema], returning the generated type
/// along with a boolean flag indicating whether the type is considered
/// "optional" in the schema
fn schema_type<'a>(
    generator: &'a SchemaGenerator,
    queue: &mut ModuleQueue<'a>,
    schema: &'a Value,
) -> (AnyTsType, bool, Option<&'a str>) {
    if let Some(boolean_schema) = schema.as_bool() {
        if boolean_schema {
            // Types defined as `true` in the schema always pass validation,
            // map them to the `any` type
            (
                AnyTsType::from(make::ts_any_type(make::token(T![any]))),
                true,
                None,
            )
        } else {
            // Types defined as `false` in the schema never pass validation,
            // map them to the `never` type
            (
                AnyTsType::from(make::ts_never_type(make::token(T![never]))),
                false,
                None,
            )
        }
    } else if let Some(schema_object) = schema.as_object() {
        schema_object_type(generator, queue, schema_object)
    } else {
        unreachable!("schema must be a boolean or an object")
    }
}

pub fn include_subschemas(generator: &mut SchemaGenerator) {
    // HACK: List explicit subschemas:
    generator.subschema_for::<BiomePath>();
    generator.subschema_for::<CodeAction>();
    generator.subschema_for::<Configuration>();
    generator.subschema_for::<Diagnostic>();
    generator.subschema_for::<DocumentFileSource>();
    generator.subschema_for::<FeatureKind>();
    generator.subschema_for::<FeatureName>();
    generator.subschema_for::<FeaturesSupported>();
    generator.subschema_for::<FileContent>();
    generator.subschema_for::<FixAction>();
    generator.subschema_for::<FixFileMode>();
    generator.subschema_for::<GritTargetLanguage>();
    generator.subschema_for::<PatternId>();
    generator.subschema_for::<ProjectKey>();
    generator.subschema_for::<RuleCategories>();
    generator.subschema_for::<RuleDomain>();
    generator.subschema_for::<RuleDomainValue>();
    generator.subschema_for::<RuleSelector>();
    generator.subschema_for::<ScanKind>();
    generator.subschema_for::<SourceMarker>();
    generator.subschema_for::<SupportKind>();
}

/// Generate and emit all the types defined in `root_schema` into the `module`
pub fn generate_type<'a>(
    generator: &'a SchemaGenerator,
    module: &mut Vec<(AnyJsDeclaration, Option<&'a str>)>,
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a Schema,
) -> AnyTsType {
    // Read the root type of the schema and push it to the queue
    let root_name = root_schema
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or_default();

    match root_name {
        "Null" => return AnyTsType::TsVoidType(make::ts_void_type(make::token(T![void]))),
        "Boolean" => {
            return AnyTsType::TsBooleanType(make::ts_boolean_type(make::token(T![boolean])));
        }
        "String" => return AnyTsType::TsStringType(make::ts_string_type(make::token(T![string]))),
        _ => {}
    }

    queue.push_back((root_name, root_schema.as_object().unwrap()));

    while let Some((name, schema)) = queue.pop_front() {
        // Detect if the type being emitted is an object, emit it as an
        // interface definition if that's the case
        let is_interface = schema
            .get("additionalProperties")
            .is_none_or(|additional_properties| {
                matches!(additional_properties, Value::Bool(false))
            })
            && schema.get("type").is_none_or(|instance_type| {
                instance_type
                    .as_str()
                    .is_some_and(|instance_type| instance_type == "object")
            });

        if is_interface {
            let mut members = Vec::new();

            // Create a property signature member in the interface for each
            // property of the corresponding schema object
            for (property_str, schema) in schema
                .get("properties")
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|properties| properties.iter())
            {
                let (ts_type, optional, description) = schema_type(generator, queue, schema);

                let mut property = make::ident(property_str);
                if let Some(description) = description {
                    let comment = format!("/**\n\t* {description} \n\t */");
                    let trivia = vec![
                        (TriviaPieceKind::Newline, "\n"),
                        (TriviaPieceKind::MultiLineComment, comment.as_str()),
                        (TriviaPieceKind::Newline, "\n"),
                    ];
                    property = property.with_leading_trivia(trivia);
                }

                let type_annotation = make::ts_type_annotation(make::token(T![:]), ts_type);

                let mut builder = make::ts_property_signature_type_member(
                    AnyJsObjectMemberName::from(make::js_literal_member_name(property)),
                )
                .with_type_annotation(type_annotation);

                if optional {
                    builder = builder.with_optional_token(make::token(T![?]));
                }

                members.push(AnyTsTypeMember::from(builder.build()));
            }

            let description = schema.get("description").and_then(Value::as_str);
            let current_module = AnyJsDeclaration::from(
                make::ts_interface_declaration(
                    make::token(T![interface]),
                    make::ts_identifier_binding(make::ident(name)).into(),
                    make::token(T!['{']),
                    make::ts_type_member_list(members),
                    make::token(T!['}']),
                )
                .build(),
            );
            module.push((current_module, description));
        } else {
            // If the schema for this type is not an object, emit it as a type alias
            let (ts_type, optional, description) = schema_object_type(generator, queue, schema);

            assert!(!optional, "optional nested types are not supported");

            let current_module = AnyJsDeclaration::from(
                make::ts_type_alias_declaration(
                    make::token(T![type]),
                    make::ts_identifier_binding(make::ident(name)).into(),
                    make::token(T![=]),
                    ts_type,
                )
                .build(),
            );
            module.push((current_module, description));
        }
    }

    AnyTsType::TsReferenceType(
        make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
            make::js_reference_identifier(make::ident(root_name)),
        ))
        .build(),
    )
}

/// Signature metadata for a [Workspace] method
pub struct WorkspaceMethod {
    /// Name of the method
    pub name: &'static str,
    /// Schema for the parameters object of the method
    pub params: Schema,
    /// Schema for the result object of the method
    pub result: Schema,
}

impl WorkspaceMethod {
    /// Construct a [WorkspaceMethod] from a name, a parameter type and a result type
    fn of<P, R>(name: &'static str) -> Self
    where
        P: JsonSchema,
        R: JsonSchema,
    {
        let params = SchemaGenerator::from(SchemaSettings::openapi3()).root_schema_for::<P>();
        let result = SchemaGenerator::from(SchemaSettings::openapi3()).root_schema_for::<R>();
        Self {
            name,
            params,
            result,
        }
    }

    /// Construct a [WorkspaceMethod] from a name and a function pointer
    fn from_method<T, P, R>(
        name: &'static str,
        _func: fn(T, P) -> Result<R, WorkspaceError>,
    ) -> Self
    where
        P: JsonSchema,
        R: JsonSchema,
    {
        Self::of::<P, R>(name)
    }
}

/// Helper macro for generated an OpenAPI schema for a type implementing JsonSchema
macro_rules! workspace_method {
    ($name:ident) => {
        WorkspaceMethod::from_method(stringify!($name), <dyn Workspace>::$name)
    };
}

/// Returns a list of signature for all the methods in the [Workspace] trait
pub fn methods() -> [WorkspaceMethod; 25] {
    [
        workspace_method!(file_features),
        workspace_method!(update_settings),
        workspace_method!(open_project),
        workspace_method!(open_file),
        workspace_method!(change_file),
        workspace_method!(close_file),
        workspace_method!(get_syntax_tree),
        workspace_method!(file_exists),
        workspace_method!(check_file_size),
        workspace_method!(get_file_content),
        workspace_method!(get_control_flow_graph),
        workspace_method!(get_formatter_ir),
        workspace_method!(get_type_info),
        workspace_method!(get_registered_types),
        workspace_method!(get_semantic_model),
        workspace_method!(pull_diagnostics),
        workspace_method!(pull_actions),
        workspace_method!(format_file),
        workspace_method!(format_range),
        workspace_method!(format_on_type),
        workspace_method!(fix_file),
        workspace_method!(rename),
        workspace_method!(parse_pattern),
        workspace_method!(search_pattern),
        workspace_method!(drop_pattern),
    ]
}

//! Utility functions to help with generating bindings for the [Workspace] API

use std::collections::VecDeque;

use biome_js_syntax::{AnyJsDeclaration, AnyTsTupleTypeElement};
use rustc_hash::FxHashSet;
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use serde_json::Value;

use crate::{workspace::*, WorkspaceError};
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
    queue: VecDeque<(&'a str, &'a SchemaObject)>,
}

impl<'a> ModuleQueue<'a> {
    /// Add a type definition to the queue if it hasn't been emitted already
    fn push_back(&mut self, item: (&'a str, &'a SchemaObject)) {
        if self.visited.insert(item.0) {
            self.queue.push_back(item);
        }
    }

    /// Pull a type name and definition from the queue
    fn pop_front(&mut self) -> Option<(&'a str, &'a SchemaObject)> {
        self.queue.pop_front()
    }

    pub fn visited(&self) -> &FxHashSet<&'a str> {
        &self.visited
    }
}

/// Generate a [TsType] node from the `instance_type` of a [SchemaObject]
fn instance_type<'a>(
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a RootSchema,
    schema: &'a SchemaObject,
    ty: InstanceType,
) -> AnyTsType {
    match ty {
        // If the instance type is an object, generate a TS object type with the corresponding properties
        InstanceType::Object => {
            let object = schema.object.as_deref().unwrap();
            AnyTsType::from(make::ts_object_type(
                make::token(T!['{']),
                make::ts_type_member_list(object.properties.iter().map(|(property, schema)| {
                    let (ts_type, optional, description) = schema_type(queue, root_schema, schema);
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
                })),
                make::token(T!['}']),
            ))
        }
        // If the instance type is an array, generate a TS array type with the corresponding item type
        InstanceType::Array => {
            let array = schema.array.as_deref().unwrap();
            let items = array.items.as_ref().unwrap();
            match items {
                SingleOrVec::Single(schema) => {
                    let (ts_type, optional, _) = schema_type(queue, root_schema, schema);
                    assert!(!optional, "optional nested types are not supported");

                    AnyTsType::from(make::ts_array_type(
                        ts_type,
                        make::token(T!['[']),
                        make::token(T![']']),
                    ))
                }
                SingleOrVec::Vec(items) => AnyTsType::from(make::ts_tuple_type(
                    make::token(T!['[']),
                    make::ts_tuple_type_element_list(
                        items.iter().map(|schema| {
                            let (ts_type, optional, _) = schema_type(queue, root_schema, schema);
                            assert!(!optional, "optional nested types are not supported");
                            AnyTsTupleTypeElement::AnyTsType(ts_type)
                        }),
                        items.iter().map(|_| make::token(T![,])),
                    ),
                    make::token(T![']']),
                )),
            }
        }

        // Map native types to the corresponding TS type
        InstanceType::Null => AnyTsType::from(make::ts_null_literal_type(make::token(T![null]))),
        InstanceType::Boolean => AnyTsType::from(make::ts_boolean_type(make::token(T![boolean]))),
        InstanceType::String => AnyTsType::from(make::ts_string_type(make::token(T![string]))),
        InstanceType::Number | InstanceType::Integer => {
            AnyTsType::from(make::ts_number_type(make::token(T![number])))
        }
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
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a RootSchema,
    schema: &'a SchemaObject,
) -> (AnyTsType, bool, Option<&'a String>) {
    // Start by detecting enum types by inspecting the `enum_values` field, i
    // the field is set return a union type generated from the literal enum values
    let description = schema
        .metadata
        .as_ref()
        .and_then(|s| s.description.as_ref());
    let ts_type = schema
        .enum_values
        .as_deref()
        .map(|enum_values| make_union_type(enum_values.iter().map(value_type)))
        // If the type isn't an enum, inspect its `instance_type` field, if the
        // field is set return a type annotation for the corresponding type
        .or_else(|| {
            Some(match schema.instance_type.as_ref()? {
                SingleOrVec::Single(ty) => instance_type(queue, root_schema, schema, **ty),
                SingleOrVec::Vec(types) => make_union_type(
                    types
                        .iter()
                        .map(|ty| instance_type(queue, root_schema, schema, *ty)),
                ),
            })
        })
        // Otherwise inspect the `reference` field of the schema, if its set return
        // a TS reference type and add the corresponding type to the queue
        .or_else(|| {
            let reference = schema.reference.as_deref()?;
            let key = reference.trim_start_matches("#/components/schemas/");
            match root_schema.definitions.get(key) {
                Some(Schema::Bool(_)) => unimplemented!(),
                Some(Schema::Object(schema)) => queue.push_back((key, schema)),
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
            let subschemas = schema.subschemas.as_deref()?;
            // First try to inspect the `all_of` list of subschemas, if it's
            // set generate an intersection type from it
            subschemas
                .all_of
                .as_deref()
                .map(|all_of| {
                    AnyTsType::from(
                        make::ts_intersection_type(make::ts_intersection_type_element_list(
                            all_of.iter().map(|ty| {
                                let (ts_type, optional, _) = schema_type(queue, root_schema, ty);
                                assert!(!optional, "optional nested types are not supported");
                                ts_type
                            }),
                            (0..all_of.len().saturating_sub(1)).map(|_| make::token(T![&])),
                        ))
                        .build(),
                    )
                })
                // Otherwise try to inspect the `any_of` list of subschemas, and
                // generate the corresponding union type for it
                .or_else(|| {
                    let any_of = subschemas
                        .any_of
                        .as_deref()
                        .or(subschemas.one_of.as_deref())?;

                    Some(make_union_type(any_of.iter().map(|ty| {
                        let (ts_type, optional, _) = schema_type(queue, root_schema, ty);
                        assert!(!optional, "optional nested types are not supported");
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
    let is_nullable = matches!(schema.extensions.get("nullable"), Some(Value::Bool(true)));
    let has_defaults = schema
        .metadata
        .as_ref()
        .is_some_and(|metadata| metadata.default.is_some());

    (ts_type, is_nullable || has_defaults, description)
}

/// Generate a [TsType] node from a [Schema], returning the generated type
/// along with a boolean flag indicating whether the type is considered
/// "optional" in the schema
fn schema_type<'a>(
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a RootSchema,
    schema: &'a Schema,
) -> (AnyTsType, bool, Option<&'a String>) {
    match schema {
        // Types defined as `true` in the schema always pass validation,
        // map them to the `any` type
        Schema::Bool(true) => (
            AnyTsType::from(make::ts_any_type(make::token(T![any]))),
            true,
            None,
        ),
        // Types defined as `false` in the schema never pass validation,
        // map them to the `never` type
        Schema::Bool(false) => (
            AnyTsType::from(make::ts_never_type(make::token(T![never]))),
            false,
            None,
        ),
        Schema::Object(schema_object) => schema_object_type(queue, root_schema, schema_object),
    }
}

/// Generate and emit all the types defined in `root_schema` into the `module`
pub fn generate_type<'a>(
    module: &mut Vec<(AnyJsDeclaration, Option<&'a String>)>,
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a RootSchema,
) -> AnyTsType {
    // Read the root type of the schema and push it to the queue
    let root_name = root_schema
        .schema
        .metadata
        .as_deref()
        .and_then(|metadata| metadata.title.as_deref())
        .unwrap();

    match root_name {
        "Null" => return AnyTsType::TsVoidType(make::ts_void_type(make::token(T![void]))),
        "Boolean" => {
            return AnyTsType::TsBooleanType(make::ts_boolean_type(make::token(T![boolean])))
        }
        "String" => return AnyTsType::TsStringType(make::ts_string_type(make::token(T![string]))),
        _ => {}
    }

    queue.push_back((root_name, &root_schema.schema));

    while let Some((name, schema)) = queue.pop_front() {
        // Detect if the type being emitted is an object, emit it as an
        // interface definition if that's the case
        let is_interface = schema.instance_type.as_ref().map_or_else(
            || schema.object.is_some(),
            |instance_type| {
                if let SingleOrVec::Single(instance_type) = instance_type {
                    matches!(**instance_type, InstanceType::Object)
                } else {
                    false
                }
            },
        );

        if is_interface {
            let mut members = Vec::new();

            // Create a property signature member in the interface for each
            // property of the corresponding schema object
            let object = schema.object.as_deref().unwrap();
            for (property_str, schema) in &object.properties {
                let (ts_type, optional, description) = schema_type(queue, root_schema, schema);

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

                let type_annotation = if let Some((container_type, key_type, value_type)) =
                    match property_str.as_str() {
                        "featuresSupported" => Some(("Map", "FeatureKind", "SupportKind")),
                        "domains" => Some(("Record", "RuleDomain", "RuleDomainValue")),
                        _ => None,
                    } {
                    // HACK: force the `featuresSupported` property to be a Map<FeatureKind, SupportKind>
                    // This is a temporary workaround to fix the type annotation for this property. The
                    // better fix would be to use the `transform` feature that is available in `schemars` 1.0 to
                    // add a metadata field that we can pick up here to generate the correct type annotation.
                    // Alternatively, we could generate these types based on the actual rust types instead of the
                    // json schema.
                    //
                    // We also manually fix the types for some other properties as well.
                    let full_type = make::ts_reference_type(
                        make::js_reference_identifier(make::ident(container_type)).into(),
                    )
                    .with_type_arguments(make::ts_type_arguments(
                        make::token(T![<]),
                        make::ts_type_argument_list(
                            [
                                make::ts_reference_type(
                                    make::js_reference_identifier(make::ident(key_type)).into(),
                                )
                                .build()
                                .into(),
                                make::ts_reference_type(
                                    make::js_reference_identifier(make::ident(value_type)).into(),
                                )
                                .build()
                                .into(),
                            ],
                            [make::token(T![,])],
                        ),
                        make::token(T![>]),
                    ))
                    .build();
                    make::ts_type_annotation(make::token(T![:]), full_type.into())
                } else {
                    make::ts_type_annotation(make::token(T![:]), ts_type)
                };

                let mut builder = make::ts_property_signature_type_member(
                    AnyJsObjectMemberName::from(make::js_literal_member_name(property)),
                )
                .with_type_annotation(type_annotation);

                if optional {
                    builder = builder.with_optional_token(make::token(T![?]));
                }

                members.push(AnyTsTypeMember::from(builder.build()));
            }

            let description = schema
                .metadata
                .as_ref()
                .and_then(|s| s.description.as_ref());
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
            let (ts_type, optional, description) = schema_object_type(queue, root_schema, schema);

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
    pub params: RootSchema,
    /// Schema for the result object of the method
    pub result: RootSchema,
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
pub fn methods() -> [WorkspaceMethod; 21] {
    [
        workspace_method!(file_features),
        workspace_method!(update_settings),
        workspace_method!(open_project),
        workspace_method!(open_file),
        workspace_method!(change_file),
        workspace_method!(close_file),
        workspace_method!(get_syntax_tree),
        workspace_method!(check_file_size),
        workspace_method!(get_file_content),
        workspace_method!(get_control_flow_graph),
        workspace_method!(get_formatter_ir),
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

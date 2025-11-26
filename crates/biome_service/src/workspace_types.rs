//! Utility functions to help with generating bindings for the [Workspace] API

use std::collections::VecDeque;

use biome_js_syntax::{AnyJsDeclaration, AnyTsTupleTypeElement};
use rustc_hash::FxHashSet;
use schemars::{JsonSchema, Schema, SchemaGenerator, generate::SchemaSettings};
use serde_json::Value;

use crate::{WorkspaceError, workspace::*};
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
    queue: VecDeque<(&'a str, &'a Value)>,
}

impl<'a> ModuleQueue<'a> {
    /// Add a type definition to the queue if it hasn't been emitted already
    fn push_back(&mut self, item: (&'a str, &'a Value)) {
        if self.visited.insert(item.0) {
            self.queue.push_back(item);
        }
    }

    /// Pull a type name and definition from the queue
    fn pop_front(&mut self) -> Option<(&'a str, &'a Value)> {
        self.queue.pop_front()
    }

    pub fn visited(&self) -> &FxHashSet<&'a str> {
        &self.visited
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

/// Generate a [TsType] node from the `type` field of a schema
fn instance_type<'a>(
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a Schema,
    schema: &'a Value,
    ty: &str,
) -> AnyTsType {
    match ty {
        // If the instance type is an object, generate a TS object type with the corresponding properties
        "object" => {
            // Get the list of required properties from the schema
            let required_properties = schema
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<std::collections::HashSet<_>>()
                })
                .unwrap_or_default();

            let properties_obj =
                schema
                    .get("properties")
                    .and_then(|v| v.as_object())
                    .map(|props| {
                        props
                            .iter()
                            .map(|(property, prop_schema)| {
                                // prop_schema is already a &Value, which is the same as &Schema in v1.0
                                let (ts_type, field_optional, description) =
                                    schema_type(queue, root_schema, prop_schema);

                                // In OpenAPI 3.0, a field is optional if it's NOT in the required array
                                // OR if the field schema itself marks it as optional (has default, etc.)
                                let optional = !required_properties.contains(property.as_str())
                                    || field_optional;

                                // If the field is optional, strip null from the type since `field?: T`
                                // already implies the field can be omitted (undefined)
                                let ts_type = if optional {
                                    strip_null_from_union(ts_type)
                                } else {
                                    ts_type
                                };

                                let mut property_ident = make::ident(property);
                                if let Some(description) = description {
                                    let comment = format!("/**\n\t* {description} \n\t */");
                                    let trivia = vec![
                                        (TriviaPieceKind::Newline, "\n"),
                                        (TriviaPieceKind::MultiLineComment, comment.as_str()),
                                        (TriviaPieceKind::Newline, "\n"),
                                    ];
                                    property_ident = property_ident.with_leading_trivia(trivia);
                                }

                                let mut builder = make::ts_property_signature_type_member(
                                    AnyJsObjectMemberName::from(make::js_literal_member_name(
                                        property_ident,
                                    )),
                                )
                                .with_type_annotation(make::ts_type_annotation(
                                    make::token(T![:]),
                                    ts_type,
                                ));

                                if optional {
                                    builder = builder.with_optional_token(make::token(T![?]));
                                }

                                AnyTsTypeMember::from(builder.build())
                            })
                            .collect::<Vec<_>>()
                    });

            let properties_type = properties_obj.and_then(|properties| {
                (!properties.is_empty()).then(|| {
                    make::ts_object_type(
                        make::token(T!['{']),
                        make::ts_type_member_list(properties),
                        make::token(T!['}']),
                    )
                    .into()
                })
            });

            // Don't use `additionalProperties: false` here.
            let additional_properties = schema.get("additionalProperties").and_then(|v| {
                if v.as_bool() == Some(false) {
                    None
                } else {
                    Some(v)
                }
            });

            // If `additionalProperties` is not empty, add a mapped or record type.
            let additional_properties_type = additional_properties.map(|add_props| {
                // If `propertyNames` is not empty, use it as the key type.
                let key_type = schema.get("propertyNames").map(|v| {
                    let (ts_type, optional, _) = schema_type(queue, root_schema, v);
                    assert!(!optional, "optional nested types are not supported");
                    ts_type
                });

                let value_type = {
                    let (ts_type, optional, _) = schema_type(queue, root_schema, add_props);
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
                        make::ts_mapped_type_optional_modifier_clause(make::token(T![?])).build(),
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

            if result.is_empty() {
                // Empty object with no properties and no additionalProperties
                // Generate an empty object type: {}
                make::ts_object_type(
                    make::token(T!['{']),
                    make::ts_type_member_list([]),
                    make::token(T!['}']),
                )
                .into()
            } else if result.len() == 1 {
                // Single type, no need for intersection
                result.into_iter().next().unwrap()
            } else {
                // Multiple types, create intersection
                let separators = (0..result.len().saturating_sub(1)).map(|_| make::token(T![&]));
                make::ts_intersection_type(make::ts_intersection_type_element_list(
                    result, separators,
                ))
                .build()
                .into()
            }
        }
        // If the instance type is an array, generate a TS array type with the corresponding item type
        "array" => {
            // OpenAPI 3.1/JSON Schema 2020-12 uses prefixItems for tuples
            let prefix_items = schema.get("prefixItems").and_then(|v| v.as_array());

            if let Some(items_array) = prefix_items {
                // Tuple type with prefixItems
                AnyTsType::from(make::ts_tuple_type(
                    make::token(T!['[']),
                    make::ts_tuple_type_element_list(
                        items_array.iter().map(|item| {
                            let (ts_type, optional, _) = schema_type(queue, root_schema, item);
                            assert!(!optional, "optional nested types are not supported");
                            AnyTsTupleTypeElement::AnyTsType(ts_type)
                        }),
                        items_array.iter().map(|_| make::token(T![,])),
                    ),
                    make::token(T![']']),
                ))
            } else {
                // Fall back to items
                let items = schema.get("items");
                match items {
                    Some(Value::Object(_)) => {
                        let (ts_type, optional, _) =
                            schema_type(queue, root_schema, items.unwrap());
                        assert!(!optional, "optional nested types are not supported");

                        AnyTsType::from(make::ts_array_type(
                            ts_type,
                            make::token(T!['[']),
                            make::token(T![']']),
                        ))
                    }
                    Some(Value::Array(items_array)) => AnyTsType::from(make::ts_tuple_type(
                        make::token(T!['[']),
                        make::ts_tuple_type_element_list(
                            items_array.iter().map(|item| {
                                let (ts_type, optional, _) = schema_type(queue, root_schema, item);
                                assert!(!optional, "optional nested types are not supported");
                                AnyTsTupleTypeElement::AnyTsType(ts_type)
                            }),
                            items_array.iter().map(|_| make::token(T![,])),
                        ),
                        make::token(T![']']),
                    )),
                    _ => AnyTsType::from(make::ts_any_type(make::token(T![any]))),
                }
            }
        }

        // Map native types to the corresponding TS type
        "null" => AnyTsType::from(make::ts_null_literal_type(make::token(T![null]))),
        "boolean" => AnyTsType::from(make::ts_boolean_type(make::token(T![boolean]))),
        "string" => AnyTsType::from(make::ts_string_type(make::token(T![string]))),
        "number" | "integer" => AnyTsType::from(make::ts_number_type(make::token(T![number]))),
        _ => AnyTsType::from(make::ts_any_type(make::token(T![any]))),
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

/// Strip null from a union type. If the type is `T | null`, returns `T`.
/// If the type is not a union or doesn't contain null, returns the type unchanged.
fn strip_null_from_union(ts_type: AnyTsType) -> AnyTsType {
    match &ts_type {
        AnyTsType::TsUnionType(union_type) => {
            let non_null_types: Vec<_> = union_type
                .types()
                .iter()
                .filter_map(|ty| ty.ok())
                .filter(|ty| !matches!(ty, AnyTsType::TsNullLiteralType(_)))
                .collect();

            if non_null_types.is_empty() {
                // All types were null, return null
                ts_type
            } else if non_null_types.len() == 1 {
                // Only one non-null type, return it directly
                non_null_types.into_iter().next().unwrap()
            } else {
                // Multiple non-null types, create a new union
                make_union_type(non_null_types)
            }
        }
        _ => ts_type,
    }
}

/// Generate a [TsType] node from a [Schema], returning the generated
/// TypeScript type along with a boolean flag indicating whether the type is
/// considered "optional" in the schema
fn schema_object_type<'a>(
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a Schema,
    schema: &'a Value,
) -> (AnyTsType, bool, Option<String>) {
    // Start by detecting enum types by inspecting the `enum` field
    let description = schema
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let ts_type = schema
        .get("enum")
        .and_then(|v| v.as_array())
        .map(|enum_values| make_union_type(enum_values.iter().map(value_type)))
        // Handle JSON Schema "const" for single constant values
        .or_else(|| {
            let const_value = schema.get("const")?;
            Some(value_type(const_value))
        })
        // If the type isn't an enum or const, inspect its `type` field
        .or_else(|| {
            let type_value = schema.get("type")?;
            match type_value {
                Value::String(ty) => Some(instance_type(queue, root_schema, schema, ty.as_str())),
                Value::Array(types) => Some(make_union_type(
                    types
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|ty| instance_type(queue, root_schema, schema, ty)),
                )),
                _ => None,
            }
        })
        // Otherwise inspect the `$ref` field of the schema
        .or_else(|| {
            let reference = schema.get("$ref")?.as_str()?;

            // Handle multiple $ref formats:
            // - JSON Schema Draft 2020-12: #/$defs/Key
            // - JSON Schema Draft-07: #/definitions/Key
            // - OpenAPI 3.0: #/components/schemas/Key
            let (key, def_schema) = if let Some(key) = reference.strip_prefix("#/$defs/") {
                let def = root_schema.get("$defs")?.as_object()?.get(key)?;
                (key, def)
            } else if let Some(key) = reference.strip_prefix("#/definitions/") {
                let def = root_schema.get("definitions")?.as_object()?.get(key)?;
                (key, def)
            } else if let Some(key) = reference.strip_prefix("#/components/schemas/") {
                let def = root_schema
                    .get("components")?
                    .as_object()?
                    .get("schemas")?
                    .as_object()?
                    .get(key)?;
                (key, def)
            } else {
                return None;
            };

            queue.push_back((key, def_schema));

            // Apply type name renaming for the reference
            let renamed_key = rename_type(key).unwrap_or_else(|| key.to_string());
            Some(AnyTsType::from(
                make::ts_reference_type(AnyTsName::from(make::js_reference_identifier(
                    make::ident(&renamed_key),
                )))
                .build(),
            ))
        })
        // Finally try to inspect subschemas (allOf, anyOf, oneOf)
        .or_else(|| {
            // First try allOf
            if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
                return Some(AnyTsType::from(
                    make::ts_intersection_type(make::ts_intersection_type_element_list(
                        all_of.iter().map(|ty| {
                            let (ts_type, optional, _) = schema_type(queue, root_schema, ty);
                            assert!(!optional, "optional nested types are not supported");
                            ts_type
                        }),
                        (0..all_of.len().saturating_sub(1)).map(|_| make::token(T![&])),
                    ))
                    .build(),
                ));
            }

            // Then try anyOf or oneOf
            let union_schemas = schema
                .get("anyOf")
                .or_else(|| schema.get("oneOf"))
                .and_then(|v| v.as_array())?;

            Some(make_union_type(union_schemas.iter().map(|ty| {
                let (ts_type, optional, _) = schema_type(queue, root_schema, ty);
                assert!(
                    !optional,
                    "optional nested types are not supported. Schema: {}",
                    serde_json::to_string_pretty(ty).unwrap_or_else(|_| format!("{:?}", ty))
                );
                ts_type
            })))
        })
        .unwrap_or_else(|| {
            // this is temporary workaround to fix the `options` field, which is not used at the moment
            AnyTsType::from(make::ts_any_type(make::token(T![any])))
        });

    // Types are considered "optional" in the serialization protocol if they
    // have the `nullable` OpenAPI extension property, or if they have a default value
    // However, if this schema is just representing a null type (enum: [null]), don't mark it as optional
    // since it will be used in a union context
    let is_null_only = schema
        .get("enum")
        .and_then(|v| v.as_array())
        .is_some_and(|arr| arr.len() == 1 && arr[0].is_null());

    let is_nullable = schema
        .get("nullable")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let has_defaults = schema.get("default").is_some();

    let is_optional = (is_nullable || has_defaults) && !is_null_only;

    (ts_type, is_optional, description)
}

/// Generate a [TsType] node from a [Schema], returning the generated type
/// along with a boolean flag indicating whether the type is considered
/// "optional" in the schema
fn schema_type<'a>(
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a Schema,
    schema: &'a Value,
) -> (AnyTsType, bool, Option<String>) {
    // Check if schema is a boolean value
    if let Some(bool_val) = schema.as_bool() {
        return if bool_val {
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
        };
    }

    // Otherwise treat as a schema object
    schema_object_type(queue, root_schema, schema)
}

/// Generate and emit all the types defined in `root_schema` into the `module`
pub fn generate_type<'a>(
    module: &mut Vec<(AnyJsDeclaration, Option<String>)>,
    queue: &mut ModuleQueue<'a>,
    root_schema: &'a Schema,
) -> AnyTsType {
    // Read the root type of the schema and push it to the queue
    let root_name = root_schema.get("title").and_then(|v| v.as_str()).unwrap();

    match root_name {
        "Null" => return AnyTsType::TsVoidType(make::ts_void_type(make::token(T![void]))),
        "Boolean" => {
            return AnyTsType::TsBooleanType(make::ts_boolean_type(make::token(T![boolean])));
        }
        "String" => return AnyTsType::TsStringType(make::ts_string_type(make::token(T![string]))),
        _ => {}
    }

    // Push the root type to the queue for processing
    queue.push_back((root_name, root_schema.as_value()));

    while let Some((name, schema)) = queue.pop_front() {
        // Apply type name renaming for complex generic types
        let name = rename_type(name).unwrap_or_else(|| name.to_string());
        let name = name.as_str();

        // Skip generating type aliases for TypeScript reserved primitive type names
        // These would create invalid definitions like "export type string = string"
        if matches!(
            name,
            "null" | "string" | "boolean" | "number" | "any" | "void" | "never" | "unknown"
        ) {
            continue;
        }

        // Detect if the type being emitted is an object, emit it as an
        // interface definition if that's the case
        let has_properties = schema
            .get("properties")
            .and_then(|v| v.as_object())
            .is_some_and(|props| !props.is_empty());
        let additional_props_is_false = schema
            .get("additionalProperties")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let type_is_object = schema
            .get("type")
            .and_then(|v| v.as_str())
            .is_none_or(|t| t == "object");

        // Generate an interface if:
        // 1. It's an object type with properties (most common case)
        // 2. OR it's an empty object with additionalProperties: false
        let is_interface = type_is_object
            && (has_properties
                || (additional_props_is_false && schema.get("properties").is_none()));

        if is_interface {
            let mut members = Vec::new();

            // Get the list of required properties from the schema
            let required_properties = schema
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<std::collections::HashSet<_>>()
                })
                .unwrap_or_default();

            // Create a property signature member in the interface for each
            // property of the corresponding schema object
            if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
                for (property_str, prop_value) in properties {
                    let (ts_type, field_optional, description) =
                        schema_type(queue, root_schema, prop_value);

                    // In OpenAPI 3.0, a field is optional if it's NOT in the required array
                    // OR if the field schema itself marks it as optional (has default, etc.)
                    let optional =
                        !required_properties.contains(property_str.as_str()) || field_optional;

                    // If the field is optional, strip null from the type since `field?: T`
                    // already implies the field can be omitted (undefined)
                    let ts_type = if optional {
                        strip_null_from_union(ts_type)
                    } else {
                        ts_type
                    };

                    let mut property = make::ident(property_str);
                    if let Some(ref description) = description {
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
            }

            let description = schema
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
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

    // Apply type name renaming for the root type reference
    let renamed_root = rename_type(root_name).unwrap_or_else(|| root_name.to_string());
    AnyTsType::TsReferenceType(
        make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
            make::js_reference_identifier(make::ident(&renamed_root)),
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
        let params = SchemaGenerator::from(SchemaSettings::draft2019_09()).root_schema_for::<P>();
        let result = SchemaGenerator::from(SchemaSettings::draft2019_09()).root_schema_for::<R>();
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
pub fn methods() -> [WorkspaceMethod; 30] {
    [
        workspace_method!(file_features),
        workspace_method!(update_settings),
        workspace_method!(open_project),
        workspace_method!(scan_project),
        workspace_method!(open_file),
        workspace_method!(change_file),
        workspace_method!(close_file),
        workspace_method!(file_exists),
        workspace_method!(is_path_ignored),
        workspace_method!(update_module_graph),
        workspace_method!(get_syntax_tree),
        workspace_method!(check_file_size),
        workspace_method!(get_file_content),
        workspace_method!(get_control_flow_graph),
        workspace_method!(get_formatter_ir),
        workspace_method!(get_type_info),
        workspace_method!(get_registered_types),
        workspace_method!(get_semantic_model),
        workspace_method!(get_module_graph),
        workspace_method!(pull_diagnostics),
        workspace_method!(pull_actions),
        workspace_method!(pull_diagnostics_and_actions),
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

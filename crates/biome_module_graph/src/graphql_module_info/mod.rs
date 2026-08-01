use biome_graphql_syntax::{
    GraphqlFieldsDefinition, GraphqlInputFieldsDefinition, GraphqlInputObjectTypeDefinition,
    GraphqlInputObjectTypeExtension, GraphqlInterfaceTypeDefinition, GraphqlInterfaceTypeExtension,
    GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension, GraphqlRoot,
};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum GraphqlTypeKind {
    Object,
    Interface,
    InputObject,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GraphqlTypeKey {
    pub kind: GraphqlTypeKind,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GraphqlModuleInfo {
    pub type_fields: FxHashMap<GraphqlTypeKey, FxHashSet<String>>,
    pub operation_names: FxHashSet<String>,
}

impl GraphqlModuleInfo {
    pub fn fields_for_type(
        &self,
        kind: GraphqlTypeKind,
        type_name: &str,
    ) -> Option<&FxHashSet<String>> {
        self.type_fields.get(&GraphqlTypeKey {
            kind,
            name: type_name.to_string(),
        })
    }

    fn insert_type_fields(
        &mut self,
        kind: GraphqlTypeKind,
        type_name: String,
        fields: impl IntoIterator<Item = String>,
    ) {
        self.type_fields
            .entry(GraphqlTypeKey {
                kind,
                name: type_name,
            })
            .or_default()
            .extend(fields);
    }

    fn insert_operation_name(&mut self, operation_name: String) {
        self.operation_names.insert(operation_name);
    }
}

pub(crate) fn collect_graphql_module_info(root: GraphqlRoot) -> GraphqlModuleInfo {
    let mut info = GraphqlModuleInfo::default();

    for definition in root.definitions() {
        if let Some(operation_definition) = definition.as_graphql_operation_definition() {
            if let Some(operation_name) = operation_definition.name()
                && let Ok(operation_name_token) = operation_name.value_token()
            {
                info.insert_operation_name(operation_name_token.text_trimmed().to_string());
            }

            continue;
        }

        if let Some(type_definition) = definition.as_any_graphql_type_definition() {
            if let Some(definition) = type_definition.as_graphql_object_type_definition() {
                if let Some(type_name) = type_name_from_object_definition(definition) {
                    info.insert_type_fields(
                        GraphqlTypeKind::Object,
                        type_name,
                        collect_field_names(definition.fields()),
                    );
                }
                continue;
            }

            if let Some(definition) = type_definition.as_graphql_interface_type_definition() {
                if let Some(type_name) = type_name_from_interface_definition(definition) {
                    info.insert_type_fields(
                        GraphqlTypeKind::Interface,
                        type_name,
                        collect_field_names(definition.fields()),
                    );
                }
                continue;
            }

            if let Some(definition) = type_definition.as_graphql_input_object_type_definition()
                && let Some(type_name) = type_name_from_input_definition(definition)
            {
                info.insert_type_fields(
                    GraphqlTypeKind::InputObject,
                    type_name,
                    collect_input_field_names(definition.input_fields()),
                );
            }

            continue;
        }

        if let Some(type_extension) = definition.as_any_graphql_type_extension() {
            if let Some(definition) = type_extension.as_graphql_object_type_extension() {
                if let Some(type_name) = type_name_from_object_extension(definition) {
                    info.insert_type_fields(
                        GraphqlTypeKind::Object,
                        type_name,
                        collect_field_names(definition.fields()),
                    );
                }
                continue;
            }

            if let Some(definition) = type_extension.as_graphql_interface_type_extension() {
                if let Some(type_name) = type_name_from_interface_extension(definition) {
                    info.insert_type_fields(
                        GraphqlTypeKind::Interface,
                        type_name,
                        collect_field_names(definition.fields()),
                    );
                }
                continue;
            }

            if let Some(definition) = type_extension.as_graphql_input_object_type_extension()
                && let Some(type_name) = type_name_from_input_extension(definition)
            {
                info.insert_type_fields(
                    GraphqlTypeKind::InputObject,
                    type_name,
                    collect_input_field_names(definition.input_fields()),
                );
            }
        }
    }

    info
}

fn collect_field_names(fields: Option<GraphqlFieldsDefinition>) -> Vec<String> {
    let Some(fields) = fields else {
        return Vec::new();
    };

    fields
        .fields()
        .into_iter()
        .filter_map(|field| {
            field
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .map(|token| token.text_trimmed().to_string())
        })
        .collect()
}

fn collect_input_field_names(fields: Option<GraphqlInputFieldsDefinition>) -> Vec<String> {
    let Some(fields) = fields else {
        return Vec::new();
    };

    fields
        .fields()
        .into_iter()
        .filter_map(|field| {
            field
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .map(|token| token.text_trimmed().to_string())
        })
        .collect()
}

fn type_name_from_object_definition(node: &GraphqlObjectTypeDefinition) -> Option<String> {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .map(|token| token.text_trimmed().to_string())
}

fn type_name_from_object_extension(node: &GraphqlObjectTypeExtension) -> Option<String> {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .map(|token| token.text_trimmed().to_string())
}

fn type_name_from_interface_definition(node: &GraphqlInterfaceTypeDefinition) -> Option<String> {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .map(|token| token.text_trimmed().to_string())
}

fn type_name_from_interface_extension(node: &GraphqlInterfaceTypeExtension) -> Option<String> {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .map(|token| token.text_trimmed().to_string())
}

fn type_name_from_input_definition(node: &GraphqlInputObjectTypeDefinition) -> Option<String> {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .map(|token| token.text_trimmed().to_string())
}

fn type_name_from_input_extension(node: &GraphqlInputObjectTypeExtension) -> Option<String> {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .map(|token| token.text_trimmed().to_string())
}

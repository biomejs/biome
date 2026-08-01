use std::collections::HashSet;

use crate::services::module_graph::GraphqlModuleGraph;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    GraphqlFieldsDefinition, GraphqlInputFieldsDefinition, GraphqlInputObjectTypeDefinition,
    GraphqlInputObjectTypeExtension, GraphqlInterfaceTypeDefinition, GraphqlInterfaceTypeExtension,
    GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
};
use biome_module_graph::{GraphqlTypeKind, ModuleInfoKind};
use biome_rowan::{TextRange, declare_node_union};
use biome_rule_options::no_duplicate_field_definition_names::NoDuplicateFieldDefinitionNamesOptions;

declare_lint_rule! {
    /// Require all fields of a type to be unique.
    ///
    /// A GraphQL complex type is only valid if all its fields are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// type SomeObject {
    ///   foo: String
    ///   foo: String
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// interface SomeObject {
    ///   foo: String
    ///   foo: String
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// input SomeObject {
    ///   foo: String
    ///   foo: String
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// type SomeObject {
    ///   foo: String
    ///   bar: String
    /// }
    /// ```
    ///
    /// ```graphql
    /// interface SomeObject {
    ///   foo: String
    ///   bar: String
    /// }
    /// ```
    ///
    /// ```graphql
    /// input SomeObject {
    ///   foo: String
    ///   bar: String
    /// }
    /// ```
    ///
    pub NoDuplicateFieldDefinitionNames {
        version: "2.3.11",
        name: "noDuplicateFieldDefinitionNames",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-field-definition-names").same()],
    }
}

impl Rule for NoDuplicateFieldDefinitionNames {
    type Query = GraphqlModuleGraph<AnyNoDuplicateFieldDefinitionNamesQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoDuplicateFieldDefinitionNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let (type_kind, type_name, fields) = match node {
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlObjectTypeDefinition(object_def) => {
                let type_name = type_name_from_object_definition(object_def)?;
                let fields = collect_fields(object_def.fields()?)?;
                (GraphqlTypeKind::Object, type_name, fields)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlObjectTypeExtension(object_ext) => {
                let type_name = type_name_from_object_extension(object_ext)?;
                let fields = collect_fields(object_ext.fields()?)?;
                (GraphqlTypeKind::Object, type_name, fields)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInterfaceTypeDefinition(
                interface_def,
            ) => {
                let type_name = type_name_from_interface_definition(interface_def)?;
                let fields = collect_fields(interface_def.fields()?)?;
                (GraphqlTypeKind::Interface, type_name, fields)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInterfaceTypeExtension(
                interface_ext,
            ) => {
                let type_name = type_name_from_interface_extension(interface_ext)?;
                let fields = collect_fields(interface_ext.fields()?)?;
                (GraphqlTypeKind::Interface, type_name, fields)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInputObjectTypeDefinition(
                input_def,
            ) => {
                let type_name = type_name_from_input_definition(input_def)?;
                let fields = collect_input_fields(input_def.input_fields()?)?;
                (GraphqlTypeKind::InputObject, type_name, fields)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInputObjectTypeExtension(input_ext) => {
                let type_name = type_name_from_input_extension(input_ext)?;
                let fields = collect_input_fields(input_ext.input_fields()?)?;
                (GraphqlTypeKind::InputObject, type_name, fields)
            }
        };

        if let Some(range) = check_local_duplicates(&fields) {
            return Some(range);
        }

        check_cross_file_duplicates(ctx, type_kind, &type_name, &fields)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *state,
                markup! {
                    "Duplicate field name."
                },
            )
            .note(markup! {
                "A GraphQL complex type is only valid if all its fields are uniquely named. Make sure to name every field differently."
            }),
        )
    }
}

type FieldNameAndRange = (String, TextRange);

fn collect_fields(fields: GraphqlFieldsDefinition) -> Option<Vec<FieldNameAndRange>> {
    let mut result = Vec::new();

    for element in fields.fields() {
        if let Some(name) = element.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            result.push((value_token.text_trimmed().to_string(), value_token.text_trimmed_range()));
        }
    }

    if result.is_empty() {
        return None;
    }

    Some(result)
}

fn collect_input_fields(fields: GraphqlInputFieldsDefinition) -> Option<Vec<FieldNameAndRange>> {
    let mut result = Vec::new();

    for element in fields.fields() {
        if let Some(name) = element.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            result.push((value_token.text_trimmed().to_string(), value_token.text_trimmed_range()));
        }
    }

    if result.is_empty() {
        return None;
    }

    Some(result)
}

fn check_local_duplicates(fields: &[FieldNameAndRange]) -> Option<TextRange> {
    let mut found = HashSet::new();

    for (name, range) in fields {
        if found.contains(name) {
            return Some(*range);
        }

        found.insert(name.clone());
    }

    None
}

fn check_cross_file_duplicates(
    ctx: &RuleContext<NoDuplicateFieldDefinitionNames>,
    type_kind: GraphqlTypeKind,
    type_name: &str,
    local_fields: &[FieldNameAndRange],
) -> Option<TextRange> {
    let db = ctx.db()?;

    let mut external_fields = HashSet::new();
    db.for_each_module(&mut |path, kind| {
        if path == ctx.file_path() {
            return;
        }

        let ModuleInfoKind::Graphql(module) = kind else {
            return;
        };

        if let Some(fields) = module.fields_for_type(type_kind, type_name) {
            external_fields.extend(fields.iter().cloned());
        }
    });

    if external_fields.is_empty() {
        return None;
    }

    for (field_name, range) in local_fields {
        if external_fields.contains(field_name) {
            return Some(*range);
        }
    }

    None
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

declare_node_union! {
    pub AnyNoDuplicateFieldDefinitionNamesQuery = GraphqlObjectTypeDefinition | GraphqlObjectTypeExtension | GraphqlInterfaceTypeDefinition | GraphqlInterfaceTypeExtension | GraphqlInputObjectTypeDefinition | GraphqlInputObjectTypeExtension
}

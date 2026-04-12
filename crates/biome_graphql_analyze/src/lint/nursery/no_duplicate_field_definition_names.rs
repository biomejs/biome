use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlDefinition, GraphqlFieldsDefinition, GraphqlInputFieldsDefinition, GraphqlRoot,
};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_duplicate_field_definition_names::NoDuplicateFieldDefinitionNamesOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Require all fields of a type to be unique.
    ///
    /// A GraphQL complex type is only valid if all its fields are uniquely named.
    /// This includes fields across type definitions and their extensions within the
    /// same document.
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

pub struct DuplicateFieldDefinitionName {
    text_range: TextRange,
}

impl Rule for NoDuplicateFieldDefinitionNames {
    type Query = Ast<GraphqlRoot>;
    type State = DuplicateFieldDefinitionName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateFieldDefinitionNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        // Maps type name -> set of field names seen so far across definitions and extensions.
        let mut type_fields: FxHashMap<TokenText, HashSet<TokenText>> = FxHashMap::default();
        let mut duplicates = vec![];

        for definition in root.definitions() {
            let extracted = extract_fields(&definition);

            if let Some((type_name, field_names, node_range)) = extracted {
                let seen = type_fields.entry(type_name).or_default();
                let mut has_duplicate = false;

                // Check for duplicates against fields already seen from previous
                // definitions or extensions of the same type.
                for name in &field_names {
                    if seen.contains(name) {
                        has_duplicate = true;
                        break;
                    }
                }

                // Also check for duplicates within this node's own field list.
                if !has_duplicate {
                    let mut local = HashSet::new();
                    for name in &field_names {
                        if !local.insert(name.clone()) {
                            has_duplicate = true;
                            break;
                        }
                    }
                }

                if has_duplicate {
                    duplicates.push(DuplicateFieldDefinitionName {
                        text_range: node_range,
                    });
                }

                seen.extend(field_names);
            }
        }

        duplicates.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                &state.text_range,
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

/// Extract the type name, field names, and node range from any definition that
/// defines or extends an object type, interface type, or input object type.
fn extract_fields(
    definition: &AnyGraphqlDefinition,
) -> Option<(TokenText, Vec<TokenText>, TextRange)> {
    match definition {
        AnyGraphqlDefinition::GraphqlObjectTypeDefinition(def) => {
            let type_name = def.name().ok()?.value_token().ok()?.token_text();
            let fields = def.fields()?;
            Some((type_name, collect_field_names(&fields), def.range()))
        }
        AnyGraphqlDefinition::GraphqlObjectTypeExtension(ext) => {
            let type_name = ext.name().ok()?.value_token().ok()?.token_text();
            let fields = ext.fields()?;
            Some((type_name, collect_field_names(&fields), ext.range()))
        }
        AnyGraphqlDefinition::GraphqlInterfaceTypeDefinition(def) => {
            let type_name = def.name().ok()?.value_token().ok()?.token_text();
            let fields = def.fields()?;
            Some((type_name, collect_field_names(&fields), def.range()))
        }
        AnyGraphqlDefinition::GraphqlInterfaceTypeExtension(ext) => {
            let type_name = ext.name().ok()?.value_token().ok()?.token_text();
            let fields = ext.fields()?;
            Some((type_name, collect_field_names(&fields), ext.range()))
        }
        AnyGraphqlDefinition::GraphqlInputObjectTypeDefinition(def) => {
            let type_name = def.name().ok()?.value_token().ok()?.token_text();
            let fields = def.input_fields()?;
            Some((type_name, collect_input_field_names(&fields), def.range()))
        }
        AnyGraphqlDefinition::GraphqlInputObjectTypeExtension(ext) => {
            let type_name = ext.name().ok()?.value_token().ok()?.token_text();
            let fields = ext.input_fields()?;
            Some((type_name, collect_input_field_names(&fields), ext.range()))
        }
        _ => None,
    }
}

/// Collect field names from a GraphqlFieldsDefinition.
fn collect_field_names(fields: &GraphqlFieldsDefinition) -> Vec<TokenText> {
    let mut names = Vec::new();
    for element in fields.fields() {
        if let Some(name) = element.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            names.push(value_token.token_text());
        }
    }
    names
}

/// Collect field names from a GraphqlInputFieldsDefinition.
fn collect_input_field_names(fields: &GraphqlInputFieldsDefinition) -> Vec<TokenText> {
    let mut names = Vec::new();
    for element in fields.fields() {
        if let Some(name) = element.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            names.push(value_token.token_text());
        }
    }
    names
}

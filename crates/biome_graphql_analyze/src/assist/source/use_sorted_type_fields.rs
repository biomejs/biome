use crate::GraphqlRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_graphql_syntax::{
    GraphqlFieldDefinition, GraphqlFieldDefinitionList, GraphqlFieldsDefinition,
    GraphqlInputFieldList, GraphqlInputFieldsDefinition, GraphqlInputObjectTypeDefinition,
    GraphqlInputObjectTypeExtension, GraphqlInputValueDefinition, GraphqlInterfaceTypeDefinition,
    GraphqlInterfaceTypeExtension, GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, NodeOrToken, SyntaxNode, TokenText, declare_node_union,
};
use biome_rule_options::use_sorted_type_fields::UseSortedTypeFieldsOptions;
use std::cmp::Ordering;

declare_source_rule! {
    /// Sort fields in GraphQL type definitions alphabetically.
    ///
    /// This rule ensures that fields within `type`, `interface`, and `input`
    /// definitions are sorted alphabetically. For GraphQL identifiers (`[A-Za-z0-9_]`),
    /// the sort order matches JavaScript's `localeCompare()`, including case handling.
    ///
    /// ## Examples
    ///
    /// ```graphql,expect_diff
    /// type User {
    ///   name: String
    ///   age: Int
    ///   id: ID
    /// }
    /// ```
    ///
    /// ```graphql,expect_diff
    /// interface Node {
    ///   name: String
    ///   id: ID
    /// }
    /// ```
    ///
    /// ```graphql,expect_diff
    /// input CreateUserInput {
    ///   name: String
    ///   age: Int
    /// }
    /// ```
    ///
    pub UseSortedTypeFields {
        version: "next",
        name: "useSortedTypeFields",
        language: "graphql",
        recommended: false,
        fix_kind: FixKind::Safe,
        sources: &[RuleSource::EslintGraphql("alphabetize").inspired()],
    }
}

impl Rule for UseSortedTypeFields {
    type Query = Ast<AnyUseSortedTypeFieldsQuery>;
    type State = UseSortedTypeFieldsState;
    type Signals = Option<Self::State>;
    type Options = UseSortedTypeFieldsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyUseSortedTypeFieldsQuery::GraphqlObjectTypeDefinition(obj) => {
                let fields = obj.fields()?;
                if is_field_definition_list_sorted(&fields.fields()) {
                    None
                } else {
                    Some(UseSortedTypeFieldsState::TypeFields(fields))
                }
            }
            AnyUseSortedTypeFieldsQuery::GraphqlObjectTypeExtension(obj) => {
                let fields = obj.fields()?;
                if is_field_definition_list_sorted(&fields.fields()) {
                    None
                } else {
                    Some(UseSortedTypeFieldsState::TypeFields(fields))
                }
            }
            AnyUseSortedTypeFieldsQuery::GraphqlInterfaceTypeDefinition(interface) => {
                let fields = interface.fields()?;
                if is_field_definition_list_sorted(&fields.fields()) {
                    None
                } else {
                    Some(UseSortedTypeFieldsState::TypeFields(fields))
                }
            }
            AnyUseSortedTypeFieldsQuery::GraphqlInterfaceTypeExtension(interface) => {
                let fields = interface.fields()?;
                if is_field_definition_list_sorted(&fields.fields()) {
                    None
                } else {
                    Some(UseSortedTypeFieldsState::TypeFields(fields))
                }
            }
            AnyUseSortedTypeFieldsQuery::GraphqlInputObjectTypeDefinition(input) => {
                let fields = input.input_fields()?;
                if is_input_field_list_sorted(&fields.fields()) {
                    None
                } else {
                    Some(UseSortedTypeFieldsState::InputFields(fields))
                }
            }
            AnyUseSortedTypeFieldsQuery::GraphqlInputObjectTypeExtension(input) => {
                let fields = input.input_fields()?;
                if is_input_field_list_sorted(&fields.fields()) {
                    None
                } else {
                    Some(UseSortedTypeFieldsState::InputFields(fields))
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            UseSortedTypeFieldsState::TypeFields(fields) => fields.syntax().text_trimmed_range(),
            UseSortedTypeFieldsState::InputFields(fields) => fields.syntax().text_trimmed_range(),
        };
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedTypeFields"),
            range,
            markup! {
                "These fields are not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<GraphqlRuleAction> {
        let mut mutation = ctx.root().begin();
        match state {
            UseSortedTypeFieldsState::TypeFields(fields_def) => {
                let sorted = make_sorted_field_definition_list(&fields_def.fields())?;
                mutation.replace_node(fields_def.fields(), sorted);
            }
            UseSortedTypeFieldsState::InputFields(input_def) => {
                let sorted = make_sorted_input_field_list(&input_def.fields())?;
                mutation.replace_node(input_def.fields(), sorted);
            }
        }
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort these fields." },
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyUseSortedTypeFieldsQuery =
        GraphqlObjectTypeDefinition
        | GraphqlObjectTypeExtension
        | GraphqlInterfaceTypeDefinition
        | GraphqlInterfaceTypeExtension
        | GraphqlInputObjectTypeDefinition
        | GraphqlInputObjectTypeExtension
}

pub enum UseSortedTypeFieldsState {
    TypeFields(GraphqlFieldsDefinition),
    InputFields(GraphqlInputFieldsDefinition),
}

fn get_field_definition_key(node: &GraphqlFieldDefinition) -> Option<TokenText> {
    node.name()
        .ok()?
        .value_token()
        .ok()
        .map(|t| t.token_text_trimmed())
}

fn get_input_value_definition_key(node: &GraphqlInputValueDefinition) -> Option<TokenText> {
    node.name()
        .ok()?
        .value_token()
        .ok()
        .map(|t| t.token_text_trimmed())
}

fn is_field_definition_list_sorted(list: &GraphqlFieldDefinitionList) -> bool {
    let mut prev: Option<TokenText> = None;
    for item in list {
        if let Some(key) = get_field_definition_key(&item) {
            if prev
                .as_ref()
                .is_some_and(|p| locale_compare(p, &key) == Ordering::Greater)
            {
                return false;
            }
            prev = Some(key);
        }
    }
    true
}

fn is_input_field_list_sorted(list: &GraphqlInputFieldList) -> bool {
    let mut prev: Option<TokenText> = None;
    for item in list {
        if let Some(key) = get_input_value_definition_key(&item) {
            if prev
                .as_ref()
                .is_some_and(|p| locale_compare(p, &key) == Ordering::Greater)
            {
                return false;
            }
            prev = Some(key);
        }
    }
    true
}

fn make_sorted_field_definition_list(
    list: &GraphqlFieldDefinitionList,
) -> Option<GraphqlFieldDefinitionList> {
    let mut items: Vec<(
        Option<TokenText>,
        biome_graphql_syntax::GraphqlFieldDefinition,
    )> = list
        .iter()
        .map(|n| (get_field_definition_key(&n), n))
        .collect();
    items.sort_by(|(k1, _), (k2, _)| compare_keys(k1, k2));
    GraphqlFieldDefinitionList::cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        items
            .into_iter()
            .map(|(_, n)| Some(NodeOrToken::Node(n.into_syntax()))),
    ))
}

fn make_sorted_input_field_list(list: &GraphqlInputFieldList) -> Option<GraphqlInputFieldList> {
    let mut items: Vec<(
        Option<TokenText>,
        biome_graphql_syntax::GraphqlInputValueDefinition,
    )> = list
        .iter()
        .map(|n| (get_input_value_definition_key(&n), n))
        .collect();
    items.sort_by(|(k1, _), (k2, _)| compare_keys(k1, k2));
    GraphqlInputFieldList::cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        items
            .into_iter()
            .map(|(_, n)| Some(NodeOrToken::Node(n.into_syntax()))),
    ))
}

fn compare_keys(k1: &Option<TokenText>, k2: &Option<TokenText>) -> Ordering {
    match (k1, k2) {
        (Some(a), Some(b)) => locale_compare(a, b),
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

// Matches JavaScript's localeCompare() for GraphQL identifiers [_A-Za-z0-9]:
// primary: _ < digits < letters (case-insensitive); tiebreaker: lowercase before uppercase.
fn locale_compare(a: &str, b: &str) -> Ordering {
    let primary_cmp = compare_primary(a, b);
    if primary_cmp != Ordering::Equal {
        return primary_cmp;
    }

    compare_tertiary(a, b)
}

fn compare_primary(a: &str, b: &str) -> Ordering {
    for (a_byte, b_byte) in a.bytes().zip(b.bytes()) {
        let a_key = primary_char_key(a_byte);
        let b_key = primary_char_key(b_byte);
        if a_key != b_key {
            return a_key.cmp(&b_key);
        }
    }

    a.len().cmp(&b.len())
}

fn primary_char_key(byte: u8) -> (u8, u8) {
    if byte == b'_' {
        return (0, 0);
    }
    if byte.is_ascii_digit() {
        return (1, byte);
    }
    if byte.is_ascii_alphabetic() {
        return (2, byte.to_ascii_lowercase());
    }

    (3, byte)
}

fn compare_tertiary(a: &str, b: &str) -> Ordering {
    for (a_byte, b_byte) in a.bytes().zip(b.bytes()) {
        if a_byte == b_byte {
            continue;
        }

        if a_byte.eq_ignore_ascii_case(&b_byte)
            && a_byte.is_ascii_alphabetic()
            && b_byte.is_ascii_alphabetic()
        {
            return match (a_byte.is_ascii_lowercase(), b_byte.is_ascii_lowercase()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal,
            };
        }

        return a_byte.cmp(&b_byte);
    }

    a.len().cmp(&b.len())
}

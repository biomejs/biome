use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlOperationDefinition, AnyGraphqlSelection, GraphqlSelectionSet,
};
use biome_rule_options::no_duplicate_fields::NoDuplicateFieldsOptions;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Disallow duplicate field names inside GraphQL selection sets.
    ///
    /// The rule checks the fields selected in each selection set. Each field name must appear only
    /// once in its selection set. Duplicate arguments and duplicate variable definitions are
    /// reported by `noDuplicateArgumentNames` and `noDuplicateVariableNames` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   users {
    ///     id
    ///     name
    ///     email
    ///     name
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   users {
    ///     id
    ///     name
    ///     email
    ///   }
    /// }
    /// ```
    ///
    pub NoDuplicateFields {
        version: "1.9.0",
        name: "noDuplicateFields",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("no-duplicate-fields").inspired()],
        recommended: true,
    }
}

impl Rule for NoDuplicateFields {
    type Query = Ast<AnyGraphqlOperationDefinition>;
    type State = DuplicatedField;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateFieldsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            // Variables are covered by `noDuplicateVariableNames`.
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(_) => Box::default(),
            // Selection fields of an operation definition are handled by the selection set traversal.
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(selection_set) => {
                check_duplicated_selection_fields(selection_set)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicatedField {
            kind,
            duplicate: DuplicateName {
                name,
                first,
                duplicates,
            },
        } = state;
        let name = name.text();
        let kind_name = kind.as_str();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The "{ kind_name }" `"<Emphasis>{ name }</Emphasis>"` is defined multiple times."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The "{ kind_name }" `"<Emphasis>{ name }</Emphasis>"` is repeated here."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "Defining the same name more than once is not allowed, even when the duplicates could be merged."
                })
                .note(markup! {
                    "Remove the duplicated "{ kind_name }"."
                }),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DuplicatedFieldKind {
    SelectionField,
}

impl DuplicatedFieldKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::SelectionField => "field",
        }
    }
}

#[derive(Debug)]
pub struct DuplicatedField {
    kind: DuplicatedFieldKind,
    duplicate: DuplicateName,
}

impl DuplicatedField {
    fn from_names(kind: DuplicatedFieldKind, names: Box<[DuplicateName]>) -> Vec<Self> {
        names
            .into_vec()
            .into_iter()
            .map(|duplicate| Self {
                kind,
                duplicate,
            })
            .collect()
    }
}

fn check_duplicated_selection_fields(selection_set: &GraphqlSelectionSet) -> Box<[DuplicatedField]> {
    let mut fields = vec![];
    for selection in selection_set.selections() {
        let AnyGraphqlSelection::GraphqlField(field) = selection else {
            continue;
        };

        // Alias is the final name of the field in the selection set.
        let Ok(name) = field.alias().map_or(field.name(), |alias| alias.value()) else {
            continue;
        };
        let Ok(token) = name.value_token() else {
            continue;
        };
        let name = token.token_text_trimmed();
        fields.push((name.clone(), name, token.text_trimmed_range()));
    }

    DuplicatedField::from_names(
        DuplicatedFieldKind::SelectionField,
        collect_duplicate_names(fields),
    )
    .into_boxed_slice()
}

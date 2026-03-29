use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_duplicate_graphql_operation_name::NoDuplicateGraphqlOperationNameOptions;
use camino::Utf8PathBuf;
use rustc_hash::FxHashMap;

use crate::services::ProjectGraphqlRoot;

declare_lint_rule! {
    /// Enforce unique operation names across a GraphQL document.
    ///
    /// This rule ensures that all GraphQL operations (queries, mutations, subscriptions) have unique names.
    /// Using unique operation names is essential for proper identification and reducing confusion.
    ///
    /// This rule always checks the current document.
    /// When the GraphQL project index is populated by the caller, it also checks
    /// for duplicate operation names defined in other files.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query user {
    ///   user {
    ///     id
    ///   }
    /// }
    ///
    /// query user {
    ///   me {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query user {
    ///   user {
    ///     id
    ///   }
    /// }
    ///
    /// query me {
    ///   me {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    pub NoDuplicateGraphqlOperationName {
        version: "2.3.6",
        name: "noDuplicateGraphqlOperationName",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-operation-name").inspired()],
    }
}

pub struct DuplicateOperationName {
    name: TokenText,
    text_range: TextRange,
    other_file: Option<Utf8PathBuf>,
}

impl Rule for NoDuplicateGraphqlOperationName {
    type Query = ProjectGraphqlRoot;
    type State = DuplicateOperationName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateGraphqlOperationNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let current_file = ctx.file_path();
        let project_index = ctx.project_index();
        let mut operation_names: FxHashMap<TokenText, TextRange> = FxHashMap::default();
        let mut duplicates = vec![];

        for definition in root.definitions() {
            if let Some(operation) = definition.as_graphql_operation_definition()
                && let Some(name_token) = operation.name()
                && let Ok(token) = name_token.value_token()
            {
                let name = token.token_text_trimmed();
                let text_range = operation.range();

                if operation_names.insert(name.clone(), text_range).is_some() {
                    duplicates.push(DuplicateOperationName {
                        name,
                        text_range,
                        other_file: None,
                    });
                    continue;
                }

                if let Some(other_file) = project_index
                    .operation_files(name.text())
                    .iter()
                    .find(|path| path.as_path() != current_file)
                {
                    duplicates.push(DuplicateOperationName {
                        name,
                        text_range,
                        other_file: Some(other_file.clone()),
                    });
                }
            }
        }

        duplicates.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateOperationName {
            name,
            text_range,
            other_file,
        } = state;

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            *text_range,
            markup! {
                "Operation named \""{ name.text() }"\" is already defined."
            },
        )
        .note(markup! {
            "GraphQL operation names must be unique to ensure proper identification."
        });

        let diagnostic = if let Some(other_file) = other_file {
            diagnostic.note(markup! {
                "Another operation with this name is indexed at "{other_file.as_str()}"."
            })
        } else {
            diagnostic
        };

        Some(diagnostic.note(markup! {
            "Rename the operation to have a unique name."
        }))
    }
}

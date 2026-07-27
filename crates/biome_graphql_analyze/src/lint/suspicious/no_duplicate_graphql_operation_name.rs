use crate::services::module_graph::GraphqlModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_graphql_syntax::GraphqlRoot;
use biome_module_graph::ModuleInfoKind;
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_duplicate_graphql_operation_name::NoDuplicateGraphqlOperationNameOptions;
use rustc_hash::{FxHashMap, FxHashSet};

declare_lint_rule! {
    /// Enforce unique operation names across a GraphQL document.
    ///
    /// This rule ensures that all GraphQL operations (queries, mutations, subscriptions) have unique names.
    /// Using unique operation names is essential for proper identification and reducing confusion.
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
        severity: Severity::Warning,
        sources: &[RuleSource::EslintGraphql("unique-operation-name").inspired()],
    }
}

pub struct DuplicateOperationName {
    name: TokenText,
    text_range: TextRange,
}

impl Rule for NoDuplicateGraphqlOperationName {
    type Query = GraphqlModuleGraph<GraphqlRoot>;
    type State = DuplicateOperationName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateGraphqlOperationNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let mut operation_names: FxHashMap<TokenText, TextRange> = FxHashMap::default();
        let mut current_file_names = FxHashMap::default();
        let mut duplicates = vec![];
        let mut seen = FxHashSet::default();

        for definition in root.definitions() {
            if let Some(operation) = definition.as_graphql_operation_definition()
                && let Some(name_token) = operation.name()
                && let Ok(token) = name_token.value_token()
            {
                let name = token.token_text_trimmed();
                let text_range = operation.range();
                current_file_names.insert(name.clone(), text_range);

                if let Some(_existing_range) = operation_names.insert(name.clone(), text_range) {
                    let key = (name.clone(), text_range);
                    if seen.insert(key) {
                        duplicates.push(DuplicateOperationName { name, text_range });
                    }
                }
            }
        }

        if let Some(db) = ctx.db() {
            let mut external_names = FxHashSet::default();
            db.for_each_module(&mut |path, kind| {
                if path == ctx.file_path() {
                    return;
                }

                let ModuleInfoKind::Graphql(graphql_module) = kind else {
                    return;
                };

                external_names.extend(graphql_module.operation_names.iter().cloned());
            });

            for (name, text_range) in current_file_names {
                if external_names.contains(name.as_ref()) {
                    let key = (name.clone(), text_range);
                    if seen.insert(key) {
                        duplicates.push(DuplicateOperationName { name, text_range });
                    }
                }
            }
        }

        duplicates.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateOperationName { name, text_range } = state;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                text_range,
                markup! {
                    "Operation named \""{ name.text() }"\" is already defined."
                },
            )
            .note(markup! {
                "GraphQL operation names must be unique to ensure proper identification."
            })
            .note(markup! {
                "Rename the operation to have a unique name."
            }),
        )
    }
}

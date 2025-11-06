use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlRoot;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::unique_operation_name::UniqueOperationNameOptions;
use rustc_hash::FxHashMap;

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
    pub UniqueOperationName {
        version: "next",
        name: "uniqueOperationName",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-operation-name").inspired()],
    }
}

pub struct DuplicateOperationName {
    name: Box<str>,
    text_range: TextRange,
}

impl Rule for UniqueOperationName {
    type Query = Ast<GraphqlRoot>;
    type State = DuplicateOperationName;
    type Signals = Box<[Self::State]>;
    type Options = UniqueOperationNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let mut operation_names: FxHashMap<Box<str>, TextRange> = FxHashMap::default();
        let mut duplicates = vec![];

        for definition in root.definitions() {
            if let Some(operation) = definition.as_graphql_operation_definition()
                && let Some(name_token) = operation.name()
            {
                let name = name_token.to_trimmed_string().into_boxed_str();
                let text_range = operation.range();

                if let Some(_existing_range) = operation_names.insert(name.clone(), text_range) {
                    duplicates.push(DuplicateOperationName { name, text_range });
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
                    "Operation named \""{ name }"\" is already defined."
                },
            )
            .note(markup! {
                "GraphQL operation names must be unique to ensure proper identification."
            }),
        )
    }
}

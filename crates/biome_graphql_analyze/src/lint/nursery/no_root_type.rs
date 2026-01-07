use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    GraphqlLanguage, GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
};
use biome_rowan::{SyntaxToken, TextRange, TokenText, declare_node_union};
use biome_rule_options::no_root_type::NoRootTypeOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Disallow the usage of specified root types
    ///
    /// Prevent the usage of certain root types (e.g. `mutation` and/or `subscription`)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "disallow": ["mutation"]
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic,use_options
    /// type Mutation {
    ///   createUser(input: CreateUserInput!): User!
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql,use_options
    /// type Query {
    ///   users: [User!]!
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `disallow`
    ///
    /// This required option lists all disallowed root types (e.g. `mutation` and/or `subscription`).
    /// These list values will be handled case-insensitive.
    ///
    /// Default `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "disallow": ["subscription"]
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic,use_options
    /// type Subscription {
    ///   user: User
    /// }
    /// ```
    ///
    pub NoRootType {
        version: "next",
        name: "noRootType",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("no-root-type").same()],
    }
}

impl Rule for NoRootType {
    type Query = Ast<NoRootTypeQuery>;
    type State = (TokenText, TextRange);
    type Signals = Option<Self::State>;
    type Options = NoRootTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let root_types = &ctx.options().disallow;

        if root_types.is_empty() {
            return None;
        }

        match node {
            NoRootTypeQuery::GraphqlObjectTypeDefinition(type_def) => {
                let name = type_def.name().ok()?;
                let value_token = name.value_token().ok()?;
                check_name(root_types, value_token)
            }
            NoRootTypeQuery::GraphqlObjectTypeExtension(type_ext) => {
                let name = type_ext.name().ok()?;
                let value_token = name.value_token().ok()?;
                check_name(root_types, value_token)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (name, range): &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The root type "{{name.to_string()}}" is forbidden."
                },
            )
            .note(markup! {
                "It's forbidden to use this root type within this project. Rework to use a different root type."
            }),
        )
    }
}

declare_node_union! {
    pub NoRootTypeQuery = GraphqlObjectTypeDefinition | GraphqlObjectTypeExtension
}

fn check_name(
    root_types: &Vec<String>,
    name: SyntaxToken<GraphqlLanguage>,
) -> Option<(TokenText, TextRange)> {
    let trimmed = name.token_text_trimmed();

    for root_type in root_types {
        if root_type.to_lowercase_cow() == trimmed.to_lowercase_cow() {
            return Some((trimmed, name.text_range()));
        }
    }

    None
}

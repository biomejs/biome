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
    /// The values of the list are case-insensitive.
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
        version: "2.3.12",
        name: "noRootType",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("no-root-type").same()],
    }
}

impl Rule for NoRootType {
    type Query = Ast<AnyNoRootTypeQuery>;
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
            AnyNoRootTypeQuery::GraphqlObjectTypeDefinition(type_def) => {
                let name = type_def.name().ok()?;
                let value_token = name.value_token().ok()?;
                check_name(root_types, value_token)
            }
            AnyNoRootTypeQuery::GraphqlObjectTypeExtension(type_ext) => {
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
                    "This schema defines the disallowed root type "<Emphasis>{name.text()}</Emphasis>"."
                },
            )
            .note(markup! {
                "This project forbids that root type to enforce a specific schema design."
            })
            .note(markup! {
                "Use a different root type, or update the rule configuration if this root type should be allowed."
            }),
        )
    }
}

declare_node_union! {
    pub AnyNoRootTypeQuery = GraphqlObjectTypeDefinition | GraphqlObjectTypeExtension
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

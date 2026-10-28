use crate::GraphqlRuleAction;
use crate::utils::{is_list_sorted, sort_list};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_graphql_factory::make;
use biome_graphql_syntax::{
    AnyGraphqlDefinition, AnyGraphqlTypeDefinition, AnyGraphqlTypeExtension, GraphqlRoot,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};
use biome_rule_options::use_graphql_sorted_definitions::UseGraphqlSortedDefinitionsOptions;

declare_source_rule! {
    /// Sort the named definitions in a GraphQL document.
    ///
    /// This action natural sorts the named definitions of a document by name: the `scalar`, `type`,
    /// `interface`, `union`, `enum`, and `input` definitions, their `extend` counterparts,
    /// the `directive` definitions, the `fragment` definitions, and the named `query`,
    /// `mutation`, and `subscription` operations.
    /// Keeping them sorted makes growing documents easier to scan and review.
    /// The order of definitions doesn't affect how a document is executed or how a schema is built.
    ///
    /// Definitions are sorted by name in a [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order),
    /// meaning that uppercase letters come before lowercase letters (e.g. `A` < `a` < `B` < `b`)
    /// and numbers are compared to their numerical value (e.g. `9` < `10`).
    ///
    /// Definitions without a name, such as `schema` definitions, anonymous operations, and
    /// shorthand queries, are placed after the named definitions.
    /// Definitions with the same name aren't sorted.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diff
    /// type User {
    ///   id: ID!
    /// }
    ///
    /// enum Role {
    ///   ADMIN
    ///   USER
    /// }
    ///
    /// directive @auth on FIELD_DEFINITION
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// directive @auth on FIELD_DEFINITION
    ///
    /// enum Role {
    ///   ADMIN
    ///   USER
    /// }
    ///
    /// type User {
    ///   id: ID!
    /// }
    /// ```
    ///
    /// ## Caveats
    ///
    /// Comments above a definition and comments at the end of its last line move with that definition.
    /// Because moving a comment can change which definition it describes or hide code, the action
    /// offers no fix in these cases:
    ///
    /// - A moved end-of-line comment would share a line with the next definition. Since a `#`
    ///   comment continues to the end of its line, it would hide that definition. For example,
    ///   sorting this input would move `Apple` before `Zebra`, but the comment would hide `Zebra`:
    ///
    ///   ```graphql,ignore
    ///   type Zebra { id: ID! } type Apple { id: ID! } # Keep Apple
    ///   ```
    ///
    /// - A definition with a comment above it is, or would become, the first definition in the document.
    ///
    /// Sort such definitions manually, or put each definition on its own line.
    ///
    pub UseGraphqlSortedDefinitions {
        version: "2.6.0",
        name: "useGraphqlSortedDefinitions",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("alphabetize").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseGraphqlSortedDefinitions {
    type Query = Ast<GraphqlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseGraphqlSortedDefinitionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if is_list_sorted(&node.definitions(), definition_name) {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().definitions().range(),
            markup! {
                "The definitions of this document are not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<GraphqlRuleAction> {
        let node = ctx.query();
        let list = node.definitions();
        let sorted = sort_list(&list, &node.eof_token().ok()?, definition_name)?;

        let mut mutation = ctx.root().begin();
        // `replace_node` would copy the trivia of the old list boundaries onto the new list,
        // overwriting the trivia computed by `sort_list`.
        mutation.replace_node_discard_trivia(list, make::graphql_definition_list(sorted));

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort the definitions." },
            mutation,
        ))
    }
}

/// Returns the name of a definition, or `None` for definitions that aren't sorted.
fn definition_name(node: &AnyGraphqlDefinition) -> Option<TokenText> {
    let token = match node {
        AnyGraphqlDefinition::AnyGraphqlTypeDefinition(definition) => match definition {
            AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(node) => {
                node.name().ok()?.value_token()
            }
        },
        AnyGraphqlDefinition::AnyGraphqlTypeExtension(extension) => match extension {
            AnyGraphqlTypeExtension::GraphqlEnumTypeExtension(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeExtension::GraphqlInputObjectTypeExtension(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeExtension::GraphqlInterfaceTypeExtension(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeExtension::GraphqlObjectTypeExtension(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeExtension::GraphqlScalarTypeExtension(node) => {
                node.name().ok()?.value_token()
            }
            AnyGraphqlTypeExtension::GraphqlUnionTypeExtension(node) => {
                node.name().ok()?.value_token()
            }
        },
        AnyGraphqlDefinition::GraphqlDirectiveDefinition(node) => node.name().ok()?.value_token(),
        AnyGraphqlDefinition::GraphqlFragmentDefinition(node) => node.name().ok()?.value_token(),
        AnyGraphqlDefinition::GraphqlOperationDefinition(node) => node.name()?.value_token(),
        AnyGraphqlDefinition::GraphqlBogusDefinition(_)
        | AnyGraphqlDefinition::GraphqlSchemaDefinition(_)
        | AnyGraphqlDefinition::GraphqlSchemaExtension(_)
        | AnyGraphqlDefinition::GraphqlSelectionSet(_) => return None,
    };
    Some(token.ok()?.token_text_trimmed())
}

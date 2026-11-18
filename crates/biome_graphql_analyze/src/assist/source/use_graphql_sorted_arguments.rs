use crate::GraphqlRuleAction;
use crate::utils::{is_list_sorted, sort_list};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_graphql_factory::make;
use biome_graphql_syntax::{
    GraphqlArgument, GraphqlArguments, GraphqlArgumentsDefinition, GraphqlDirective,
    GraphqlDirectiveDefinition, GraphqlField, GraphqlFieldDefinition, GraphqlInputValueDefinition,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText, declare_node_union};
use biome_rule_options::use_graphql_sorted_arguments::UseGraphqlSortedArgumentsOptions;

declare_source_rule! {
    /// Sort the arguments of GraphQL fields and directives in natural order.
    ///
    /// This action sorts both the arguments passed to fields and directives,
    /// and the arguments declared by field and directive definitions.
    /// Keeping them sorted makes long argument lists easier to scan and review.
    /// The order of arguments doesn't affect how a field or a directive is executed.
    ///
    /// Arguments are sorted by name in a [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order),
    /// meaning that uppercase letters come before lowercase letters (e.g. `A` < `a` < `B` < `b`)
    /// and numbers are compared to their numerical value (e.g. `9` < `10`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diff
    /// query {
    ///   users(orderBy: NAME, first: 10, after: "cursor") {
    ///     name
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diff
    /// type Query {
    ///   users(orderBy: Order, first: Int, after: String): [User]
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   users(after: "cursor", first: 10, orderBy: NAME) {
    ///     name
    ///   }
    /// }
    ///
    /// type Query {
    ///   users(after: String, first: Int, orderBy: Order): [User]
    /// }
    /// ```
    ///
    /// ## Caveats
    ///
    /// Comments above an argument and comments at the end of its line move with that argument.
    /// No fix is offered when a moved comment would end up on the same line as other code,
    /// because the comment would then hide that code or describe a different argument.
    /// Sort such arguments manually, or put each argument on its own line.
    ///
    pub UseGraphqlSortedArguments {
        version: "2.6.0",
        name: "useGraphqlSortedArguments",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("alphabetize").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseGraphqlSortedArguments {
    type Query = Ast<AnyGraphqlArguments>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseGraphqlSortedArgumentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let is_sorted = match ctx.query() {
            AnyGraphqlArguments::GraphqlArguments(node) => {
                is_list_sorted(&node.arguments(), argument_name)
            }
            AnyGraphqlArguments::GraphqlArgumentsDefinition(node) => {
                is_list_sorted(&node.arguments(), argument_definition_name)
            }
        };
        if is_sorted { None } else { Some(()) }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (kind, prefix, name) = match node.parent::<AnyArgumentsOwner>()? {
            AnyArgumentsOwner::GraphqlField(field) => ("field", "", field.name().ok()?.value_token()),
            AnyArgumentsOwner::GraphqlFieldDefinition(field) => {
                ("field", "", field.name().ok()?.value_token())
            }
            AnyArgumentsOwner::GraphqlDirective(directive) => {
                ("directive", "@", directive.name().ok()?.value_token())
            }
            AnyArgumentsOwner::GraphqlDirectiveDefinition(directive) => {
                ("directive", "@", directive.name().ok()?.value_token())
            }
        };
        let name = name.ok()?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The arguments of the "{kind}" "<Emphasis>{prefix}{name.text_trimmed()}</Emphasis>" are not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<GraphqlRuleAction> {
        let mut mutation = ctx.root().begin();
        // `replace_node` would copy the trivia of the old list boundaries onto the new list,
        // overwriting the trivia computed by `sort_list`.
        match ctx.query() {
            AnyGraphqlArguments::GraphqlArguments(node) => {
                let list = node.arguments();
                let sorted = sort_list(&list, &node.r_paren_token().ok()?, argument_name)?;
                let sorted = make::graphql_argument_list(sorted);
                mutation.replace_node_discard_trivia(list, sorted);
            }
            AnyGraphqlArguments::GraphqlArgumentsDefinition(node) => {
                let list = node.arguments();
                let sorted =
                    sort_list(&list, &node.r_paren_token().ok()?, argument_definition_name)?;
                let sorted = make::graphql_argument_definition_list(sorted);
                mutation.replace_node_discard_trivia(list, sorted);
            }
        }

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort the arguments." },
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyGraphqlArguments = GraphqlArguments | GraphqlArgumentsDefinition
}

declare_node_union! {
    pub AnyArgumentsOwner =
        GraphqlField
        | GraphqlFieldDefinition
        | GraphqlDirective
        | GraphqlDirectiveDefinition
}

fn argument_name(node: &GraphqlArgument) -> Option<TokenText> {
    Some(node.name().ok()?.value_token().ok()?.token_text_trimmed())
}

fn argument_definition_name(node: &GraphqlInputValueDefinition) -> Option<TokenText> {
    Some(node.name().ok()?.value_token().ok()?.token_text_trimmed())
}

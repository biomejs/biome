use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_graphql_factory::make;
use biome_graphql_syntax::{GraphqlOperationDefinition, GraphqlOperationType};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_string_case::Case;

use crate::GraphqlRuleAction;

declare_lint_rule! {
    /// Enforce specifying the name of GraphQL operations.
    ///
    /// This is useful because most GraphQL client libraries use the operation name for caching purposes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query Human {
    ///   name
    /// }
    /// ```
    ///
    pub UseNamedOperation {
        version: "next",
        name: "useNamedOperation",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("no-anonymous-operations")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseNamedOperation {
    type Query = Ast<GraphqlOperationDefinition>;
    type State = GraphqlOperationType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let operation_type = node.ty().ok()?;
        if node.name().is_some() {
            None
        } else {
            Some(operation_type)
        }
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        operation_type: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                operation_type.range(),
                markup! {
                    "Anonymous GraphQL operations are forbidden. Make sure to name your " {operation_type.to_trimmed_string()}"."
                },
            )
            .note(markup! {
                "Most GraphQL client libraries use the operation name for caching purposes."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, operation_type: &Self::State) -> Option<GraphqlRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query().clone();
        let operation_type = operation_type.to_trimmed_string();
        let suggested_name = get_suggested_name(&node, operation_type.clone());
        let new_name = make::graphql_name_binding(make::ident(&suggested_name));
        let new_node = node.clone().with_name(Some(new_name));
        mutation.replace_node(node, new_node);

        Some(GraphqlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Rename this "{operation_type}" to "{suggested_name}"."
            },
            mutation,
        ))
    }
}

fn get_suggested_name(operation: &GraphqlOperationDefinition, operation_type: String) -> String {
    let suggested_name = operation
        .selection_set()
        .ok()
        .and_then(|selection_set| {
            selection_set
                .selections()
                .into_iter()
                .find_map(|selection| selection.as_graphql_field().cloned())
        })
        .and_then(|first_field| {
            first_field
                .alias()
                .map(|alias| alias.to_trimmed_string())
                .or(first_field.name().ok().map(|name| name.to_trimmed_string()))
        })
        .unwrap_or(operation_type);
    Case::Pascal.convert(&suggested_name)
}

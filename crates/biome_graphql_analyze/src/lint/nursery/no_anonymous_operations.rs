use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_graphql_factory::make;
use biome_graphql_syntax::GraphqlOperationDefinition;
use biome_rowan::{AstNode, AstNodeExt, BatchMutationExt};
use biome_string_case::Case;

use crate::GraphqlRuleAction;

declare_lint_rule! {
    /// Require specifying name for GraphQL operations.
    ///
    /// This is useful since most GraphQL client libraries are using the operation name for caching purposes.
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
    pub NoAnonymousOperations {
        version: "next",
        name: "noAnonymousOperations",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("no-anonymous-operations")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAnonymousOperations {
    type Query = Ast<GraphqlOperationDefinition>;
    type State = NoAnonymousOperationsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let operation_type = node.ty().ok()?.text();
        if node.name().is_some() {
            None
        } else {
            Some(NoAnonymousOperationsState {
                operation_type: operation_type.clone(),
                suggested_name: get_suggested_name(node, operation_type),
            })
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Anonymous GraphQL operations are forbidden. Make sure to name your " {state.operation_type}"!"
                },
            )
            .note(markup! {
                "Most GraphQL client libraries use operation name for caching purposes."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<GraphqlRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query().clone();
        let new_name = make::graphql_name_binding(make::ident(&state.suggested_name));
        let new_node = node.clone().detach().with_name(Some(new_name));
        mutation.replace_node(node, new_node);

        Some(GraphqlRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! {
                "Rename this "{state.operation_type}" to "{state.suggested_name}"."
            },
            mutation,
        ))
    }
}

fn get_suggested_name(operation: &GraphqlOperationDefinition, operation_type: String) -> String {
    let suggested_name = get_suggested_name_base_on_content(operation).unwrap_or(operation_type);
    Case::Pascal.convert(&suggested_name)
}

fn get_suggested_name_base_on_content(operation: &GraphqlOperationDefinition) -> Option<String> {
    let selection_set = operation.selection_set().ok()?;
    let first_field = selection_set
        .selections()
        .into_iter()
        .find_map(|selection| selection.as_graphql_field().cloned())?;

    first_field
        .alias()
        .map(|alias| alias.text())
        .or(first_field.name().ok().map(|name| name.text()))
}

pub struct NoAnonymousOperationsState {
    operation_type: String,
    suggested_name: String,
}

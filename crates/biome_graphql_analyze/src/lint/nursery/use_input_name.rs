use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    GraphqlFieldDefinition, GraphqlFieldDefinitionList, GraphqlFieldsDefinition,
    GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::use_input_name::UseInputNameOptions;

declare_lint_rule! {
    /// Require mutation argument to be always called “input” and input type to be called Mutation name + “Input”.
    ///
    /// Require mutation argument to be always called “input” and input type to be called Mutation name + “Input”.
    /// Using the same name for all input parameters will make your schemas easier to consume and more predictable.
    /// Using the same name as mutation for InputType will make it easier to find mutations that InputType belongs to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// type Mutation {
    ///   SetMessage(message: InputMessage): String
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// type Mutation {
    ///   SetMessage(input: SetMessageInput): String
    /// }
    /// ```
    ///
    pub UseInputName {
        version: "next",
        name: "useInputName",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("input-name").inspired()],
    }
}

impl Rule for UseInputName {
    type Query = Ast<GraphqlFieldDefinition>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseInputNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let def_list = node
            .syntax()
            .parent()
            .and_then(GraphqlFieldDefinitionList::cast)?;
        let fields_def = def_list
            .syntax()
            .parent()
            .and_then(GraphqlFieldsDefinition::cast)?;
        let is_query = fields_def.syntax().parent().is_some(|parent| {})?;

        if !is_query {
            return None;
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected empty block is not allowed"
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

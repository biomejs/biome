use biome_diagnostics::Severity;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlVariableDefinitions;
use biome_rowan::AstNode;
use biome_rule_options::no_duplicate_variable_names::NoDuplicateVariableNamesOptions;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Require all variable definitions to be unique.
    ///
    /// A GraphQL operation is only valid if all its variables are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query ($x: Int, $x: Int) {
    ///   field
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query ($x: Int, $y: Int) {
    ///   field
    /// }
    /// ```
    ///
    pub NoDuplicateVariableNames {
        version: "2.3.11",
        name: "noDuplicateVariableNames",
        language: "graphql",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintGraphql("unique-variable-names").same()],
    }
}

impl Rule for NoDuplicateVariableNames {
    type Query = Ast<GraphqlVariableDefinitions>;
    type State = DuplicateName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateVariableNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        collect_duplicate_names(ctx.query().elements().into_iter().filter_map(|element| {
            let variable = element.variable().ok()?;
            let name = variable.name().ok()?.value_token().ok()?.token_text_trimmed();
            Some((name.clone(), name, variable.range()))
        }))
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateName {
            name,
            first,
            duplicates,
        } = state;
        let name = name.text();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The variable name \""<Emphasis>"$"{name}</Emphasis>"\" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The variable \""<Emphasis>"$"{name}</Emphasis>"\" is repeated here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "A GraphQL operation is only valid if all its variables are uniquely named. Make sure to name every variable differently."
        }))
    }
}

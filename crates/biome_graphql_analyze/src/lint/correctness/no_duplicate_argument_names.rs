use biome_diagnostics::Severity;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlArguments;
use biome_rule_options::no_duplicate_argument_names::NoDuplicateArgumentNamesOptions;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Require unique argument names for fields and directives.
    ///
    /// A GraphQL field or directive is only valid if all supplied arguments are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   field(arg1: "value", arg1: "value")
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   field(arg1: "value", arg2: "value")
    /// }
    /// ```
    ///
    pub NoDuplicateArgumentNames {
        version: "2.3.11",
        name: "noDuplicateArgumentNames",
        language: "graphql",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintGraphql("unique-argument-names").same()],
    }
}

impl Rule for NoDuplicateArgumentNames {
    type Query = Ast<GraphqlArguments>;
    type State = DuplicateName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateArgumentNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        collect_duplicate_names(ctx.query().arguments().into_iter().filter_map(|argument| {
            let token = argument.name().ok()?.value_token().ok()?;
            let name = token.token_text_trimmed();
            Some((name.clone(), name, token.text_trimmed_range()))
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
                "The argument name \""<Emphasis>{name}</Emphasis>"\" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The argument \""<Emphasis>{name}</Emphasis>"\" is repeated here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "A GraphQL field or directive is only valid if all supplied arguments are uniquely named. Make sure to name every argument differently."
        }))
    }
}

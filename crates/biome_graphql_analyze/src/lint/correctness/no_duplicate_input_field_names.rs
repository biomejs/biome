use biome_diagnostics::Severity;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlObjectValue;
use biome_rule_options::no_duplicate_input_field_names::NoDuplicateInputFieldNamesOptions;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Require fields within an input object to be unique.
    ///
    /// A GraphQL input object value is only valid if all supplied fields are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   field(arg: { f1: "value", f1: "value" })
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   field(arg: { f1: "value", f2: "value" })
    /// }
    /// ```
    ///
    pub NoDuplicateInputFieldNames {
        version: "2.3.11",
        name: "noDuplicateInputFieldNames",
        language: "graphql",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintGraphql("unique-input-field-names").same()],
    }
}

impl Rule for NoDuplicateInputFieldNames {
    type Query = Ast<GraphqlObjectValue>;
    type State = DuplicateName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateInputFieldNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        collect_duplicate_names(ctx.query().members().into_iter().filter_map(|member| {
            let token = member.name().ok()?.value_token().ok()?;
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
                "The input field name \""<Emphasis>{name}</Emphasis>"\" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The input field \""<Emphasis>{name}</Emphasis>"\" is repeated here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "A GraphQL input object value is only valid if all supplied fields are uniquely named. Make sure to name every input field differently."
        }))
    }
}

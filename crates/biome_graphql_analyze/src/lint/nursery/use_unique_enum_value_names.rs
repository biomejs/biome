use std::{borrow::Cow, collections::HashSet};

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlEnumTypeDefinition;
use biome_rowan::AstNode;
use biome_rule_options::use_unique_enum_value_names::UseUniqueEnumValueNamesOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Require all enum value names to be unique.
    ///
    /// A GraphQL enum type is only valid if all its values are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum A { TEST TesT }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// enum A { TEST }
    /// ```
    ///
    pub UseUniqueEnumValueNames {
        version: "next",
        name: "useUniqueEnumValueNames",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-enum-value-names").same()],
    }
}

impl Rule for UseUniqueEnumValueNames {
    type Query = Ast<GraphqlEnumTypeDefinition>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseUniqueEnumValueNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut found: HashSet<Cow<'_, str>> = HashSet::new();

        let enum_values = node.enum_values()?;
        for element in enum_values.values() {
            if let Some(name) = element.value().ok()
                && let Some(value_token) = name.value_token().ok()
            {
                let value_token = value_token.token_text();
                let lowercase_text = value_token.to_lowercase_cow();
                if found.contains(&lowercase_text) {
                    return Some(());
                } else {
                    found.insert(lowercase_text.into_owned().into());
                }
            }
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
                   "Duplicate enum value name."
                },
            )
            .note(markup! {
                "A GraphQL enum type is only valid if all its values are uniquely named. Make sure to name every enum value differently."
            }),
        )
    }
}

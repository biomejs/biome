use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlEnumTypeDefinition;
use biome_rowan::{AstNodeList, TextRange};
use biome_rule_options::use_unique_enum_value_names::UseUniqueEnumValueNamesOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Require all enum value names to be unique.
    ///
    /// A GraphQL enum type is only valid if all its values are uniquely named.
    /// The enum value names are case insensitive, meaning `TEST` & `Test` are seen as the same enum value name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum A {
    ///   TEST
    ///   OTHER
    ///   TEST
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// enum B {
    ///   TEST
    ///   TesT
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// enum A {
    ///   TEST
    ///   OTHER
    /// }
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
    type State = Vec<TextRange>;
    type Signals = Option<Self::State>;
    type Options = UseUniqueEnumValueNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // We can't use TokenText (to minimize allocations) because of lowercasing and Cow can't be used in a HashSet
        let mut found: HashSet<String> = HashSet::new();

        let enum_values = node.enum_values()?;
        let duplicates: Vec<TextRange> = enum_values
            .values()
            .iter()
            .filter_map(|enum_value| {
                if let Some(name) = enum_value.value().ok()
                    && let Some(value_token) = name.value_token().ok()
                {
                    let string = value_token.token_text().to_lowercase_cow().to_string();
                    if found.insert(string) {
                        return None;
                    } else {
                        let range = value_token.text_range();
                        return Some(range);
                    }
                }

                None
            })
            .collect();

        if duplicates.is_empty() {
            None
        } else {
            Some(duplicates)
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.first()?,
            markup! {
               "Duplicate enum value name."
            },
        );

        for range in &state[1..] {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "Another duplicate enum value."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "A GraphQL enum type is only valid if all its values are uniquely named. Make sure to name every enum value differently."
        }))
    }
}

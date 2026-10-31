use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_graphql_syntax::GraphqlEnumTypeDefinition;
use biome_rowan::AstNodeList;
use biome_rule_options::no_duplicate_enum_value_names::NoDuplicateEnumValueNamesOptions;
use biome_string_case::StrOnlyExtension;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Disallow duplicate names in GraphQL enums.
    ///
    /// This rule compares enum values without regard to letter case, so `TEST` and `Test` are
    /// treated as the same name.
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
    pub NoDuplicateEnumValueNames {
        version: "2.3.12",
        name: "noDuplicateEnumValueNames",
        language: "graphql",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintGraphql("unique-enum-value-names").same()],
    }
}

impl Rule for NoDuplicateEnumValueNames {
    type Query = Ast<GraphqlEnumTypeDefinition>;
    type State = DuplicateName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateEnumValueNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(enum_values) = ctx.query().enum_values() else {
            return Box::default();
        };
        // The key is lowercased because enum values are compared case-insensitively.
        collect_duplicate_names(enum_values.values().iter().filter_map(|enum_value| {
            let token = enum_value.value().ok()?.value_token().ok()?;
            let name = token.token_text_trimmed();
            Some((
                name.to_lowercase_cow().into_owned(),
                name,
                token.text_trimmed_range(),
            ))
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
                "The enum value name \""<Emphasis>{name}</Emphasis>"\" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "This value matches \""<Emphasis>{name}</Emphasis>"\" ignoring letter case."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "A GraphQL enum type is only valid if all its values are uniquely named. Make sure to name every enum value differently."
        }))
    }
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlDirective;
use biome_rowan::AstNode;
use biome_rule_options::use_deprecated_date::UseDeprecatedDateOptions;
use jiff::{Timestamp, tz::TimeZone};

declare_lint_rule! {
    /// Require the `@deprecated` directive to specify a deletion date.
    ///
    /// Suggests removing deprecated code when the due date has been passed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   member @deprecated(reason: "Use `members` instead") {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   member @deprecated(reason: "Use `members` instead", deletionDate: "2099-12-25") {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    pub UseDeprecatedDate {
        version: "2.2.6",
        name: "useDeprecatedDate",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("require-deprecation-date").same()],
        recommended: false,
    }
}

pub enum DeprecatedDateIssue {
    Missing,
    Invalid,
    Due,
}

impl Rule for UseDeprecatedDate {
    type Query = Ast<GraphqlDirective>;
    type State = (DeprecatedDateIssue, GraphqlDirective);
    type Signals = Option<Self::State>;
    type Options = UseDeprecatedDateOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let name = node.name().ok()?;

        if name.to_trimmed_string() != "deprecated" {
            return None;
        }

        let Some(arguments) = node.arguments() else {
            return Some((DeprecatedDateIssue::Missing, node.clone()));
        };
        let arguments = arguments.arguments();
        let argument_name = ctx.options().argument_name.clone();
        let Some(argument) = arguments.into_iter().find(|argument| {
            argument.name().is_ok_and(|name| {
                name.value_token()
                    .is_ok_and(|token| token.text_trimmed() == argument_name)
            })
        }) else {
            return Some((DeprecatedDateIssue::Missing, node.clone()));
        };

        let argument_value = argument.value().ok()?;

        let Some(argument_string_value) = argument_value.as_graphql_string_value() else {
            return Some((DeprecatedDateIssue::Invalid, node.clone()));
        };

        let due_date_value = argument_string_value.inner_string_text().ok()?;
        let due_date = due_date_value.text().parse();

        if due_date.is_err() {
            return Some((DeprecatedDateIssue::Invalid, node.clone()));
        }

        let now = Timestamp::now().to_zoned(TimeZone::UTC).date();

        if now > due_date.ok()? {
            return Some((DeprecatedDateIssue::Due, node.clone()));
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        let argument_name = ctx.options().argument_name.clone();

        match state.0 {
            DeprecatedDateIssue::Missing => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    span,
                    markup! {
                        "The directive `@deprecated` should have a `"{argument_name}"` argument."
                    },
                )
                .note(markup! {
                        "Add a `"{argument_name}"` argument to the directive."
                }),
            ),
            DeprecatedDateIssue::Invalid => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    span,
                    markup! {
                        "The `"{argument_name}"` argument in the directive `@deprecated` has an invalid value."
                    },
                )
                .note(markup! {
                    "The argument must match the `YYYY-MM-DD` format."
                }),
            ),
            DeprecatedDateIssue::Due => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    span,
                    markup! {
                        "The deprecation has passed its due date."
                    },
                )
                .note(markup! {
                    "Remove deprecated code or move deprecation date to the future."
                }),
            ),
        }
    }
}

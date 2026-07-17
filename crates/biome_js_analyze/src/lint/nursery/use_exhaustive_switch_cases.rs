use std::io;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{
    fmt::{Display, Formatter},
    markup,
};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsStatement, AnyJsSwitchClause, JsSwitchStatement, T,
};
use biome_js_type_info::InferredSwitchCase;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::use_exhaustive_switch_cases::UseExhaustiveSwitchCasesOptions;

use crate::JsRuleAction;
use crate::services::typed::Typed;

declare_lint_rule! {
    /// Require switch-case statements to be exhaustive.
    ///
    /// When working with union types in TypeScript, it's common to want to write a switch statement
    /// intended to contain a case for each possible variant.
    /// However, if the union type changes, it's easy to forget to modify the cases to account for
    /// any new types.
    ///
    /// This rule reports when a switch statement over a value typed as a union of literals lacks
    /// a case for any of those literal types and does not have a default clause.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid.ts
    /// type Day =
    ///   | 'Monday'
    ///   | 'Tuesday'
    ///   | 'Wednesday'
    ///   | 'Thursday'
    ///   | 'Friday'
    ///   | 'Saturday'
    ///   | 'Sunday';
    ///
    /// declare const day: Day;
    /// let result = 0;
    ///
    /// switch (day) {
    ///   case 'Monday':
    ///     result = 1;
    ///     break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// type Day =
    ///   | 'Monday'
    ///   | 'Tuesday'
    ///   | 'Wednesday'
    ///   | 'Thursday'
    ///   | 'Friday'
    ///   | 'Saturday'
    ///   | 'Sunday';
    ///
    /// declare const day: Day;
    /// let result = 0;
    ///
    /// switch (day) {
    ///   case 'Monday':
    ///     result = 1;
    ///     break;
    ///   case 'Tuesday':
    ///     result = 2;
    ///     break;
    ///   case 'Wednesday':
    ///     result = 3;
    ///     break;
    ///   case 'Thursday':
    ///     result = 4;
    ///     break;
    ///   case 'Friday':
    ///     result = 5;
    ///     break;
    ///   case 'Saturday':
    ///     result = 6;
    ///     break;
    ///   case 'Sunday':
    ///     result = 7;
    ///     break;
    /// }
    /// ```
    ///
    pub UseExhaustiveSwitchCases {
        version: "2.0.0",
        name: "useExhaustiveSwitchCases",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintTypeScript("switch-exhaustiveness-check").same()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

pub struct MissingCase(InferredSwitchCase);

impl Rule for UseExhaustiveSwitchCases {
    type Query = Typed<JsSwitchStatement>;
    type State = Vec<MissingCase>;
    type Signals = Option<Self::State>;
    type Options = UseExhaustiveSwitchCasesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let stmt = ctx.query();
        let cases = stmt.cases();

        let has_default_case = cases
            .iter()
            .any(|case| matches!(case, AnyJsSwitchClause::JsDefaultClause(_)));
        if has_default_case {
            return None;
        }

        let mut found_cases = Vec::new();
        for case in cases.iter() {
            let AnyJsSwitchClause::JsCaseClause(case) = case else {
                continue;
            };
            let Ok(test) = case.test() else {
                continue;
            };
            let ty = ctx.inferred_type_of_expression(&test)?;
            let variants = ty.try_switch_case_variants()?;
            if let [variant] = variants.as_slice()
                && *variant != InferredSwitchCase::UnsupportedLiteral
            {
                found_cases.push(variant.clone());
            }
        }

        let mut missing_cases = Vec::new();

        let discriminant = stmt.discriminant().ok()?;

        let is_switch_true_condition_group = is_true_literal_expression(&discriminant)
            && found_cases.contains(&InferredSwitchCase::Boolean);
        if is_switch_true_condition_group {
            return None;
        }

        let has_boolean_case = |value: bool| {
            found_cases
                .iter()
                .any(|case| *case == InferredSwitchCase::BooleanLiteral(value))
        };

        let variants = ctx
            .inferred_type_of_expression(&discriminant)?
            .try_switch_case_variants()?;

        for variant in variants {
            if variant == InferredSwitchCase::Boolean {
                if !has_boolean_case(true) {
                    missing_cases.push(MissingCase(InferredSwitchCase::BooleanLiteral(true)));
                }
                if !has_boolean_case(false) {
                    missing_cases.push(MissingCase(InferredSwitchCase::BooleanLiteral(false)));
                }
                continue;
            }

            if found_cases.contains(&variant) {
                continue;
            }

            missing_cases.push(MissingCase(variant));
        }

        if missing_cases.is_empty() {
            return None;
        }

        Some(missing_cases)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "The switch statement is not exhaustive." },
            )
            .note("Some variants of the union type are not handled here.")
            .footer_list(
                "These cases are missing:",
                state,
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let stmt = ctx.query();
        let mut mutation = ctx.root().begin();

        let error_expr = make::js_new_expression(
            make::token(T![new]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::js_call_expression(
                make::js_identifier_expression(make::js_reference_identifier(make::ident("Error")))
                    .into(),
                make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list(
                        [AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                make::js_string_literal_expression(make::js_string_literal(
                                    "TODO: Not implemented yet",
                                ))
                                .into(),
                            ),
                        )],
                        [],
                    ),
                    make::token(T![')']),
                ),
            )
            .build()
            .into(),
        )
        .build();

        let case_list = stmt.cases();
        let mut clauses = case_list.iter().collect::<Vec<_>>();

        let leading_trivia = case_list
            .last()
            .and_then(|case| case.syntax().first_leading_trivia());

        let throw_stmt: AnyJsStatement = make::js_throw_statement(
            make::token(T![throw]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            error_expr.clone().into(),
        )
        .with_semicolon_token(make::token(T![;]))
        .build()
        .into();

        for ty in state {
            let mut case_token =
                make::token(T![case]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

            if let Some(leading_trivia) = &leading_trivia {
                case_token = case_token.with_leading_trivia_pieces(leading_trivia.pieces());
            } else {
                case_token = case_token.with_leading_trivia([
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, "  "),
                ]);
            }

            let clause = AnyJsSwitchClause::JsCaseClause(make::js_case_clause(
                case_token,
                missing_case_to_expression(ty)?,
                make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                make::js_statement_list([throw_stmt.clone()]),
            ));

            clauses.push(clause);
        }

        mutation.replace_node(case_list, make::js_switch_case_list(clauses));

        let message = markup! { "Add the missing cases to the switch statement." };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

// A condition-group switch must use the literal `true`, not a boolean expression.
fn is_true_literal_expression(expr: &AnyJsExpression) -> bool {
    expr.as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_boolean_literal_expression())
        .and_then(|literal| literal.value_token().ok())
        .is_some_and(|token| token.kind() == T![true])
}

impl Display for MissingCase {
    fn fmt(&self, formatter: &mut Formatter) -> io::Result<()> {
        match &self.0 {
            InferredSwitchCase::BooleanLiteral(value) => {
                formatter.write_str(if *value { "true" } else { "false" })
            }
            InferredSwitchCase::BigInt(bigint) => formatter.write_str(bigint.text()),
            InferredSwitchCase::Number(number) => formatter.write_str(number.text()),
            InferredSwitchCase::String(string) => {
                formatter.write_fmt(format_args!("\"{}\"", string.text()))
            }
            InferredSwitchCase::Null => formatter.write_str("null"),
            InferredSwitchCase::Undefined => formatter.write_str("undefined"),
            InferredSwitchCase::Boolean
            | InferredSwitchCase::Symbol
            | InferredSwitchCase::UnsupportedLiteral => formatter.write_str("unknown"),
        }
    }
}

fn missing_case_to_expression(case: &MissingCase) -> Option<AnyJsExpression> {
    match case {
        MissingCase(InferredSwitchCase::BooleanLiteral(value)) => Some(
            AnyJsExpression::AnyJsLiteralExpression(
            make::js_boolean_literal_expression(make::token(match value {
                true => T![true],
                false => T![false],
            }))
            .into(),
            ),
        ),
        MissingCase(InferredSwitchCase::Number(number)) => Some(
            AnyJsExpression::AnyJsLiteralExpression(
                make::js_number_literal_expression(make::js_number_literal(number.text())).into(),
            ),
        ),
        MissingCase(InferredSwitchCase::BigInt(_)) => None,
        MissingCase(InferredSwitchCase::String(string)) => Some(
            AnyJsExpression::AnyJsLiteralExpression(
                make::js_string_literal_expression(make::js_string_literal(string.text())).into(),
            ),
        ),
        MissingCase(InferredSwitchCase::Null) => Some(AnyJsExpression::AnyJsLiteralExpression(
            make::js_null_literal_expression(make::token(T![null])).into(),
        )),
        MissingCase(InferredSwitchCase::Undefined) => Some(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("undefined")))
                .into(),
        ),
        MissingCase(
            InferredSwitchCase::Boolean
            | InferredSwitchCase::Symbol
            | InferredSwitchCase::UnsupportedLiteral,
        ) => None,
    }
}

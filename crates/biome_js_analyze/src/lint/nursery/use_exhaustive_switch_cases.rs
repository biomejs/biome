use std::{io, ops::Deref};

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
use biome_js_type_info::{Literal, Type, TypeData};
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

pub enum MissingCase {
    Type(Type),
    BooleanLiteral(bool),
}

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

        let found_cases = cases
            .iter()
            .filter_map(|case| match case {
                AnyJsSwitchClause::JsCaseClause(case) => {
                    let test = case.test().ok()?;
                    flatten_type(&ctx.type_of_expression(&test))
                        .as_deref()
                        .cloned()
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let mut missing_cases = Vec::new();

        let discriminant = stmt.discriminant().ok()?;

        let is_switch_true_condition_group = is_true_literal_expression(&discriminant)
            && found_cases.iter().any(|case| matches!(case, TypeData::Boolean));
        if is_switch_true_condition_group {
            return None;
        }

        let has_boolean_case = |value: bool| {
            found_cases
                .iter()
                .any(|case| is_boolean_literal_data_with_value(case, value))
        };

        let discriminant_ty = flatten_type(&ctx.type_of_expression(&discriminant))?;

        let variants = match discriminant_ty.is_union() {
            true => Type::normalized_boolean_union_variants(
                discriminant_ty.flattened_union_variants().collect(),
            ),
            false => vec![discriminant_ty],
        };

        for intersection_part in variants {
            let intersection_part = flatten_type(&intersection_part)?;

            if matches!(intersection_part.deref(), TypeData::Boolean) {
                if !has_boolean_case(true) {
                    missing_cases.push(MissingCase::BooleanLiteral(true));
                }
                if !has_boolean_case(false) {
                    missing_cases.push(MissingCase::BooleanLiteral(false));
                }
                continue;
            }

            if !matches!(
                intersection_part.deref(),
                TypeData::Literal(_) | TypeData::Null | TypeData::Undefined | TypeData::Symbol
            ) || found_cases.contains(&intersection_part)
            {
                continue;
            }

            missing_cases.push(MissingCase::Type(intersection_part));
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

/// Unwraps [`TypeInner::InstanceOf`] and [`TypeInner::TypeofType`] to get the inner type.
// TODO: I am not sure if this should be in the type flattening logic.
fn flatten_type(ty: &Type) -> Option<Type> {
    match ty.deref() {
        TypeData::InstanceOf(instance) => ty.resolve(&instance.ty),
        TypeData::Reference(reference) => ty.resolve(reference),
        TypeData::TypeofType(inner) => ty.resolve(inner),
        _ => Some(ty.clone()),
    }
}

// A condition-group switch must use the literal `true`, not a boolean expression.
fn is_true_literal_expression(expr: &AnyJsExpression) -> bool {
    expr.as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_boolean_literal_expression())
        .and_then(|literal| literal.value_token().ok())
        .is_some_and(|token| token.kind() == T![true])
}

// Only boolean literal cases satisfy individual `true`/`false` variants.
fn is_boolean_literal_data_with_value(ty: &TypeData, value: bool) -> bool {
    matches!(
        ty,
        TypeData::Literal(lit)
            if matches!(lit.as_ref(), Literal::Boolean(b) if b.as_bool() == value)
    )
}

impl Display for MissingCase {
    fn fmt(&self, formatter: &mut Formatter) -> io::Result<()> {
        match self {
            Self::Type(case_type) => write_type(case_type, formatter),
            Self::BooleanLiteral(value) => {
                formatter.write_str(if *value { "true" } else { "false" })
            }
        }
    }
}

// Render directly into the diagnostic formatter to avoid a temporary `String`.
fn write_type(case_type: &Type, formatter: &mut Formatter) -> io::Result<()> {
    match case_type.deref() {
        TypeData::Literal(literal) => match literal.as_ref() {
            Literal::Boolean(boolean) => {
                formatter.write_str(if boolean.as_bool() { "true" } else { "false" })
            }
            Literal::Number(number) => formatter.write_str(number.as_str()),
            Literal::String(string) => {
                formatter.write_fmt(format_args!("\"{}\"", string.as_str()))
            }
            _ => formatter.write_str("unknown"),
        },
        TypeData::Null => formatter.write_str("null"),
        TypeData::Undefined => formatter.write_str("undefined"),
        _ => formatter.write_str("unknown"),
    }
}

fn missing_case_to_expression(case: &MissingCase) -> Option<AnyJsExpression> {
    match case {
        MissingCase::Type(ty) => type_to_expression(ty),
        MissingCase::BooleanLiteral(value) => Some(AnyJsExpression::AnyJsLiteralExpression(
            make::js_boolean_literal_expression(make::token(match value {
                true => T![true],
                false => T![false],
            }))
            .into(),
        )),
    }
}

fn type_to_expression(ty: &Type) -> Option<AnyJsExpression> {
    Some(match ty.deref() {
        TypeData::Literal(lit) => AnyJsExpression::AnyJsLiteralExpression(match lit.as_ref() {
            Literal::Boolean(b) => {
                make::js_boolean_literal_expression(make::token(match b.as_bool() {
                    true => T![true],
                    false => T![false],
                }))
                .into()
            }
            Literal::Number(n) => {
                let text = n.text();
                make::js_number_literal_expression(make::js_number_literal(text)).into()
            }
            Literal::String(s) => {
                make::js_string_literal_expression(make::js_string_literal(s.as_str())).into()
            }
            _ => return None,
        }),
        TypeData::Null => AnyJsExpression::AnyJsLiteralExpression(
            make::js_null_literal_expression(make::token(T![null])).into(),
        ),
        TypeData::Undefined => {
            make::js_identifier_expression(make::js_reference_identifier(make::ident("undefined")))
                .into()
        }
        _ => return None,
    })
}

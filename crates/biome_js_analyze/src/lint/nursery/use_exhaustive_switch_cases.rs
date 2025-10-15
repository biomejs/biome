use std::ops::Deref;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
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
        domains: &[RuleDomain::Project],
    }
}

impl Rule for UseExhaustiveSwitchCases {
    type Query = Typed<JsSwitchStatement>;
    type State = Vec<Type>;
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
        let discriminant_ty = flatten_type(&ctx.type_of_expression(&discriminant))?;

        for intersection_part in match discriminant_ty.is_union() {
            true => discriminant_ty.flattened_union_variants().collect(),
            false => vec![discriminant_ty],
        } {
            let intersection_part = flatten_type(&intersection_part)?;

            if !matches!(
                intersection_part.deref(),
                TypeData::Literal(_) | TypeData::Null | TypeData::Undefined | TypeData::Symbol
            ) || found_cases.contains(&intersection_part)
            {
                continue;
            }

            missing_cases.push(intersection_part);
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
            .footer_list("These cases are missing:", state.iter().map(type_to_string)),
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
                type_to_expression(ty)?,
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

fn type_to_string(ty: &Type) -> String {
    match ty.deref() {
        TypeData::Literal(lit) => match lit.as_ref() {
            Literal::Boolean(b) => b.as_bool().to_string(),
            Literal::Number(n) => n.text().to_string(),
            Literal::String(s) => format!("\"{}\"", s.as_str()),
            _ => "unknown".to_string(),
        },
        TypeData::Null => "null".to_string(),
        TypeData::Undefined => "undefined".to_string(),
        _ => "unknown".to_string(),
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

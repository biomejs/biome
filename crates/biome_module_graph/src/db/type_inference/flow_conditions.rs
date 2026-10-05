//! Shares supported condition syntax between flow evaluation and subject discovery.

use biome_js_control_flow::FlowOutcome;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryOperator, JsCallExpression, JsLogicalOperator,
    JsReferenceIdentifier, JsUnaryOperator,
};
use biome_js_type_info::TypeofKind;
use biome_rowan::{AstNode, SyntaxKind};

pub(super) const MAX_CONDITION_DEPTH: usize = 32;

pub(in crate::db) enum SyntaxGuard {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    Literal(AnyJsLiteralExpression),
    Undefined {
        reference: JsReferenceIdentifier,
        strict: bool,
    },
}

impl SyntaxGuard {
    pub(super) fn is_applicable(&self, model: &SemanticModel) -> bool {
        !matches!(self, Self::Undefined { reference, .. } if model.binding(reference).is_some())
    }
}

/// One syntax step, with child expressions left unevaluated.
///
/// Unsupported steps preserve the incoming type, including in logical alternatives.
/// Literal conversion belongs to evaluation; applicability only inspects bindings.
pub(in crate::db) enum ConditionStep {
    Unsupported,
    Guard {
        subject: JsReferenceIdentifier,
        guard: SyntaxGuard,
        positive: bool,
    },
    Call {
        call: JsCallExpression,
        positive: bool,
    },
    Negated {
        argument: AnyJsExpression,
        outcome: FlowOutcome,
    },
    Logical {
        left: AnyJsExpression,
        right: AnyJsExpression,
        outcome: FlowOutcome,
        sequential: bool,
    },
}

/// Returns direct identifier subjects without resolving their types.
///
/// Subjects may repeat. An empty result proves there is no supported target.
/// `None` means required syntax is missing or the work/depth limit was reached;
/// callers must not use it to exclude candidates. The semantic model rules out
/// comparisons against a variable that shadows `undefined`. The call callback
/// selects predicate arguments from raw signatures without resolving types.
pub(in crate::db) fn condition_subjects(
    expression: AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    call_subject: &impl Fn(&JsCallExpression) -> Option<JsReferenceIdentifier>,
    remaining: &mut usize,
) -> Option<Vec<JsReferenceIdentifier>> {
    let mut subjects = Vec::new();
    collect_subjects(
        expression,
        outcome,
        model,
        call_subject,
        0,
        remaining,
        &mut subjects,
    )?;
    Some(subjects)
}

fn collect_subjects(
    expression: AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    call_subject: &impl Fn(&JsCallExpression) -> Option<JsReferenceIdentifier>,
    depth: usize,
    remaining: &mut usize,
    subjects: &mut Vec<JsReferenceIdentifier>,
) -> Option<()> {
    if depth >= MAX_CONDITION_DEPTH {
        return None;
    }
    match condition_step(expression, outcome, remaining)? {
        ConditionStep::Unsupported => {}
        ConditionStep::Guard { subject, guard, .. } => {
            if guard.is_applicable(model) {
                subjects.push(subject);
            }
        }
        ConditionStep::Call { call, .. } => {
            if let Some(subject) = call_subject(&call) {
                subjects.push(subject);
            }
        }
        ConditionStep::Negated { argument, outcome } => {
            collect_subjects(
                argument,
                outcome,
                model,
                call_subject,
                depth + 1,
                remaining,
                subjects,
            )?;
        }
        ConditionStep::Logical {
            left,
            right,
            outcome,
            ..
        } => {
            collect_subjects(
                left,
                outcome,
                model,
                call_subject,
                depth + 1,
                remaining,
                subjects,
            )?;
            collect_subjects(
                right,
                outcome,
                model,
                call_subject,
                depth + 1,
                remaining,
                subjects,
            )?;
        }
    }
    Some(())
}

/// Decomposes one condition, spending work on the step and parenthesis traversal.
/// Missing syntax and exhausted work return `None`, not `Unsupported`.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only supported narrowing syntax is decomposed."
)]
pub(in crate::db) fn condition_step(
    expression: AnyJsExpression,
    outcome: FlowOutcome,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    *remaining = remaining.checked_sub(1)?;
    let expression = omit_parentheses(expression, remaining)?;
    let positive = matches!(outcome, FlowOutcome::Truthy | FlowOutcome::Nullish);
    if let AnyJsExpression::JsIdentifierExpression(identifier) = expression {
        let subject = identifier.name().ok()?;
        subject.value_token().ok()?;
        let guard = match outcome {
            FlowOutcome::Truthy | FlowOutcome::Falsy => SyntaxGuard::Truthy,
            FlowOutcome::Nullish | FlowOutcome::NonNullish => SyntaxGuard::Nullish,
        };
        return Some(ConditionStep::Guard {
            subject,
            guard,
            positive,
        });
    }
    if matches!(outcome, FlowOutcome::Nullish | FlowOutcome::NonNullish) {
        return Some(ConditionStep::Unsupported);
    }
    match expression {
        AnyJsExpression::JsCallExpression(call) => Some(ConditionStep::Call { call, positive }),
        AnyJsExpression::JsUnaryExpression(unary) => {
            let operator = unary.operator().ok()?;
            let argument = unary.argument().ok()?;
            if operator != JsUnaryOperator::LogicalNot {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Negated {
                argument,
                outcome: if positive {
                    FlowOutcome::Falsy
                } else {
                    FlowOutcome::Truthy
                },
            })
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            let left = logical.left().ok()?;
            let right = logical.right().ok()?;
            let operator = logical.operator().ok()?;
            if operator == JsLogicalOperator::NullishCoalescing {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Logical {
                left,
                right,
                outcome,
                sequential: (operator == JsLogicalOperator::LogicalAnd) == positive,
            })
        }
        AnyJsExpression::JsBinaryExpression(binary) => {
            let left = binary.left().ok()?;
            let right = binary.right().ok()?;
            let operator = binary.operator().ok()?;
            let equal = match operator {
                JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality => positive,
                JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality => !positive,
                _ => return Some(ConditionStep::Unsupported),
            };
            let strict = matches!(
                operator,
                JsBinaryOperator::StrictEquality | JsBinaryOperator::StrictInequality
            );
            let left = omit_parentheses(left, remaining)?;
            let right = omit_parentheses(right, remaining)?;
            for (subject, value) in [(&left, &right), (&right, &left)] {
                let step = comparison_step(subject, value, strict, equal, remaining)?;
                if !matches!(step, ConditionStep::Unsupported) {
                    return Some(step);
                }
            }
            Some(ConditionStep::Unsupported)
        }
        _ => Some(ConditionStep::Unsupported),
    }
}

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Comparisons only support direct identifiers and primitive guards."
)]
fn comparison_step(
    subject: &AnyJsExpression,
    value: &AnyJsExpression,
    strict: bool,
    positive: bool,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    let (subject, guard) = match subject {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let guard = match value {
                AnyJsExpression::AnyJsLiteralExpression(literal) => {
                    literal.value_token().ok()?;
                    match literal {
                        AnyJsLiteralExpression::JsNullLiteralExpression(_) if !strict => {
                            SyntaxGuard::Nullish
                        }
                        AnyJsLiteralExpression::JsNullLiteralExpression(_)
                        | AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
                        | AnyJsLiteralExpression::JsStringLiteralExpression(_)
                        | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                        | AnyJsLiteralExpression::JsBigintLiteralExpression(_)
                            if strict =>
                        {
                            SyntaxGuard::Literal(literal.clone())
                        }
                        _ => return Some(ConditionStep::Unsupported),
                    }
                }
                AnyJsExpression::JsIdentifierExpression(identifier) => {
                    let reference = identifier.name().ok()?;
                    if reference.value_token().ok()?.text_trimmed() != "undefined" {
                        return Some(ConditionStep::Unsupported);
                    }
                    SyntaxGuard::Undefined { reference, strict }
                }
                _ => return Some(ConditionStep::Unsupported),
            };
            (identifier.name().ok()?, guard)
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? != JsUnaryOperator::Typeof {
                return Some(ConditionStep::Unsupported);
            }
            let argument = unary.argument().ok()?;
            let AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(literal),
            ) = value
            else {
                return Some(ConditionStep::Unsupported);
            };
            let value = literal.inner_string_text().ok()?;
            let kind = match value.text() {
                "undefined" => TypeofKind::Undefined,
                "object" => TypeofKind::Object,
                "boolean" => TypeofKind::Boolean,
                "number" => TypeofKind::Number,
                "bigint" => TypeofKind::BigInt,
                "string" => TypeofKind::String,
                "symbol" => TypeofKind::Symbol,
                "function" => TypeofKind::Function,
                _ => return Some(ConditionStep::Unsupported),
            };
            let AnyJsExpression::JsIdentifierExpression(identifier) =
                omit_parentheses(argument, remaining)?
            else {
                return Some(ConditionStep::Unsupported);
            };
            (identifier.name().ok()?, SyntaxGuard::Typeof(kind))
        }
        _ => return Some(ConditionStep::Unsupported),
    };
    subject.value_token().ok()?;
    Some(ConditionStep::Guard {
        subject,
        guard,
        positive,
    })
}

fn omit_parentheses(
    mut expression: AnyJsExpression,
    remaining: &mut usize,
) -> Option<AnyJsExpression> {
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized) = expression {
        *remaining = remaining.checked_sub(1)?;
        parenthesized.l_paren_token().ok()?;
        parenthesized.r_paren_token().ok()?;
        expression = parenthesized.expression().ok()?;
    }
    let kind = expression.syntax().kind();
    (!kind.is_bogus() && !kind.is_metavariable()).then_some(expression)
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsIfStatement;
    use biome_languages::JsFileSource;

    fn condition(source: &str) -> (AnyJsExpression, SemanticModel) {
        let parsed = parse(
            &format!("if ({source}) {{}}"),
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let expression = parsed
            .syntax()
            .descendants()
            .find_map(JsIfStatement::cast)
            .unwrap()
            .test()
            .unwrap();
        (expression, model)
    }

    #[test]
    fn subjects_match_supported_condition_shapes() {
        for (source, outcome, expected) in [
            ("value", FlowOutcome::Truthy, vec!["value"]),
            ("(!((value)))", FlowOutcome::Falsy, vec!["value"]),
            (
                "typeof (value) !== 'number'",
                FlowOutcome::Truthy,
                vec!["value"],
            ),
            (
                "'string' === typeof value",
                FlowOutcome::Falsy,
                vec!["value"],
            ),
            ("(value) == (null)", FlowOutcome::Truthy, vec!["value"]),
            ("undefined != value", FlowOutcome::Truthy, vec!["value"]),
            ("value === 1", FlowOutcome::Falsy, vec!["value"]),
            (
                "value && other",
                FlowOutcome::Truthy,
                vec!["value", "other"],
            ),
            ("value || other", FlowOutcome::Falsy, vec!["value", "other"]),
            ("value", FlowOutcome::Nullish, vec!["value"]),
            ("value.length", FlowOutcome::Truthy, vec![]),
            ("check(value)", FlowOutcome::Truthy, vec![]),
            ("value === other", FlowOutcome::Truthy, vec![]),
            ("value == 1", FlowOutcome::Truthy, vec![]),
            ("value === -1", FlowOutcome::Truthy, vec![]),
            ("value === void 0", FlowOutcome::Truthy, vec![]),
            ("value ?? other", FlowOutcome::Truthy, vec![]),
            ("value && other", FlowOutcome::Nullish, vec![]),
        ] {
            let (expression, model) = condition(source);
            let subjects =
                condition_subjects(expression, outcome, &model, &|_| None, &mut 1024).unwrap();
            let names = subjects
                .iter()
                .map(|subject| subject.value_token().unwrap().text_trimmed().to_owned())
                .collect::<Vec<_>>();
            assert_eq!(names, expected, "{source}");
        }
    }

    #[test]
    fn incomplete_discovery_does_not_prove_an_empty_subject_set() {
        for (source, mut remaining) in [("value", 0), ("((value))", 1), ("value ===", 1024)] {
            let (expression, model) = condition(source);
            assert!(
                condition_subjects(
                    expression,
                    FlowOutcome::Truthy,
                    &model,
                    &|_| None,
                    &mut remaining
                )
                .is_none()
            );
        }
        let source = format!("{}value", "!".repeat(MAX_CONDITION_DEPTH));
        let (expression, model) = condition(&source);
        assert!(
            condition_subjects(
                expression,
                FlowOutcome::Truthy,
                &model,
                &|_| None,
                &mut 1024
            )
            .is_none()
        );
    }
}

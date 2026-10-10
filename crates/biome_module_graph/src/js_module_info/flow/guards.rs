use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryOperator, JsIdentifierExpression,
    JsLogicalOperator, JsReferenceIdentifier, JsUnaryOperator,
};
use biome_js_type_info::TypeofKind;
use biome_rowan::{AstNode, SyntaxKind, TextRange};

/// Limits the nested `!`, `&&`, and `||` operators decomposed in one
/// condition. Deeper operands keep the incoming type.
const MAX_CONDITION_DEPTH: usize = 32;
/// Limits the syntax steps spent decomposing one condition.
const MAX_CONDITION_STEPS: usize = 16_384;
/// Limits the descendants scanned for variables when decomposition stops early.
const MAX_FALLBACK_NODES: usize = 1024;

/// The result of a condition that a flow path has observed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FlowOutcome {
    Truthy,
    Falsy,
    Nullish,
    NonNullish,
}

/// One step of a decomposed condition, stored in an arena per execution root.
///
/// Applying a guard to a variable's incoming type yields its type once the
/// condition holds. `Keep` and tests of other variables return the incoming
/// type, while `Incomplete` makes it unknown.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FlowGuard {
    /// The step cannot refine any variable, so the incoming type is kept.
    Keep,
    /// Decomposition stopped at its work limit or at missing syntax, so the
    /// incoming type becomes unknown.
    Incomplete,
    /// A runtime test of the variable declared at `binding`.
    Test {
        binding: TextRange,
        test: FlowTest,
        positive: bool,
    },
    /// The operands of `&&` or `||`. A sequential pair applies `right` to the
    /// result of `left`; otherwise both apply to the incoming type and their
    /// results are joined.
    Both {
        left: usize,
        right: usize,
        sequential: bool,
    },
}

/// A runtime test whose result a guard assumes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FlowTest {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    /// Strict equality with `null`.
    Null,
    /// Strict equality with the primitive literal expression at this range.
    Literal(TextRange),
    /// Equality with `undefined`; loose equality also matches `null`.
    Undefined {
        strict: bool,
    },
}

/// A condition decomposed into the arena.
pub(super) struct DecomposedCondition {
    pub(super) guard: usize,
    /// Bindings whose type the condition may refine.
    ///
    /// When decomposition stops early, this conservatively lists every
    /// variable read directly in the condition.
    pub(super) mentions: Vec<TextRange>,
}

/// Decomposes `expression`, assuming `outcome`, into guards appended to `guards`.
pub(super) fn decompose_condition(
    expression: &AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    guards: &mut Vec<FlowGuard>,
) -> DecomposedCondition {
    let mut decomposer = Decomposer {
        model,
        guards,
        remaining: MAX_CONDITION_STEPS,
        mentions: Vec::new(),
        complete: true,
    };
    let guard = decomposer.guard(expression.clone(), outcome, 0);
    let mentions = if decomposer.complete {
        decomposer.mentions
    } else {
        expression
            .syntax()
            .descendants()
            .take(MAX_FALLBACK_NODES)
            .filter_map(JsIdentifierExpression::cast)
            .filter_map(|identifier| model.binding(&identifier.name().ok()?))
            .map(|binding| binding.range())
            .collect()
    };
    DecomposedCondition { guard, mentions }
}

struct Decomposer<'a> {
    model: &'a SemanticModel,
    guards: &'a mut Vec<FlowGuard>,
    remaining: usize,
    mentions: Vec<TextRange>,
    complete: bool,
}

impl Decomposer<'_> {
    fn guard(&mut self, expression: AnyJsExpression, outcome: FlowOutcome, depth: usize) -> usize {
        if depth >= MAX_CONDITION_DEPTH {
            self.complete = false;
            return self.push(FlowGuard::Keep);
        }
        let Some(step) = condition_step(expression, outcome, &mut self.remaining) else {
            self.complete = false;
            return self.push(FlowGuard::Incomplete);
        };
        let guard = match step {
            ConditionStep::Unsupported => FlowGuard::Keep,
            ConditionStep::Guard {
                subject,
                guard,
                positive,
            } => self.test(&subject, guard, positive),
            ConditionStep::Negated { argument, outcome } => {
                return self.guard(argument, outcome, depth + 1);
            }
            ConditionStep::Logical {
                left,
                right,
                outcome,
                sequential,
            } => {
                let left = self.guard(left, outcome, depth + 1);
                let right = self.guard(right, outcome, depth + 1);
                FlowGuard::Both {
                    left,
                    right,
                    sequential,
                }
            }
        };
        self.push(guard)
    }

    fn test(
        &mut self,
        subject: &JsReferenceIdentifier,
        guard: SyntaxGuard,
        positive: bool,
    ) -> FlowGuard {
        let test = match guard {
            SyntaxGuard::Truthy => FlowTest::Truthy,
            SyntaxGuard::Nullish => FlowTest::Nullish,
            SyntaxGuard::Typeof(kind) => FlowTest::Typeof(kind),
            SyntaxGuard::Literal(AnyJsLiteralExpression::JsNullLiteralExpression(_)) => {
                FlowTest::Null
            }
            SyntaxGuard::Literal(literal) => FlowTest::Literal(literal.range()),
            // A local variable named `undefined` can hold any value.
            SyntaxGuard::Undefined { reference, .. }
                if self.model.binding(&reference).is_some() =>
            {
                return FlowGuard::Keep;
            }
            SyntaxGuard::Undefined { strict, .. } => FlowTest::Undefined { strict },
        };
        let Some(binding) = self.model.binding(subject) else {
            return FlowGuard::Keep;
        };
        self.mentions.push(binding.range());
        FlowGuard::Test {
            binding: binding.range(),
            test,
            positive,
        }
    }

    fn push(&mut self, guard: FlowGuard) -> usize {
        self.guards.push(guard);
        self.guards.len() - 1
    }
}

enum SyntaxGuard {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    Literal(AnyJsLiteralExpression),
    Undefined {
        reference: JsReferenceIdentifier,
        strict: bool,
    },
}

/// One syntax step, with child expressions left undecomposed.
enum ConditionStep {
    Unsupported,
    Guard {
        subject: JsReferenceIdentifier,
        guard: SyntaxGuard,
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

/// Decomposes one condition step, spending work on the step and on removing
/// parentheses. Missing syntax and exhausted work return `None`, not
/// `Unsupported`.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only supported narrowing syntax is decomposed."
)]
fn condition_step(
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

    /// Decomposes the `if` test `source` and names the mentioned bindings.
    fn decompose(source: &str, outcome: FlowOutcome) -> (Vec<FlowGuard>, Vec<String>) {
        let parsed = parse(
            &format!("function f(value, other) {{ if ({source}) {{}} }}"),
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let test = parsed
            .syntax()
            .descendants()
            .find_map(JsIfStatement::cast)
            .unwrap()
            .test()
            .unwrap();
        let mut guards = Vec::new();
        let condition = decompose_condition(&test, outcome, &model, &mut guards);
        let names = condition
            .mentions
            .iter()
            .map(|range| {
                model
                    .as_binding_by_range(*range)
                    .unwrap()
                    .tree()
                    .name_token()
                    .unwrap()
                    .text_trimmed()
                    .to_owned()
            })
            .collect();
        (guards, names)
    }

    fn mentioned_names(source: &str, outcome: FlowOutcome) -> Vec<String> {
        decompose(source, outcome).1
    }

    #[test]
    fn mentions_match_supported_condition_shapes() {
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
            ("global", FlowOutcome::Truthy, vec![]),
        ] {
            assert_eq!(mentioned_names(source, outcome), expected, "{source}");
        }
    }

    #[test]
    fn missing_operands_make_the_guard_incomplete() {
        let (guards, names) = decompose("other === null || value ===", FlowOutcome::Truthy);
        assert!(guards.contains(&FlowGuard::Incomplete));
        assert_eq!(names, ["other", "value"]);
    }

    #[test]
    fn incomplete_decomposition_mentions_every_direct_read() {
        let source = format!("{}value && other.length", "!".repeat(MAX_CONDITION_DEPTH));
        assert_eq!(
            mentioned_names(&source, FlowOutcome::Truthy),
            ["value", "other"]
        );
    }
}

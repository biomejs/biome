use crate::{JsRuleAction, services::semantic::Semantic, utils::is_node_equal};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::{make, syntax::T};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyTsType, JsAssignmentExpression, JsBinaryOperator,
    JsCallExpression, JsConditionalExpression, JsFormalParameter, JsImportCallExpression,
    JsNewExpression, JsPostUpdateExpression, JsPreUpdateExpression, JsSyntaxToken,
    JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxTriviaPiece};
use biome_rule_options::use_math_min_max::UseMathMinMaxOptions;

declare_lint_rule! {
    /// Prefer `Math.min()` and `Math.max()` over ternaries for simple comparisons.
    ///
    /// Replacing ternary comparisons like `a > b ? b : a` with `Math.min(a, b)` makes the intent clearer and keeps equivalent min/max comparisons consistent across a codebase.
    ///
    /// This rule only targets straightforward min/max ternaries and ignores operands that are obviously not numeric, such as `bigint` and `Date` values.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// height > 50 ? 50 : height;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// height < 50 ? 50 : height;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Math.min(height, 50);
    /// ```
    ///
    /// ```js
    /// Math.max(height, 50);
    /// ```
    ///
    /// ```js
    /// foo ? foo : bar;
    /// ```
    ///
    pub UseMathMinMax {
        version: "next",
        name: "useMathMinMax",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-math-min-max").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseMathMinMax {
    type Query = Semantic<JsConditionalExpression>;
    type State = UseMathMinMaxState;
    type Signals = Option<Self::State>;
    type Options = UseMathMinMaxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let conditional = ctx.query();
        let model = ctx.model();
        let comparison = extract_comparison(conditional)?;

        if has_unsupported_operand(ctx, &comparison.left)
            || has_unsupported_operand(ctx, &comparison.right)
            || has_unsupported_identifier_operand(model, &comparison.left).unwrap_or(false)
            || has_unsupported_identifier_operand(model, &comparison.right).unwrap_or(false)
            || has_shadowed_math(model, conditional)
        {
            return None;
        }

        let left_repeatability = operand_repeatability(&comparison.left)?;
        let right_repeatability = operand_repeatability(&comparison.right)?;

        if matches!(
            (left_repeatability, right_repeatability),
            (OperandRepeatability::DefinitelyNonIdempotent, _)
                | (_, OperandRepeatability::DefinitelyNonIdempotent)
        ) {
            return None;
        }

        Some(UseMathMinMaxState {
            preferred_method: classify_method(&comparison, conditional)?,
            can_fix: !matches!(
                (left_repeatability, right_repeatability),
                (OperandRepeatability::MaybeNonIdempotent, _)
                    | (_, OperandRepeatability::MaybeNonIdempotent)
            ),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let operation = match state.preferred_method {
            PreferredMathMethod::Min => "smaller",
            PreferredMathMethod::Max => "larger",
        };

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "This ternary is selecting the "<Emphasis>{operation}</Emphasis>" of the same two operands, which can be simplified."
            },
        ).note(markup! {
            "Dedicated min/max functions like "<Emphasis>"Math."{state.preferred_method.as_str()}"()"</Emphasis>" are clearer and more concise for this purpose."
        });

        Some(if state.can_fix {
            diagnostic
        } else {
            diagnostic
                .note(markup! {
                    "Consider using "<Emphasis>"Math."{state.preferred_method.as_str()}"()"</Emphasis>" instead."
                })
                .note(markup! {
                    "Biome could not provide a fix because it can't guarantee that both arguments don't have side effects."
                })
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.can_fix {
            return None;
        }

        let conditional = ctx.query();
        let comparison = extract_comparison(conditional)?;
        let test = conditional.test().ok()?.omit_parentheses();
        let binary = test.as_js_binary_expression()?;
        let consequent = conditional.consequent().ok()?;
        let alternate = conditional.alternate().ok()?;
        let operator_token = binary.operator_token().ok()?;
        let question_mark_token = conditional.question_mark_token().ok()?;
        let colon_token = conditional.colon_token().ok()?;

        let callee = AnyJsExpression::JsStaticMemberExpression(make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("Math")))
                .into(),
            make::token(T![.]),
            make::js_name(make::ident(state.preferred_method.as_str())).into(),
        ));

        let args = {
            let args = [
                AnyJsCallArgument::AnyJsExpression(build_argument_expression(
                    conditional,
                    &comparison.left,
                    matching_branch_expression(&comparison.left, &consequent, &alternate)?,
                    None,
                    matching_branch_separator_token(
                        &comparison.left,
                        &consequent,
                        &alternate,
                        &question_mark_token,
                        &colon_token,
                    )?,
                )?),
                AnyJsCallArgument::AnyJsExpression(build_argument_expression(
                    conditional,
                    &comparison.right,
                    matching_branch_expression(&comparison.right, &consequent, &alternate)?,
                    Some(&operator_token),
                    matching_branch_separator_token(
                        &comparison.right,
                        &consequent,
                        &alternate,
                        &question_mark_token,
                        &colon_token,
                    )?,
                )?),
            ];

            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list(args, [make::token_decorated_with_space(T![,])]),
                make::token(T![')']),
            )
        };
        let replacement =
            AnyJsExpression::JsCallExpression(make::js_call_expression(callee, args).build());
        let replacement = replacement
            .prepend_trivia_pieces(conditional.syntax().first_leading_trivia()?.pieces())?
            .append_trivia_pieces(conditional.syntax().last_trailing_trivia()?.pieces())?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            AnyJsExpression::JsConditionalExpression(conditional.clone()),
            replacement,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"Math."{state.preferred_method.as_str()}"()"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UseMathMinMaxState {
    preferred_method: PreferredMathMethod,
    can_fix: bool,
}

fn has_shadowed_math(model: &SemanticModel, conditional: &JsConditionalExpression) -> bool {
    model.scope(conditional.syntax()).ancestors().any(|scope| {
        scope
            .get_binding("Math")
            .is_some_and(|binding| binding.tree().declaration().is_some())
    })
}

fn matching_branch_expression<'a>(
    operand: &AnyJsExpression,
    consequent: &'a AnyJsExpression,
    alternate: &'a AnyJsExpression,
) -> Option<&'a AnyJsExpression> {
    let operand = unwrap_expression(operand.clone())?;
    let consequent_unwrapped = unwrap_expression(consequent.clone())?;
    let alternate_unwrapped = unwrap_expression(alternate.clone())?;

    if is_node_equal(operand.syntax(), consequent_unwrapped.syntax()) {
        Some(consequent)
    } else if is_node_equal(operand.syntax(), alternate_unwrapped.syntax()) {
        Some(alternate)
    } else {
        None
    }
}

/// Builds a call argument from the operand used in the condition and the matching branch
/// expression, carrying over comment trivia from both occurrences.
fn build_argument_expression(
    conditional: &JsConditionalExpression,
    condition_expression: &AnyJsExpression,
    branch_expression: &AnyJsExpression,
    condition_separator: Option<&JsSyntaxToken>,
    branch_separator: Option<&JsSyntaxToken>,
) -> Option<AnyJsExpression> {
    let argument = clean_argument(condition_expression)?;
    let leading_comments = separator_comment_pieces(condition_separator)
        .into_iter()
        .chain(comment_pieces(
            conditional,
            condition_expression,
            CommentPosition::Leading,
        ))
        .chain(separator_comment_pieces(branch_separator))
        .chain(comment_pieces(
            conditional,
            branch_expression,
            CommentPosition::Leading,
        ))
        .collect::<Vec<_>>();
    let trailing_comments =
        comment_pieces(conditional, condition_expression, CommentPosition::Trailing)
            .into_iter()
            .chain(comment_pieces(
                conditional,
                branch_expression,
                CommentPosition::Trailing,
            ))
            .collect::<Vec<_>>();

    let argument = if leading_comments.is_empty() {
        argument
    } else {
        argument.prepend_trivia_pieces(leading_comments)?
    };

    if trailing_comments.is_empty() {
        Some(argument)
    } else {
        argument.append_trivia_pieces(trailing_comments)
    }
}

fn matching_branch_separator_token<'a>(
    operand: &AnyJsExpression,
    consequent: &'a AnyJsExpression,
    alternate: &'a AnyJsExpression,
    question_mark_token: &'a JsSyntaxToken,
    colon_token: &'a JsSyntaxToken,
) -> Option<Option<&'a JsSyntaxToken>> {
    let operand = unwrap_expression(operand.clone())?;
    let consequent_unwrapped = unwrap_expression(consequent.clone())?;
    let alternate_unwrapped = unwrap_expression(alternate.clone())?;

    if is_node_equal(operand.syntax(), consequent_unwrapped.syntax()) {
        Some(Some(question_mark_token))
    } else if is_node_equal(operand.syntax(), alternate_unwrapped.syntax()) {
        Some(Some(colon_token))
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum CommentPosition {
    Leading,
    Trailing,
}

fn comment_pieces(
    conditional: &JsConditionalExpression,
    expression: &AnyJsExpression,
    position: CommentPosition,
) -> Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> {
    let token = match position {
        CommentPosition::Leading => expression.syntax().first_token(),
        CommentPosition::Trailing => expression.syntax().last_token(),
    };

    let is_outer_edge = token.as_ref().is_some_and(|token| match position {
        CommentPosition::Leading => conditional
            .syntax()
            .first_token()
            .is_some_and(|first| first == *token),
        CommentPosition::Trailing => conditional
            .syntax()
            .last_token()
            .is_some_and(|last| last == *token),
    });

    if is_outer_edge {
        return Vec::new();
    }

    token
        .into_iter()
        .flat_map(|token| match position {
            CommentPosition::Leading => token.leading_trivia().pieces().collect::<Vec<_>>(),
            CommentPosition::Trailing => token.trailing_trivia().pieces().collect::<Vec<_>>(),
        })
        .filter(|piece| piece.is_comments())
        .collect()
}

fn separator_comment_pieces(
    token: Option<&JsSyntaxToken>,
) -> Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> {
    token
        .into_iter()
        .flat_map(|token| token.trailing_trivia().pieces().collect::<Vec<_>>())
        .filter(|piece| piece.is_comments())
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub enum PreferredMathMethod {
    Min,
    Max,
}

impl PreferredMathMethod {
    /// Returns the `Math` method name used in diagnostics and fixes.
    fn as_str(self) -> &'static str {
        match self {
            Self::Min => "min",
            Self::Max => "max",
        }
    }
}

struct ComparisonOperands {
    operator: JsBinaryOperator,
    left: AnyJsExpression,
    right: AnyJsExpression,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OperandRepeatability {
    /// The operand is a simple expression that can be safely repeated in the fix.
    Idempotent,
    /// The operand may have side effects or be expensive to compute, but we can't be sure. The fix is still correct, but we choose not to apply it automatically.
    ///
    /// It's important to note that we do not consider static member accesses (e.g. `obj.prop`) to be in this category, even though they could potentially have side effects via getters. This is because such patterns are common and often still worth fixing, and we don't want to rely on heuristics that attempt to detect getters.
    MaybeNonIdempotent,
    /// The operand has clear side effects, so applying the fix would change the behavior of the code. We don't want to apply the fix in this case.
    DefinitelyNonIdempotent,
}

/// Extracts the comparison expression, if its present.
///
/// ```js
/// const foo = a > b ? a : b
/// //          ^^^^^ this part
/// ```
fn extract_comparison(conditional: &JsConditionalExpression) -> Option<ComparisonOperands> {
    let test = conditional.test().ok()?.omit_parentheses();
    let binary = test.as_js_binary_expression()?;
    let operator = binary.operator().ok()?;

    if matches!(
        operator,
        JsBinaryOperator::GreaterThan
            | JsBinaryOperator::GreaterThanOrEqual
            | JsBinaryOperator::LessThan
            | JsBinaryOperator::LessThanOrEqual
    ) {
        Some(ComparisonOperands {
            operator,
            left: binary.left().ok()?,
            right: binary.right().ok()?,
        })
    } else {
        None
    }
}

/// Determine whether the given comparison is a min or max pattern, and which operand corresponds
/// to the selected branch of the ternary. The main business logic of this rule.
fn classify_method(
    comparison: &ComparisonOperands,
    conditional: &JsConditionalExpression,
) -> Option<PreferredMathMethod> {
    let consequent = unwrap_expression(conditional.consequent().ok()?)?;
    let alternate = unwrap_expression(conditional.alternate().ok()?)?;
    let left = unwrap_expression(comparison.left.clone())?;
    let right = unwrap_expression(comparison.right.clone())?;

    // Upstream compares the unwrapped operands textually so that `a as number` can still match
    // `a` in the selected branch while non-number assertions remain excluded.
    let left_matches_consequent = is_node_equal(left.syntax(), consequent.syntax());
    let left_matches_alternate = is_node_equal(left.syntax(), alternate.syntax());
    let right_matches_consequent = is_node_equal(right.syntax(), consequent.syntax());
    let right_matches_alternate = is_node_equal(right.syntax(), alternate.syntax());

    match comparison.operator {
        JsBinaryOperator::GreaterThan | JsBinaryOperator::GreaterThanOrEqual => {
            if left_matches_alternate && right_matches_consequent {
                Some(PreferredMathMethod::Min)
            } else if left_matches_consequent && right_matches_alternate {
                Some(PreferredMathMethod::Max)
            } else {
                None
            }
        }
        JsBinaryOperator::LessThan | JsBinaryOperator::LessThanOrEqual => {
            if left_matches_consequent && right_matches_alternate {
                Some(PreferredMathMethod::Min)
            } else if left_matches_alternate && right_matches_consequent {
                Some(PreferredMathMethod::Max)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn operand_repeatability(expression: &AnyJsExpression) -> Option<OperandRepeatability> {
    let root = expression.syntax().clone();
    let mut has_potential_repeated_evaluation = false;

    for node in std::iter::once(root.clone()).chain(root.descendants()) {
        if JsAssignmentExpression::can_cast(node.kind())
            || JsPreUpdateExpression::can_cast(node.kind())
            || JsPostUpdateExpression::can_cast(node.kind())
        {
            return Some(OperandRepeatability::DefinitelyNonIdempotent);
        }

        if JsUnaryExpression::cast(node.clone())
            .is_some_and(|expression| expression.operator().ok() == Some(JsUnaryOperator::Delete))
        {
            return Some(OperandRepeatability::DefinitelyNonIdempotent);
        }

        if JsCallExpression::can_cast(node.kind())
            || JsImportCallExpression::can_cast(node.kind())
            || JsNewExpression::can_cast(node.kind())
        {
            has_potential_repeated_evaluation = true;
        }
    }

    Some(if has_potential_repeated_evaluation {
        OperandRepeatability::MaybeNonIdempotent
    } else {
        OperandRepeatability::Idempotent
    })
}

/// Removes trivia from an operand before inserting it into `Math.min()`/`Math.max()`.
///
/// This keeps the replacement call from inheriting leading or trailing comments/whitespace
/// that belong to the original ternary expression.
fn clean_argument(expression: &AnyJsExpression) -> Option<AnyJsExpression> {
    expression
        .clone()
        .trim_trivia()?
        .with_leading_trivia_pieces([])?
        .with_trailing_trivia_pieces([])
}

/// Detect whether the expression is obviously not compatible with `Math.min`/`Math.max`,
/// such as non-number literals, `bigint`, and `Date`.
fn has_unsupported_operand(
    _ctx: &RuleContext<UseMathMinMax>,
    expression: &AnyJsExpression,
) -> bool {
    unwrap_expression(expression.clone()).is_none_or(|expr| {
        is_non_number_literal(&expr) || is_bigint_like(&expr) || is_date_construction(&expr)
    })
}

/// Strip TS-only wrappers that do not change which value the ternary returns.
///
/// Returning `None` here lets the caller reject assertions like `as string`, where
/// the upstream rule intentionally avoids rewriting non-number comparisons.
fn unwrap_expression(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    match expression.omit_parentheses() {
        AnyJsExpression::TsAsExpression(assertion) => {
            is_supported_numeric_annotation(&assertion.ty().ok()?)
                .then(|| unwrap_expression(assertion.expression().ok()?))?
        }
        AnyJsExpression::TsTypeAssertionExpression(assertion) => {
            is_supported_numeric_annotation(&assertion.ty().ok()?)
                .then(|| unwrap_expression(assertion.expression().ok()?))?
        }
        AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
            unwrap_expression(assertion.expression().ok()?)
        }
        expression => Some(expression),
    }
}

/// Is the type annotation of a binding a supported numeric type? We want to allow `number` and `Number` annotations.
///
/// ```ts
/// let a: number = 5; // supported
/// let b: Number = 5; // supported
/// let c: string = "foo"; // not supported
/// ```
///
/// Ideally, we would use type information if its available, but right now we don't have a way to do that without forcing type information to be required for this rule.
/// Additionally, this is exactly what the upstream ESLint rule does, so at least we know it's a reasonable heuristic for avoiding common false positives.
fn is_supported_numeric_annotation(annotation: &AnyTsType) -> bool {
    annotation.as_ts_number_type().is_some()
        || annotation
            .as_ts_reference_type()
            .and_then(|reference| reference.name().ok())
            .is_some_and(|name| name.syntax().text_trimmed() == "Number")
}

// Is it a `bigint` literal or a `BigInt()` call? Math.min/max don't work with bigints, so we want to
// avoid suggesting a replacement that would break the code.
fn is_bigint_like(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.as_js_bigint_literal_expression().is_some()
        }
        AnyJsExpression::JsCallExpression(call) => is_named_call(call, "BigInt"),
        _ => false,
    }
}

/// Checks whether an identifier operand is declared in a way that makes a `Math.min`/`Math.max`
/// replacement unsafe or misleading.
///
/// This mirrors the upstream rule's heuristic approach by inspecting binding syntax instead of
/// requiring full type information.
fn has_unsupported_identifier_operand(
    model: &SemanticModel,
    expression: &AnyJsExpression,
) -> Option<bool> {
    let unwrapped = unwrap_expression(expression.clone())?;
    let identifier = unwrapped.as_js_identifier_expression()?;
    let reference = identifier.name().ok()?;
    let binding = model.binding(&reference)?;
    let declaration = binding.tree().declaration()?;

    Some(
        match declaration
            .parent_binding_pattern_declaration()
            .unwrap_or(declaration)
        {
            AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
                parameter_has_unsupported_metadata(&parameter)
            }
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                declarator_has_unsupported_metadata(&declarator)
            }
            _ => false,
        },
    )
}

/// Returns `true` when a parameter's annotation or default value suggests that it should not be
/// treated as a numeric operand.
fn parameter_has_unsupported_metadata(parameter: &JsFormalParameter) -> bool {
    parameter
        .type_annotation()
        .and_then(|annotation| annotation.ty().ok())
        .is_some_and(|annotation| !is_supported_numeric_annotation(&annotation))
        || parameter
            .initializer()
            .and_then(|initializer| initializer.expression().ok())
            .is_some_and(|expression| {
                is_non_number_literal(&expression) || is_date_construction(&expression)
            })
}

/// Returns `true` when a variable declarator's annotation or initializer suggests that it should
/// not be treated as a numeric operand.
fn declarator_has_unsupported_metadata(declarator: &JsVariableDeclarator) -> bool {
    declarator.variable_annotation().is_some_and(|annotation| {
        annotation
            .as_ts_type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|annotation| !is_supported_numeric_annotation(&annotation))
    }) || declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .is_some_and(|expression| {
            is_non_number_literal(&expression) || is_date_construction(&expression)
        })
}

/// Detects literal initializers that are clearly not numbers.
///
/// ```js
/// const a = "text"; // true
/// const b = 10; // false
/// ```
fn is_non_number_literal(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .is_some_and(|literal| literal.as_js_number_literal_expression().is_none())
}

/// Is it a `new Date()`?
fn is_date_construction(expression: &AnyJsExpression) -> bool {
    expression
        .as_js_new_expression()
        .is_some_and(|new_expression| is_named_constructor(new_expression, "Date"))
}

/// Checks whether a call expression targets a specific bare identifier, such as `BigInt(...)`.
fn is_named_call(call: &JsCallExpression, name: &str) -> bool {
    call.callee()
        .ok()
        .map(|callee| callee.omit_parentheses())
        .and_then(|callee| callee.as_js_identifier_expression().cloned())
        .and_then(|identifier| identifier.name().ok())
        .and_then(|name_token| name_token.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == name)
}

/// Checks whether a constructor expression targets a specific bare identifier, such as `new Date(...)`.
fn is_named_constructor(new_expression: &JsNewExpression, name: &str) -> bool {
    new_expression
        .callee()
        .ok()
        .map(|callee| callee.omit_parentheses())
        .and_then(|callee| callee.as_js_identifier_expression().cloned())
        .and_then(|identifier| identifier.name().ok())
        .and_then(|name_token| name_token.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == name)
}

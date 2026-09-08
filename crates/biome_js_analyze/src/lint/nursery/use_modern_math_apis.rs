use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsName,
    JsBinaryExpression, JsBinaryOperator, JsCallExpression, T, global_identifier,
    numbers::canonicalize_js_bigint_literal, unescape_js_string,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction};
use biome_rule_options::use_modern_math_apis::UseModernMathApisOptions;

declare_lint_rule! {
    /// Use modern `Math` APIs for common mathematical operations.
    ///
    /// Dedicated `Math` methods express mathematical intent directly and avoid reimplementing standard operations.
    /// This rule recognizes logarithm conversions, sums of squares, and square roots of squared values.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Math.log(x) * Math.LOG10E;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Math.sqrt(a * a + b * b);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Math.sqrt(x ** 2);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Math.log10(x);
    /// Math.hypot(a, b);
    /// Math.abs(x);
    /// ```
    ///
    pub UseModernMathApis {
        version: "2.5.12",
        name: "useModernMathApis",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-modern-math-apis").same()],
        recommended: true,
        severity: Severity::Warning,
        // unsafe because the fix is complex. we can mark it safe later when we're confident
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseModernMathApis {
    type Query = Semantic<JsCallExpression>;
    type State = UseModernMathApisState;
    type Signals = Option<Self::State>;
    type Options = UseModernMathApisOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        match math_method_kind(call, model)? {
            LegacyMathMethod::Log => {
                let (expression, replacement) = logarithm_replacement(call, model)?;
                Some(UseModernMathApisState {
                    expression: AnyJsExpression::JsBinaryExpression(expression),
                    replacement,
                })
            }
            LegacyMathMethod::Sqrt => {
                let argument = single_expression_argument(call)?;
                let term_count = square_term_count(&argument)?;
                Some(UseModernMathApisState {
                    expression: AnyJsExpression::JsCallExpression(call.clone()),
                    replacement: if term_count == 1 {
                        ModernMathMethod::Abs
                    } else {
                        ModernMathMethod::Hypot
                    },
                })
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = match state.replacement {
            ModernMathMethod::Abs => "This expression manually computes an absolute value.",
            ModernMathMethod::Hypot => {
                "This expression manually computes the square root of a sum of squares."
            }
            ModernMathMethod::Log2 | ModernMathMethod::Log10 => {
                "This expression manually converts a natural logarithm."
            }
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.expression.range(),
                markup! { {message} },
            )
            .note(markup! {
                "The dedicated Math API expresses the operation directly."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if has_inner_comments(&state.expression) {
            return None;
        }

        let call = ctx.query();
        let replacement = match state.replacement {
            ModernMathMethod::Log2 | ModernMathMethod::Log10 => {
                replace_math_method(call, state.replacement)?
            }
            ModernMathMethod::Abs | ModernMathMethod::Hypot => {
                replace_square_root(call, state.replacement)?
            }
        };
        let replacement = replacement.trim_trivia()?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node(state.expression.clone(), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"Math."{state.replacement}"()"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct UseModernMathApisState {
    expression: AnyJsExpression,
    replacement: ModernMathMethod,
}

#[derive(Debug, Clone, Copy)]
enum LegacyMathMethod {
    Log,
    Sqrt,
}

#[derive(Debug, Clone, Copy)]
enum ModernMathMethod {
    Abs,
    Hypot,
    Log2,
    Log10,
}

impl ModernMathMethod {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Abs => "abs",
            Self::Hypot => "hypot",
            Self::Log2 => "log2",
            Self::Log10 => "log10",
        }
    }
}

impl biome_console::fmt::Display for ModernMathMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.as_str())
    }
}

fn math_method_kind(call: &JsCallExpression, model: &SemanticModel) -> Option<LegacyMathMethod> {
    if call.optional_chain_token().is_some() {
        return None;
    }

    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    if member.operator_token().ok()?.kind() != T![.] {
        return None;
    }

    let object = member.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
    if name.text() != "Math" || model.binding(&reference).is_some() {
        return None;
    }

    match member.member().ok()?.value_token().ok()?.text_trimmed() {
        "log" => Some(LegacyMathMethod::Log),
        "sqrt" => Some(LegacyMathMethod::Sqrt),
        _ => None,
    }
}

fn single_expression_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let [Some(AnyJsCallArgument::AnyJsExpression(argument)), None] =
        call.arguments().ok()?.get_arguments_by_index([0, 1])
    else {
        return None;
    };
    Some(argument)
}

/// Returns a supported logarithm conversion and the modern method that performs it directly.
///
/// Each pair shows a recognized expression followed by its replacement:
///
/// ```js
/// Math.log(value) * Math.LOG10E;
/// Math.log10(value);
///
/// Math.log(value) / Math.LN2;
/// Math.log2(value);
/// ```
fn logarithm_replacement(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Option<(JsBinaryExpression, ModernMathMethod)> {
    single_expression_argument(call)?;
    let parent = call
        .syntax()
        .ancestors()
        .skip(1)
        .filter_map(AnyJsExpression::cast)
        .find(|expression| !matches!(expression, AnyJsExpression::JsParenthesizedExpression(_)))?;
    let binary = parent.as_js_binary_expression()?;
    let left = binary.left().ok()?.omit_parentheses();
    let right = binary.right().ok()?.omit_parentheses();
    let call_expression = AnyJsExpression::JsCallExpression(call.clone());

    let replacement = match binary.operator().ok()? {
        JsBinaryOperator::Times if left == call_expression => {
            logarithm_constant(&right, model, "LOG10E", ModernMathMethod::Log10)
                .or_else(|| logarithm_constant(&right, model, "LOG2E", ModernMathMethod::Log2))?
        }
        JsBinaryOperator::Times if right == call_expression => {
            logarithm_constant(&left, model, "LOG10E", ModernMathMethod::Log10)
                .or_else(|| logarithm_constant(&left, model, "LOG2E", ModernMathMethod::Log2))?
        }
        JsBinaryOperator::Divide if left == call_expression => {
            logarithm_constant(&right, model, "LN10", ModernMathMethod::Log10)
                .or_else(|| logarithm_constant(&right, model, "LN2", ModernMathMethod::Log2))?
        }
        _ => return None,
    };

    Some((binary.clone(), replacement))
}

/// Returns `replacement` when `expression` is an unshadowed
/// `Math.<expected_name>` static member access. Returns `None` for a different
/// property, access form, or `Math` binding.
fn logarithm_constant(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    expected_name: &str,
    replacement: ModernMathMethod,
) -> Option<ModernMathMethod> {
    let member = expression.as_js_static_member_expression()?;
    if member.operator_token().ok()?.kind() != T![.]
        || member.member().ok()?.value_token().ok()?.text_trimmed() != expected_name
    {
        return None;
    }

    let object = member.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
    (name.text() == "Math" && model.binding(&reference).is_none()).then_some(replacement)
}

/// Returns the number of squared terms in an expression joined by addition.
///
/// For example, `a * a + b ** 2` contains two squared terms. Parentheses do not
/// affect the count. Returns `None` if any term is neither `base ** 2` nor
/// `base * base` with equivalent operands.
fn square_term_count(expression: &AnyJsExpression) -> Option<usize> {
    let mut expressions = vec![expression.clone()];
    let mut term_count = 0usize;

    while let Some(expression) = expressions.pop() {
        let expression = expression.omit_parentheses();
        if let AnyJsExpression::JsBinaryExpression(binary) = &expression
            && binary.operator().ok()? == JsBinaryOperator::Plus
        {
            expressions.push(binary.right().ok()?);
            expressions.push(binary.left().ok()?);
        } else {
            square_base(&expression)?;
            term_count = term_count.checked_add(1)?;
        }
    }

    Some(term_count)
}

/// Returns the base of an expression shaped as `base ** 2` or `base * base`.
/// The multiplication form requires both operands to refer to the same value.
/// Returns `None` for every other expression shape.
fn square_base(expression: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expression.clone().omit_parentheses();
    let binary = binary.as_js_binary_expression()?;
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    match binary.operator().ok()? {
        JsBinaryOperator::Exponent
            if right
                .clone()
                .omit_parentheses()
                .as_any_js_literal_expression()?
                .as_js_number_literal_expression()?
                .as_number()
                == Some(2.0) =>
        {
            Some(left)
        }
        JsBinaryOperator::Times if is_same_reference(left.clone(), right).unwrap_or(false) => {
            Some(left)
        }
        _ => None,
    }
}

fn is_same_reference(left: AnyJsExpression, right: AnyJsExpression) -> Option<bool> {
    let mut expressions = vec![(left, right)];

    while let Some((left, right)) = expressions.pop() {
        let left = unwrap_reference(left)?;
        let right = unwrap_reference(right)?;

        match (&left, &right) {
            (
                AnyJsExpression::JsIdentifierExpression(left),
                AnyJsExpression::JsIdentifierExpression(right),
            ) => {
                if left.name().ok()?.to_trimmed_text() != right.name().ok()?.to_trimmed_text() {
                    return Some(false);
                }
            }
            (AnyJsExpression::JsThisExpression(_), AnyJsExpression::JsThisExpression(_))
            | (AnyJsExpression::JsSuperExpression(_), AnyJsExpression::JsSuperExpression(_)) => {}
            (
                AnyJsExpression::AnyJsLiteralExpression(left),
                AnyJsExpression::AnyJsLiteralExpression(right),
            ) => {
                if !is_same_literal(left, right)? {
                    return Some(false);
                }
            }
            _ => {
                let left_member = AnyJsMemberExpression::cast(left.into_syntax())?;
                let right_member = AnyJsMemberExpression::cast(right.into_syntax())?;
                expressions.push((left_member.object().ok()?, right_member.object().ok()?));

                match (left_member, right_member) {
                    (
                        AnyJsMemberExpression::JsStaticMemberExpression(left),
                        AnyJsMemberExpression::JsStaticMemberExpression(right),
                    ) => {
                        if left.operator_token().ok()?.kind()
                            != right.operator_token().ok()?.kind()
                            || left.member().ok()?.value_token().ok()?.text_trimmed()
                                != right.member().ok()?.value_token().ok()?.text_trimmed()
                        {
                            return Some(false);
                        }
                    }
                    (
                        AnyJsMemberExpression::JsComputedMemberExpression(left),
                        AnyJsMemberExpression::JsComputedMemberExpression(right),
                    ) => {
                        if left.optional_chain_token().is_some()
                            != right.optional_chain_token().is_some()
                        {
                            return Some(false);
                        }
                        expressions.push((left.member().ok()?, right.member().ok()?));
                    }
                    _ => return Some(false),
                }
            }
        }
    }

    Some(true)
}

fn is_same_literal(left: &AnyJsLiteralExpression, right: &AnyJsLiteralExpression) -> Option<bool> {
    match (left, right) {
        (
            AnyJsLiteralExpression::JsBigintLiteralExpression(left),
            AnyJsLiteralExpression::JsBigintLiteralExpression(right),
        ) => Some(
            canonicalize_js_bigint_literal(left.value_token().ok()?.text_trimmed())?
                == canonicalize_js_bigint_literal(right.value_token().ok()?.text_trimmed())?,
        ),
        (
            AnyJsLiteralExpression::JsBooleanLiteralExpression(left),
            AnyJsLiteralExpression::JsBooleanLiteralExpression(right),
        ) => Some(
            left.value_token().ok()?.text_trimmed() == right.value_token().ok()?.text_trimmed(),
        ),
        (
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
        ) => Some(true),
        (
            AnyJsLiteralExpression::JsNumberLiteralExpression(left),
            AnyJsLiteralExpression::JsNumberLiteralExpression(right),
        ) => Some(left.as_number()? == right.as_number()?),
        (
            AnyJsLiteralExpression::JsRegexLiteralExpression(left),
            AnyJsLiteralExpression::JsRegexLiteralExpression(right),
        ) => Some(left.to_trimmed_text() == right.to_trimmed_text()),
        (
            AnyJsLiteralExpression::JsStringLiteralExpression(left),
            AnyJsLiteralExpression::JsStringLiteralExpression(right),
        ) => Some(
            unescape_js_string(left.inner_string_text().ok()?)
                == unescape_js_string(right.inner_string_text().ok()?),
        ),
        _ => Some(false),
    }
}

fn unwrap_reference(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    let mut expression = expression;

    loop {
        expression = match expression.omit_parentheses() {
            AnyJsExpression::TsAsExpression(expression) => expression.expression().ok()?,
            AnyJsExpression::TsSatisfiesExpression(expression) => expression.expression().ok()?,
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                expression.expression().ok()?
            }
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                expression.expression().ok()?
            }
            expression => return Some(expression),
        };
    }
}

fn replace_math_method(
    call: &JsCallExpression,
    replacement: ModernMathMethod,
) -> Option<AnyJsExpression> {
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee
        .as_js_static_member_expression()?
        .clone()
        .with_member(AnyJsName::JsName(make::js_name(make::ident(
            replacement.as_str(),
        ))));
    Some(AnyJsExpression::JsCallExpression(call.clone().with_callee(
        AnyJsExpression::JsStaticMemberExpression(member),
    )))
}

/// Rewrites a square root of squared values to the corresponding modern method.
///
/// Each pair shows a recognized expression followed by its replacement:
///
/// ```js
/// Math.sqrt(x * x);
/// Math.abs(x);
///
/// Math.sqrt(x * x + y ** 2);
/// Math.hypot(x, y);
/// ```
fn replace_square_root(
    call: &JsCallExpression,
    replacement: ModernMathMethod,
) -> Option<AnyJsExpression> {
    let argument = single_expression_argument(call)?;
    let mut bases = Vec::new();
    collect_square_bases(&argument, &mut bases)?;

    let original_arguments = call.arguments().ok()?;
    let has_trailing_comma = matches!(replacement, ModernMathMethod::Hypot)
        && original_arguments.args().trailing_separator().is_some();
    let argument_count = bases.len();
    let arguments = bases
        .into_iter()
        .map(|expression| {
            Some(AnyJsCallArgument::AnyJsExpression(
                expression
                    .trim_trivia()?
                    .with_leading_trivia_pieces([])?
                    .with_trailing_trivia_pieces([])?,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    let separator_count = argument_count.saturating_sub(1) + usize::from(has_trailing_comma);
    let separators = (0..separator_count).map(|index| {
        if has_trailing_comma && index + 1 == separator_count {
            make::token(T![,])
        } else {
            make::token_decorated_with_space(T![,])
        }
    });
    let arguments =
        original_arguments.with_args(make::js_call_argument_list(arguments, separators));

    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee
        .as_js_static_member_expression()?
        .clone()
        .with_member(AnyJsName::JsName(make::js_name(make::ident(
            replacement.as_str(),
        ))));

    Some(AnyJsExpression::JsCallExpression(
        call.clone()
            .with_callee(AnyJsExpression::JsStaticMemberExpression(member))
            .with_arguments(arguments),
    ))
}

fn collect_square_bases(
    expression: &AnyJsExpression,
    bases: &mut Vec<AnyJsExpression>,
) -> Option<()> {
    let mut expressions = vec![expression.clone()];

    while let Some(expression) = expressions.pop() {
        let expression = expression.omit_parentheses();
        if let AnyJsExpression::JsBinaryExpression(binary) = &expression
            && binary.operator().ok()? == JsBinaryOperator::Plus
        {
            expressions.push(binary.right().ok()?);
            expressions.push(binary.left().ok()?);
        } else {
            bases.push(square_base(&expression)?);
        }
    }

    Some(())
}

fn has_inner_comments(expression: &AnyJsExpression) -> bool {
    let mut tokens = expression
        .syntax()
        .descendants_tokens(Direction::Next)
        .peekable();
    let Some(first) = tokens.next() else {
        return false;
    };
    if first.has_trailing_comments() {
        return true;
    }
    while let Some(token) = tokens.next() {
        if token.has_leading_comments() {
            return true;
        }
        if tokens.peek().is_some() && token.has_trailing_comments() {
            return true;
        }
    }
    false
}

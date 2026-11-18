use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody,
    AnyJsParameter, JsBinaryOperator, JsCallExpression, JsComputedMemberExpression, JsParameters,
    JsSyntaxToken, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, declare_node_union};
use biome_rule_options::no_array_sort_for_min_max::NoArraySortForMinMaxOptions;

use crate::{
    JsRuleAction, services::semantic::Semantic,
    utils::indexed_collection::is_known_non_numeric_indexed_collection,
};

declare_lint_rule! {
    /// Disallow sorting an array only to read its smallest or largest value.
    ///
    /// Sorting puts every item of an array in order, which takes far more work than finding
    /// the smallest or largest value. `Math.min()` and `Math.max()` look at each item only once.
    /// In addition, `sort()` changes the order of the original array, which is often unintended.
    ///
    /// This rule reports reading the first item (`[0]` or `.at(0)`) or the last item (`.at(-1)`)
    /// of an array that was just sorted with `sort()` or `toSorted()`, when the comparison
    /// function only subtracts one number from the other, such as `(a, b) => a - b`.
    /// Sorting without a comparison function, or with any other comparison, is not reported.
    ///
    /// The suggested fix replaces the expression with `Math.min(...array)` or `Math.max(...array)`.
    /// The fix is unsafe because the result can differ:
    /// - an empty array gives `undefined` when sorted, but `Infinity` or `-Infinity` with `Math.min()` or `Math.max()`;
    /// - `sort()` changes the original array, and the replacement does not;
    /// - arrays containing `NaN` or empty slots can give different results;
    /// - spreading a very large array into a function call can throw an error.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const minimum = array.sort((a, b) => a - b)[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const minimum = array.toSorted((a, b) => a - b).at(0);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const maximum = array.sort((a, b) => a - b).at(-1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const maximum = array.toSorted((a, b) => b - a)[0];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const minimum = Math.min(...array);
    /// const maximum = Math.max(...array);
    /// const sorted = array.toSorted((a, b) => a - b);
    /// const first = array.toSorted((a, b) => a.value - b.value)[0];
    /// const second = array.toSorted((a, b) => a - b)[1];
    /// ```
    ///
    pub NoArraySortForMinMax {
        version: "2.6.3",
        name: "noArraySortForMinMax",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-array-sort-for-min-max").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    /// An expression that reads the first or last item of an array: `array[0]` or `array.at(index)`.
    pub AnyArrayEndAccess = JsComputedMemberExpression | JsCallExpression
}

impl Rule for NoArraySortForMinMax {
    type Query = Semantic<AnyArrayEndAccess>;
    type State = NoArraySortForMinMaxState;
    type Signals = Option<Self::State>;
    type Options = NoArraySortForMinMaxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (sorted, end) = match node {
            AnyArrayEndAccess::JsComputedMemberExpression(member) => {
                if member.is_optional() || !is_number(&member.member().ok()?, 0.0) {
                    return None;
                }
                (member.object().ok()?, ArrayEnd::First)
            }
            AnyArrayEndAccess::JsCallExpression(call) => {
                let (object, argument) = single_argument_method_call(call, "at")?;
                let end = if is_number(&argument, 0.0) {
                    ArrayEnd::First
                } else if is_negative_one(&argument) {
                    ArrayEnd::Last
                } else {
                    return None;
                };
                (object, end)
            }
        };

        if is_deleted(node) {
            return None;
        }

        let sort_call = sorted.omit_parentheses();
        let sort_call = sort_call.as_js_call_expression()?;
        let (receiver, comparator) = single_argument_method_call(sort_call, "sort")
            .or_else(|| single_argument_method_call(sort_call, "toSorted"))?;
        let direction = sort_direction(&comparator)?;

        if has_optional_chain_element(&receiver)
            || is_known_non_numeric_indexed_collection(&receiver, ctx.model())
        {
            return None;
        }

        let method = match (direction, end) {
            (SortDirection::Ascending, ArrayEnd::First)
            | (SortDirection::Descending, ArrayEnd::Last) => MathMethod::Min,
            (SortDirection::Ascending, ArrayEnd::Last)
            | (SortDirection::Descending, ArrayEnd::First) => MathMethod::Max,
        };

        Some(NoArraySortForMinMaxState { method, receiver })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let extreme = match state.method {
            MathMethod::Min => "smallest",
            MathMethod::Max => "largest",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This sorts the whole array only to read its "{extreme}" value."
                },
            )
            .note(markup! {
                "Sorting does much more work than finding the "{extreme}" value, which only needs to look at each item once."
            })
            .note(markup! {
                "Use "<Emphasis>"Math."{state.method}"()"</Emphasis>" with the array spread into its arguments instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if state.receiver.as_js_super_expression().is_some()
            || node.syntax().has_inner_comments()
            || is_math_shadowed(ctx.model(), node)
        {
            return None;
        }

        let callee = make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("Math")))
                .into(),
            make::token(T![.]),
            make::js_name(make::ident(match state.method {
                MathMethod::Min => "min",
                MathMethod::Max => "max",
            }))
            .into(),
        );
        let spread = make::js_spread(make::token(T![...]), state.receiver.clone().trim_trivia()?);
        let arguments = make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list([AnyJsCallArgument::JsSpread(spread)], []),
            make::token(T![')']),
        );
        let replacement: AnyJsExpression = make::js_call_expression(callee.into(), arguments)
            .build()
            .into();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(AnyJsExpression::cast(node.syntax().clone())?, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"Math."{state.method}"()"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

pub struct NoArraySortForMinMaxState {
    method: MathMethod,
    /// The array being sorted, which becomes the spread argument of the replacement.
    receiver: AnyJsExpression,
}

#[derive(Clone, Copy)]
enum MathMethod {
    Min,
    Max,
}

impl biome_console::fmt::Display for MathMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Min => "min",
            Self::Max => "max",
        })
    }
}

#[derive(Clone, Copy)]
enum ArrayEnd {
    First,
    Last,
}

#[derive(Clone, Copy)]
enum SortDirection {
    Ascending,
    Descending,
}

/// Returns the object and the only argument of a call like `object.name(argument)`.
///
/// Returns `None` for optional calls (`object?.name()`, `object.name?.()`), computed method names
/// (`object["name"]()`), spread arguments, and any other number of arguments.
fn single_argument_method_call(
    call: &JsCallExpression,
    name: &str,
) -> Option<(AnyJsExpression, AnyJsExpression)> {
    if call.is_optional() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    if member.is_optional()
        || member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != name
    {
        return None;
    }

    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    let argument = arguments.first()?.ok()?.as_any_js_expression()?.clone();
    Some((member.object().ok()?, argument))
}

/// Returns `true` if `expression` is a number literal equal to `value`, ignoring parentheses.
fn is_number(expression: &AnyJsExpression, value: f64) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression()?.as_number())
        .is_some_and(|number| number == value)
}

fn is_negative_one(expression: &AnyJsExpression) -> bool {
    let expression = expression.clone().omit_parentheses();
    expression.as_js_unary_expression().is_some_and(|unary| {
        unary.operator() == Ok(JsUnaryOperator::Minus)
            && unary
                .argument()
                .is_ok_and(|argument| is_number(&argument, 1.0))
    })
}

/// Returns `true` if `node` is the operand of `delete`, possibly wrapped in parentheses.
fn is_deleted(node: &AnyArrayEndAccess) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| {
            ancestor.kind() != biome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
        })
        .and_then(biome_js_syntax::JsUnaryExpression::cast)
        .is_some_and(|unary| unary.operator() == Ok(JsUnaryOperator::Delete))
}

/// Returns the sort order produced by a comparison function shaped like `(a, b) => a - b`
/// (ascending) or `(a, b) => b - a` (descending).
///
/// The function must be a non-async, non-generator function with exactly two plain parameters
/// whose body returns the subtraction directly, either as an arrow function expression body or
/// as the only statement `return a - b;`.
fn sort_direction(comparator: &AnyJsExpression) -> Option<SortDirection> {
    let (parameters, body) = match comparator.clone().omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => {
            if arrow.async_token().is_some() {
                return None;
            }
            let AnyJsArrowFunctionParameters::JsParameters(parameters) = arrow.parameters().ok()?
            else {
                return None;
            };
            (parameters, arrow.body().ok()?)
        }
        AnyJsExpression::JsFunctionExpression(function) => {
            if function.async_token().is_some() || function.star_token().is_some() {
                return None;
            }
            (
                function.parameters().ok()?,
                AnyJsFunctionBody::JsFunctionBody(function.body().ok()?),
            )
        }
        _ => return None,
    };

    let (first, second) = two_parameter_names(&parameters)?;
    if first.text_trimmed() == second.text_trimmed() {
        return None;
    }

    let returned = match body {
        AnyJsFunctionBody::AnyJsExpression(expression) => expression,
        AnyJsFunctionBody::JsFunctionBody(body) => {
            let statements = body.statements();
            if statements.len() != 1 {
                return None;
            }
            statements.first()?.as_js_return_statement()?.argument()?
        }
    };
    let returned = omit_type_wrappers(returned)?;
    let binary = returned.as_js_binary_expression()?;
    if binary.operator().ok()? != JsBinaryOperator::Minus {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;
    if is_reference_to(&left, &first) && is_reference_to(&right, &second) {
        Some(SortDirection::Ascending)
    } else if is_reference_to(&left, &second) && is_reference_to(&right, &first) {
        Some(SortDirection::Descending)
    } else {
        None
    }
}

/// Returns the name tokens of a parameter list made of exactly two identifiers without default
/// values.
fn two_parameter_names(parameters: &JsParameters) -> Option<(JsSyntaxToken, JsSyntaxToken)> {
    let items = parameters.items();
    if items.len() != 2 {
        return None;
    }
    let mut names = items.iter().map(|parameter| {
        let AnyJsParameter::AnyJsFormalParameter(parameter) = parameter.ok()? else {
            return None;
        };
        let parameter = parameter.as_js_formal_parameter()?;
        if parameter.initializer().is_some() {
            return None;
        }
        parameter
            .binding()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()
    });
    Some((names.next()??, names.next()??))
}

fn is_reference_to(expression: &AnyJsExpression, name: &JsSyntaxToken) -> bool {
    omit_type_wrappers(expression.clone())
        .as_ref()
        .and_then(AnyJsExpression::as_js_identifier_expression)
        .and_then(|identifier| identifier.name().ok()?.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == name.text_trimmed())
}

/// Removes parentheses and TypeScript-only wrappers such as `as`, `satisfies`, and `!`, which
/// don't change the value at runtime.
fn omit_type_wrappers(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    let mut expression = expression;
    loop {
        expression = match expression {
            AnyJsExpression::JsParenthesizedExpression(inner) => inner.expression().ok()?,
            AnyJsExpression::TsAsExpression(inner) => inner.expression().ok()?,
            AnyJsExpression::TsSatisfiesExpression(inner) => inner.expression().ok()?,
            AnyJsExpression::TsNonNullAssertionExpression(inner) => inner.expression().ok()?,
            AnyJsExpression::TsTypeAssertionExpression(inner) => inner.expression().ok()?,
            AnyJsExpression::TsInstantiationExpression(inner) => inner.expression().ok()?,
            expression => return Some(expression),
        };
    }
}

/// Returns `true` if any member access or call along `expression`'s chain is optional, as in
/// `object?.array` or `object?.getArray()`.
///
/// Replacing such an expression would turn a short-circuited `undefined` result into an error.
fn has_optional_chain_element(expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone();
    loop {
        let Some(unwrapped) = omit_type_wrappers(expression) else {
            return false;
        };
        let next = match unwrapped {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                if member.is_optional() {
                    return true;
                }
                member.object()
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                if member.is_optional() {
                    return true;
                }
                member.object()
            }
            AnyJsExpression::JsCallExpression(call) => {
                if call.is_optional() {
                    return true;
                }
                call.callee()
            }
            _ => return false,
        };
        let Ok(next) = next else {
            return false;
        };
        expression = next;
    }
}

/// Returns `true` if a local variable named `Math` is visible at `node`, so that a reference to
/// `Math` written there wouldn't point to the built-in object.
fn is_math_shadowed(model: &SemanticModel, node: &AnyArrayEndAccess) -> bool {
    model
        .scope(node.syntax())
        .ancestors()
        .any(|scope| scope.get_binding("Math").is_some())
}

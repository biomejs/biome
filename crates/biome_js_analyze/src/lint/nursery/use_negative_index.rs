use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsStaticMemberExpression, T,
    global_identifier,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxResult, TokenText,
};
use biome_rule_options::use_negative_index::UseNegativeIndexOptions;

use crate::{
    JsRuleAction,
    services::semantic::Semantic,
    utils::{indexed_collection::TYPED_ARRAY_CONSTRUCTORS, same_reference::is_same_reference},
};

declare_lint_rule! {
    /// Prefer negative indexes over subtracting from `.length`.
    ///
    /// Methods such as `slice()` and `at()` accept negative indexes, which count back from the end.
    /// `array.at(-1)` is easier to read than `array.at(array.length - 1)`.
    ///
    /// The rule checks `slice()`, `at()`, `subarray()`, `splice()`, and `toSpliced()`,
    /// including calls through `call()` and `apply()`, such as `Array.prototype.slice.call(array, array.length - 1)`.
    ///
    /// The fix is unsafe, because the rule can't tell whether the value is really an array or a string.
    /// Also, subtracting more than the length gives a different result:
    /// `[1, 2, 3].slice(3 - 4)` returns `[3]`, while `[1, 2, 3].slice(-4)` returns the whole array.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo.slice(foo.length - 2, foo.length - 1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.splice(foo.length - 1, 1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.at(foo.length - 1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Array.prototype.slice.call(foo, foo.length - 2, foo.length - 1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Array.prototype.slice.apply(foo, [foo.length - 2, foo.length - 1]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo.slice(-2, -1);
    /// foo.splice(-1, 1);
    /// foo.at(-1);
    /// Array.prototype.slice.call(foo, -2, -1);
    /// Array.prototype.slice.apply(foo, [-2, -1]);
    /// foo.slice(bar.length - 1);
    /// foo.with(foo.length - 1, value);
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to use `at()` instead of reading an element by its index, see [`useAtIndex`](https://biomejs.dev/linter/rules/use-at-index/).
    ///
    pub UseNegativeIndex {
        version: "2.6.10",
        name: "useNegativeIndex",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-negative-index").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

/// A call whose index arguments subtract a number from the `.length` of the value the method
/// runs on.
pub struct LengthSubtractions {
    method: Method,
    /// The subtractions in the index arguments, such as `array.length - 1`, by argument position.
    /// Methods have at most two index arguments.
    subtractions: [Option<JsBinaryExpression>; 2],
}

impl Rule for UseNegativeIndex {
    type Query = Semantic<JsCallExpression>;
    type State = LengthSubtractions;
    type Signals = Option<Self::State>;
    type Options = UseNegativeIndexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = static_callee(&call.callee().ok()?)?;
        let arguments = call.arguments().ok()?.args();
        let callee_name = member_name(&callee)?;

        let (method, receiver, index_arguments) = if let Some(method) = Method::new(&callee_name) {
            (
                method,
                callee.object().ok()?,
                leading_call_arguments(arguments.iter()),
            )
        } else {
            let method_member = static_callee(&callee.object().ok()?)?;
            let method = Method::new(&member_name(&method_member)?)?;
            if !is_method_owner(&method_member.object().ok()?, method, ctx.model()) {
                return None;
            }
            let mut arguments = arguments.iter();
            let receiver = arguments.next()?.ok()?.as_any_js_expression()?.clone();
            match callee_name.text() {
                // `Array.prototype.slice.call(array, start, end)`
                "call" => (method, receiver, leading_call_arguments(arguments)),
                // `Array.prototype.slice.apply(array, [start, end])`
                "apply" => {
                    let AnyJsExpression::JsArrayExpression(array) =
                        arguments.next()?.ok()?.as_any_js_expression()?.clone()
                    else {
                        return None;
                    };
                    (
                        method,
                        receiver,
                        leading_array_elements(array.elements().iter()),
                    )
                }
                _ => return None,
            }
        };

        let mut subtractions = [None, None];
        for (subtraction, argument) in subtractions
            .iter_mut()
            .zip(index_arguments)
            .take(method.index_argument_count())
        {
            *subtraction = argument.and_then(|argument| length_subtraction(&argument, &receiver));
        }
        subtractions
            .iter()
            .any(Option::is_some)
            .then_some(LengthSubtractions {
                method,
                subtractions,
            })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let method = state.method;
        let range = state
            .subtractions
            .iter()
            .flatten()
            .map(AstNode::range)
            .reduce(|first, last| first.cover(last))?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>{method}"()"</Emphasis>" call subtracts from "<Emphasis>".length"</Emphasis>" to count from the end."
                },
            )
            .note(markup! {
                <Emphasis>{method}"()"</Emphasis>" accepts negative indexes, which count back from the end, so the subtraction makes the position harder to read."
            })
            .note(markup! {
                "Use a negative index instead, such as "<Emphasis>"-1"</Emphasis>" for "<Emphasis>"array.length - 1"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        for subtraction in state.subtractions.iter().flatten() {
            // The fix keeps only the number, so a comment anywhere else would be lost.
            if subtraction.syntax().has_inner_comments() {
                return None;
            }
            let negative_index = make::js_unary_expression(
                make::token(T![-]),
                subtraction.right().ok()?.trim_trivia()?,
            );
            mutation.replace_node(
                AnyJsExpression::from(subtraction.clone()),
                AnyJsExpression::from(negative_index),
            );
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a negative index." }.to_owned(),
            mutation,
        ))
    }
}

/// The methods whose index arguments accept negative indexes.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Method {
    At,
    Slice,
    Splice,
    Subarray,
    ToSpliced,
}

impl Method {
    fn new(name: &str) -> Option<Self> {
        Some(match name {
            "at" => Self::At,
            "slice" => Self::Slice,
            "splice" => Self::Splice,
            "subarray" => Self::Subarray,
            "toSpliced" => Self::ToSpliced,
            _ => return None,
        })
    }

    /// Returns how many of the leading arguments are indexes.
    ///
    /// The second argument of `splice()` and `toSpliced()` is a number of elements, not an index.
    const fn index_argument_count(self) -> usize {
        match self {
            Self::Slice | Self::Subarray => 2,
            Self::At | Self::Splice | Self::ToSpliced => 1,
        }
    }

    /// Returns whether `Global.prototype` has this method, where `Global` is the name of a
    /// built-in global, such as `Array`.
    fn is_on_prototype_of(self, global: &str) -> bool {
        match global {
            "Array" => self != Self::Subarray,
            "String" => matches!(self, Self::At | Self::Slice),
            "ArrayBuffer" => self == Self::Slice,
            _ => {
                matches!(self, Self::At | Self::Slice | Self::Subarray)
                    && TYPED_ARRAY_CONSTRUCTORS.binary_search(&global).is_ok()
            }
        }
    }
}

impl biome_console::fmt::Display for Method {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::At => "at",
            Self::Slice => "slice",
            Self::Splice => "splice",
            Self::Subarray => "subarray",
            Self::ToSpliced => "toSpliced",
        })
    }
}

/// Returns `expression` as a property read with dot notation, such as `array.slice`.
///
/// Parentheses are ignored.
fn static_callee(expression: &AnyJsExpression) -> Option<JsStaticMemberExpression> {
    expression
        .clone()
        .omit_parentheses()
        .as_js_static_member_expression()
        .cloned()
}

fn member_name(member: &JsStaticMemberExpression) -> Option<TokenText> {
    Some(
        member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed(),
    )
}

/// Returns whether `object` is a value that the method can be read from to call it on
/// another value: an empty array `[]`, an empty string `''` for `slice()`, or the
/// prototype of a built-in global that has the method, such as `Array.prototype`.
fn is_method_owner(object: &AnyJsExpression, method: Method, model: &SemanticModel) -> bool {
    match object.clone().omit_parentheses() {
        AnyJsExpression::JsArrayExpression(array) => array.elements().is_empty(),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => {
            method == Method::Slice
                && string
                    .inner_string_text()
                    .is_ok_and(|text| text.text().is_empty())
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            method == Method::Slice && template.tag().is_none() && template.elements().is_empty()
        }
        AnyJsExpression::JsStaticMemberExpression(prototype) => {
            if !member_name(&prototype).is_some_and(|name| name.text() == "prototype") {
                return false;
            }
            prototype
                .object()
                .ok()
                .and_then(|object| {
                    object
                        .omit_parentheses()
                        .as_any_global_identifier_expression()
                })
                .and_then(|global| global_identifier(&global))
                .is_some_and(|(reference, name)| {
                    method.is_on_prototype_of(name.text()) && model.binding(&reference).is_none()
                })
        }
        _ => false,
    }
}

/// Returns the first two arguments, stopping at the first spread argument (`...`),
/// because the positions of the arguments after it are unknown.
fn leading_call_arguments(
    arguments: impl Iterator<Item = SyntaxResult<AnyJsCallArgument>>,
) -> [Option<AnyJsExpression>; 2] {
    let mut leading = [None, None];
    for (slot, argument) in leading.iter_mut().zip(arguments) {
        let Ok(AnyJsCallArgument::AnyJsExpression(argument)) = argument else {
            break;
        };
        *slot = Some(argument);
    }
    leading
}

/// Returns the first two array elements, stopping at the first spread element (`...`),
/// because the positions of the elements after it are unknown. Holes, as in `[, a]`, are `None`.
fn leading_array_elements(
    elements: impl Iterator<Item = SyntaxResult<AnyJsArrayElement>>,
) -> [Option<AnyJsExpression>; 2] {
    let mut leading = [None, None];
    for (slot, element) in leading.iter_mut().zip(elements) {
        match element {
            Ok(AnyJsArrayElement::AnyJsExpression(element)) => *slot = Some(element),
            Ok(AnyJsArrayElement::JsArrayHole(_)) => {}
            Ok(AnyJsArrayElement::JsSpread(_)) | Err(_) => break,
        }
    }
    leading
}

/// Returns the subtraction in `argument` that subtracts a positive whole number from the
/// `.length` of `receiver`, such as `array.length - 1` in `array.length - 1 - offset`.
///
/// Parentheses and TypeScript type assertions are ignored.
fn length_subtraction(
    argument: &AnyJsExpression,
    receiver: &AnyJsExpression,
) -> Option<JsBinaryExpression> {
    let mut expression = argument.clone().omit_type_wrappers()?;
    loop {
        let AnyJsExpression::JsBinaryExpression(subtraction) = expression else {
            return None;
        };
        if subtraction.operator().ok()? != JsBinaryOperator::Minus
            || !is_positive_integer(&subtraction.right().ok()?)
        {
            return None;
        }
        let left = subtraction.left().ok()?;
        if is_length_of(&left, receiver) {
            return Some(subtraction);
        }
        expression = left.omit_type_wrappers()?;
    }
}

/// Returns whether `expression` is a number literal for a positive whole number that a
/// JavaScript number represents exactly.
///
/// `slice(length - n)` and `slice(-n)` only agree when `n` is a whole number of positions.
fn is_positive_integer(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_type_wrappers()
        .and_then(|expression| {
            expression
                .as_any_js_literal_expression()?
                .as_js_number_literal_expression()?
                .as_number()
        })
        .is_some_and(|value| value > 0.0 && value.fract() == 0.0 && value <= MAX_SAFE_INTEGER)
}

/// The largest whole number that a JavaScript number can represent exactly.
const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

/// Returns whether `expression` reads `.length` from `receiver`, as in `array.length`.
///
/// Optional chaining (`array?.length`) and computed access (`array["length"]`) don't match.
fn is_length_of(expression: &AnyJsExpression, receiver: &AnyJsExpression) -> bool {
    let Some(AnyJsExpression::JsStaticMemberExpression(member)) =
        expression.clone().omit_type_wrappers()
    else {
        return false;
    };
    !member.is_optional()
        && member_name(&member).is_some_and(|name| name.text() == "length")
        && member
            .object()
            .ok()
            .and_then(|object| is_same_reference(object, receiver.clone()))
            .unwrap_or(false)
}

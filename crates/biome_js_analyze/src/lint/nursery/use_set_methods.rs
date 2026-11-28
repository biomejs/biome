use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression,
    AnyJsFormalParameter, AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsParameter,
    JsArrayExpression, JsBinaryExpression, JsBinaryOperator, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsComputedMemberExpression, JsExtendsClause, JsIdentifierBinding,
    JsLanguage, JsNewExpression, JsParenthesizedExpression, JsStaticMemberExpression,
    JsSyntaxToken, JsTemplateExpression, JsUnaryOperator, T, TsAsExpression,
    TsInstantiationExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
    TsTypeAssertionExpression,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, TriviaPieceKind,
    declare_node_union,
};
use biome_rule_options::use_set_methods::UseSetMethodsOptions;

use crate::{
    JsRuleAction,
    ast_utils::{
        is_in_new_callee, keyword_directly_before, may_have_side_effects,
        needs_parentheses_as_object, starts_unterminated_statement,
    },
    services::semantic::Semantic,
    utils::builtin_collection::{CollectionKind, collection_kind, constructed_collection_kind},
};

declare_lint_rule! {
    /// Prefer the methods of `Set` for combining and comparing `Set`s.
    ///
    /// A `Set` has methods that combine two `Set`s, such as `union()`, `intersection()`, and `difference()`, and methods that compare two `Set`s, such as `isSubsetOf()` and `isDisjointFrom()`.
    /// These methods say what the code does in a single call.
    /// Copying the values of a `Set` into an array, and checking each value with `has()`, is longer and harder to read, and creates an array that is thrown away right after.
    ///
    /// The rule reports the following code, where `a` and `b` are `Set`s:
    ///
    /// | Code | Replacement |
    /// | --- | --- |
    /// | `new Set([...a, ...b])` | `a.union(b)` |
    /// | `[...a].filter(value => b.has(value))` | `a.intersection(b)` |
    /// | `[...a].filter(value => !b.has(value))` | `a.difference(b)` |
    /// | `[...a].every(value => b.has(value))` | `a.isSubsetOf(b)` |
    /// | `[...a].every(value => !b.has(value))` | `a.isDisjointFrom(b)` |
    /// | `[...a].some(value => b.has(value))` | `!a.isDisjointFrom(b)` |
    /// | `[...a].some(value => !b.has(value))` | `!a.isSubsetOf(b)` |
    /// | `a.intersection(b).size === 0` | `a.isDisjointFrom(b)` |
    /// | `a.difference(b).size === 0` | `a.isSubsetOf(b)` |
    ///
    /// The rule also reports a `filter()` call wrapped in `new Set()`, such as `new Set([...a].filter(value => b.has(value)))`.
    /// It reports the size checks written with `!==`, with the `0` on the left, and as `size > 0` or `0 < size`; these are replaced with the negated method, such as `!a.isDisjointFrom(b)`.
    ///
    /// The rule only reports code when it can tell that the values are `Set`s. This is the case when a value is:
    ///
    /// - created with `new Set()`;
    /// - a variable or a function parameter with the type `Set` or `ReadonlySet`;
    /// - a `const` variable that is initialized with a `Set`;
    /// - marked as a `Set` or a `ReadonlySet` with `as` or `satisfies`;
    /// - a conditional expression (`condition ? a : b`) whose two results are both a `Set`.
    ///
    /// The function passed to `filter()`, `every()`, or `some()` must be an arrow function with a single parameter, that only returns `b.has(value)` or `!b.has(value)`.
    /// The `Set` that `has()` is called on can't be a function call or another operation that can change a value, because the function evaluates it once per value, while the replacement evaluates it only once.
    /// For the same reason, the `Set`s spread into `new Set([...a, ...b])` can't contain such operations.
    ///
    /// A `filter()` call that isn't wrapped in `new Set()` is ignored when its result is used right away, such as in `[...a].filter(value => b.has(value)).map(format)` or `format([...a].filter(value => b.has(value)))`, because the replacement returns a `Set` instead of an array.
    ///
    /// :::caution
    /// The fix is unsafe for the following reasons:
    ///
    /// - The rule trusts type annotations and `as` expressions without checking them. If a value isn't a `Set` when the code runs, the fixed code can throw an error.
    /// - `filter()` returns an array, while `intersection()` and `difference()` return a `Set`. Code that uses the result as an array, for example by reading its `length`, stops working.
    /// - `intersection()` can return the values in a different order than `filter()`.
    /// - These methods were added to JavaScript in 2024. Make sure that every environment that runs your code supports them.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = new Set([1, 2]);
    /// const b = new Set([2, 3]);
    /// const all = new Set([...a, ...b]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = new Set([1, 2]);
    /// const b = new Set([2, 3]);
    /// const shared = [...a].filter((value) => b.has(value));
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function onlyInFirst(a: Set<number>, b: Set<number>) {
    ///     return new Set([...a].filter((value) => !b.has(value)));
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function hasAll(a: Set<string>, b: ReadonlySet<string>) {
    ///     return [...a].every((value) => b.has(value));
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function hasNothingInCommon(a: Set<string>, b: Set<string>) {
    ///     return a.intersection(b).size === 0;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function compare(a: Set<string>, b: Set<string>) {
    ///     const all = a.union(b);
    ///     const shared = a.intersection(b);
    ///     const hasAll = a.isSubsetOf(b);
    ///     const hasNothingInCommon = a.isDisjointFrom(b);
    /// }
    ///
    /// // The rule can't tell whether `first` and `second` are `Set`s.
    /// const combined = new Set([...first, ...second]);
    ///
    /// // The function doesn't only check whether a `Set` has the value.
    /// const large = [...new Set(numbers)].filter((value) => value > 10);
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow copying a value into a new `Set` right before or after calling one of these methods, see [`noUselessSetConstruction`](https://biomejs.dev/linter/rules/no-useless-set-construction/).
    ///
    pub UseSetMethods {
        version: "2.6.10",
        name: "useSetMethods",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-set-methods").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyUseSetMethodsQuery = JsNewExpression | JsCallExpression | JsBinaryExpression
}

/// Code that a call to a `Set` method, or its negation, can replace.
pub struct SetOperation {
    method: SetMethod,
    /// Whether the replacement is negated, as in `!set.isDisjointFrom(other)`.
    negated: bool,
    operands: SetOperands,
}

/// The `Set`s that the replacement combines or compares.
pub enum SetOperands {
    /// The array in `new Set([...a, ...b])`, whose elements spread the `Set`s.
    Spread(JsArrayExpression),
    /// The `Set`s in `set.method(other)`.
    Pair {
        set: AnyJsExpression,
        other: AnyJsExpression,
    },
}

impl Rule for UseSetMethods {
    type Query = Semantic<AnyUseSetMethodsQuery>;
    type State = SetOperation;
    type Signals = Option<Self::State>;
    type Options = UseSetMethodsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            AnyUseSetMethodsQuery::JsNewExpression(new) => {
                if constructed_collection_kind(new, model)? != CollectionKind::Set {
                    return None;
                }
                let argument = single_argument(&new.arguments()?)?;
                if let AnyJsExpression::JsArrayExpression(array) = argument.clone().omit_parentheses()
                    && is_union_of_sets(&array, model)
                {
                    return Some(SetOperation {
                        method: SetMethod::Union,
                        negated: false,
                        operands: SetOperands::Spread(array),
                    });
                }
                filtered_set(argument.omit_type_wrappers()?.as_js_call_expression()?, model)
            }
            AnyUseSetMethodsQuery::JsCallExpression(call) => {
                if let Some(operation) = set_predicate(call, model) {
                    return Some(operation);
                }
                if is_result_used_as_array(call) {
                    return None;
                }
                filtered_set(call, model)
            }
            AnyUseSetMethodsQuery::JsBinaryExpression(binary) => size_comparison(binary, model),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let method = state.method;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            match (method, state.negated) {
                (SetMethod::Union, _) => markup! {
                    "This code spreads "<Emphasis>"Set"</Emphasis>"s into an array to combine them into a new "<Emphasis>"Set"</Emphasis>"."
                },
                (SetMethod::Intersection, _) => markup! {
                    "This code filters the values of a "<Emphasis>"Set"</Emphasis>" to keep the ones that another "<Emphasis>"Set"</Emphasis>" has."
                },
                (SetMethod::Difference, _) => markup! {
                    "This code filters the values of a "<Emphasis>"Set"</Emphasis>" to remove the ones that another "<Emphasis>"Set"</Emphasis>" has."
                },
                (SetMethod::IsSubsetOf, false) => markup! {
                    "This code checks whether every value of a "<Emphasis>"Set"</Emphasis>" is also in another "<Emphasis>"Set"</Emphasis>"."
                },
                (SetMethod::IsSubsetOf, true) => markup! {
                    "This code checks whether a "<Emphasis>"Set"</Emphasis>" has a value that another "<Emphasis>"Set"</Emphasis>" doesn't have."
                },
                (SetMethod::IsDisjointFrom, false) => markup! {
                    "This code checks whether two "<Emphasis>"Set"</Emphasis>"s have no values in common."
                },
                (SetMethod::IsDisjointFrom, true) => markup! {
                    "This code checks whether two "<Emphasis>"Set"</Emphasis>"s have a value in common."
                },
            },
        )
        .note(markup! {
            "Use the "<Emphasis>{method}</Emphasis>" method instead. It does the same work in a single call, without creating a temporary array or "<Emphasis>"Set"</Emphasis>"."
        });
        let is_bare_filter = matches!(node, AnyUseSetMethodsQuery::JsCallExpression(_))
            && matches!(method, SetMethod::Intersection | SetMethod::Difference);
        Some(if is_bare_filter {
            diagnostic.note(markup! {
                "Unlike "<Emphasis>"filter()"</Emphasis>", this method returns a "<Emphasis>"Set"</Emphasis>" instead of an array."
            })
        } else {
            diagnostic
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let query = ctx.query();
        let node = match query {
            AnyUseSetMethodsQuery::JsNewExpression(new) => AnyJsExpression::from(new.clone()),
            AnyUseSetMethodsQuery::JsCallExpression(call) => call.clone().into(),
            AnyUseSetMethodsQuery::JsBinaryExpression(binary) => binary.clone().into(),
        };

        let replacement = match &state.operands {
            SetOperands::Spread(array) => {
                let operands = array
                    .elements()
                    .iter()
                    .map(|element| match element.ok()? {
                        AnyJsArrayElement::JsSpread(spread) => spread.argument().ok(),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                if has_comments_outside(&node, &operands) {
                    return None;
                }
                let (first, rest) = operands.split_first()?;
                let mut union = receiver(first)?;
                for operand in rest {
                    union = method_call(union, state.method, operand.clone().trim_trivia()?).into();
                }
                union
            }
            SetOperands::Pair { set, other } => {
                if has_comments_outside(&node, &[set.clone(), other.clone()]) {
                    return None;
                }
                method_call(receiver(set)?, state.method, other.clone().trim_trivia()?).into()
            }
        };

        let replacement: AnyJsExpression = if state.negated {
            let negation = make::js_unary_expression(make::token(T![!]), replacement);
            if needs_parentheses_as_unary(&node) {
                make::parenthesized(negation).into()
            } else {
                negation.into()
            }
        } else if let AnyUseSetMethodsQuery::JsNewExpression(new) = query
            && is_in_new_callee(new)
        {
            make::parenthesized(replacement).into()
        } else {
            replacement
        };

        let first_token_kind = replacement.syntax().first_token()?.kind();
        if first_token_kind == T!['('] && starts_unterminated_statement(&node) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        // In `return[...a].filter(…)`, the keyword is directly followed by `[`. A replacement
        // that starts with a name, such as `a`, must not be joined to the keyword.
        if !first_token_kind.is_punct()
            && let Some(keyword) = keyword_directly_before(&node)
        {
            mutation.replace_token_discard_trivia(
                keyword.clone(),
                keyword.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            );
        }
        mutation.replace_node(node, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{state.method}</Emphasis>" instead." },
            mutation,
        ))
    }
}

/// The `Set` method that replaces the reported code.
#[derive(Clone, Copy)]
pub enum SetMethod {
    Union,
    Intersection,
    Difference,
    IsSubsetOf,
    IsDisjointFrom,
}

impl SetMethod {
    const fn name(self) -> &'static str {
        match self {
            Self::Union => "union",
            Self::Intersection => "intersection",
            Self::Difference => "difference",
            Self::IsSubsetOf => "isSubsetOf",
            Self::IsDisjointFrom => "isDisjointFrom",
        }
    }
}

impl biome_console::fmt::Display for SetMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Union => "union()",
            Self::Intersection => "intersection()",
            Self::Difference => "difference()",
            Self::IsSubsetOf => "isSubsetOf()",
            Self::IsDisjointFrom => "isDisjointFrom()",
        })
    }
}

/// A call to an array method on a copy of a single `Set`, such as `[...set].filter(callback)`.
struct ArrayMethodCall {
    /// The name of the array method, such as `filter`.
    method: JsSyntaxToken,
    set: AnyJsExpression,
    callback: AnyJsExpression,
}

impl ArrayMethodCall {
    /// Returns `None` for optional calls such as `[...set]?.filter(callback)`, computed
    /// methods such as `[...set]["filter"](callback)`, calls with more or less than one
    /// argument, and arrays that contain more than the spread `Set`.
    fn from_call(call: &JsCallExpression, model: &SemanticModel) -> Option<Self> {
        if call.is_optional() {
            return None;
        }
        let callee = call.callee().ok()?.omit_parentheses();
        let callee = callee.as_js_static_member_expression()?;
        if callee.is_optional() {
            return None;
        }
        let method = callee.member().ok()?.as_js_name()?.value_token().ok()?;
        let array = callee.object().ok()?.omit_parentheses();
        let mut elements = array.as_js_array_expression()?.elements().iter();
        let AnyJsArrayElement::JsSpread(spread) = elements.next()?.ok()? else {
            return None;
        };
        if elements.next().is_some() {
            return None;
        }
        let set = spread.argument().ok()?;
        if collection_kind(&set, model)? != CollectionKind::Set {
            return None;
        }
        Some(Self {
            method,
            set,
            callback: single_argument(&call.arguments().ok()?)?,
        })
    }
}

/// Returns `true` if `array` has two or more elements, and every element spreads a
/// `Set` that can be evaluated without side effects, or an empty `new Set()`, as in
/// `[...a, ...b]`.
///
/// `new Set([...a, ...b])` reads the values of `a` before it evaluates `b`, while
/// `a.union(b)` evaluates `b` first, so evaluating `b` must not change `a`.
fn is_union_of_sets(array: &JsArrayExpression, model: &SemanticModel) -> bool {
    let elements = array.elements();
    elements.len() >= 2
        && elements.iter().all(|element| {
            let Ok(AnyJsArrayElement::JsSpread(spread)) = element else {
                return false;
            };
            spread.argument().is_ok_and(|set| {
                is_empty_set_construction(&set, model)
                    || (collection_kind(&set, model) == Some(CollectionKind::Set)
                        && !may_have_side_effects(&set))
            })
        })
}

/// Returns `true` if `expression` is `new Set()` without arguments, which creates an empty
/// `Set` without running other code.
fn is_empty_set_construction(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let AnyJsExpression::JsNewExpression(new) = expression.clone().omit_parentheses() else {
        return false;
    };
    constructed_collection_kind(&new, model) == Some(CollectionKind::Set)
        && new
            .arguments()
            .is_none_or(|arguments| arguments.args().is_empty())
}

/// Returns the operation for `[...set].filter(value => other.has(value))` or
/// `[...set].filter(value => !other.has(value))`.
fn filtered_set(call: &JsCallExpression, model: &SemanticModel) -> Option<SetOperation> {
    let call = ArrayMethodCall::from_call(call, model)?;
    if call.method.text_trimmed() != "filter" {
        return None;
    }
    let (other, has_negation) = membership_check(&call.callback, model)?;
    Some(SetOperation {
        method: if has_negation {
            SetMethod::Difference
        } else {
            SetMethod::Intersection
        },
        negated: false,
        operands: SetOperands::Pair {
            set: call.set,
            other,
        },
    })
}

/// Returns the operation for `[...set].every(callback)` or `[...set].some(callback)`, where
/// `callback` checks whether `other` has, or doesn't have, a value.
fn set_predicate(call: &JsCallExpression, model: &SemanticModel) -> Option<SetOperation> {
    let call = ArrayMethodCall::from_call(call, model)?;
    let is_some = match call.method.text_trimmed() {
        "every" => false,
        "some" => true,
        _ => return None,
    };
    let (other, has_negation) = membership_check(&call.callback, model)?;
    // `some(value => other.has(value))` is the negation of `every(value => !other.has(value))`,
    // and `some(value => !other.has(value))` is the negation of `every(value => other.has(value))`.
    Some(SetOperation {
        method: if has_negation == is_some {
            SetMethod::IsSubsetOf
        } else {
            SetMethod::IsDisjointFrom
        },
        negated: is_some,
        operands: SetOperands::Pair {
            set: call.set,
            other,
        },
    })
}

/// Returns the operation for a check of whether `set.intersection(other)` or
/// `set.difference(other)` is empty, such as `set.intersection(other).size === 0`.
fn size_comparison(binary: &JsBinaryExpression, model: &SemanticModel) -> Option<SetOperation> {
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;
    let operator = binary.operator().ok()?;
    let (size, negated) = match operator {
        JsBinaryOperator::StrictEquality | JsBinaryOperator::StrictInequality => {
            let negated = operator == JsBinaryOperator::StrictInequality;
            if is_zero(&right) {
                (left, negated)
            } else if is_zero(&left) {
                (right, negated)
            } else {
                return None;
            }
        }
        JsBinaryOperator::GreaterThan if is_zero(&right) => (left, true),
        JsBinaryOperator::LessThan if is_zero(&left) => (right, true),
        _ => return None,
    };

    let size = size.omit_parentheses();
    let size = size.as_js_static_member_expression()?;
    if size.is_optional() || size.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed() != "size" {
        return None;
    }
    let call = size.object().ok()?.omit_parentheses();
    let call = call.as_js_call_expression()?;
    if call.is_optional() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee.is_optional() {
        return None;
    }
    let method = match callee.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed() {
        "intersection" => SetMethod::IsDisjointFrom,
        "difference" => SetMethod::IsSubsetOf,
        _ => return None,
    };
    let set = callee.object().ok()?;
    let other = single_argument(&call.arguments().ok()?)?;
    if collection_kind(&set, model)? != CollectionKind::Set
        || collection_kind(&other, model)? != CollectionKind::Set
    {
        return None;
    }
    Some(SetOperation {
        method,
        negated,
        operands: SetOperands::Pair { set, other },
    })
}

/// Returns the `Set` that `callback` checks a value against, and whether the check is
/// negated, if `callback` is `value => other.has(value)` or `value => !other.has(value)`.
///
/// The callback must be a synchronous arrow function with a single parameter that is only
/// used as the argument of `has()`. `other` must be a `Set` that can be evaluated without
/// side effects, because the callback evaluates it once per value.
fn membership_check(
    callback: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<(AnyJsExpression, bool)> {
    let callback = callback.clone().omit_parentheses();
    let arrow = callback.as_js_arrow_function_expression()?;
    if arrow.async_token().is_some() {
        return None;
    }
    let parameter = single_identifier_parameter(arrow.parameters().ok()?)?;
    let body = arrow
        .body()
        .ok()?
        .as_any_js_expression()?
        .clone()
        .omit_parentheses();
    let (has_call, negated) = match body {
        AnyJsExpression::JsUnaryExpression(unary)
            if unary.operator().ok()? == JsUnaryOperator::LogicalNot =>
        {
            (unary.argument().ok()?.omit_parentheses(), true)
        }
        body => (body, false),
    };

    let has_call = has_call.as_js_call_expression()?;
    if has_call.is_optional() {
        return None;
    }
    let callee = has_call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee.is_optional()
        || callee.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed() != "has"
    {
        return None;
    }
    let argument = single_argument(&has_call.arguments().ok()?)?.omit_parentheses();
    let argument = argument.as_js_identifier_expression()?.name().ok()?;
    let mut references = model.as_binding(&parameter).all_references();
    let reference = references.next()?;
    if references.next().is_some() || reference.syntax() != *argument.syntax() {
        return None;
    }

    let other = callee.object().ok()?;
    (collection_kind(&other, model) == Some(CollectionKind::Set) && !may_have_side_effects(&other))
        .then_some((other, negated))
}

/// Returns the parameter of an arrow function that has a single parameter, if the parameter
/// is a name without a default value, as in `value => …` or `(value: string) => …`.
fn single_identifier_parameter(
    parameters: AnyJsArrowFunctionParameters,
) -> Option<JsIdentifierBinding> {
    let binding = match parameters {
        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => binding,
        AnyJsArrowFunctionParameters::JsParameters(parameters) => {
            let mut items = parameters.items().iter();
            let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                parameter,
            )) = items.next()?.ok()?
            else {
                return None;
            };
            if items.next().is_some() || parameter.initializer().is_some() {
                return None;
            }
            parameter.binding().ok()?.as_any_js_binding()?.clone()
        }
    };
    binding.as_js_identifier_binding().cloned()
}

/// Returns `true` if the result of a `filter()` call is used right away as an array: the
/// call is the object of a member expression, the callee or an argument of another call,
/// or the operand of `as`, `satisfies`, or another TypeScript syntax that only affects types.
///
/// The `filter()` call in `new Set([...set].filter(callback))` is an argument too, and is
/// reported through the `new Set()` instead.
fn is_result_used_as_array(call: &JsCallExpression) -> bool {
    let expression = AnyJsExpression::from(call.clone());
    let has_type_wrapper = expression
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
        .is_some_and(|parent| TYPE_WRAPPER_KINDS.matches(parent.kind()));
    if has_type_wrapper {
        return true;
    }
    let Some(outer) = expression.outer_expression() else {
        return false;
    };
    let Some(parent) = outer.syntax().parent() else {
        return false;
    };
    if JsCallArgumentList::can_cast(parent.kind()) {
        return true;
    }
    if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
        return member
            .object()
            .is_ok_and(|object| object.syntax() == outer.syntax());
    }
    match AnyJsExpression::cast(parent) {
        Some(AnyJsExpression::JsCallExpression(call)) => call
            .callee()
            .is_ok_and(|callee| callee.syntax() == outer.syntax()),
        Some(AnyJsExpression::JsNewExpression(new)) => new
            .callee()
            .is_ok_and(|callee| callee.syntax() == outer.syntax()),
        _ => false,
    }
}

/// Returns the only argument of a call, or `None` if the call has a spread argument or
/// more or less than one argument.
fn single_argument(arguments: &JsCallArguments) -> Option<AnyJsExpression> {
    let mut arguments = arguments.args().iter();
    let argument = arguments.next()?.ok()?;
    if arguments.next().is_some() {
        return None;
    }
    argument.as_any_js_expression().cloned()
}

/// Returns `true` if `expression` is the number `0`.
fn is_zero(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(number))
            if number.as_number() == Some(0.0)
    )
}

/// Returns `set` without its leading and trailing trivia, wrapped in parentheses if a
/// method can't be called on it directly.
fn receiver(set: &AnyJsExpression) -> Option<AnyJsExpression> {
    let set = set.clone().trim_trivia()?;
    Some(if needs_parentheses_as_object(&set) {
        make::parenthesized(set).into()
    } else {
        set
    })
}

/// Returns the call `object.method(argument)`.
fn method_call(
    object: AnyJsExpression,
    method: SetMethod,
    argument: AnyJsExpression,
) -> JsCallExpression {
    make::js_call_expression(
        make::js_static_member_expression(
            object,
            make::token(T![.]),
            make::js_name(make::ident(method.name())).into(),
        )
        .into(),
        make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(argument)], []),
            make::token(T![')']),
        ),
    )
    .build()
}

/// Returns `true` if replacing `node` with a negation, such as `!a.isSubsetOf(b)`, needs
/// parentheses to keep the negation applied to the whole replacement.
///
/// This is the case when `node` is the object of a member expression, the callee of a call,
/// the left operand of `**`, or in another place where only a call or a name can be written
/// without parentheses.
fn needs_parentheses_as_unary(node: &AnyJsExpression) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    // In all of these parents, the place that needs parentheses is the first child node.
    let is_first_child = parent
        .first_child()
        .is_some_and(|child| child == *node.syntax());
    if let Some(binary) = JsBinaryExpression::cast_ref(&parent) {
        return is_first_child && binary.operator() == Ok(JsBinaryOperator::Exponent);
    }
    is_first_child && UNARY_PARENTHESES_PARENT_KINDS.matches(parent.kind())
}

/// Returns `true` if a comment inside `node` is outside all of the `kept` expressions, so
/// the fix would remove it.
fn has_comments_outside(node: &AnyJsExpression, kept: &[AnyJsExpression]) -> bool {
    let node_range = node.syntax().text_trimmed_range();
    node.syntax().borrowed_tokens().any(|token| {
        token
            .leading_trivia_pieces()
            .chain(token.trailing_trivia_pieces())
            .any(|piece| {
                piece.kind().is_comment()
                    && node_range.contains_range(piece.text_range())
                    && !kept.iter().any(|expression| {
                        expression
                            .syntax()
                            .text_trimmed_range()
                            .contains_range(piece.text_range())
                    })
            })
    })
}

/// The TypeScript expressions that only affect types, such as `value as Type`.
const TYPE_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = TsAsExpression::KIND_SET
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsTypeAssertionExpression::KIND_SET)
    .union(TsInstantiationExpression::KIND_SET);

/// The parents in which a negation needs parentheses when it is their first child node,
/// such as the object of `a.b`, the callee of `a()` and `new a()`, and the tag of
/// `` a`text` ``.
const UNARY_PARENTHESES_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsStaticMemberExpression::KIND_SET
    .union(JsComputedMemberExpression::KIND_SET)
    .union(JsCallExpression::KIND_SET)
    .union(JsNewExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsInstantiationExpression::KIND_SET)
    .union(JsExtendsClause::KIND_SET);

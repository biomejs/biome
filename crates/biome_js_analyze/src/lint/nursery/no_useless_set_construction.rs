use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, JsCallArgumentList, JsCallArguments, JsCallExpression, JsNewExpression,
    JsParenthesizedExpression, JsStaticMemberExpression, JsSyntaxNode, JsSyntaxToken, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_useless_set_construction::NoUselessSetConstructionOptions;

use crate::{
    JsRuleAction,
    ast_utils::{
        is_in_new_callee, may_have_side_effects, needs_parentheses_as_object,
        starts_unterminated_statement,
    },
    services::semantic::Semantic,
    utils::builtin_collection::{CollectionKind, collection_kind, constructed_collection_kind},
};

declare_lint_rule! {
    /// Disallow creating a `Set` that a `Set` method such as `union()` doesn't need.
    ///
    /// The `Set` methods `union()`, `intersection()`, `difference()`, `symmetricDifference()`, `isSubsetOf()`, `isSupersetOf()`, and `isDisjointFrom()` combine or compare two collections without changing either of them.
    /// Copying a value into a new `Set` right before or right after calling one of these methods does extra work, and creates a `Set` that is thrown away right after.
    ///
    /// The rule reports three kinds of unnecessary copies:
    ///
    /// - A copy of the argument, such as `a.union(new Set(b))`.
    ///   These methods read a `Set` directly, so `a.union(b)` gives the same result.
    ///   They also read a `Map` as the collection of its keys, so `a.union(new Set(map.keys()))` can be written as `a.union(map)`.
    /// - A copy of the `Set` that the method is called on, such as `new Set(a).union(b)`.
    ///   The method doesn't change `a`, so `a.union(b)` gives the same result.
    /// - A copy of the result, such as `new Set(a.union(b))`.
    ///   `union()`, `intersection()`, `difference()`, and `symmetricDifference()` already return a new `Set`.
    ///
    /// The rule only reports code when it can tell that the values are a `Set` or a `Map`. This is the case when a value is:
    ///
    /// - created with `new Set()` or `new Map()`;
    /// - a variable or a function parameter with the type `Set`, `ReadonlySet`, `Map`, or `ReadonlyMap`;
    /// - a `const` variable that is initialized with a `Set` or a `Map`, and that doesn't have one of these types;
    /// - marked as one of these types with `as` or `satisfies`;
    /// - a conditional expression (`condition ? a : b`) whose two results are both a `Set`, or both a `Map`.
    ///
    /// When a `const` variable, or a value marked with `as` or `satisfies`, has another type, such as `Iterable<string>`, the rule looks at the value itself.
    /// For example, `const values: Iterable<string> = new Set()` is a `Set`.
    ///
    /// A copy of the `Set` that the method is called on is kept when the argument contains a function call, an assignment, or another operation that can change the original `Set`.
    /// For example, `new Set(a).union(update(a))` reads the values that `a` had before `update(a)` ran.
    /// Reading a property and using a tagged template, such as ``tag`text` ``, don't count as such operations, although both can run code.
    ///
    /// The rule ignores a `new Set()` with explicit type arguments, such as `new Set<string>(values)`, because removing it can change the types of the code.
    /// For the same reason, it ignores a copy of the argument, and a copy of the `Set` that the method is called on, when `as` or `satisfies` is applied to the `new Set()`, as in `a.union(new Set(b) as Set<string>)`.
    ///
    /// :::caution
    /// The fix is unsafe because the rule trusts type annotations and `as` expressions without checking them.
    /// If a value isn't a `Set` or a `Map` when the code runs, the fixed code throws an error.
    /// This happens for an array that was cast with `as Set<string>`, and for an optional parameter, such as `values?: Set<string>`, that is `undefined`.
    /// The fixed code can also fail type checking when a value has a type that the method doesn't accept, such as `Iterable<string>`.
    /// The fixed code can also behave differently if a class that extends `Set` or `Map` replaces the methods that the copy reads, or if a property or a tagged template in the argument of the method runs code that changes the original `Set`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// selected.union(new Set(other));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const records = new Map([[2, "value"]]);
    /// selected.intersection(new Set(records.keys()));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// new Set(selected).isSubsetOf(other);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// const combined = new Set(selected.union(other));
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function overlap(selected: Set<number>, other: ReadonlySet<number>) {
    ///     return selected.intersection(new Set(other));
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// const records = new Map([[2, "value"]]);
    ///
    /// selected.union(other);
    /// selected.intersection(records);
    /// selected.isSubsetOf(other);
    /// const combined = selected.union(other);
    ///
    /// // A copy that isn't used with one of these methods is allowed.
    /// const copy = new Set(selected);
    ///
    /// // The values of a `Map` are different from its keys.
    /// selected.intersection(new Set(records.values()));
    ///
    /// // The rule can't tell whether `items` is a `Set`.
    /// selected.union(new Set(items));
    ///
    /// // `isSubsetOf()` returns a boolean, not a `Set`.
    /// const flags = new Set(selected.isSubsetOf(other));
    /// ```
    ///
    pub NoUselessSetConstruction {
        version: "2.6.9",
        name: "noUselessSetConstruction",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-set-construction").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because the rule can't prove that a value with a `Set` or `Map` type
        // annotation is one of the built-in collections when the code runs, and because
        // a value with another type, such as `Iterable`, may not be accepted by the method.
        fix_kind: FixKind::Unsafe,
    }
}

/// A `new Set()` that only copies a value used with one of the [`SET_METHODS`].
pub struct UselessSetConstruction {
    position: SetPosition,
    /// The name of the `Set` method, such as `union`.
    method: JsSyntaxToken,
    /// The expression that can replace the `new Set()`.
    replacement: AnyJsExpression,
}

impl Rule for NoUselessSetConstruction {
    type Query = Semantic<JsNewExpression>;
    type State = UselessSetConstruction;
    type Signals = Option<Self::State>;
    type Options = NoUselessSetConstructionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        if node.type_arguments().is_some()
            || constructed_collection_kind(node, model)? != CollectionKind::Set
        {
            return None;
        }
        let argument = single_argument(&node.arguments()?)?;

        if let AnyJsExpression::JsCallExpression(call) = argument.clone().omit_parentheses()
            && let Some(call) = SetMethodCall::from_call(&call)
            && call.returns_set()
            && collection_kind(&call.receiver, model) == Some(CollectionKind::Set)
        {
            return Some(UselessSetConstruction {
                position: SetPosition::Result,
                method: call.method,
                replacement: argument,
            });
        }

        let parent = parent_outside_parentheses(&AnyJsExpression::from(node.clone()))?;
        if let Some(arguments) = JsCallArgumentList::cast_ref(&parent) {
            let call = arguments
                .parent::<JsCallArguments>()?
                .parent::<JsCallExpression>()?;
            let call = SetMethodCall::from_call(&call)?;
            if collection_kind(&call.receiver, model)? != CollectionKind::Set {
                return None;
            }
            let (replacement, _) = copied_collection(&argument, model)?;
            return Some(UselessSetConstruction {
                position: SetPosition::Argument,
                method: call.method,
                replacement,
            });
        }

        // The object of a member expression is its only expression child, and so is the
        // callee of a call.
        let member = JsStaticMemberExpression::cast(parent)?;
        let call = JsCallExpression::cast(parent_outside_parentheses(&member.into())?)?;
        let call = SetMethodCall::from_call(&call)?;
        let (replacement, CollectionKind::Set) = copied_collection(&argument, model)? else {
            return None;
        };
        // The copy is made before the argument is evaluated, so it is only useless if the
        // argument can't change the original `Set`, neither while it is evaluated nor
        // while the method reads it.
        if !matches!(
            collection_kind(&call.argument, model),
            Some(CollectionKind::Set | CollectionKind::Map)
        ) || may_have_side_effects(&call.argument)
        {
            return None;
        }
        Some(UselessSetConstruction {
            position: SetPosition::Receiver,
            method: call.method,
            replacement,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"Set"</Emphasis>" copy isn't needed to use "<Emphasis>{state.method.text_trimmed()}"()"</Emphasis>"."
                },
            )
            .note(match state.position {
                SetPosition::Argument => markup! {
                    "The method reads a "<Emphasis>"Set"</Emphasis>", or the keys of a "<Emphasis>"Map"</Emphasis>", directly and doesn't change them. Creating another "<Emphasis>"Set"</Emphasis>" only takes extra time and memory."
                },
                SetPosition::Receiver => markup! {
                    "The method doesn't change the "<Emphasis>"Set"</Emphasis>" it is called on. Copying the "<Emphasis>"Set"</Emphasis>" first only takes extra time and memory."
                },
                SetPosition::Result => markup! {
                    "The method already returns a new "<Emphasis>"Set"</Emphasis>". Copying it only takes extra time and memory."
                },
            })
            .note(match state.position {
                SetPosition::Argument => markup! {
                    "Pass the original "<Emphasis>"Set"</Emphasis>" or "<Emphasis>"Map"</Emphasis>" to the method instead."
                },
                SetPosition::Receiver => markup! {
                    "Call the method on the original "<Emphasis>"Set"</Emphasis>" instead."
                },
                SetPosition::Result => markup! {
                    "Use the "<Emphasis>"Set"</Emphasis>" that the method returns instead."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if has_comments_outside(node, &state.replacement) {
            return None;
        }

        let replacement = state.replacement.clone().trim_trivia()?;
        let needs_parentheses = match state.position {
            SetPosition::Argument => false,
            SetPosition::Receiver => needs_parentheses_as_object(&replacement),
            SetPosition::Result => {
                !matches!(replacement, AnyJsExpression::JsParenthesizedExpression(_))
                    && is_in_new_callee(node)
            }
        };
        let replacement = if needs_parentheses {
            make::parenthesized(replacement).into()
        } else {
            replacement
        };

        let node = AnyJsExpression::from(node.clone());
        let starts_with_parenthesis = replacement
            .syntax()
            .first_token()
            .is_some_and(|token| token.kind() == T!['(']);
        if starts_with_parenthesis && starts_unterminated_statement(&node) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(node, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary copy." },
            mutation,
        ))
    }
}

/// Where a useless `new Set()` is, relative to the call of the `Set` method.
#[derive(Clone, Copy)]
enum SetPosition {
    /// The argument of the method, as in `a.union(new Set(b))`.
    Argument,
    /// The object the method is called on, as in `new Set(a).union(b)`.
    Receiver,
    /// A copy of the value the method returns, as in `new Set(a.union(b))`.
    Result,
}

/// The `Set` methods that take another collection and don't change the `Set` they are
/// called on, sorted.
const SET_METHODS: &[&str] = &[
    "difference",
    "intersection",
    "isDisjointFrom",
    "isSubsetOf",
    "isSupersetOf",
    "symmetricDifference",
    "union",
];

/// A call to one of the [`SET_METHODS`] with a single argument, such as `a.union(b)`.
struct SetMethodCall {
    /// The name of the method, such as `union`.
    method: JsSyntaxToken,
    /// The object the method is called on, such as `a` in `a.union(b)`.
    receiver: AnyJsExpression,
    argument: AnyJsExpression,
}

impl SetMethodCall {
    /// Returns `None` for optional calls such as `a?.union(b)`, computed methods such as
    /// `a["union"](b)`, and calls with a spread argument or with more or less than one
    /// argument.
    fn from_call(call: &JsCallExpression) -> Option<Self> {
        if call.is_optional() {
            return None;
        }
        let callee = call.callee().ok()?.omit_parentheses();
        let callee = callee.as_js_static_member_expression()?;
        if callee.is_optional() {
            return None;
        }
        let method = callee.member().ok()?.as_js_name()?.value_token().ok()?;
        SET_METHODS.binary_search(&method.text_trimmed()).ok()?;
        Some(Self {
            method,
            receiver: callee.object().ok()?,
            argument: single_argument(&call.arguments().ok()?)?,
        })
    }

    /// Returns `true` if the method returns a new `Set`, and `false` if it returns a boolean.
    fn returns_set(&self) -> bool {
        matches!(
            self.method.text_trimmed(),
            "difference" | "intersection" | "symmetricDifference" | "union"
        )
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

/// Returns the parent of `expression`, ignoring the parentheses around `expression`.
fn parent_outside_parentheses(expression: &AnyJsExpression) -> Option<JsSyntaxNode> {
    expression
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
}

/// Returns the `Set` or `Map` that `new Set(argument)` copies, and its kind.
///
/// The argument is either a `Set`, or a call that iterates the values of a `Set` or the
/// keys of a `Map`, such as `set.values()` or `map.keys()`. The entries of a `Set` and
/// the values and entries of a `Map` are different from what the methods read from the
/// collection itself.
fn copied_collection(
    argument: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<(AnyJsExpression, CollectionKind)> {
    if collection_kind(argument, model) == Some(CollectionKind::Set) {
        return Some((argument.clone(), CollectionKind::Set));
    }

    let call = argument.clone().omit_parentheses();
    let call = call.as_js_call_expression()?;
    if call.is_optional() || !call.arguments().ok()?.args().is_empty() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee.is_optional() {
        return None;
    }
    let method = callee.member().ok()?.as_js_name()?.value_token().ok()?;
    let object = callee.object().ok()?;
    let kind = collection_kind(&object, model)?;
    match (kind, method.text_trimmed()) {
        (CollectionKind::Set, "keys" | "values") | (CollectionKind::Map, "keys") => {
            Some((object, kind))
        }
        _ => None,
    }
}

/// Returns `true` if a comment inside `node` is outside of `replacement`, so the fix
/// would remove it.
fn has_comments_outside(node: &JsNewExpression, replacement: &AnyJsExpression) -> bool {
    let node_range = node.syntax().text_trimmed_range();
    let replacement_range = replacement.syntax().text_trimmed_range();
    node.syntax().borrowed_tokens().any(|token| {
        token
            .leading_trivia_pieces()
            .chain(token.trailing_trivia_pieces())
            .any(|piece| {
                piece.kind().is_comment()
                    && node_range.contains_range(piece.text_range())
                    && !replacement_range.contains_range(piece.text_range())
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_methods_sorted() {
        for pair in SET_METHODS.windows(2) {
            assert!(pair[0] < pair[1], "{} must come after {}", pair[0], pair[1]);
        }
    }
}

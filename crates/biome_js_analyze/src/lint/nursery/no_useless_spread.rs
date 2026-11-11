use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsObjectMember,
    AnyJsObjectMemberName, JsArrayElementList, JsArrayExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsClassExpression, JsComputedMemberExpression, JsExpressionStatement, JsForOfStatement,
    JsFunctionExpression, JsInitializerClause, JsLanguage, JsNewExpression, JsObjectExpression,
    JsObjectMemberList, JsParenthesizedExpression, JsPropertyObjectMember, JsReturnStatement,
    JsSpread, JsStaticMemberExpression, JsSuperExpression, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken, JsSyntaxTrivia, JsTemplateElement, JsThrowStatement, JsYieldArgument,
    expression_left_side::AnyJsExpressionLeftSide, global_identifier, static_value::StaticValue,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, Direction, SyntaxKindSet,
};
use biome_rule_options::no_useless_spread::NoUselessSpreadOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow spread syntax that has no effect.
    ///
    /// Spread syntax (`...`) copies the items of an array, an object, or any other iterable value,
    /// such as a `Set` or a generator.
    /// In some places the copy is not needed:
    ///
    /// - Spreading an array literal into another array literal, or into the arguments of a call.
    ///   Write the elements directly instead.
    /// - Spreading an object literal into another object literal.
    ///   Write the properties directly instead.
    /// - Creating an object that only contains spread properties to pass it to `Object.assign()`.
    ///   `Object.assign()` already copies the properties of each argument, so pass the values directly.
    /// - Spreading an iterable into the only argument of `new Map()`, `new WeakMap()`, `new Set()`, or `new WeakSet()`.
    ///   These constructors read a single iterable, so each item becomes a separate argument and only the first one is used.
    /// - Converting an iterable to an array before passing it to something that accepts any iterable:
    ///   a `for...of` loop, `yield*`, `new Map()`, `new WeakMap()`, `new Set()`, `new WeakSet()`, `Array.from()`,
    ///   `Object.fromEntries()`, `Promise.all()`, `Promise.allSettled()`, `Promise.any()`, `Promise.race()`,
    ///   and the `from()` method of typed arrays such as `Uint8Array.from()`.
    /// - Copying an array that was just created by a method such as `concat()`, `slice()`, `toSorted()`, or `Object.keys()`.
    ///
    /// The rule assumes that a method with one of these names returns a new array.
    /// It can't know whether a value such as `foo` in `foo.slice()` is an array or a string,
    /// so it doesn't offer a fix for `concat()` and `slice()`, which strings also have.
    ///
    /// The rule doesn't offer a fix for an object literal with a getter or a setter, such as `{ ...{ get value() {} } }`.
    /// Spreading the object reads the getter once and copies the value,
    /// while writing the getter in the outer object would run it every time the property is read.
    ///
    /// Spreading an array also turns empty slots into `undefined`.
    /// If your code relies on that, disable the rule where needed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const array = [first, ...[second, third]];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = { first, ...{ second } };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo(first, ...[second]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign(target, { ...source });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const set = new Set(...items);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const set = new Set([...items]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const item of [...items]) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const keys = [...Object.keys(object)];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const array = [first, ...rest];
    /// const object = { first, ...rest };
    /// foo(first, ...rest);
    /// Object.assign(target, source);
    /// const set = new Set(items);
    /// for (const item of items) {}
    /// const keys = Object.keys(object);
    /// const copy = [...array];
    /// ```
    ///
    pub NoUselessSpread {
        version: "2.6.7",
        name: "noUselessSpread",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-spread").same()],
        recommended: true,
        // Unsafe because the rule can't prove that values are arrays, and removing a copy
        // changes behavior when the original value is modified while it is read.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessSpread {
    type Query = Semantic<JsSpread>;
    type State = UselessSpread;
    type Signals = Option<Self::State>;
    type Options = NoUselessSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let spread = ctx.query();
        let model = ctx.model();
        let argument = spread.argument().ok()?.omit_parentheses();
        let list = spread.syntax().parent()?;

        if let Some(elements) = JsArrayElementList::cast_ref(&list) {
            if argument.as_js_array_expression().is_some() {
                return Some(UselessSpread::Literal { has_fix: true });
            }
            if elements.len() != 1 {
                return None;
            }
            let array = elements.parent::<JsArrayExpression>()?;
            if let Some(consumer) = IterableConsumer::from_array(&array, model) {
                return Some(UselessSpread::IterableToArray(consumer));
            }
            return array_copy(&argument, model)
                .map(|has_fix| UselessSpread::ArrayCopy { has_fix });
        }

        if let Some(arguments) = JsCallArgumentList::cast_ref(&list) {
            let parent = arguments.parent::<JsCallArguments>()?.syntax().parent()?;
            if let Some(array) = argument.as_js_array_expression() {
                if !JsCallExpression::can_cast(parent.kind())
                    && !JsNewExpression::can_cast(parent.kind())
                {
                    return None;
                }
                // Removing the spread from `new Set(...[...items])` would leave `new Set(...items)`,
                // which is also wrong.
                let has_fix = !(arguments.len() == 1
                    && is_single_spread(array)
                    && JsNewExpression::cast(parent)
                        .is_some_and(|new| collection_constructor(&new, model).is_some()))
                    // Empty slots are passed as `undefined`, which must not refer to a local variable.
                    && !(has_empty_slot(array) && is_undefined_shadowed(spread, model));
                return Some(UselessSpread::Literal { has_fix });
            }
            if arguments.len() != 1 {
                return None;
            }
            let callee = collection_constructor(&JsNewExpression::cast(parent)?, model)?;
            return Some(UselessSpread::CollectionConstructorSpread(callee));
        }

        let members = JsObjectMemberList::cast(list)?;
        if let Some(object) = argument.as_js_object_expression() {
            if has_prototype_setter(object) || has_super(object) {
                return None;
            }
            return Some(UselessSpread::Literal {
                has_fix: !has_accessor(object),
            });
        }
        // An `Object.assign()` source is reported once, at its first spread.
        if members.first()?.ok()?.syntax() != spread.syntax() {
            return None;
        }
        is_object_assign_source(&members, model).then_some(UselessSpread::ObjectAssignSource)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let spread = ctx.query();
        let diagnostic = match state {
            UselessSpread::Literal { .. } => {
                let list_kind = spread.syntax().parent()?.kind();
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    spread.range(),
                    if list_kind == JsSyntaxKind::JS_OBJECT_MEMBER_LIST {
                        markup! { "Spreading an object literal into another object literal is unnecessary." }
                    } else if list_kind == JsSyntaxKind::JS_CALL_ARGUMENT_LIST {
                        markup! { "Spreading an array literal into the arguments of a call is unnecessary." }
                    } else {
                        markup! { "Spreading an array literal into another array literal is unnecessary." }
                    },
                );
                if list_kind == JsSyntaxKind::JS_OBJECT_MEMBER_LIST {
                    diagnostic.note(markup! {
                        "Write the properties directly in the surrounding object."
                    })
                } else if list_kind == JsSyntaxKind::JS_CALL_ARGUMENT_LIST {
                    diagnostic.note(markup! {
                        "Pass the elements directly as arguments."
                    })
                } else {
                    diagnostic.note(markup! {
                        "Write the elements directly in the surrounding array."
                    })
                }
            }
            UselessSpread::ObjectAssignSource => {
                let object = spread.syntax().ancestors().find_map(JsObjectExpression::cast)?;
                RuleDiagnostic::new(
                    rule_category!(),
                    object.range(),
                    markup! {
                        "This object only copies the properties of other values before "<Emphasis>"Object.assign()"</Emphasis>" copies them again."
                    },
                )
                .note(markup! {
                    <Emphasis>"Object.assign()"</Emphasis>" copies the properties of each argument, so pass the spread values as separate arguments."
                })
            }
            UselessSpread::CollectionConstructorSpread(callee) => RuleDiagnostic::new(
                rule_category!(),
                spread.range(),
                markup! {
                    "Spreading into "<Emphasis>"new "{TrimmedText(callee)}"()"</Emphasis>" passes each item as a separate argument."
                },
            )
            .note(markup! {
                "The constructor reads a single iterable, so only the first item is used."
            })
            .note(markup! {
                "Pass the iterable directly, without spreading it."
            }),
            UselessSpread::IterableToArray(consumer) => {
                let array = spread.parent::<JsArrayElementList>()?.parent::<JsArrayExpression>()?;
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    array.range(),
                    markup! {
                        "Converting this iterable to an array is unnecessary, because "<Emphasis>{consumer}</Emphasis>" accepts any iterable."
                    },
                )
                .note(markup! {
                    "Pass the iterable directly to avoid copying every item."
                });
                if matches!(consumer, IterableConsumer::ForOf) {
                    diagnostic.note(markup! {
                        "Keep the copy if the loop changes the iterable while it runs."
                    })
                } else {
                    diagnostic
                }
            }
            UselessSpread::ArrayCopy { .. } => {
                let array = spread.parent::<JsArrayElementList>()?.parent::<JsArrayExpression>()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    array.range(),
                    markup! {
                        "This spread copies an array that was just created."
                    },
                )
                .note(markup! {
                    "The spread value is already a new array, so it can be used directly."
                })
            }
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let spread = ctx.query();
        let mut mutation = ctx.root().begin();
        let message = match state {
            UselessSpread::Literal { has_fix: true } => {
                if inline_literal(spread, &mut mutation)? == JsSyntaxKind::JS_OBJECT_EXPRESSION {
                    FixMessage::RemoveObjectSpread
                } else {
                    FixMessage::RemoveArraySpread
                }
            }
            UselessSpread::ObjectAssignSource => {
                inline_object_assign_source(spread, &mut mutation)?;
                FixMessage::PassSpreadValues
            }
            UselessSpread::IterableToArray(_) => {
                replace_array_with_spread_value(spread, &mut mutation, false)?;
                FixMessage::PassIterable
            }
            UselessSpread::ArrayCopy { has_fix: true } => {
                replace_array_with_spread_value(spread, &mut mutation, true)?;
                FixMessage::RemoveCopy
            }
            UselessSpread::Literal { has_fix: false }
            | UselessSpread::ArrayCopy { has_fix: false }
            | UselessSpread::CollectionConstructorSpread(_) => return None,
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

pub enum UselessSpread {
    /// An array literal spread into an array literal or arguments, or an object
    /// literal spread into an object literal.
    Literal { has_fix: bool },
    /// The first spread of an object that only contains spreads and is passed
    /// to `Object.assign()` after the target.
    ObjectAssignSource,
    /// The only argument of a collection constructor is a spread. Holds the callee.
    CollectionConstructorSpread(AnyJsExpression),
    /// The only element of an array that is passed to something that accepts any iterable.
    IterableToArray(IterableConsumer),
    /// The only element of an array that copies a value that is already a new array.
    ArrayCopy { has_fix: bool },
}

/// The place that receives an array created with `[...iterable]`, and accepts
/// any iterable.
pub enum IterableConsumer {
    ForOf,
    YieldStar,
    /// `new Map()`, `new Set()`, and similar. Holds the callee.
    Constructor(AnyJsExpression),
    /// `Promise.all()`, `Array.from()`, and similar. Holds the callee.
    Function(AnyJsExpression),
}

impl IterableConsumer {
    /// Returns the place that receives `array`, if it accepts any iterable.
    fn from_array(array: &JsArrayExpression, model: &SemanticModel) -> Option<Self> {
        let (parent, child) = parent_skipping_parentheses(array)?;
        if let Some(for_of) = JsForOfStatement::cast_ref(&parent) {
            return (for_of.expression().ok()?.syntax() == &child).then_some(Self::ForOf);
        }
        if let Some(argument) = JsYieldArgument::cast(parent.clone()) {
            return argument.star_token().is_some().then_some(Self::YieldStar);
        }
        let arguments = JsCallArgumentList::cast(parent)?;
        if arguments.len() != 1 {
            return None;
        }
        let call_like = arguments.parent::<JsCallArguments>()?.syntax().parent()?;
        if let Some(new) = JsNewExpression::cast_ref(&call_like) {
            return collection_constructor(&new, model).map(Self::Constructor);
        }
        let call = JsCallExpression::cast(call_like)?;
        let (object, method) = static_method_call(&call)?;
        let object_name = global_name(&object, model)?;
        let accepts_iterable = match object_name.text() {
            "Promise" => matches!(method.text_trimmed(), "all" | "allSettled" | "any" | "race"),
            "Object" => method.text_trimmed() == "fromEntries",
            "Array" => method.text_trimmed() == "from",
            name => method.text_trimmed() == "from" && TYPED_ARRAYS.binary_search(&name).is_ok(),
        };
        accepts_iterable.then_some(Self::Function(call.callee().ok()?))
    }
}

impl biome_console::fmt::Display for IterableConsumer {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::ForOf => fmt.write_str("for...of"),
            Self::YieldStar => fmt.write_str("yield*"),
            Self::Constructor(callee) => {
                fmt.write_str("new ")?;
                TrimmedText(callee).fmt(fmt)?;
                fmt.write_str("()")
            }
            Self::Function(callee) => {
                TrimmedText(callee).fmt(fmt)?;
                fmt.write_str("()")
            }
        }
    }
}

/// The label of the fix that the rule offers.
#[derive(Clone, Copy)]
enum FixMessage {
    RemoveArraySpread,
    RemoveObjectSpread,
    PassSpreadValues,
    PassIterable,
    RemoveCopy,
}

impl biome_console::fmt::Display for FixMessage {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::RemoveArraySpread => "Remove the spread and the square brackets.",
            Self::RemoveObjectSpread => "Remove the spread and the curly braces.",
            Self::PassSpreadValues => "Pass the spread values as separate arguments.",
            Self::PassIterable => "Pass the iterable directly.",
            Self::RemoveCopy => "Remove the copy.",
        })
    }
}

/// Writes the source text of an expression without trivia, such as `Promise.all` for `Promise . all`.
struct TrimmedText<'a>(&'a AnyJsExpression);

impl biome_console::fmt::Display for TrimmedText<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for token in self.0.syntax().descendants_tokens(Direction::Next) {
            fmt.write_str(token.text_trimmed())?;
        }
        Ok(())
    }
}

/// Built-in constructors of typed arrays. Sorted for binary search.
const TYPED_ARRAYS: &[&str] = &[
    "BigInt64Array",
    "BigUint64Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
];

/// Array methods that return a new array. Sorted for binary search.
const ARRAY_COPY_METHODS: &[&str] = &[
    "flat",
    "slice",
    "splice",
    "toReversed",
    "toSorted",
    "toSpliced",
    "with",
];

/// The parents of an array where `await` doesn't need parentheses.
const AWAIT_SAFE_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsInitializerClause::KIND_SET
    .union(JsCallArgumentList::KIND_SET)
    .union(JsArrayElementList::KIND_SET)
    .union(JsReturnStatement::KIND_SET)
    .union(JsThrowStatement::KIND_SET)
    .union(JsPropertyObjectMember::KIND_SET)
    .union(JsParenthesizedExpression::KIND_SET)
    .union(JsExpressionStatement::KIND_SET)
    .union(JsTemplateElement::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsYieldArgument::KIND_SET);

/// Expressions that need parentheses when they start a statement or the body of an arrow function.
const STATEMENT_START_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionExpression::KIND_SET
    .union(JsClassExpression::KIND_SET)
    .union(JsObjectExpression::KIND_SET);

/// Returns the first ancestor of `array` that isn't a parenthesized expression,
/// along with its child that contains `array`.
fn parent_skipping_parentheses(array: &JsArrayExpression) -> Option<(JsSyntaxNode, JsSyntaxNode)> {
    let mut child = array.syntax().clone();
    loop {
        let parent = child.parent()?;
        if !JsParenthesizedExpression::can_cast(parent.kind()) {
            return Some((parent, child));
        }
        child = parent;
    }
}

/// Returns `true` if `array` only contains one element, which is a spread, such as `[...items]`.
fn is_single_spread(array: &JsArrayExpression) -> bool {
    let elements = array.elements();
    elements.len() == 1
        && elements
            .first()
            .is_some_and(|element| matches!(element, Ok(AnyJsArrayElement::JsSpread(_))))
}

/// Returns the name of the global variable that `expression` refers to.
/// Returns `None` if a local declaration shadows it.
fn global_name(expression: &AnyJsExpression, model: &SemanticModel) -> Option<StaticValue> {
    let (reference, name) = global_identifier(&expression.as_any_global_identifier_expression()?)?;
    model.binding(&reference).is_none().then_some(name)
}

/// Returns the callee of `new` if it is one of the global `Map`, `WeakMap`, `Set`,
/// or `WeakSet` constructors, which read a single iterable.
fn collection_constructor(new: &JsNewExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    let callee = new.callee().ok()?;
    let name = global_name(&callee, model)?;
    matches!(name.text(), "Map" | "WeakMap" | "Set" | "WeakSet").then_some(callee)
}

/// Returns the object and method name of a call that uses dot notation without optional
/// chaining, such as `object` and `method` for `object.method()`.
fn static_method_call(call: &JsCallExpression) -> Option<(AnyJsExpression, JsSyntaxToken)> {
    if call.is_optional() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    if member.is_optional() {
        return None;
    }
    let name = member.member().ok()?.as_js_name()?.value_token().ok()?;
    Some((member.object().ok()?, name))
}

/// Returns `true` if `object` sets its prototype with a `__proto__: value` property.
/// Spreading the object would copy the property instead of the prototype.
fn has_prototype_setter(object: &JsObjectExpression) -> bool {
    object.members().iter().flatten().any(|member| {
        let AnyJsObjectMember::JsPropertyObjectMember(property) = member else {
            return false;
        };
        property.name().is_ok_and(|name| {
            matches!(name, AnyJsObjectMemberName::JsLiteralMemberName(_))
                && name.name().is_some_and(|name| name.text() == "__proto__")
        })
    })
}

/// Returns `true` if `object` has a getter or a setter.
fn has_accessor(object: &JsObjectExpression) -> bool {
    object.members().iter().any(|member| {
        matches!(
            member,
            Ok(AnyJsObjectMember::JsGetterObjectMember(_)
                | AnyJsObjectMember::JsSetterObjectMember(_))
        )
    })
}

/// Returns `true` if `array` has an empty slot, such as `[a, , b]`.
fn has_empty_slot(array: &JsArrayExpression) -> bool {
    array
        .elements()
        .iter()
        .any(|element| matches!(element, Ok(AnyJsArrayElement::JsArrayHole(_))))
}

/// Returns `true` if a variable named `undefined` is declared in a scope that contains `spread`.
fn is_undefined_shadowed(spread: &JsSpread, model: &SemanticModel) -> bool {
    model
        .scope(spread.syntax())
        .ancestors()
        .any(|scope| scope.get_binding("undefined").is_some())
}

/// Returns `true` if `object` contains `super`.
/// A method that uses `super` reads a different prototype once it moves to another object.
fn has_super(object: &JsObjectExpression) -> bool {
    object
        .syntax()
        .preorder_of_kinds(JsSuperExpression::KIND_SET)
        .next()
        .is_some()
}

/// Returns `true` if `members` belongs to an object that only contains spreads of values
/// other than object literals, and is passed to `Object.assign()` after the target.
fn is_object_assign_source(members: &JsObjectMemberList, model: &SemanticModel) -> bool {
    let all_spreads = members.iter().all(|member| {
        let Ok(AnyJsObjectMember::JsSpread(spread)) = member else {
            return false;
        };
        spread.argument().is_ok_and(|argument| {
            argument
                .omit_parentheses()
                .as_js_object_expression()
                .is_none()
        })
    });
    if !all_spreads {
        return false;
    }
    let Some(object) = members.parent::<JsObjectExpression>() else {
        return false;
    };
    let Some(arguments) = object.parent::<JsCallArgumentList>() else {
        return false;
    };
    let Some(call) = arguments
        .parent::<JsCallArguments>()
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
    else {
        return false;
    };
    let Some((receiver, method)) = static_method_call(&call) else {
        return false;
    };
    if method.text_trimmed() != "assign"
        || global_name(&receiver, model).is_none_or(|name| name.text() != "Object")
    {
        return false;
    }
    // The target must be an argument before the object that isn't a spread, because
    // a spread such as `...args` can be empty and leave the object as the target.
    let mut has_target = false;
    for argument in arguments.iter() {
        match argument {
            Ok(AnyJsCallArgument::AnyJsExpression(expression)) => {
                if expression.syntax() == object.syntax() {
                    return has_target;
                }
                has_target = true;
            }
            Ok(AnyJsCallArgument::JsSpread(_)) => {}
            Err(_) => return false,
        }
    }
    false
}

/// Returns whether spreading `argument` into a new array copies an array that was just created.
/// The inner value is whether the copy can be removed with a fix.
fn array_copy(argument: &AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    match argument {
        AnyJsExpression::JsNewExpression(new) => {
            // `new Array(3)` has empty slots, which the spread turns into `undefined`.
            let is_array = global_name(&new.callee().ok()?, model)?.text() == "Array";
            is_array.then_some(false)
        }
        AnyJsExpression::JsAwaitExpression(await_expression) => {
            let argument = await_expression.argument().ok()?.omit_parentheses();
            let call = argument.as_js_call_expression()?;
            let (object, method) = static_method_call(call)?;
            let is_copy = matches!(method.text_trimmed(), "all" | "allSettled")
                && call.arguments().ok()?.args().len() == 1
                && global_name(&object, model)?.text() == "Promise";
            is_copy.then_some(true)
        }
        AnyJsExpression::JsCallExpression(call) => {
            let (object, method) = static_method_call(call)?;
            let method = method.text_trimmed();
            if let Some(name) = global_name(&object, model) {
                let is_copy = match name.text() {
                    "Object" => {
                        matches!(method, "keys" | "values")
                            && call.arguments().ok()?.args().len() == 1
                    }
                    "Array" => matches!(method, "from" | "of"),
                    _ => false,
                };
                if is_copy {
                    return Some(true);
                }
            }
            if method == "split" {
                return Some(true);
            }
            let object = object.omit_parentheses();
            // Strings also have `concat()` and `slice()`.
            if matches!(
                object,
                AnyJsExpression::AnyJsLiteralExpression(_)
                    | AnyJsExpression::JsTemplateExpression(_)
            ) {
                return None;
            }
            if method == "concat" {
                // `Iterator.concat()` returns an iterator.
                let is_iterator = object
                    .as_js_identifier_expression()
                    .and_then(|identifier| identifier.name().ok()?.value_token().ok())
                    .is_some_and(|name| name.text_trimmed() == "Iterator");
                // Strings also have `concat()`, which returns a string.
                return (!is_iterator).then_some(false);
            }
            // `slice()` is also a method of strings, which spread into an array of characters.
            ARRAY_COPY_METHODS
                .binary_search(&method)
                .is_ok()
                .then_some(method != "slice")
        }
        _ => None,
    }
}

/// Replaces a spread array or object literal with its elements or properties.
/// Returns the kind of the literal.
fn inline_literal(
    spread: &JsSpread,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<JsSyntaxKind> {
    let dotdotdot = spread.dotdotdot_token().ok()?;
    let mut literal = spread.argument().ok()?;
    let mut has_lost_comments = dotdotdot.has_trailing_comments();
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized) = literal {
        has_lost_comments |= has_comments(&parenthesized.l_paren_token().ok()?)
            || has_comments(&parenthesized.r_paren_token().ok()?);
        literal = parenthesized.expression().ok()?;
    }
    if has_lost_comments {
        return None;
    }
    let leading = dotdotdot.leading_trivia();
    let list = spread.syntax().parent()?;
    let kind = literal.syntax().kind();

    match literal {
        AnyJsExpression::JsArrayExpression(array) => {
            let inner = array.elements();
            if has_comments(&array.l_brack_token().ok()?)
                || has_comments(&array.r_brack_token().ok()?)
            {
                return None;
            }
            let inner_trailing = inner.trailing_separator();
            // In an array, a trailing empty slot needs the comma after it, so `[...[,]]`
            // keeps it. In arguments, the slot becomes `undefined` and the comma can go.
            let keeps_trailing = JsArrayElementList::can_cast(list.kind())
                && matches!(inner.last(), Some(Ok(AnyJsArrayElement::JsArrayHole(_))));
            if !keeps_trailing && inner_trailing.as_ref().is_some_and(has_comments) {
                return None;
            }
            let mut replacement = Vec::with_capacity(inner.len());
            for element in inner.elements() {
                replacement.push((
                    element.node().ok()?.clone(),
                    element.trailing_separator().ok()?.cloned(),
                ));
            }
            if let Some(last) = replacement.last_mut()
                && !keeps_trailing
            {
                last.1 = None;
            }

            if let Some(outer) = JsArrayElementList::cast_ref(&list) {
                let target = AnyJsArrayElement::JsSpread(spread.clone());
                let (items, separators) = splice(&outer, &target, replacement, &leading)?;
                mutation.replace_node_discard_trivia(
                    outer,
                    make::js_array_element_list(items, separators),
                );
            } else {
                let outer = JsCallArgumentList::cast(list)?;
                let replacement = replacement
                    .into_iter()
                    .map(|(element, separator)| {
                        let argument = match element {
                            AnyJsArrayElement::AnyJsExpression(expression) => {
                                AnyJsCallArgument::AnyJsExpression(expression)
                            }
                            AnyJsArrayElement::JsSpread(spread) => {
                                AnyJsCallArgument::JsSpread(spread)
                            }
                            // An empty slot is passed as `undefined`.
                            AnyJsArrayElement::JsArrayHole(_) => {
                                AnyJsCallArgument::AnyJsExpression(
                                    make::js_identifier_expression(make::js_reference_identifier(
                                        make::ident("undefined"),
                                    ))
                                    .into(),
                                )
                            }
                        };
                        (argument, separator)
                    })
                    .collect();
                let target = AnyJsCallArgument::JsSpread(spread.clone());
                let (items, separators) = splice(&outer, &target, replacement, &leading)?;
                mutation.replace_node_discard_trivia(
                    outer,
                    make::js_call_argument_list(items, separators),
                );
            }
        }
        AnyJsExpression::JsObjectExpression(object) => {
            let inner = object.members();
            if has_comments(&object.l_curly_token().ok()?)
                || has_comments(&object.r_curly_token().ok()?)
                || inner
                    .trailing_separator()
                    .as_ref()
                    .is_some_and(has_comments)
            {
                return None;
            }
            let mut replacement = Vec::with_capacity(inner.len());
            for member in inner.elements() {
                replacement.push((
                    member.node().ok()?.clone(),
                    member.trailing_separator().ok()?.cloned(),
                ));
            }
            if let Some(last) = replacement.last_mut() {
                last.1 = None;
            }
            let outer = JsObjectMemberList::cast(list)?;
            let target = AnyJsObjectMember::JsSpread(spread.clone());
            let (items, separators) = splice(&outer, &target, replacement, &leading)?;
            mutation
                .replace_node_discard_trivia(outer, make::js_object_member_list(items, separators));
        }
        _ => return None,
    }
    Some(kind)
}

/// Replaces an object passed to `Object.assign()` with the values that it spreads.
fn inline_object_assign_source(
    spread: &JsSpread,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let members = spread.parent::<JsObjectMemberList>()?;
    let object = members.parent::<JsObjectExpression>()?;
    if object.syntax().has_comments_descendants() {
        return None;
    }
    let outer = object.parent::<JsCallArgumentList>()?;
    let mut replacement = Vec::with_capacity(members.len());
    for member in members.elements() {
        let AnyJsObjectMember::JsSpread(spread) = member.node().ok()? else {
            return None;
        };
        // Keeps the line break and indentation before each spread.
        let leading = spread.dotdotdot_token().ok()?.leading_trivia();
        let argument = spread
            .argument()
            .ok()?
            .prepend_trivia_pieces(leading.pieces())?;
        replacement.push((
            AnyJsCallArgument::AnyJsExpression(argument),
            member.trailing_separator().ok()?.cloned(),
        ));
    }
    if let Some(last) = replacement.last_mut() {
        last.1 = None;
    }
    let target = AnyJsCallArgument::AnyJsExpression(object.into());
    let leading = target.syntax().first_token()?.leading_trivia();
    let (items, separators) = splice(&outer, &target, replacement, &leading)?;
    mutation.replace_node_discard_trivia(outer, make::js_call_argument_list(items, separators));
    Some(())
}

/// Replaces an array such as `[...value]` with `value`.
///
/// When `may_need_parentheses` is `true`, `value` is wrapped in parentheses if it
/// can't replace the array without them.
fn replace_array_with_spread_value(
    spread: &JsSpread,
    mutation: &mut BatchMutation<JsLanguage>,
    may_need_parentheses: bool,
) -> Option<()> {
    let array = spread
        .parent::<JsArrayElementList>()?
        .parent::<JsArrayExpression>()?;
    let l_brack = array.l_brack_token().ok()?;
    let r_brack = array.r_brack_token().ok()?;
    let argument = spread.argument().ok()?;
    let has_lost_comments = l_brack.has_trailing_comments()
        || has_comments(&spread.dotdotdot_token().ok()?)
        || array
            .elements()
            .trailing_separator()
            .as_ref()
            .is_some_and(has_comments)
        || r_brack.has_leading_comments()
        || argument.syntax().has_leading_comments()
        || argument.syntax().has_trailing_comments();
    if has_lost_comments {
        return None;
    }

    // `return[...value]` must become `return value`, not `returnvalue`.
    if let Some(previous) = l_brack.prev_token()
        && previous.kind().is_keyword()
        && previous.trailing_trivia().pieces().len() == 0
        && l_brack.leading_trivia().pieces().len() == 0
    {
        mutation.replace_token_discard_trivia(previous.clone(), previous.with_trailing_space());
    }

    let replacement = if may_need_parentheses && copy_needs_parentheses(&array, &argument) {
        make::parenthesized(argument).into()
    } else {
        argument
    };
    mutation.replace_node(AnyJsExpression::from(array), replacement);
    Some(())
}

/// Returns `true` if `argument` needs parentheses to replace the array `array`.
fn copy_needs_parentheses(array: &JsArrayExpression, argument: &AnyJsExpression) -> bool {
    match argument {
        AnyJsExpression::JsParenthesizedExpression(_) => false,
        AnyJsExpression::JsAwaitExpression(_) => array
            .syntax()
            .parent()
            .is_none_or(|parent| !AWAIT_SAFE_PARENT_KINDS.matches(parent.kind())),
        AnyJsExpression::JsCallExpression(call) => {
            // `function () {}()` and `{}.method()` can't start a statement or the body
            // of an arrow function.
            let starts_with_literal = call.callee().is_ok_and(|callee| {
                STATEMENT_START_KINDS
                    .matches(AnyJsExpressionLeftSide::leftmost(callee).syntax().kind())
            });
            starts_with_literal || is_in_new_callee(array)
        }
        _ => true,
    }
}

/// Returns `true` if `array` is the object that a `new` callee reads a property from,
/// such as in `new [...items].constructor()`. A call in that place needs parentheses,
/// because `new items.slice().constructor()` calls `items.slice` with `new`.
fn is_in_new_callee(array: &JsArrayExpression) -> bool {
    let mut current = array.syntax().clone();
    while let Some(parent) = current.parent() {
        let is_object = match parent.kind() {
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                JsStaticMemberExpression::unwrap_cast(parent.clone())
                    .object()
                    .is_ok_and(|object| object.syntax() == &current)
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                JsComputedMemberExpression::unwrap_cast(parent.clone())
                    .object()
                    .is_ok_and(|object| object.syntax() == &current)
            }
            JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,
            JsSyntaxKind::JS_NEW_EXPRESSION => {
                return JsNewExpression::unwrap_cast(parent)
                    .callee()
                    .is_ok_and(|callee| callee.syntax() == &current);
            }
            _ => false,
        };
        if !is_object {
            return false;
        }
        current = parent;
    }
    false
}

fn has_comments(token: &JsSyntaxToken) -> bool {
    token.has_leading_comments() || token.has_trailing_comments()
}

/// Returns the items and separators of `list`, with `target` replaced by the items of `replacement`.
///
/// Each replacement item comes with the separator that follows it. The separator after
/// `target` follows the last replacement item, unless `target` is the last item without
/// a separator, in which case the separator of the last replacement item is used.
/// `leading` is added before the first replacement item, to keep the line break and
/// indentation that came before `target`.
fn splice<N>(
    list: &impl AstSeparatedList<Language = JsLanguage, Node = N>,
    target: &N,
    replacement: Vec<(N, Option<JsSyntaxToken>)>,
    leading: &JsSyntaxTrivia,
) -> Option<(Vec<N>, Vec<JsSyntaxToken>)>
where
    N: AstNode<Language = JsLanguage> + Clone,
{
    let mut items = Vec::with_capacity(list.len() + replacement.len());
    let mut separators = Vec::with_capacity(list.len() + replacement.len());
    let mut replacement = Some(replacement);
    for element in list.elements() {
        let node = element.node().ok()?;
        let separator = element.trailing_separator().ok()?.cloned();
        if node.syntax() != target.syntax() {
            items.push(node.clone());
            separators.extend(separator);
            continue;
        }
        let replacement = replacement.take()?;
        if replacement.is_empty() && leading.pieces().any(|piece| piece.is_comments()) {
            // The comments before `target` would be removed with it.
            return None;
        }
        let count = replacement.len();
        for (index, (item, item_separator)) in replacement.into_iter().enumerate() {
            let has_leading_comments = leading.pieces().any(|piece| piece.is_comments());
            // An item that starts on its own line already has its own indentation.
            let item =
                if index != 0 || (!has_leading_comments && item.syntax().has_leading_newline()) {
                    item
                } else if item.syntax().first_token().is_some() {
                    item.prepend_trivia_pieces(leading.pieces())?
                } else if has_leading_comments {
                    return None;
                } else {
                    item
                };
            items.push(item);
            if index + 1 == count {
                separators.extend(separator.clone().or(item_separator));
            } else {
                separators.push(item_separator?);
            }
        }
    }
    replacement.is_none().then_some((items, separators))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_lists_are_sorted() {
        for names in [TYPED_ARRAYS, ARRAY_COPY_METHODS] {
            for pair in names.windows(2) {
                assert!(
                    pair[0].cmp(pair[1]).is_lt(),
                    "{} must come before {}",
                    pair[1],
                    pair[0]
                );
            }
        }
    }
}

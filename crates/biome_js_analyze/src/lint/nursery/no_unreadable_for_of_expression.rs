use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsLiteralExpression,
    JsArrayExpression, JsCallExpression, JsForOfStatement, JsIdentifierExpression, JsLanguage,
    JsStaticMemberExpression, JsSuperExpression, JsThisExpression,
    binding_ext::AnyJsBindingDeclaration, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TokenText};
use biome_rule_options::no_unreadable_for_of_expression::NoUnreadableForOfExpressionOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow complex expressions as the value that a `for...of` loop goes through.
    ///
    /// The parentheses of a `for...of` or `for await...of` loop hold two things: the variable
    /// that receives each item, and the value that the loop goes through.
    /// When that value is a long or nested expression, it is harder to see what the loop works on.
    /// Storing the value in a variable before the loop gives it a name and keeps the loop easy to scan.
    ///
    /// The rule accepts these expressions as the value to loop over:
    ///
    /// - A variable, `this`, an array such as `[first, second]`, or a literal such as a string
    ///   in quotes. A template string (a string in backticks) is not accepted here.
    /// - A property read with a dot, such as `object.items`, when the part before the dot is
    ///   also in this list.
    /// - A call such as `getItems(argument)` or `object.getItems(argument)`. The function must be
    ///   a variable or a property that is in this list, and every argument must be short.
    ///   A short argument is a variable, a literal, a template string, a property read with a dot,
    ///   a call without arguments, or an array of those.
    ///
    /// The rule reports every other expression. Examples are a call that takes another call with
    /// arguments, a `new` expression, an `await` expression, a condition such as `first || second`,
    /// optional chaining (`items?.values()`), and a property read with brackets (`items[index]`).
    /// A method call on such an expression is also reported, as in `(first || second).values()`.
    ///
    /// Two kinds of method calls follow their own rules:
    ///
    /// - A call to `.map()` or `.filter()` that produces the value to loop over is not reported,
    ///   whatever its arguments are.
    /// - A call to `.map()`, `.filter()`, `.flatMap()`, `.take()`, or `.drop()` on an iterator is
    ///   always reported. An iterator is an object that produces its values one at a time.
    ///   The rule recognizes the iterators returned by `.entries()`, `.keys()`, `.values()`,
    ///   `.matchAll()`, methods called on `Iterator` itself such as `Iterator.from()`, and generator
    ///   functions (`function*`) declared in the same file. It also follows a `const` variable
    ///   declared before the loop to the value that the variable holds.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (const item of getItems(createArgument(seed))) {
    ///     console.log(item);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const key of Object.keys(Object.fromEntries(entries))) {
    ///     console.log(key);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const item of new Set(items)) {
    ///     console.log(item);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const value of items.values().map(callback)) {
    ///     console.log(value);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const argument = createArgument(seed);
    /// for (const item of getItems(argument)) {
    ///     console.log(item);
    /// }
    ///
    /// for (const item of object.items) {
    ///     console.log(item);
    /// }
    ///
    /// for (const value of Object.values(object)) {
    ///     console.log(value);
    /// }
    ///
    /// for (const [index, item] of [first, second].entries()) {
    ///     console.log(index, item);
    /// }
    ///
    /// for (const item of items.filter(item => item.isEnabled)) {
    ///     console.log(item);
    /// }
    /// ```
    ///
    pub NoUnreadableForOfExpression {
        version: "2.6.3",
        name: "noUnreadableForOfExpression",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-unreadable-for-of-expression").same()],
        recommended: false,
        severity: Severity::Information,
    }
}

impl Rule for NoUnreadableForOfExpression {
    type Query = Semantic<JsForOfStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnreadableForOfExpressionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query().expression().ok()?.inner_expression()?;

        if let AnyJsExpression::JsCallExpression(call) = &expression {
            if is_iterator_helper_call(ctx.model(), call) {
                return Some(());
            }
            if is_array_callback_call(call) {
                return None;
            }
        }

        (!is_simple_iterable(&expression)).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let loop_kind = if node.await_token().is_some() {
            "for await...of"
        } else {
            "for...of"
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.expression().ok()?.range(),
                markup! {
                    "This "<Emphasis>{loop_kind}</Emphasis>" loop goes through the result of a complex expression."
                },
            )
            .note(markup! {
                "A complex expression between the parentheses of a loop makes it harder to see what the loop works on."
            })
            .note(markup! {
                "Store the expression in a variable before the loop, and loop over that variable instead."
            }),
        )
    }
}

/// Expressions that are simple wherever they appear: as the value to loop over, as the object
/// of a property access, or as a call argument.
const SIMPLE_OPERAND_KINDS: SyntaxKindSet<JsLanguage> = JsIdentifierExpression::KIND_SET
    .union(JsThisExpression::KIND_SET)
    .union(JsSuperExpression::KIND_SET)
    .union(AnyJsLiteralExpression::KIND_SET);

/// Expressions that are simple as the value to loop over, or as the object of a property access.
///
/// An array literal is simple here whatever its elements are.
const SIMPLE_ITERABLE_KINDS: SyntaxKindSet<JsLanguage> =
    SIMPLE_OPERAND_KINDS.union(JsArrayExpression::KIND_SET);

/// Returns `true` if `expression` is simple enough to be the value that a loop goes through.
///
/// `expression` must already be unwrapped with [`AnyJsExpression::inner_expression`].
fn is_simple_iterable(expression: &AnyJsExpression) -> bool {
    if SIMPLE_ITERABLE_KINDS.matches(expression.syntax().kind()) {
        return true;
    }

    match expression {
        AnyJsExpression::JsStaticMemberExpression(member) => is_simple_member(member),
        AnyJsExpression::JsCallExpression(call) => {
            !is_array_callback_call(call)
                && has_simple_callee(call)
                && call.arguments().is_ok_and(|arguments| {
                    arguments
                        .args()
                        .iter()
                        .all(|argument| argument.is_ok_and(is_simple_call_argument))
                })
        }
        _ => false,
    }
}

/// Returns `true` if `member` is a property access with a dot on a simple object,
/// such as `object.items` or `getObject().items`.
fn is_simple_member(member: &JsStaticMemberExpression) -> bool {
    !member.is_optional()
        && member
            .object()
            .ok()
            .and_then(|object| object.inner_expression())
            .is_some_and(|object| is_simple_iterable(&object))
}

/// Returns `true` if `call` is a non-optional call of a variable or of a simple property,
/// such as `getItems()` or `object.getItems()`.
fn has_simple_callee(call: &JsCallExpression) -> bool {
    if call.is_optional() {
        return false;
    }

    match call.callee().map(AnyJsExpression::omit_parentheses) {
        Ok(AnyJsExpression::JsIdentifierExpression(_)) => true,
        Ok(AnyJsExpression::JsStaticMemberExpression(member)) => is_simple_member(&member),
        _ => false,
    }
}

/// Returns `true` if `argument` is a simple value, or an array literal that only holds
/// simple values.
fn is_simple_call_argument(argument: AnyJsCallArgument) -> bool {
    let AnyJsCallArgument::AnyJsExpression(expression) = argument else {
        return false;
    };

    match expression.inner_expression() {
        Some(AnyJsExpression::JsArrayExpression(array)) => array.elements().iter().all(|element| {
            element.is_ok_and(|element| match element {
                AnyJsArrayElement::AnyJsExpression(element) => is_simple_value(&element),
                AnyJsArrayElement::JsSpread(_) | AnyJsArrayElement::JsArrayHole(_) => false,
            })
        }),
        Some(expression) => is_simple_value(&expression),
        None => false,
    }
}

/// Returns `true` if `expression` is a variable, a literal, a template string, a simple property
/// access, or a simple call without arguments.
fn is_simple_value(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    if SIMPLE_OPERAND_KINDS.matches(expression.syntax().kind()) {
        return true;
    }

    match expression {
        AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
        AnyJsExpression::JsStaticMemberExpression(member) => is_simple_member(&member),
        AnyJsExpression::JsCallExpression(call) => {
            has_simple_callee(&call) && argument_count(&call) == Some(0)
        }
        _ => false,
    }
}

/// Returns the object and the method name of `call` if it has the form `object.method()`.
fn method_call(call: &JsCallExpression) -> Option<(JsStaticMemberExpression, TokenText)> {
    let AnyJsExpression::JsStaticMemberExpression(member) = call.callee().ok()?.omit_parentheses()
    else {
        return None;
    };
    let name = member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .token_text_trimmed();

    Some((member, name))
}

fn argument_count(call: &JsCallExpression) -> Option<usize> {
    Some(call.arguments().ok()?.args().len())
}

/// Returns `true` if `call` is a non-optional call to `.map()` or `.filter()` with at least
/// one argument.
///
/// These calls are accepted as the value to loop over even when their arguments are complex,
/// unless [`is_iterator_helper_call`] also matches.
fn is_array_callback_call(call: &JsCallExpression) -> bool {
    !call.is_optional()
        && argument_count(call).is_some_and(|count| count > 0)
        && method_call(call).is_some_and(|(member, name)| {
            !member.is_optional() && matches!(&*name, "filter" | "map")
        })
}

/// Returns `true` if `call` calls a method that transforms an iterator, such as
/// `items.values().map(callback)`.
fn is_iterator_helper_call(model: &SemanticModel, call: &JsCallExpression) -> bool {
    argument_count(call).is_some_and(|count| count > 0)
        && method_call(call).is_some_and(|(member, name)| {
            matches!(&*name, "drop" | "filter" | "flatMap" | "map" | "take")
                && member
                    .object()
                    .is_ok_and(|object| is_iterator(model, &object))
        })
}

/// Returns `true` if `expression` is known to produce an iterator.
///
/// This recognizes calls that return an iterator, and `const` variables initialized with one.
fn is_iterator(model: &SemanticModel, expression: &AnyJsExpression) -> bool {
    match expression.inner_expression() {
        Some(AnyJsExpression::JsCallExpression(call)) => is_iterator_call(model, &call),
        Some(AnyJsExpression::JsIdentifierExpression(identifier)) => {
            unchanged_declaration(model, &identifier)
                .and_then(|declaration| const_initializer(declaration, &identifier))
                .is_some_and(|initializer| is_iterator(model, &initializer))
        }
        _ => false,
    }
}

/// Returns `true` if `call` is known to return an iterator.
fn is_iterator_call(model: &SemanticModel, call: &JsCallExpression) -> bool {
    let Some((member, name)) = method_call(call) else {
        return !call.is_optional()
            && call
                .callee()
                .is_ok_and(|callee| is_generator_function(model, &callee));
    };

    match &*name {
        "concat" | "from" | "zip" | "zipKeyed" => member
            .object()
            .is_ok_and(|object| is_global_iterator(model, &object)),
        "entries" | "keys" | "values" => argument_count(call) == Some(0),
        "matchAll" => call.arguments().is_ok_and(|arguments| {
            let mut arguments = arguments.args().iter();
            matches!(
                (arguments.next(), arguments.next()),
                (None | Some(Ok(AnyJsCallArgument::AnyJsExpression(_))), None)
            )
        }),
        _ => is_iterator_helper_call(model, call),
    }
}

/// Returns `true` if `expression` refers to the global `Iterator` object, either directly or
/// through `globalThis` or `window`.
fn is_global_iterator(model: &SemanticModel, expression: &AnyJsExpression) -> bool {
    expression
        .inner_expression()
        .and_then(|expression| expression.as_any_global_identifier_expression())
        .and_then(|expression| global_identifier(&expression))
        .is_some_and(|(reference, name)| {
            name.text() == "Iterator" && model.binding(&reference).is_none()
        })
}

/// Returns `true` if `callee` is a generator function that isn't `async`, or a variable that
/// holds one. Calling such a function returns an iterator.
fn is_generator_function(model: &SemanticModel, callee: &AnyJsExpression) -> bool {
    match callee.inner_expression() {
        Some(AnyJsExpression::JsFunctionExpression(function)) => {
            is_sync_generator(&function.into())
        }
        Some(AnyJsExpression::JsIdentifierExpression(identifier)) => {
            let Some(declaration) = unchanged_declaration(model, &identifier) else {
                return false;
            };
            match AnyJsFunction::cast_ref(declaration.syntax()) {
                Some(function) => is_sync_generator(&function),
                None => const_initializer(declaration, &identifier)
                    .is_some_and(|initializer| is_generator_function(model, &initializer)),
            }
        }
        _ => false,
    }
}

fn is_sync_generator(function: &AnyJsFunction) -> bool {
    function.is_generator() && !function.is_async()
}

/// Returns the declaration of the variable or function that `identifier` refers to, if nothing
/// assigns a new value to it after its declaration.
fn unchanged_declaration(
    model: &SemanticModel,
    identifier: &JsIdentifierExpression,
) -> Option<AnyJsBindingDeclaration> {
    let binding = model.binding(&identifier.name().ok()?)?;
    if binding.all_writes().next().is_some() {
        return None;
    }

    binding.tree().declaration()
}

/// Returns the value that `declaration` initializes a `const` variable with, where `identifier`
/// is a use of that variable.
///
/// Returns `None` for variables with a type annotation, for variables created by destructuring,
/// and for variables declared after `identifier`. Skipping later declarations also guarantees
/// that following a chain of variables never leads back to `identifier`.
fn const_initializer(
    declaration: AnyJsBindingDeclaration,
    identifier: &JsIdentifierExpression,
) -> Option<AnyJsExpression> {
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };

    if declarator.variable_annotation().is_some()
        || declarator.range().end() > identifier.range().start()
        || !declarator.declaration()?.is_const()
    {
        return None;
    }

    declarator.initializer()?.expression().ok()
}

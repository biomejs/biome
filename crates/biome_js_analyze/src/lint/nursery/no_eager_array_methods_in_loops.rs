use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyTsCastExpression, AnyTsType, JsCallExpression,
    JsForOfStatement, JsIdentifierExpression, JsReferenceIdentifier, JsStaticMemberExpression,
    JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_eager_array_methods_in_loops::NoEagerArrayMethodsInLoopsOptions;
use std::ops::ControlFlow;

declare_lint_rule! {
    /// Disallow `.map()` and `.filter()` calls in the header of `for...of` loops.
    ///
    /// Calling `.map()` or `.filter()` on an array in the header of a `for...of` or
    /// `for await...of` loop goes through the items twice.
    /// The method visits every item to build a new array, and then the loop visits every item of that new array.
    /// Doing the same work inside the loop body avoids the extra array and the second pass.
    ///
    /// The rule only checks calls written directly in the loop header.
    /// It doesn't report an array that is stored in a variable before the loop.
    ///
    /// The rule ignores calls on iterators. An iterator is an object that produces its values one at a time,
    /// so its `.map()` and `.filter()` methods don't build an array.
    /// The rule treats the following as iterators:
    ///
    /// - The result of `.values()`, `.keys()`, or `.entries()` called without arguments, or of `.matchAll()`.
    /// - The result of `Iterator.from()`, `Iterator.concat()`, `Iterator.zip()`, or `Iterator.zipKeyed()`.
    /// - The result of calling `.map()`, `.filter()`, `.flatMap()`, `.take()`, or `.drop()` on an iterator.
    /// - The result of calling a generator function declared in the same file.
    /// - A `const` variable that is assigned one of the values above.
    /// - In TypeScript, a value whose type annotation or type assertion names a built-in iterator type,
    ///   such as `Iterator<string>` or `Generator<number>`, or a type alias for one.
    ///   This includes the result of calling a function declared in the same file with such a return type.
    ///
    /// :::caution
    /// Moving an asynchronous callback into the loop body changes when the work starts.
    /// `urls.map(async (url) => fetch(url))` starts every request before the loop begins,
    /// while `await fetch(url)` inside the loop body starts each request only after the previous one finishes.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (const name of users.map((user) => user.name)) {
    ///     console.log(name);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const user of users.filter((user) => user.isActive)) {
    ///     console.log(user);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (const user of users) {
    ///     console.log(user.name);
    /// }
    ///
    /// for (const user of users) {
    ///     if (user.isActive) {
    ///         console.log(user);
    ///     }
    /// }
    ///
    /// // `.values()` returns an iterator, so `.map()` doesn't build an array.
    /// for (const name of users.values().map((user) => user.name)) {
    ///     console.log(name);
    /// }
    ///
    /// // The array is created before the loop.
    /// const names = users.map((user) => user.name);
    /// for (const name of names) {
    ///     console.log(name);
    /// }
    /// ```
    ///
    pub NoEagerArrayMethodsInLoops {
        version: "2.6.3",
        name: "noEagerArrayMethodsInLoops",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-duplicate-loops").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoEagerArrayMethodsInLoops {
    type Query = Semantic<JsForOfStatement>;
    /// The `.map` or `.filter` member access that is called in the loop header.
    type State = JsStaticMemberExpression;
    type Signals = Option<Self::State>;
    type Options = NoEagerArrayMethodsInLoopsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query().expression().ok()?.inner_expression()?;
        let call = expression.as_js_call_expression()?;
        if call.is_optional() || call.arguments().ok()?.args().is_empty() {
            return None;
        }

        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?;
        if member.is_optional() {
            return None;
        }

        let name = member.member().ok()?.as_js_name()?.value_token().ok()?;
        if !matches!(name.text_trimmed(), "map" | "filter") {
            return None;
        }

        if is_iterator_expression(&member.object().ok()?, ctx.model()) {
            return None;
        }

        Some(member.clone())
    }

    fn diagnostic(ctx: &RuleContext<Self>, member: &Self::State) -> Option<RuleDiagnostic> {
        let name = member.member().ok()?.as_js_name()?.value_token().ok()?;
        let method = name.text_trimmed();
        let loop_kind = if ctx.query().await_token().is_some() {
            "for await...of"
        } else {
            "for...of"
        };

        // The range starts at the receiver so that a suppression comment above the loop also
        // applies when the method name is on a later line than the receiver.
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            member.range(),
            markup! {
                "This "<Emphasis>"."{method}"()"</Emphasis>" call builds a new array that is only used by the "<Emphasis>{loop_kind}</Emphasis>" loop."
            },
        )
        .note(markup! {
            <Emphasis>"."{method}"()"</Emphasis>" visits every item to build the array before the loop starts, and the loop then visits every item again. The array is discarded after the loop."
        });

        Some(if method == "map" {
            diagnostic.note(markup! {
                "Loop over the original items and do the callback's work inside the loop body."
            })
        } else {
            diagnostic.note(markup! {
                "Loop over the original items and skip the unwanted ones inside the loop body, for example with an "<Emphasis>"if"</Emphasis>" statement."
            })
        })
    }
}

/// Returns `true` if `expression` is known to produce an iterator rather than an array.
///
/// The check is syntactic: it recognizes built-in calls that return iterators, calls to generator
/// functions, type annotations that name a built-in iterator type, and `const` variables
/// initialized with one of these. Any other expression returns `false`, even when it is an
/// iterator at runtime.
///
/// The search follows receivers and `const` initializers in a loop instead of recursing, so that
/// a long chain of declarations can't overflow the stack. It follows at most
/// [MAX_FOLLOWED_DECLARATIONS] declarations.
fn is_iterator_expression(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let mut declarations_left = MAX_FOLLOWED_DECLARATIONS;
    let mut expression = expression.clone();
    loop {
        if let Some(cast) = AnyTsCastExpression::cast_ref(expression.syntax())
            && cast
                .cast_type()
                .is_some_and(|ty| is_iterator_type(&ty, model))
        {
            return true;
        }

        let step = match &expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                follow(parenthesized.expression().ok())
            }
            AnyJsExpression::TsAsExpression(cast) => follow(cast.expression().ok()),
            AnyJsExpression::TsSatisfiesExpression(cast) => follow(cast.expression().ok()),
            AnyJsExpression::TsTypeAssertionExpression(cast) => follow(cast.expression().ok()),
            AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
                follow(assertion.expression().ok())
            }
            AnyJsExpression::JsCallExpression(call) => {
                classify_call(call, model, &mut declarations_left)
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                classify_variable(identifier, model, &mut declarations_left)
            }
            _ => ControlFlow::Break(false),
        };

        match step {
            ControlFlow::Break(is_iterator) => return is_iterator,
            ControlFlow::Continue(next) => expression = next,
        }
    }
}

/// Continues the search in [is_iterator_expression] with `expression`, or ends it with `false`
/// if the expression is missing.
fn follow(expression: Option<AnyJsExpression>) -> ControlFlow<bool, AnyJsExpression> {
    expression.map_or(ControlFlow::Break(false), ControlFlow::Continue)
}

/// Decides whether `call` returns an iterator.
///
/// Returns `Break` with the answer when the call decides it. Returns `Continue` with the receiver
/// of the call when the call returns an iterator only if the receiver is one.
fn classify_call(
    call: &JsCallExpression,
    model: &SemanticModel,
    declarations_left: &mut u8,
) -> ControlFlow<bool, AnyJsExpression> {
    let Ok(callee) = call.callee() else {
        return ControlFlow::Break(false);
    };

    match callee.omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(identifier) => ControlFlow::Break(
            identifier
                .name()
                .ok()
                .and_then(|reference| called_function(&reference, model, declarations_left))
                .is_some_and(|function| returns_iterator(&function, model)),
        ),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let Some(name) = member
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
            else {
                return ControlFlow::Break(false);
            };
            let Ok(index) =
                ITERATOR_METHODS.binary_search_by(|(method, _)| (*method).cmp(name.text_trimmed()))
            else {
                return ControlFlow::Break(false);
            };
            let (Ok(object), Ok(arguments)) = (member.object(), call.arguments()) else {
                return ControlFlow::Break(false);
            };
            let argument_count = arguments.args().len();

            match ITERATOR_METHODS[index].1 {
                IteratorMethod::Static => ControlFlow::Break(is_global_iterator(&object, model)),
                IteratorMethod::View => ControlFlow::Break(argument_count == 0),
                IteratorMethod::MatchAll => ControlFlow::Break(argument_count <= 1),
                IteratorMethod::Helper if argument_count >= 1 => ControlFlow::Continue(object),
                IteratorMethod::Helper => ControlFlow::Break(false),
            }
        }
        _ => ControlFlow::Break(false),
    }
}

/// Decides whether the variable that `identifier` refers to holds an iterator.
///
/// Returns `Break` with the answer for a variable or parameter that has a type annotation, which
/// is judged by that annotation alone. Returns `Continue` with the initializer for a `const`
/// variable without an annotation, which holds an iterator only if its initializer is one.
fn classify_variable(
    identifier: &JsIdentifierExpression,
    model: &SemanticModel,
    declarations_left: &mut u8,
) -> ControlFlow<bool, AnyJsExpression> {
    let Some(declaration) = identifier
        .name()
        .ok()
        .and_then(|reference| model.binding(&reference))
        .and_then(|binding| binding.tree().declaration())
    else {
        return ControlFlow::Break(false);
    };

    let annotation = match declaration {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter.type_annotation(),
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => parameter
            .formal_parameter()
            .ok()
            .and_then(|parameter| parameter.type_annotation()),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let Some(annotation) = declarator.variable_annotation() else {
                return follow(constant_initializer(&declarator, declarations_left));
            };
            annotation.type_annotation().ok().flatten()
        }
        _ => None,
    };

    ControlFlow::Break(
        annotation
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| is_iterator_type(&ty, model)),
    )
}

/// Returns the function that a call through `reference` is known to run.
///
/// The function is known when `reference` names a function declaration that is never reassigned,
/// or a `const` variable initialized with a function or with another such name.
fn called_function(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
    declarations_left: &mut u8,
) -> Option<AnyJsFunction> {
    let mut reference = reference.clone();
    loop {
        let binding = model.binding(&reference)?;
        match binding.tree().declaration()? {
            AnyJsBindingDeclaration::JsFunctionDeclaration(function) => {
                return binding
                    .all_writes()
                    .next()
                    .is_none()
                    .then(|| function.into());
            }
            AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(function) => {
                return binding
                    .all_writes()
                    .next()
                    .is_none()
                    .then(|| function.into());
            }
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                let initializer = constant_initializer(&declarator, declarations_left)?;
                match initializer.inner_expression()? {
                    AnyJsExpression::JsFunctionExpression(function) => {
                        return Some(function.into());
                    }
                    AnyJsExpression::JsArrowFunctionExpression(function) => {
                        return Some(function.into());
                    }
                    AnyJsExpression::JsIdentifierExpression(identifier) => {
                        reference = identifier.name().ok()?;
                    }
                    _ => return None,
                }
            }
            _ => return None,
        }
    }
}

/// Returns the initializer of `declarator` if the variable always holds that value.
///
/// This is the case for a `const` variable without a type annotation. Each returned initializer
/// uses up one of `declarations_left`, and the function returns `None` once none are left.
fn constant_initializer(
    declarator: &JsVariableDeclarator,
    declarations_left: &mut u8,
) -> Option<AnyJsExpression> {
    if declarator.variable_annotation().is_some() || !declarator.declaration()?.is_const() {
        return None;
    }
    *declarations_left = declarations_left.checked_sub(1)?;

    declarator.initializer()?.expression().ok()
}

/// Returns `true` if calling `function` is known to return an iterator.
///
/// This is the case for a generator function, and for a function whose return type annotation
/// names a built-in iterator type.
fn returns_iterator(function: &AnyJsFunction, model: &SemanticModel) -> bool {
    function.is_generator()
        || function
            .return_type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .and_then(|ty| ty.as_any_ts_type().cloned())
            .is_some_and(|ty| is_iterator_type(&ty, model))
}

/// Returns `true` if `object` refers to the global `Iterator` object, either directly or through
/// a global object such as `globalThis`.
fn is_global_iterator(object: &AnyJsExpression, model: &SemanticModel) -> bool {
    object
        .inner_expression()
        .and_then(|object| object.as_any_global_identifier_expression())
        .and_then(|object| global_identifier(&object))
        .is_some_and(|(reference, name)| {
            name.text() == "Iterator" && model.binding(&reference).is_none()
        })
}

/// Returns `true` if `ty` names a built-in iterator type.
///
/// A name counts when it is one of the built-in iterator types and isn't declared or imported in
/// the file, or when it is a type alias for a type that counts, or a type parameter whose
/// constraint counts. A union counts when it has at least one such member and every other member
/// is `null` or `undefined`.
///
/// The types still to check are kept in a list instead of recursing, so that a long chain of type
/// aliases can't overflow the stack. The search follows at most [MAX_FOLLOWED_DECLARATIONS] type
/// aliases and type parameters.
fn is_iterator_type(ty: &AnyTsType, model: &SemanticModel) -> bool {
    let mut declarations_left = MAX_FOLLOWED_DECLARATIONS;
    let mut pending = vec![ty.clone()];
    let mut has_iterator = false;

    while let Some(ty) = pending.pop() {
        match ty.omit_parentheses() {
            AnyTsType::TsNullLiteralType(_) | AnyTsType::TsUndefinedType(_) => {}
            AnyTsType::TsUnionType(union) => pending.extend(union.types().iter().flatten()),
            AnyTsType::TsReferenceType(reference) => {
                let Some(reference) = reference
                    .name()
                    .ok()
                    .and_then(|name| name.as_js_reference_identifier().cloned())
                else {
                    return false;
                };

                let Some(binding) = model.binding(&reference) else {
                    if !reference.value_token().is_ok_and(|name| {
                        ITERATOR_TYPE_NAMES
                            .binary_search(&name.text_trimmed())
                            .is_ok()
                    }) {
                        return false;
                    }
                    has_iterator = true;
                    continue;
                };

                let Some(left) = declarations_left.checked_sub(1) else {
                    return false;
                };
                declarations_left = left;
                let declared = match binding.tree().declaration() {
                    Some(AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias)) => alias.ty().ok(),
                    Some(AnyJsBindingDeclaration::TsTypeParameter(parameter)) => parameter
                        .constraint()
                        .and_then(|constraint| constraint.ty().ok()),
                    _ => None,
                };
                let Some(declared) = declared else {
                    return false;
                };
                pending.push(declared);
            }
            _ => return false,
        }
    }

    has_iterator
}

/// The number of declarations that one search may follow to find out what a name refers to.
///
/// The limit ends the search on declarations that refer to each other, and it keeps the cost of
/// checking one loop independent of how many declarations the file chains together.
const MAX_FOLLOWED_DECLARATIONS: u8 = 16;

/// How a method name relates to iterators.
#[derive(Clone, Copy)]
enum IteratorMethod {
    /// A static method of the global `Iterator` object that returns an iterator.
    Static,
    /// A method that returns an iterator over a collection when called without arguments.
    View,
    /// `matchAll()`, which returns an iterator when called with at most one argument.
    MatchAll,
    /// A method that returns a new iterator when called on an iterator with at least one argument.
    Helper,
}

/// The method names that can produce an iterator, sorted for binary search.
const ITERATOR_METHODS: &[(&str, IteratorMethod)] = &[
    ("concat", IteratorMethod::Static),
    ("drop", IteratorMethod::Helper),
    ("entries", IteratorMethod::View),
    ("filter", IteratorMethod::Helper),
    ("flatMap", IteratorMethod::Helper),
    ("from", IteratorMethod::Static),
    ("keys", IteratorMethod::View),
    ("map", IteratorMethod::Helper),
    ("matchAll", IteratorMethod::MatchAll),
    ("take", IteratorMethod::Helper),
    ("values", IteratorMethod::View),
    ("zip", IteratorMethod::Static),
    ("zipKeyed", IteratorMethod::Static),
];

/// The names of the built-in TypeScript iterator types, sorted for binary search.
const ITERATOR_TYPE_NAMES: &[&str] = &[
    "ArrayIterator",
    "Generator",
    "IterableIterator",
    "Iterator",
    "IteratorObject",
    "MapIterator",
    "RegExpStringIterator",
    "SetIterator",
    "StringIterator",
];

#[cfg(test)]
mod tests {
    use super::{ITERATOR_METHODS, ITERATOR_TYPE_NAMES};

    #[test]
    fn iterator_methods_are_sorted() {
        for pair in ITERATOR_METHODS.windows(2) {
            assert!(
                pair[0].0.cmp(pair[1].0).is_lt(),
                "{} should come before {}",
                pair[1].0,
                pair[0].0
            );
        }
    }

    #[test]
    fn iterator_type_names_are_sorted() {
        for pair in ITERATOR_TYPE_NAMES.windows(2) {
            assert!(
                pair[0].cmp(pair[1]).is_lt(),
                "{} should come before {}",
                pair[1],
                pair[0]
            );
        }
    }
}

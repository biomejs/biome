use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsCallExpression, JsDecorator, JsNewOrCallExpression,
    JsParenthesizedExpression, static_value::StaticValue,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_capitalized_constructors::UseCapitalizedConstructorsOptions;
use biome_string_case::Case;

declare_lint_rule! {
    /// Require constructor names to begin with a capital letter.
    ///
    /// A constructor is a function or class that creates an object when it's called with `new`,
    /// such as `new Date()`.
    /// By convention, constructor names start with an uppercase letter,
    /// and the names of other functions start with a lowercase letter.
    /// When code follows this convention, a function's name shows whether it needs `new`.
    /// This makes two mistakes easier to spot: calling a constructor without `new`,
    /// and calling a regular function with `new`.
    ///
    /// This rule reports:
    /// - `new` followed by a name that starts with a lowercase letter, such as `new person()`;
    /// - a call without `new` to a name that starts with an uppercase letter, such as `Person()`.
    ///
    /// For a property access, such as `people.Person()` or `people["Person"]()`,
    /// the rule checks the last name, `Person`.
    /// The rule ignores names that don't start with a letter, such as `$()` or `_()`.
    /// It also ignores calls whose name is only known when the code runs,
    /// such as `factories[index]()`.
    ///
    /// The following built-in functions can be called without `new`:
    /// `Array`, `BigInt`, `Boolean`, `Date`, `Error`, `Function`, `Number`, `Object`,
    /// `RegExp`, `String`, and `Symbol`.
    /// The built-in `Date.UTC()` method is allowed too.
    /// These exceptions also apply when the built-ins are accessed through the global object
    /// as `globalThis`, `window`, `self`, or `global`, such as `globalThis.String()`.
    /// They only apply to the built-ins: if your code declares its own variable, function,
    /// parameter, or import with one of these names, calls to it are reported.
    ///
    /// The rule ignores calls used as decorators, such as `@Component()` in TypeScript,
    /// because frameworks such as Angular and NestJS give these functions names
    /// that start with an uppercase letter.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const friend = new person();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const friend = new people.person();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const friend = Person();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const friend = people.Person();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // This `String` is a local function, not the built-in `String`.
    /// function String(value) {
    ///     return value;
    /// }
    /// String(42);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const friend = new Person();
    /// const neighbor = new people.Person();
    /// const name = person();
    /// const id = $("#id");
    /// const label = String(42);
    /// const date = globalThis.Date.UTC(2000, 0);
    /// ```
    ///
    pub UseCapitalizedConstructors {
        version: "next",
        name: "useCapitalizedConstructors",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::Eslint("new-cap").same()],
    }
}

impl Rule for UseCapitalizedConstructors {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseCapitalizedConstructorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?.omit_parentheses();
        let (name, _) = callee_name(&callee)?;
        let name = name.text();

        let is_violation = match node {
            JsNewOrCallExpression::JsNewExpression(_) => first_char_case(name) == Case::Lower,
            JsNewOrCallExpression::JsCallExpression(call) => {
                first_char_case(name) == Case::NumberableCapital
                    && !is_decorator(call)
                    && !is_allowed_call(&callee, name, ctx.model())
            }
        };
        is_violation.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (_, range) = callee_name(&node.callee().ok()?.omit_parentheses())?;
        let diagnostic = match node {
            JsNewOrCallExpression::JsNewExpression(_) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This constructor name starts with a lowercase letter."
                },
            )
            .note(markup! {
                "By convention, only constructors start with an uppercase letter, so a lowercase name suggests a regular function that isn't meant to be called with "<Emphasis>"new"</Emphasis>"."
            })
            .note(markup! {
                "Rename the constructor so that it starts with an uppercase letter, or remove "<Emphasis>"new"</Emphasis>" if this is a regular function."
            }),
            JsNewOrCallExpression::JsCallExpression(_) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This function name starts with an uppercase letter, but it's called without "<Emphasis>"new"</Emphasis>"."
                },
            )
            .note(markup! {
                "By convention, names starting with an uppercase letter are reserved for constructors, which usually must be called with "<Emphasis>"new"</Emphasis>"."
            })
            .note(markup! {
                "Add "<Emphasis>"new"</Emphasis>" if this is a constructor, or rename the function so that it starts with a lowercase letter."
            }),
        };
        Some(diagnostic)
    }
}

/// Global built-in functions that are allowed to be called without `new`.
///
/// Must stay sorted because [is_allowed_call] uses a binary search.
const CAPS_ALLOWED: &[&str] = &[
    "Array", "BigInt", "Boolean", "Date", "Error", "Function", "Number", "Object", "RegExp",
    "String", "Symbol",
];

/// Identifiers that refer to the global object when they are not shadowed.
const GLOBAL_OBJECT_NAMES: &[&str] = &["global", "globalThis", "self", "window"];

/// Returns the name of the callee and the range to report.
///
/// For a member expression, the name is the property name and the range covers only the
/// property, such as `Foo` in `a.Foo()` or `"Foo"` in `a["Foo"]()`.
///
/// Returns `None` when the callee is neither an identifier nor a member expression,
/// such as `getFactory()()`, or when the property name isn't statically known,
/// such as `a[index]()`.
fn callee_name(callee: &AnyJsExpression) -> Option<(StaticValue, TextRange)> {
    if let Some(reference) = callee.as_js_reference_identifier() {
        let token = reference.value_token().ok()?;
        let range = token.text_trimmed_range();
        return Some((StaticValue::String(token), range));
    }
    let member = AnyJsMemberExpression::cast_ref(callee.syntax())?;
    let name = member.member_name()?;
    let range = match &member {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => member.member().ok()?.range(),
        AnyJsMemberExpression::JsComputedMemberExpression(member) => {
            member.member().ok()?.omit_parentheses().range()
        }
    };
    Some((name, range))
}

/// Returns the [Case] of the first character of `name`.
///
/// Returns [Case::Lower] for a lowercase letter and [Case::NumberableCapital] for an
/// uppercase letter. Any other first character, such as `$`, `_`, a digit, or a letter
/// without case such as `안`, returns another variant, as does an empty `name`.
///
/// Letters are classified by their Unicode lowercase and uppercase properties.
/// Titlecase letters such as `ǅ` therefore count as neither, whereas ESLint treats
/// them as lowercase.
fn first_char_case(name: &str) -> Case {
    let first_len = name.chars().next().map_or(0, char::len_utf8);
    Case::identify(&name[..first_len], false)
}

/// Returns `true` if `call` is the expression of a decorator, such as `@Component()`,
/// including when the call is wrapped in parentheses, such as `@(Component())`.
fn is_decorator(call: &JsCallExpression) -> bool {
    call.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
        .is_some_and(|ancestor| JsDecorator::can_cast(ancestor.kind()))
}

/// Returns `true` if `callee`, whose name is `name`, may be called without `new`.
///
/// This is the case for the global `Date.UTC` method and for the names in [CAPS_ALLOWED],
/// as long as they refer to the global built-in rather than to a local declaration.
fn is_allowed_call(callee: &AnyJsExpression, name: &str, model: &SemanticModel) -> bool {
    if name == "UTC"
        && let Some(member) = AnyJsMemberExpression::cast_ref(callee.syntax())
        && member
            .object()
            .is_ok_and(|object| is_global_built_in(&object, "Date", model))
    {
        return true;
    }
    CAPS_ALLOWED.binary_search(&name).is_ok() && is_global_built_in(callee, name, model)
}

/// Returns `true` if `expression` refers to the global built-in `name`,
/// either directly, such as `Date`, or through the global object, such as `globalThis.Date`.
///
/// Returns `false` if a local declaration shadows `name` or the global object name.
fn is_global_built_in(expression: &AnyJsExpression, name: &str, model: &SemanticModel) -> bool {
    let expression = expression.clone().omit_parentheses();
    if let Some(member) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
        return member
            .member_name()
            .is_some_and(|member| member.text() == name)
            && member
                .object()
                .is_ok_and(|object| is_global_object(&object, model));
    }
    expression
        .as_js_reference_identifier()
        .is_some_and(|reference| reference.has_name(name) && model.binding(&reference).is_none())
}

/// Returns `true` if `expression` is an unshadowed reference to the global object,
/// such as `globalThis` or `window`.
fn is_global_object(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_js_reference_identifier()
        .is_some_and(|reference| {
            reference
                .value_token()
                .is_ok_and(|token| GLOBAL_OBJECT_NAMES.contains(&token.text_trimmed()))
                && model.binding(&reference).is_none()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caps_allowed_is_sorted() {
        for pair in CAPS_ALLOWED.windows(2) {
            assert!(
                pair[0].cmp(pair[1]).is_lt(),
                "CAPS_ALLOWED is not sorted: {:?} should come after {:?}",
                pair[0],
                pair[1]
            );
        }
    }
}

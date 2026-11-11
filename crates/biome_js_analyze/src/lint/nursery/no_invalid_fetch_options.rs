use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsObjectMember, JsNewOrCallExpression,
    JsObjectExpression, JsReferenceIdentifier, JsUnaryOperator, binding_ext::AnyJsBindingDeclaration,
    global_identifier, numbers::parse_js_number, static_value::StaticValue,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_invalid_fetch_options::NoInvalidFetchOptionsOptions;

declare_lint_rule! {
    /// Disallow a request body in `fetch()` and `new Request()` when the method is `GET` or `HEAD`.
    ///
    /// [`fetch()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch) and
    /// [`new Request()`](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)
    /// throw a `TypeError` when the options contain a `body` and the `method` is `GET` or `HEAD`.
    /// When the options don't contain a `method`, the request uses `GET`.
    ///
    /// The rule checks options written as an object directly in the call.
    /// It reports a `method` only when its value is known without running the code,
    /// for example a string, or a `const` variable that holds a string.
    /// A `body` set to `undefined` or `null` counts as no body.
    /// A spread, such as `...options`, placed after `body` or `method` can replace them,
    /// so the rule doesn't check those options.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await fetch("/", { body: "foo=bar" });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Request("/", { method: "GET", body: "foo=bar" });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await fetch("/", { method: "HEAD", body: "foo=bar" });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await fetch("/", { method: "HEAD" });
    /// await fetch("/", { method: "POST", body: "foo=bar" });
    /// new Request("/", { method: "POST", body: "foo=bar" });
    /// await fetch("/", { body: undefined });
    /// ```
    ///
    pub NoInvalidFetchOptions {
        version: "2.6.7",
        name: "noInvalidFetchOptions",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintUnicorn("no-invalid-fetch-options").inspired()],
    }
}

impl Rule for NoInvalidFetchOptions {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = InvalidFetchOptions;
    type Signals = Option<Self::State>;
    type Options = NoInvalidFetchOptionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let function_name = match node {
            JsNewOrCallExpression::JsNewExpression(_) => "Request",
            JsNewOrCallExpression::JsCallExpression(_) => "fetch",
        };
        if !is_global(&node.callee().ok()?, function_name, model) {
            return None;
        }

        let [
            Some(AnyJsCallArgument::AnyJsExpression(_)),
            Some(AnyJsCallArgument::AnyJsExpression(options)),
        ] = node.arguments()?.get_arguments_by_index([0, 1])
        else {
            return None;
        };
        let AnyJsExpression::JsObjectExpression(options) = options.inner_expression()? else {
            return None;
        };

        find_invalid_body(&options, model)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.body_range,
                markup! {
                    "This request has a "<Emphasis>"body"</Emphasis>", but its method is "<Emphasis>{state.method}</Emphasis>"."
                },
            )
            .note(markup! {
                "Requests that use the "<Emphasis>"GET"</Emphasis>" or "<Emphasis>"HEAD"</Emphasis>" method can't have a body, so this code throws a "<Emphasis>"TypeError"</Emphasis>" when it runs. A request without a "<Emphasis>"method"</Emphasis>" uses "<Emphasis>"GET"</Emphasis>"."
            })
            .note(markup! {
                "Remove the "<Emphasis>"body"</Emphasis>", or use a method that allows one, such as "<Emphasis>"POST"</Emphasis>"."
            }),
        )
    }
}

pub struct InvalidFetchOptions {
    /// The range of the name of the `body` property.
    body_range: TextRange,
    method: BodylessMethod,
}

/// An HTTP method that doesn't allow a request body.
#[derive(Clone, Copy)]
pub enum BodylessMethod {
    Get,
    Head,
}

impl biome_console::fmt::Display for BodylessMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
        })
    }
}

/// The `method` of the request options, as far as the rule can tell from the
/// properties read so far.
enum MethodProperty {
    /// No property sets the method, so the request uses `GET`.
    Missing,
    /// A spread can set the method to any value.
    Unknown,
    Member(AnyJsObjectMember),
}

/// Returns the problem with `options` if it contains a body and its method is `GET` or `HEAD`.
///
/// A later property with the same name overwrites an earlier one, so only the last `body` and
/// the last `method` count. A spread can overwrite both, so a spread after `body` hides it and a
/// spread after `method` makes the method unknown.
fn find_invalid_body(
    options: &JsObjectExpression,
    model: &SemanticModel,
) -> Option<InvalidFetchOptions> {
    let mut body = None;
    let mut method = MethodProperty::Missing;
    for member in options.members() {
        let member = member.ok()?;
        if member.as_js_spread().is_some() {
            body = None;
            method = MethodProperty::Unknown;
            continue;
        }
        let Some(name) = member.name() else {
            continue;
        };
        match name.text() {
            "body" => body = Some(member),
            "method" => method = MethodProperty::Member(member),
            _ => {}
        }
    }

    let body = body?;
    if !has_body(&body, model)? {
        return None;
    }

    let method = match method {
        MethodProperty::Missing => BodylessMethod::Get,
        MethodProperty::Unknown => return None,
        MethodProperty::Member(member) => {
            let value = member_static_value(&member, model)?;
            let text = value.as_string_constant()?;
            if text.eq_ignore_ascii_case("GET") {
                BodylessMethod::Get
            } else if text.eq_ignore_ascii_case("HEAD") {
                BodylessMethod::Head
            } else {
                return None;
            }
        }
    };

    Some(InvalidFetchOptions {
        body_range: member_name_range(&body)?,
        method,
    })
}

/// Returns whether the `body` property sets a body.
///
/// The value `undefined` or `null`, or any `void` expression, means there is no body.
/// A method or a getter counts as a body. Returns `None` for a setter, because a getter
/// declared before it can still provide the value.
fn has_body(member: &AnyJsObjectMember, model: &SemanticModel) -> Option<bool> {
    let value = match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            let value = property.value().ok()?;
            if let Some(AnyJsExpression::JsUnaryExpression(unary)) = value.inner_expression()
                && unary.operator().ok()? == JsUnaryOperator::Void
            {
                return Some(false);
            }
            static_value(&value, model)
        }
        AnyJsObjectMember::JsShorthandPropertyObjectMember(property) => {
            reference_static_value(&property.name().ok()?, model)
        }
        AnyJsObjectMember::JsMethodObjectMember(_) | AnyJsObjectMember::JsGetterObjectMember(_) => {
            return Some(true);
        }
        _ => return None,
    };
    Some(!value.is_some_and(|value| value.is_null_or_undefined()))
}

/// Returns the value of a property or a shorthand property, if it is known without running
/// the code.
fn member_static_value(member: &AnyJsObjectMember, model: &SemanticModel) -> Option<StaticValue> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            static_value(&property.value().ok()?, model)
        }
        AnyJsObjectMember::JsShorthandPropertyObjectMember(property) => {
            reference_static_value(&property.name().ok()?, model)
        }
        _ => None,
    }
}

fn member_name_range(member: &AnyJsObjectMember) -> Option<TextRange> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => Some(property.name().ok()?.range()),
        AnyJsObjectMember::JsShorthandPropertyObjectMember(property) => {
            Some(property.name().ok()?.range())
        }
        AnyJsObjectMember::JsMethodObjectMember(method) => Some(method.name().ok()?.range()),
        AnyJsObjectMember::JsGetterObjectMember(getter) => Some(getter.name().ok()?.range()),
        _ => None,
    }
}

/// Returns the value of `expression` if it is known without running the code.
///
/// This covers literals, template strings without substitutions, `const` variables that
/// hold such a value, and conditional expressions whose condition is known.
fn static_value(expression: &AnyJsExpression, model: &SemanticModel) -> Option<StaticValue> {
    match expression.inner_expression()? {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            reference_static_value(&identifier.name().ok()?, model)
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            let branch = if is_truthy(&static_value(&conditional.test().ok()?, model)?)? {
                conditional.consequent()
            } else {
                conditional.alternate()
            };
            static_value(&branch.ok()?, model)
        }
        expression => expression.as_static_value(),
    }
}

/// Returns the value of the variable that `reference` refers to, if the variable is a `const`
/// declared before `reference` and its value is known without running the code.
///
/// A reference to the global `undefined` has the value `undefined`.
fn reference_static_value(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<StaticValue> {
    let Some(binding) = model.binding(reference) else {
        if reference.is_undefined() {
            return Some(StaticValue::Undefined(reference.value_token().ok()?));
        }
        return None;
    };
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = binding.tree().declaration()?
    else {
        return None;
    };
    // Skipping declarations that end after the reference also guarantees that following a
    // chain of variables never leads back to `reference`.
    if declarator.range().end() > reference.range().start()
        || !declarator.declaration()?.is_const()
    {
        return None;
    }
    static_value(&declarator.initializer()?.expression().ok()?, model)
}

/// Returns whether `value` is truthy, or `None` for a number that can't be parsed.
fn is_truthy(value: &StaticValue) -> Option<bool> {
    match value {
        // `StaticValue::is_falsy` only treats the text `0` as zero, which misses `0.0` and `0x0`.
        StaticValue::Number(token) => Some(parse_js_number(token.text_trimmed())? != 0.0),
        value => Some(!value.is_falsy()),
    }
}

/// Returns whether `expression` refers to the global variable `name`, directly or through
/// `globalThis` or `window`, and isn't shadowed by a local declaration.
fn is_global(expression: &AnyJsExpression, name: &str, model: &SemanticModel) -> bool {
    expression
        .inner_expression()
        .and_then(|expression| expression.as_any_global_identifier_expression())
        .and_then(|expression| global_identifier(&expression))
        .is_some_and(|(reference, global_name)| {
            global_name.text() == name && model.binding(&reference).is_none()
        })
}

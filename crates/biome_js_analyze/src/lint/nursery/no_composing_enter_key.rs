use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_string_case::StrLikeExtension;
use biome_js_syntax::{
    AnyJsExpression, AnyJsName, JsArrowFunctionExpression, JsAssignmentExpression,
    JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsFunctionDeclaration,
    JsFunctionExpression, JsIfStatement, JsSwitchStatement, JsSyntaxNode, JsxAttribute,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_composing_enter_key::NoComposingEnterKeyOptions;

declare_lint_rule! {
    /// Enforce IME-safe `Enter` handling in keyboard event callbacks.
    ///
    /// Handling `Enter` directly inside `keydown` or `keyup` can break text input for users typing
    /// with an Input Method Editor (IME), such as Japanese, Chinese, or Korean users. During
    /// composition, pressing `Enter` confirms the current candidate, but the keyboard event still
    /// fires and can accidentally trigger submit or other shortcuts.
    ///
    /// This rule requires an IME guard before handling `Enter` in `keydown` and `keyup`, flags
    /// `keypress` handlers that react to `Enter`, and can require the Safari fallback
    /// `event.keyCode === 229`.
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "checkKeyCodeForSafari": false,
    ///         "guardFunctions": ["guardIsComposing"]
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// input.addEventListener("keydown", (event) => {
    ///     if (event.key === "Enter") {
    ///         submit();
    ///     }
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// input.addEventListener("keypress", (event) => {
    ///     if (event.key === "Enter") {
    ///         submit();
    ///     }
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// input.addEventListener("keydown", (event) => {
    ///     if (event.isComposing || event.keyCode === 229) {
    ///         return;
    ///     }
    ///
    ///     if (event.key === "Enter") {
    ///         submit();
    ///     }
    /// });
    /// ```
    ///
    /// ```jsx
    /// <form onSubmit={(event) => {
    ///     event.preventDefault();
    ///     submit();
    /// }}>
    ///     <input />
    /// </form>
    /// ```
    pub NoComposingEnterKey {
        version: "2.4.12",
        name: "noComposingEnterKey",
        language: "js",
        sources: &[RuleSource::EslintImeSafeForm("require-ime-safe-submit").inspired()],
        recommended: false,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub NoComposingEnterKeyQuery = JsxAttribute | JsCallExpression | JsAssignmentExpression
}

#[derive(Clone, Debug)]
pub struct State {
    range: TextRange,
    diagnostic_kind: DiagnosticKind,
}

#[derive(Clone, Debug)]
enum DiagnosticKind {
    MissingGuard(Box<str>),
    DeprecatedKeypress(Box<str>),
    MissingKeyCode229,
}

enum Handler {
    Arrow(JsArrowFunctionExpression),
    Function(JsFunctionExpression),
}

impl Handler {
    fn body(&self) -> Option<JsSyntaxNode> {
        match self {
            Self::Arrow(handler) => Some(handler.body().ok()?.syntax().clone()),
            Self::Function(handler) => Some(handler.body().ok()?.syntax().clone()),
        }
    }
}

impl Rule for NoComposingEnterKey {
    type Query = Ast<NoComposingEnterKeyQuery>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoComposingEnterKeyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();

        match ctx.query() {
            NoComposingEnterKeyQuery::JsCallExpression(call_expression) => {
                let event_name = listener_event_name(call_expression)?;
                let handler = listener_handler(call_expression)?;
                let report_range = call_expression.range();
                analyze_handler(&handler, &event_name, report_range, options)
            }
            NoComposingEnterKeyQuery::JsAssignmentExpression(assignment_expression) => {
                let (event_name, report_range) = assignment_event_name(assignment_expression)?;
                let handler = assignment_handler(assignment_expression)?;
                analyze_handler(&handler, &event_name, report_range, options)
            }
            NoComposingEnterKeyQuery::JsxAttribute(attribute) => {
                let event_name = jsx_attribute_name(attribute)?;
                let handler = jsx_handler(attribute)?;
                let report_range = attribute.name().ok()?.range();
                analyze_handler(&handler, &event_name, report_range, options)
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match &state.diagnostic_kind {
            DiagnosticKind::MissingGuard(event_name) => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Enter key detected in "{event_name.as_ref()}" without an IME composition guard."
                },
            )
            .note(markup! {
                "Add an IME guard before checking "<Emphasis>"Enter"</Emphasis>", or handle submission via the "<Emphasis>"submit"</Emphasis>" event."
            }),
            DiagnosticKind::DeprecatedKeypress(event_name) => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    {event_name.as_ref()}" is deprecated for "<Emphasis>"Enter"</Emphasis>" handling."
                },
            )
            .note(markup! {
                "Use "<Emphasis>"keydown"</Emphasis>" with an IME guard instead, or handle submission via the "<Emphasis>"submit"</Emphasis>" event."
            }),
            DiagnosticKind::MissingKeyCode229 => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The IME guard is missing the Safari fallback "<Emphasis>"event.keyCode === 229"</Emphasis>"."
                },
            )
            .note(markup! {
                "Safari can dispatch the final "<Emphasis>"keydown"</Emphasis>" after composition ends, so "<Emphasis>"event.isComposing"</Emphasis>" alone can miss IME confirmation."
            }),
        };

        Some(diagnostic)
    }
}

fn analyze_handler(
    handler: &Handler,
    event_name: &str,
    report_range: TextRange,
    options: &NoComposingEnterKeyOptions,
) -> Option<State> {
    let body = handler.body()?;
    if !contains_enter_key_check(&body) {
        return None;
    }

    if is_deprecated_keypress_event(event_name) {
        return Some(State {
            range: report_range,
            diagnostic_kind: DiagnosticKind::DeprecatedKeypress(event_name.to_string().into_boxed_str()),
        });
    }

    if has_is_composing_check(&body) {
        if options.check_key_code_for_safari && !has_key_code_229_check(&body) {
            return Some(State {
                range: report_range,
                diagnostic_kind: DiagnosticKind::MissingKeyCode229,
            });
        }

        return None;
    }

    if has_guard_function_call(&body, &options.guard_functions) {
        return None;
    }

    Some(State {
        range: report_range,
        diagnostic_kind: DiagnosticKind::MissingGuard(event_name.to_string().into_boxed_str()),
    })
}

fn listener_event_name(call_expression: &JsCallExpression) -> Option<Box<str>> {
    if call_expression
        .callee()
        .ok()?
        .omit_parentheses()
        .get_callee_member_name()?
        .text()
        != "addEventListener"
    {
        return None;
    }

    let arguments = call_expression.arguments().ok()?;
    let [event_argument, _handler_argument] = arguments.get_arguments_by_index([0, 1]);
    let event_argument = event_argument?;
    string_literal_text(event_argument.as_any_js_expression()?).filter(|event_name| {
        matches!(event_name.as_ref(), "keydown" | "keyup" | "keypress")
    })
}

fn listener_handler(call_expression: &JsCallExpression) -> Option<Handler> {
    let arguments = call_expression.arguments().ok()?;
    let [_event_argument, handler_argument] = arguments.get_arguments_by_index([0, 1]);
    let handler_argument = handler_argument?;
    let handler_expression = handler_argument.as_any_js_expression()?;
    inline_handler(handler_expression)
}

fn assignment_event_name(assignment_expression: &JsAssignmentExpression) -> Option<(Box<str>, TextRange)> {
    let left = assignment_expression.left().ok()?;
    let assignment = left.as_any_js_assignment()?.as_js_static_member_assignment()?;
    let member_name = js_name_text(&assignment.member().ok()?)?;
    let normalized = member_name.to_ascii_lowercase_cow();

    match normalized.as_ref() {
        "onkeydown" | "onkeyup" | "onkeypress" => {
            Some((normalized.into_owned().into_boxed_str(), assignment.range()))
        }
        _ => None,
    }
}

fn assignment_handler(assignment_expression: &JsAssignmentExpression) -> Option<Handler> {
    let handler_expression = assignment_expression.right().ok()?;
    inline_handler(&handler_expression)
}

fn jsx_attribute_name(attribute: &JsxAttribute) -> Option<Box<str>> {
    let name = attribute.name().ok()?;
    let name = name.as_jsx_name()?;
    let value = name.value_token().ok()?;
    let name = value.text_trimmed();

    matches!(name, "onKeyDown" | "onKeyUp" | "onKeyPress").then(|| name.to_string().into_boxed_str())
}

fn jsx_handler(attribute: &JsxAttribute) -> Option<Handler> {
    let expression = attribute
        .initializer()?
        .value()
        .ok()?
        .as_jsx_expression_attribute_value()?
        .expression()
        .ok()?;

    inline_handler(&expression)
}

fn inline_handler(expression: &AnyJsExpression) -> Option<Handler> {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(handler) => Some(Handler::Arrow(handler)),
        AnyJsExpression::JsFunctionExpression(handler) => Some(Handler::Function(handler)),
        _ => None,
    }
}

fn contains_enter_key_check(root: &JsSyntaxNode) -> bool {
    body_matches(root, |node| {
        JsBinaryExpression::cast(node.clone()).is_some_and(|binary| is_enter_key_binary_expression(&binary))
            || JsSwitchStatement::cast(node.clone())
                .is_some_and(|switch_statement| is_enter_key_switch_statement(&switch_statement))
    })
}

fn has_is_composing_check(root: &JsSyntaxNode) -> bool {
    body_matches(root, |node| {
        JsIfStatement::cast(node.clone())
            .and_then(|if_statement| if_statement.test().ok())
            .is_some_and(|test| expression_matches(&test.syntax().clone(), |child| {
                AnyJsExpression::cast(child.clone()).is_some_and(|expression| {
                    static_member_name(&expression)
                        .is_some_and(|member_name| member_name.as_ref() == "isComposing")
                })
            }))
    })
}

fn has_key_code_229_check(root: &JsSyntaxNode) -> bool {
    body_matches(root, |node| {
        JsIfStatement::cast(node.clone())
            .and_then(|if_statement| if_statement.test().ok())
            .is_some_and(|test| expression_matches(&test.syntax().clone(), |child| {
                JsBinaryExpression::cast(child.clone())
                    .is_some_and(|binary| is_key_code_229_binary_expression(&binary))
            }))
    })
}

fn has_guard_function_call(root: &JsSyntaxNode, guard_functions: &[String]) -> bool {
    if guard_functions.is_empty() {
        return false;
    }

    body_matches(root, |node| {
        JsIfStatement::cast(node.clone())
            .and_then(|if_statement| if_statement.test().ok())
            .is_some_and(|test| expression_matches(&test.syntax().clone(), |child| {
                JsCallExpression::cast(child.clone()).is_some_and(|call_expression| {
                    call_expression
                        .callee()
                        .ok()
                        .and_then(|callee| identifier_name(&callee))
                        .is_some_and(|name| guard_functions.iter().any(|guard| guard == name.as_ref()))
                })
            }))
    })
}

fn body_matches(root: &JsSyntaxNode, predicate: impl Fn(&JsSyntaxNode) -> bool) -> bool {
    predicate(root)
        || root
            .descendants()
            .filter(|node| !is_nested_function_node(root, node))
            .any(|node| predicate(&node))
}

fn expression_matches(root: &JsSyntaxNode, predicate: impl Fn(&JsSyntaxNode) -> bool) -> bool {
    predicate(root) || root.descendants().any(|node| predicate(&node))
}

fn is_nested_function_node(root: &JsSyntaxNode, node: &JsSyntaxNode) -> bool {
    let mut ancestor = node.parent();

    while let Some(current) = ancestor {
        if current == *root {
            return false;
        }

        if is_function_boundary(&current) {
            return true;
        }

        ancestor = current.parent();
    }

    false
}

fn is_function_boundary(node: &JsSyntaxNode) -> bool {
    JsArrowFunctionExpression::can_cast(node.kind())
        || JsFunctionExpression::can_cast(node.kind())
        || JsFunctionDeclaration::can_cast(node.kind())
}

fn is_enter_key_binary_expression(binary: &JsBinaryExpression) -> bool {
    let Ok(operator) = binary.operator() else {
        return false;
    };
    if !matches!(
        operator,
        JsBinaryOperator::StrictEquality
            | JsBinaryOperator::Equality
            | JsBinaryOperator::StrictInequality
            | JsBinaryOperator::Inequality
    ) {
        return false;
    }

    let Ok(left) = binary.left() else {
        return false;
    };
    let Ok(right) = binary.right() else {
        return false;
    };

    is_enter_string_comparison(&left, &right)
        || is_enter_string_comparison(&right, &left)
        || is_enter_code_comparison(&left, &right)
        || is_enter_code_comparison(&right, &left)
}

fn is_enter_string_comparison(left: &AnyJsExpression, right: &AnyJsExpression) -> bool {
    static_member_name(left)
        .is_some_and(|member_name| matches!(member_name.as_ref(), "key" | "code"))
        && string_literal_text(right).is_some_and(|text| text.as_ref() == "Enter")
}

fn is_enter_code_comparison(left: &AnyJsExpression, right: &AnyJsExpression) -> bool {
    static_member_name(left).is_some_and(|member_name| matches!(member_name.as_ref(), "keyCode" | "which"))
        && number_literal(right).is_some_and(|value| value == 13.0)
}

fn is_enter_key_switch_statement(switch_statement: &JsSwitchStatement) -> bool {
    let Ok(discriminant) = switch_statement.discriminant() else {
        return false;
    };
    let Some(member_name) = static_member_name(&discriminant) else {
        return false;
    };

    match member_name.as_ref() {
        "key" | "code" => switch_statement.cases().into_iter().any(|case_clause| {
            case_clause.as_js_case_clause().and_then(|case_clause| case_clause.test().ok())
                .and_then(|test| string_literal_text(&test))
                .is_some_and(|text| text.as_ref() == "Enter")
        }),
        "keyCode" | "which" => switch_statement.cases().into_iter().any(|case_clause| {
            case_clause.as_js_case_clause().and_then(|case_clause| case_clause.test().ok())
                .and_then(|test| number_literal(&test))
                .is_some_and(|value| value == 13.0)
        }),
        _ => false,
    }
}

fn is_key_code_229_binary_expression(binary: &JsBinaryExpression) -> bool {
    let Ok(operator) = binary.operator() else {
        return false;
    };
    if !matches!(operator, JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality) {
        return false;
    }

    let Ok(left) = binary.left() else {
        return false;
    };
    let Ok(right) = binary.right() else {
        return false;
    };

    (static_member_name(&left).is_some_and(|member_name| member_name.as_ref() == "keyCode")
        && number_literal(&right).is_some_and(|value| value == 229.0))
        || (static_member_name(&right).is_some_and(|member_name| member_name.as_ref() == "keyCode")
            && number_literal(&left).is_some_and(|value| value == 229.0))
}

fn static_member_name(expression: &AnyJsExpression) -> Option<Box<str>> {
    let expression = expression.clone().omit_parentheses();
    let member = expression.as_js_static_member_expression()?;
    js_name_text(&member.member().ok()?)
}

fn identifier_name(expression: &AnyJsExpression) -> Option<Box<str>> {
    let expression = expression.clone().omit_parentheses();
    let identifier = expression.as_js_identifier_expression()?;
    let name = identifier.name().ok()?;
    let token = name.value_token().ok()?;
    Some(token.text_trimmed().to_string().into_boxed_str())
}

fn js_name_text(name: &AnyJsName) -> Option<Box<str>> {
    let name = name.as_js_name()?;
    let token = name.value_token().ok()?;
    Some(token.text_trimmed().to_string().into_boxed_str())
}

fn string_literal_text(expression: &AnyJsExpression) -> Option<Box<str>> {
    expression
        .clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_string_literal_expression())
        .and_then(|literal| literal.inner_string_text().ok())
        .map(|text| text.text().to_string().into_boxed_str())
}

fn number_literal(expression: &AnyJsExpression) -> Option<f64> {
    expression
        .clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression())
        .and_then(|literal| literal.as_number())
}

fn is_deprecated_keypress_event(event_name: &str) -> bool {
    matches!(event_name, "keypress" | "onkeypress" | "onKeyPress")
}

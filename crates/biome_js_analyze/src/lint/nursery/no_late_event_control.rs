use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression, JsSyntaxToken};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_late_event_control::NoLateEventControlOptions;

use crate::services::semantic::Semantic;
use crate::shared::late_event_handler::{
    AnyFunctionBoundary, find_earlier_pause, find_event_handler,
};

declare_lint_rule! {
    /// Disallow calling `preventDefault()`, `stopPropagation()`, or `stopImmediatePropagation()` after the browser has finished handling the event.
    ///
    /// When an event happens, the browser runs each event handler for it, one after another.
    /// `preventDefault()`, `stopPropagation()`, and `stopImmediatePropagation()` only work while
    /// the browser is still running these handlers. When a handler pauses, the browser moves on
    /// without waiting for it, so calling these methods afterwards does nothing: the browser still
    /// performs the default action, such as following a link, and other handlers still receive
    /// the event.
    ///
    /// Code in a handler runs too late in these cases:
    ///
    /// - The code comes after an `await` or a `for await` loop in an `async` function.
    /// - The code is in a function nested inside the handler, such as a `setTimeout()` or `.then()` callback. The nested function can run after the handler has finished.
    /// - The handler is a generator function (`function*`). Calling a generator function doesn't run its body.
    ///
    /// Call these methods directly in the handler, before the first `await`.
    ///
    /// The rule checks calls on a function parameter whose name looks like an event, such as `e`,
    /// `event`, `evt`, or a camelCase name ending with `Event` like `clickEvent`. In TypeScript,
    /// it also checks parameters with an event type, such as `MouseEvent` or React's
    /// `SyntheticEvent`, and ignores parameters with a type that isn't an event, such as `string`.
    ///
    /// The rule has these limitations:
    ///
    /// - Every call inside a nested function is reported, even when the nested function runs right away, like the callback of `array.map()`. Move the call to the handler itself to make it clear that it runs in time.
    /// - The rule doesn't follow the order in which code runs. A call is reported when an `await` appears earlier in the handler, even if the `await` is in a branch that never runs together with the call.
    /// - When the event is passed to another function, calls inside that function are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// element.addEventListener("click", async (event) => {
    ///     await save();
    ///     event.preventDefault();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.addEventListener("click", (event) => {
    ///     setTimeout(() => {
    ///         event.stopPropagation();
    ///     });
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.onclick = function* (event) {
    ///     event.preventDefault();
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// element.addEventListener("click", async (event) => {
    ///     event.preventDefault();
    ///     await save();
    /// });
    ///
    /// element.addEventListener("click", (event) => {
    ///     event.stopPropagation();
    ///     setTimeout(() => {
    ///         update();
    ///     });
    /// });
    /// ```
    ///
    pub NoLateEventControl {
        version: "2.6.5",
        name: "noLateEventControl",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-late-event-control").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoLateEventControl {
    type Query = Semantic<JsCallExpression>;
    type State = LateEventControl;
    type Signals = Option<Self::State>;
    type Options = NoLateEventControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        let callee =
            AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        let method = EventControlMethod::from_name(callee.member_name()?.text())?;
        let event = callee
            .object()
            .ok()?
            .omit_type_wrappers()?
            .as_js_reference_identifier()?;

        let handler = find_event_handler(model, &event)?;

        let enclosing_function = call
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyFunctionBoundary::cast)?;
        let kind = if enclosing_function != handler {
            LateEventControlKind::NestedFunction
        } else if handler.is_generator() {
            LateEventControlKind::Generator
        } else if handler.is_async() {
            LateEventControlKind::AfterPause(find_earlier_pause(call, &handler)?)
        } else {
            return None;
        };

        Some(LateEventControl {
            kind,
            method,
            event: event.value_token().ok()?,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let LateEventControl {
            kind,
            method,
            event,
        } = state;
        let event = event.text_trimmed();
        let call = markup! { <Emphasis>{event}"."{method}"()"</Emphasis> };
        let diagnostic = match kind {
            LateEventControlKind::AfterPause(pause) => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {call}" is called after the event handler pauses." },
            )
            .detail(pause, markup! { "The event handler pauses here." })
            .note(markup! {
                "When the handler pauses, the browser finishes handling the event without waiting for it, so this call has no effect."
            })
            .note(markup! {
                "Call "{call}" before the event handler pauses."
            }),
            LateEventControlKind::NestedFunction => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {call}" is called inside a function nested in the event handler." },
            )
            .note(markup! {
                "The nested function may run after the browser finishes handling the event, when this call has no effect."
            })
            .note(markup! {
                "Call "{call}" directly in the event handler, before any delayed work."
            }),
            LateEventControlKind::Generator => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {call}" is called inside a generator function." },
            )
            .note(markup! {
                "Calling a generator function doesn't run its body, so this call doesn't happen while the browser handles the event."
            })
            .note(markup! {
                "Use a regular function as the event handler."
            }),
        };
        Some(diagnostic)
    }
}

pub struct LateEventControl {
    kind: LateEventControlKind,
    method: EventControlMethod,
    /// The name token of the event parameter, as written at the call.
    event: JsSyntaxToken,
}

pub enum LateEventControlKind {
    /// The call runs after the handler pauses at the `await`, `yield`, or `for await` loop at
    /// this range.
    AfterPause(TextRange),
    /// The call is inside a function nested in the handler.
    NestedFunction,
    /// The handler is a generator function.
    Generator,
}

#[derive(Clone, Copy)]
pub enum EventControlMethod {
    PreventDefault,
    StopImmediatePropagation,
    StopPropagation,
}

impl EventControlMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "preventDefault" => Some(Self::PreventDefault),
            "stopImmediatePropagation" => Some(Self::StopImmediatePropagation),
            "stopPropagation" => Some(Self::StopPropagation),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for EventControlMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::PreventDefault => "preventDefault",
            Self::StopImmediatePropagation => "stopImmediatePropagation",
            Self::StopPropagation => "stopPropagation",
        })
    }
}

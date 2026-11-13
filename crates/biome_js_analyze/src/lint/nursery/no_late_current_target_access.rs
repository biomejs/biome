use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsStaticMemberExpression, JsSyntaxToken};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_late_current_target_access::NoLateCurrentTargetAccessOptions;

use crate::services::semantic::Semantic;
use crate::shared::late_event_handler::{
    AnyFunctionBoundary, find_earlier_pause, find_event_handler,
};

declare_lint_rule! {
    /// Disallow reading `event.currentTarget` after the browser has finished handling the event.
    ///
    /// `event.currentTarget` is the element that the event handler was added to. The browser
    /// only sets it while it runs the event handlers for the event, one after another. When a
    /// handler pauses, the browser moves on without waiting for it and resets `currentTarget`
    /// to `null`. Reading `currentTarget` afterwards gives `null` instead of the element.
    ///
    /// Code in a handler reads `currentTarget` too late in these cases:
    ///
    /// - The code comes after an `await`, a `yield`, or a `for await` loop in the handler.
    /// - The code is in a function nested inside the handler, such as a `setTimeout()` or `.then()` callback. The nested function can run after the handler has finished.
    ///
    /// Save `event.currentTarget` to a variable before the handler pauses, and use that variable instead.
    ///
    /// The rule checks a function parameter whose name looks like an event, such as `e`,
    /// `event`, `evt`, or a camelCase name ending with `Event` like `clickEvent`. In TypeScript,
    /// it also checks parameters with an event type, such as `MouseEvent` or React's
    /// `SyntheticEvent`, and ignores parameters with a type that isn't an event, such as `string`.
    ///
    /// The rule has these limitations:
    ///
    /// - Every `currentTarget` read inside a nested function is reported, even when the nested function runs right away, like the callback of `array.map()`. Save `currentTarget` to a variable in the handler to make it clear that it's read in time.
    /// - The rule doesn't follow the order in which code runs. A read is reported when an `await` appears earlier in the handler, even if the `await` is in a branch that never runs together with the read.
    /// - When the event is passed to another function, reads inside that function are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// element.addEventListener("click", async (event) => {
    ///     await save();
    ///     event.currentTarget.disabled = false;
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.addEventListener("click", (event) => {
    ///     setTimeout(() => {
    ///         event.currentTarget.focus();
    ///     });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// element.addEventListener("click", async (event) => {
    ///     const button = event.currentTarget;
    ///     button.disabled = true;
    ///     await save();
    ///     button.disabled = false;
    /// });
    ///
    /// element.addEventListener("click", (event) => {
    ///     const { currentTarget } = event;
    ///     setTimeout(() => {
    ///         currentTarget.focus();
    ///     });
    /// });
    /// ```
    ///
    pub NoLateCurrentTargetAccess {
        version: "next",
        name: "noLateCurrentTargetAccess",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-late-current-target-access").inspired()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoLateCurrentTargetAccess {
    type Query = Semantic<JsStaticMemberExpression>;
    type State = LateCurrentTargetAccess;
    type Signals = Option<Self::State>;
    type Options = NoLateCurrentTargetAccessOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member = ctx.query();
        if member.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed() != "currentTarget"
        {
            return None;
        }
        let event = member
            .object()
            .ok()?
            .omit_type_wrappers()?
            .as_js_reference_identifier()?;

        let handler = find_event_handler(ctx.model(), &event)?;

        let enclosing_function = member
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyFunctionBoundary::cast)?;
        let kind = if enclosing_function != handler {
            LateCurrentTargetAccessKind::NestedFunction
        } else {
            LateCurrentTargetAccessKind::AfterPause(find_earlier_pause(member, &handler)?)
        };

        Some(LateCurrentTargetAccess {
            kind,
            event: event.value_token().ok()?,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let LateCurrentTargetAccess { kind, event } = state;
        let access = markup! { <Emphasis>{event.text_trimmed()}".currentTarget"</Emphasis> };
        let diagnostic = match kind {
            LateCurrentTargetAccessKind::AfterPause(pause) => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {access}" is read after the event handler pauses." },
            )
            .detail(pause, markup! { "The event handler pauses here." })
            .note(markup! {
                "When the handler pauses, the browser finishes handling the event and sets "<Emphasis>"currentTarget"</Emphasis>" to "<Emphasis>"null"</Emphasis>"."
            })
            .note(markup! {
                "Save "{access}" to a variable before the event handler pauses, and use that variable instead."
            }),
            LateCurrentTargetAccessKind::NestedFunction => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {access}" is read inside a function nested in the event handler." },
            )
            .note(markup! {
                "The nested function may run after the browser finishes handling the event, when "<Emphasis>"currentTarget"</Emphasis>" is "<Emphasis>"null"</Emphasis>"."
            })
            .note(markup! {
                "Save "{access}" to a variable in the event handler, and use that variable in the nested function."
            }),
        };
        Some(diagnostic)
    }
}

pub struct LateCurrentTargetAccess {
    kind: LateCurrentTargetAccessKind,
    /// The name token of the event parameter, as written at the read.
    event: JsSyntaxToken,
}

pub enum LateCurrentTargetAccessKind {
    /// The read runs after the handler pauses at the `await`, `yield`, or `for await` loop at
    /// this range.
    AfterPause(TextRange),
    /// The read is inside a function nested in the handler.
    NestedFunction,
}

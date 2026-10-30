use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, JsCallExpression,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_invalid_remove_event_listener::NoInvalidRemoveEventListenerOptions;

declare_lint_rule! {
    /// Disallow passing a newly created function to `removeEventListener()`.
    ///
    /// `removeEventListener()` only removes a listener when it receives the exact same function
    /// that was passed to `addEventListener()`. An inline function, or the result of calling
    /// `.bind()`, creates a new function every time the code runs. Such a function was never
    /// added as a listener, so the call does nothing and the original listener stays attached.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// window.removeEventListener("click", listener.bind(window));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.removeEventListener("click", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.removeEventListener("click", function () {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class MyElement extends HTMLElement {
    ///     handler() {}
    ///
    ///     disconnectedCallback() {
    ///         this.removeEventListener("click", this.handler.bind(this));
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const listener = () => {};
    /// window.addEventListener("click", listener);
    /// window.removeEventListener("click", listener);
    ///
    /// window.removeEventListener("click", getListener());
    ///
    /// class MyElement extends HTMLElement {
    ///     constructor() {
    ///         super();
    ///         this.handler = this.handler.bind(this);
    ///     }
    ///
    ///     handler() {}
    ///
    ///     disconnectedCallback() {
    ///         this.removeEventListener("click", this.handler);
    ///     }
    /// }
    /// ```
    ///
    pub NoInvalidRemoveEventListener {
        version: "next",
        name: "noInvalidRemoveEventListener",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-invalid-remove-event-listener").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoInvalidRemoveEventListener {
    type Query = Ast<JsCallExpression>;
    type State = InvalidListener;
    type Signals = Option<Self::State>;
    type Options = NoInvalidRemoveEventListenerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional() {
            return None;
        }
        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?;
        if member.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed()
            != "removeEventListener"
        {
            return None;
        }

        let [event, listener] = call.arguments().ok()?.get_arguments_by_index([0, 1]);
        // A spread first argument hides which argument is the listener.
        if let AnyJsCallArgument::JsSpread(_) = event? {
            return None;
        }
        let listener = listener?.as_any_js_expression()?.clone().omit_parentheses();

        if let Some(function) = AnyJsFunction::cast_ref(listener.syntax()) {
            return function_head_range(&function).map(InvalidListener::Function);
        }

        let AnyJsExpression::JsCallExpression(bind_call) = listener else {
            return None;
        };
        if bind_call.is_optional() {
            return None;
        }
        let bind_callee =
            AnyJsMemberExpression::cast(bind_call.callee().ok()?.omit_parentheses().into_syntax())?;
        let property_range = match &bind_callee {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                if member.is_optional() {
                    return None;
                }
                member.member().ok()?.range()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                if member.is_optional() {
                    return None;
                }
                member.member().ok()?.range()
            }
        };
        (bind_callee.member_name()?.text() == "bind").then_some(InvalidListener::Bind(property_range))
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            InvalidListener::Function(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This inline function is never removed by "<Emphasis>"removeEventListener()"</Emphasis>"."
                },
            )
            .note(markup! {
                "An inline function is a new function, so it can't be the same function that was passed to "<Emphasis>"addEventListener()"</Emphasis>". The original listener stays attached."
            }),
            InvalidListener::Bind(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The function returned by "<Emphasis>".bind()"</Emphasis>" is never removed by "<Emphasis>"removeEventListener()"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>".bind()"</Emphasis>" returns a new function every time it's called, so it can't be the same function that was passed to "<Emphasis>"addEventListener()"</Emphasis>". The original listener stays attached."
            }),
        };
        Some(diagnostic.note(markup! {
            "Store the listener in a variable or property, and pass that same reference to both "<Emphasis>"addEventListener()"</Emphasis>" and "<Emphasis>"removeEventListener()"</Emphasis>"."
        }))
    }
}

/// A listener argument that creates a new function each time it's evaluated.
pub enum InvalidListener {
    /// An inline function; holds the range of the function up to its body.
    Function(TextRange),
    /// A `.bind()` call; holds the range of the `bind` property.
    Bind(TextRange),
}

/// Returns the range from the start of `function` to the end of the token preceding its body,
/// so the diagnostic doesn't span the whole function.
fn function_head_range(function: &AnyJsFunction) -> Option<TextRange> {
    let last_head_token = function.body().ok()?.syntax().first_token()?.prev_token()?;
    Some(TextRange::new(
        function.syntax().text_trimmed_range().start(),
        last_head_token.text_trimmed_range().end(),
    ))
}

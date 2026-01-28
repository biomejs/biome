use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    JsCallExpression, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsIdentifierAssignment, JsIdentifierExpression, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeCast, TextRange, declare_node_union};
use biome_rule_options::use_global_this::UseGlobalThisOptions;

use crate::services::semantic::Semantic;

const GLOBAL_IDENTIFIERS: [&str; 3] = ["window", "self", "global"];

const WINDOW_SPECIFIC_EVENTS: [&str; 15] = [
    "resize",
    "blur",
    "focus",
    "load",
    "scroll",
    "scrollend",
    "wheel",
    "beforeunload", // Browsers might have specific behaviors on exactly `window.onbeforeunload =`
    "message",
    "messageerror",
    "pagehide",
    "pagereveal",
    "pageshow",
    "pageswap",
    "unload",
];

const WINDOW_SPECIFIC_APIS: [&str; 66] = [
    // Properties and methods
    // https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-window-object
    "name",
    "locationbar",
    "menubar",
    "personalbar",
    "scrollbars",
    "statusbar",
    "toolbar",
    "status",
    "close",
    "closed",
    "stop",
    "focus",
    "blur",
    "frames",
    "length",
    "top",
    "opener",
    "parent",
    "frameElement",
    "open",
    "originAgentCluster",
    "postMessage",
    "navigation",
    // Events commonly associated with "window"
    "onresize",
    "onblur",
    "onfocus",
    "onload",
    "onscroll",
    "onscrollend",
    "onwheel",
    "onbeforeunload", // Browsers might have specific behaviors on exactly `window.onbeforeunload =`
    "onmessage",
    "onmessageerror",
    "onpagehide",
    "onpagereveal",
    "onpageshow",
    "onpageswap",
    "onunload",
    // To add/remove/dispatch events that are commonly associated with "window"
    // https://www.w3.org/TR/DOM-Level-2-Events/events.html#Events-flow
    "addEventListener",
    "removeEventListener",
    "dispatchEvent",
    // https://dom.spec.whatwg.org/#idl-index
    "event", // Deprecated and quirky, best left untouched
    // https://drafts.csswg.org/cssom-view/#idl-index
    "screen",
    "visualViewport",
    "moveTo",
    "moveBy",
    "resizeTo",
    "resizeBy",
    "innerWidth",
    "innerHeight",
    "outerWidth",
    "outerHeight",
    "scrollX",
    "pageXOffset",
    "scrollY",
    "pageYOffset",
    "scroll",
    "scrollTo",
    "scrollBy",
    "screenX",
    "screenLeft",
    "screenY",
    "screenTop",
    "screenWidth",
    "screenHeight",
    "devicePixelRatio",
];

const WEB_WORKER_SPECIFIC_APIS: [&str; 17] = [
    "addEventListener",
    "removeEventListener",
    "dispatchEvent",
    // https://html.spec.whatwg.org/multipage/workers.html#the-workerglobalscope-common-interface
    "self",
    "location",
    "navigator",
    "importScripts",
    "onerror",
    "onlanguagechange",
    "onoffline",
    "ononline",
    "onrejectionhandled",
    "onunhandledrejection",
    // https://html.spec.whatwg.org/multipage/workers.html#dedicated-workers-and-the-dedicatedworkerglobalscope-interface
    "name",
    "postMessage",
    "close",
    // https://html.spec.whatwg.org/multipage/workers.html#sharedworkerglobalscope
    "onconnect",
];

declare_lint_rule! {
    /// Enforce the use of `globalThis` over `window`, `self`, and `global`.
    ///
    /// `globalThis` is a standard way to access the global object across platforms such as browsers, Web Workers, Node.js and so on, and using it can make your code portable.
    ///
    /// However, there are several exceptions that are allowed:
    ///
    /// 1. Certain window/Web Workers-specific APIs, such as `window.innerHeight` and `self.postMessage`
    /// 2. Window-specific events, such as `window.addEventListener('resize')`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// window.foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.addEventListener('click', () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// globalThis.foo;
    /// ```
    ///
    /// ```js
    /// globalThis.addEventListener('click', () => {});
    /// ```
    ///
    /// ```js
    /// // window/Web Workers-specific APIs are allowed
    /// window.innerWidth;
    /// self.postMessage({ type: 'ready' });
    /// ```
    ///
    /// ```js
    /// // window-specific events are allowed
    /// window.addEventListener('resize', () => {});
    /// ```
    ///
    pub UseGlobalThis {
        version: "next",
        name: "useGlobalThis",
        language: "js",
        severity: Severity::Warning,
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-global-this").same()],
    }
}

declare_node_union! {
    pub UseGlobalThisQuery = JsIdentifierExpression | JsIdentifierAssignment
}

impl Rule for UseGlobalThis {
    type Query = Semantic<UseGlobalThisQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseGlobalThisOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();
        let model = ctx.model();
        check_expression(expr, model)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Prefer "<Emphasis>"globalThis"</Emphasis>" over "<Emphasis>"window"</Emphasis>", "<Emphasis>"self"</Emphasis>" and "<Emphasis>"global"</Emphasis>""
                },
            )
            .note(markup! {
                "The "<Emphasis>"globalThis"</Emphasis>" is the standard way to access the global object across environments."
            }),
        )
    }
}

fn check_expression(expr: &UseGlobalThisQuery, model: &SemanticModel) -> Option<TextRange> {
    match expr {
        UseGlobalThisQuery::JsIdentifierExpression(expr) => {
            let reference = expr.name().ok()?;
            let ident_name = reference.to_trimmed_text();
            if !is_global_identifier(&ident_name) || model.binding(&reference).is_some() {
                return None;
            }

            let parent = expr.syntax().parent()?;
            match parent.kind() {
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                    let assignment = parent.cast::<JsStaticMemberAssignment>()?;
                    let member_name = assignment.member().ok()?.as_js_name()?.to_trimmed_text();
                    if check_is_window_specific_api(&ident_name, &member_name, None)
                        || check_is_web_worker_specific_api(&ident_name, &member_name)
                    {
                        return None;
                    }
                }
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                    let expr = parent.cast::<JsStaticMemberExpression>()?;
                    let member_name = expr.member().ok()?.as_js_name()?.to_trimmed_text();
                    let call_expr = expr
                        .syntax()
                        .parent()
                        .and_then(|node| node.cast::<JsCallExpression>());
                    if check_is_window_specific_api(&ident_name, &member_name, call_expr)
                        || check_is_web_worker_specific_api(&ident_name, &member_name)
                    {
                        return None;
                    }
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                    let assignment = parent.cast::<JsComputedMemberAssignment>()?;
                    let member_name = assignment.member().ok()?.to_trimmed_text();
                    let member_name = member_name.trim_matches(['\'', '"', '`']);
                    if check_is_window_specific_api(&ident_name, member_name, None)
                        || check_is_web_worker_specific_api(&ident_name, member_name)
                    {
                        return None;
                    }
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                    let expr = parent.cast::<JsComputedMemberExpression>()?;
                    let member_name = expr.member().ok()?.to_trimmed_text();
                    let member_name = member_name.trim_matches(['\'', '"', '`']);
                    let call_expr = expr
                        .syntax()
                        .parent()
                        .and_then(|node| node.cast::<JsCallExpression>());
                    if check_is_window_specific_api(&ident_name, member_name, call_expr)
                        || check_is_web_worker_specific_api(&ident_name, member_name)
                    {
                        return None;
                    }
                }
                _ => {}
            }
            Some(expr.range())
        }
        UseGlobalThisQuery::JsIdentifierAssignment(assignment) => {
            let name = assignment.to_trimmed_text();
            if !is_global_identifier(&name) || model.binding(assignment).is_some() {
                return None;
            }
            Some(assignment.range())
        }
    }
}

const EVENT_TARGET_METHODS: [&str; 3] =
    ["addEventListener", "removeEventListener", "dispatchEvent"];

fn check_is_window_specific_api(
    obj_name: &str,
    member_name: &str,
    call_expr: Option<JsCallExpression>,
) -> bool {
    if obj_name != "window" {
        return false;
    }

    if !is_window_specific_api(member_name) {
        return false;
    }

    // For addEventListener/removeEventListener/dispatchEvent, check if the event is window-specific
    if let Some(call_expr) = call_expr
        && EVENT_TARGET_METHODS.contains(&member_name)
        && let Ok(args) = call_expr.arguments()
        && let Some(Ok(first_arg)) = args.args().first()
    {
        let event = first_arg.to_trimmed_text();
        let event = event.trim_matches(['\'', '"', '`']);
        return is_window_specific_event(event);
    }

    // For other window-specific APIs, always return true
    true
}

fn check_is_web_worker_specific_api(obj_name: &str, member_name: &str) -> bool {
    obj_name == "self" && is_web_workers_specific_api(member_name)
}

fn is_global_identifier(name: &str) -> bool {
    GLOBAL_IDENTIFIERS.contains(&name)
}

fn is_window_specific_api(name: &str) -> bool {
    WINDOW_SPECIFIC_APIS.contains(&name)
}

fn is_web_workers_specific_api(name: &str) -> bool {
    WEB_WORKER_SPECIFIC_APIS.contains(&name)
}

fn is_window_specific_event(name: &str) -> bool {
    WINDOW_SPECIFIC_EVENTS.contains(&name)
}

use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    JsCallExpression, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsIdentifierAssignment, JsIdentifierExpression, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeCast, TextRange, TokenText, declare_node_union};
use biome_rule_options::use_global_this::UseGlobalThisOptions;

use crate::services::semantic::Semantic;

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
        version: "2.3.14",
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
                    "Prefer "<Emphasis>"globalThis"</Emphasis>" over "<Emphasis>"window"</Emphasis>", "<Emphasis>"self"</Emphasis>" and "<Emphasis>"global"</Emphasis>"."
                },
            )
            .note(markup! {
                ""<Emphasis>"globalThis"</Emphasis>" is the standard way to access the global object across environments, which improves code portability."
            }),
        )
    }
}

fn check_expression(expr: &UseGlobalThisQuery, model: &SemanticModel) -> Option<TextRange> {
    match expr {
        UseGlobalThisQuery::JsIdentifierExpression(expr) => {
            let reference = expr.name().ok()?;
            let ident_name = reference.value_token().ok()?;
            let ident_name = ident_name.token_text_trimmed();
            if !is_global_identifier(&ident_name) || model.binding(&reference).is_some() {
                return None;
            }

            let parent = expr.syntax().parent()?;
            match parent.kind() {
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                    if let Some(assignment) = parent.cast::<JsStaticMemberAssignment>()
                        && let Ok(member) = assignment.member()
                        && let Some(member_name) = member.as_js_name()
                        && let Ok(member_name) = member_name.value_token()
                    {
                        let member_name = member_name.token_text_trimmed();
                        if check_is_window_specific_api(&ident_name, &member_name, None)
                            || check_is_web_worker_specific_api(&ident_name, &member_name)
                        {
                            return None;
                        }
                    }
                }
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                    if let Some(expr) = parent.cast::<JsStaticMemberExpression>()
                        && let Ok(member) = expr.member()
                        && let Some(member_name) = member.as_js_name()
                        && let Ok(member_name) = member_name.value_token()
                    {
                        let member_name = member_name.token_text_trimmed();
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
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                    if let Some(assignment) = parent.cast::<JsComputedMemberAssignment>()
                        && let Ok(member_expr) = assignment.member()
                        && let Some(member_expr) = member_expr.as_any_js_literal_expression()
                        && let Some(member_expr) = member_expr.as_js_string_literal_expression()
                        && let Ok(member_name) = member_expr.inner_string_text()
                        && (check_is_window_specific_api(&ident_name, &member_name, None)
                            || check_is_web_worker_specific_api(&ident_name, &member_name))
                        {
                            return None;
                        }
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                    if let Some(expr) = parent.cast::<JsComputedMemberExpression>()
                        && let Ok(member_expr) = expr.member()
                        && let Some(member_expr) = member_expr.as_any_js_literal_expression()
                        && let Some(member_expr) = member_expr.as_js_string_literal_expression()
                        && let Ok(member_name) = member_expr.inner_string_text()
                    {
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
                }
                _ => {}
            }
            Some(expr.range())
        }
        UseGlobalThisQuery::JsIdentifierAssignment(assignment) => {
            let name = assignment.name_token().ok()?;
            let name = name.token_text_trimmed();
            if !is_global_identifier(&name) || model.binding(assignment).is_some() {
                return None;
            }
            Some(assignment.range())
        }
    }
}

const GLOBAL_IDENTIFIERS: [&str; 3] = ["global", "self", "window"];

const WINDOW_SPECIFIC_EVENTS: [&str; 15] = [
    "beforeunload", // Browsers might have specific behaviors on exactly `window.onbeforeunload =`
    "blur",
    "focus",
    "load",
    "message",
    "messageerror",
    "pagehide",
    "pagereveal",
    "pageshow",
    "pageswap",
    "resize",
    "scroll",
    "scrollend",
    "unload",
    "wheel",
];

// Window-specific properties, methods, and event handlers
// References:
// - https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-window-object
// - https://dom.spec.whatwg.org/#idl-index
// - https://drafts.csswg.org/cssom-view/#idl-index
const WINDOW_SPECIFIC_APIS: [&str; 66] = [
    "addEventListener",
    "blur",
    "close",
    "closed",
    "devicePixelRatio",
    "dispatchEvent",
    "event", // Deprecated and quirky, best left untouched
    "focus",
    "frameElement",
    "frames",
    "innerHeight",
    "innerWidth",
    "length",
    "locationbar",
    "menubar",
    "moveBy",
    "moveTo",
    "name",
    "navigation",
    "onbeforeunload", // Browsers might have specific behaviors on exactly `window.onbeforeunload =`
    "onblur",
    "onfocus",
    "onload",
    "onmessage",
    "onmessageerror",
    "onpagehide",
    "onpagereveal",
    "onpageshow",
    "onpageswap",
    "onresize",
    "onscroll",
    "onscrollend",
    "onunload",
    "onwheel",
    "open",
    "opener",
    "originAgentCluster",
    "outerHeight",
    "outerWidth",
    "pageXOffset",
    "pageYOffset",
    "parent",
    "personalbar",
    "postMessage",
    "removeEventListener",
    "resizeBy",
    "resizeTo",
    "screen",
    "screenHeight",
    "screenLeft",
    "screenTop",
    "screenWidth",
    "screenX",
    "screenY",
    "scroll",
    "scrollBy",
    "scrollTo",
    "scrollX",
    "scrollY",
    "scrollbars",
    "status",
    "statusbar",
    "stop",
    "toolbar",
    "top",
    "visualViewport",
];

// Web Worker-specific properties, methods, and event handlers
// References:
// - https://html.spec.whatwg.org/multipage/workers.html#the-workerglobalscope-common-interface
// - https://html.spec.whatwg.org/multipage/workers.html#dedicated-workers-and-the-dedicatedworkerglobalscope-interface
// - https://html.spec.whatwg.org/multipage/workers.html#sharedworkerglobalscope
const WEB_WORKER_SPECIFIC_APIS: [&str; 17] = [
    "addEventListener",
    "close",
    "dispatchEvent",
    "importScripts",
    "location",
    "name",
    "navigator",
    "onconnect",
    "onerror",
    "onlanguagechange",
    "onoffline",
    "ononline",
    "onrejectionhandled",
    "onunhandledrejection",
    "postMessage",
    "removeEventListener",
    "self",
];

const EVENT_TARGET_METHODS: [&str; 3] =
    ["addEventListener", "removeEventListener", "dispatchEvent"];

fn check_is_window_specific_api(
    obj_name: &TokenText,
    member_name: &TokenText,
    call_expr: Option<JsCallExpression>,
) -> bool {
    if obj_name.text() != "window" {
        return false;
    }

    if !is_window_specific_api(member_name) {
        return false;
    }

    // For addEventListener/removeEventListener/dispatchEvent, check if the event is window-specific
    if let Some(call_expr) = call_expr 
        && EVENT_TARGET_METHODS.contains(&member_name.text()) {

        if let Ok(args) = call_expr.arguments()
            && let Some(Ok(first_arg)) = args.args().first()
            && let Some(first_arg_expr) = first_arg.as_any_js_expression()
            && let Some(first_arg_expr) = first_arg_expr.as_any_js_literal_expression()
            && let Some(first_arg_expr) = first_arg_expr.as_js_string_literal_expression()
            && let Ok(event) = first_arg_expr.inner_string_text() {
                return is_window_specific_event(&event);
            } else {
                return false;
            }
    }

    // For other window-specific APIs, always return true
    true
}

fn check_is_web_worker_specific_api(obj_name: &TokenText, member_name: &TokenText) -> bool {
    obj_name.text() == "self" && is_web_workers_specific_api(member_name)
}

fn is_global_identifier(name: &str) -> bool {
    GLOBAL_IDENTIFIERS.binary_search(&name).is_ok()
}

fn is_window_specific_api(name: &str) -> bool {
    WINDOW_SPECIFIC_APIS.binary_search(&name).is_ok()
}

fn is_web_workers_specific_api(name: &str) -> bool {
    WEB_WORKER_SPECIFIC_APIS.binary_search(&name).is_ok()
}

fn is_window_specific_event(name: &str) -> bool {
    WINDOW_SPECIFIC_EVENTS.binary_search(&name).is_ok()
}

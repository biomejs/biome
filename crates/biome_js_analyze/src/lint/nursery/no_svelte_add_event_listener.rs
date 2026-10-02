use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsNamedImportSpecifier, JsCallExpression, JsImport,
    T,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_svelte_add_event_listener::NoSvelteAddEventListenerOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow `addEventListener` in Svelte files.
    ///
    /// Svelte relies on event delegation for performance and to guarantee the order in which
    /// event handlers run. Listeners attached with `addEventListener` bypass this mechanism, so
    /// they may run before or after delegated handlers in an unexpected order. The `on`
    /// function from `svelte/events` attaches listeners that cooperate with event delegation,
    /// and it returns a function that removes the listener.
    ///
    /// The rule provides a fix when `on` is already imported from `svelte/events`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// window.addEventListener("resize", handler);
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// addEventListener("message", handler);
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// import { on } from "svelte/events";
    ///
    /// on(window, "resize", handler);
    /// </script>
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte event delegation](https://svelte.dev/docs/svelte/basic-markup#Events-Event-delegation)
    /// - [`on` from `svelte/events`](https://svelte.dev/docs/svelte/svelte-events#on)
    pub NoSvelteAddEventListener {
        version: "next",
        name: "noSvelteAddEventListener",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-add-event-listener").same()],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

const ADD_EVENT_LISTENER: &str = "addEventListener";
const SVELTE_EVENTS_MODULE: &str = "svelte/events";

pub enum RuleState {
    /// `target.addEventListener(...)`; holds `target`.
    Member(AnyJsExpression),
    /// `addEventListener(...)` referring to the global function.
    Global,
}

impl Rule for NoSvelteAddEventListener {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoSvelteAddEventListenerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<JsFileSource>().as_embedding_kind().is_svelte() {
            return None;
        }

        match ctx.query().callee().ok()? {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let name = member.member().ok()?.value_token().ok()?;
                (name.text_trimmed() == ADD_EVENT_LISTENER)
                    .then(|| member.object().ok().map(RuleState::Member))?
            }
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let reference = ident.name().ok()?;
                (reference.value_token().ok()?.text_trimmed() == ADD_EVENT_LISTENER
                    && ctx.model().binding(&reference).is_none())
                .then_some(RuleState::Global)
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected use of "<Emphasis>"addEventListener"</Emphasis>"."
                },
            )
            .note(markup! {
                "Listeners attached with "<Emphasis>"addEventListener"</Emphasis>" bypass Svelte's event delegation, so they may not run in the expected order relative to other event handlers."
            })
            .note(markup! {
                "Use the "<Emphasis>"on"</Emphasis>" function from "<Emphasis>"svelte/events"</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let model = ctx.model();

        // Only fix when `on` already refers to the `svelte/events` import, so the fix doesn't
        // produce an unresolved reference or bind to an unrelated `on`.
        let on_binding = resolve_in_scope(model, call, "on")?;
        if !is_svelte_events_on(&on_binding) {
            return None;
        }

        let target = match state {
            RuleState::Member(object) => {
                let callee = call.callee().ok()?;
                let member = callee.as_js_static_member_expression()?;
                // `target?.addEventListener()` short-circuits when `target` is nullish,
                // whereas `on(target)` throws.
                if member.is_optional() || object.as_js_super_expression().is_some() {
                    return None;
                }
                match object.clone().omit_parentheses() {
                    // Keep the parentheses so the sequence stays a single argument.
                    AnyJsExpression::JsSequenceExpression(_) => object.clone(),
                    inner => inner,
                }
            }
            RuleState::Global => {
                // Don't reference a local `window` that shadows the global one.
                if resolve_in_scope(model, call, "window").is_some() {
                    return None;
                }
                make::js_identifier_expression(make::js_reference_identifier(make::ident(
                    "window",
                )))
                .into()
            }
        };

        let callee = call.callee().ok()?;
        let callee_syntax = callee.syntax();
        let on_token = make::ident("on")
            .with_leading_trivia_pieces(callee_syntax.first_leading_trivia()?.pieces())
            .with_trailing_trivia_pieces(callee_syntax.last_trailing_trivia()?.pieces());
        let new_callee: AnyJsExpression =
            make::js_identifier_expression(make::js_reference_identifier(on_token)).into();

        let args = call.arguments().ok()?.args();
        let mut items = vec![AnyJsCallArgument::AnyJsExpression(target.trim_trivia()?)];
        let mut separators = Vec::new();
        if !args.is_empty() {
            separators.push(
                make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            );
        }
        for element in args.elements() {
            items.push(element.node().ok()?.clone());
            if let Some(separator) = element.trailing_separator().ok()? {
                separators.push(separator.clone());
            }
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(callee, new_callee);
        mutation.replace_node(args, make::js_call_argument_list(items, separators));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"on"</Emphasis>" from "<Emphasis>"svelte/events"</Emphasis>" instead." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Finds the binding named `name` visible from `call`.
fn resolve_in_scope(model: &SemanticModel, call: &JsCallExpression, name: &str) -> Option<Binding> {
    model
        .scope(call.syntax())
        .ancestors()
        .find_map(|scope| scope.get_binding(name))
}

/// Whether `binding` is `on` imported from `svelte/events`.
fn is_svelte_events_on(binding: &Binding) -> bool {
    let syntax = binding.syntax();
    let Some(specifier) = syntax.ancestors().find_map(AnyJsNamedImportSpecifier::cast) else {
        return false;
    };
    if specifier
        .imported_name()
        .is_none_or(|name| name.text_trimmed() != "on")
    {
        return false;
    }
    syntax
        .ancestors()
        .find_map(JsImport::cast)
        .and_then(|import| import.source_text().ok())
        .is_some_and(|source| source.text() == SVELTE_EVENTS_MODULE)
}

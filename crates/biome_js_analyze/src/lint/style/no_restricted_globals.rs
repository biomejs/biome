use crate::services::semantic::SemanticServices;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, BindingExtensions};
use biome_js_syntax::{AnyJsIdentifierUsage, JsSyntaxToken};
use biome_rowan::AstNode;
use biome_rule_options::no_restricted_globals::NoRestrictedGlobalsOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// This rule allows you to specify global variable names that you don’t want to use in your application.
    ///
    /// References to the global identifiers `error` and `event` are always disallowed by this rule.
    ///
    /// > Disallowing usage of specific global variables can be useful if you want to allow a set of
    /// global variables by enabling an environment but still want to disallow some of those.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log(event)
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// function f(event) {
    ///     console.log(event)
    /// }
    /// ```
    /// ## Options
    ///
    /// Use the options to specify additional globals that you want to restrict in your
    /// source code.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "deniedGlobals": {
    ///             "$": "jQuery is not allowed. Use native DOM manipulation instead.",
    ///             "MooTools": "Do not use MooTools, use MeowTools instead."
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// In the example above, the rule will emit a diagnostics if tried to use `$` or `MooTools` without
    /// creating a local variable.
    ///
    pub NoRestrictedGlobals {
        version: "1.0.0",
        name: "noRestrictedGlobals",
        language: "js",
        sources: &[RuleSource::Eslint("no-restricted-globals").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

const RESTRICTED_GLOBALS: [&str; 2] = ["event", "error"];

impl Rule for NoRestrictedGlobals {
    type Query = SemanticServices;
    type State = JsSyntaxToken;
    type Signals = Box<[Self::State]>;
    type Options = NoRestrictedGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let options = ctx.options();

        let unresolved_reference_nodes = model
            .all_unresolved_references()
            .map(|reference| reference.syntax().clone());
        let global_references_nodes = model
            .all_global_references()
            .map(|reference| reference.syntax().clone());

        unresolved_reference_nodes
            .chain(global_references_nodes)
            .filter_map(|node| {
                let node = AnyJsIdentifierUsage::unwrap_cast(node);
                let (token, binding) = match node {
                    AnyJsIdentifierUsage::JsReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyJsIdentifierUsage::JsxReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyJsIdentifierUsage::JsIdentifierAssignment(node) => {
                        (node.name_token(), node.binding(model))
                    }
                };
                let token = token.ok()?;
                let text = token.text_trimmed();

                if is_restricted(text, &binding, options.denied_globals.as_ref()) {
                    Some(token)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, token: &Self::State) -> Option<RuleDiagnostic> {
        let text = token.text_trimmed();
        let message = custom_restricted_message(text, _ctx.options().denied_globals.as_ref());

        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            token.text_trimmed_range(),
            markup! {
                "Do not use the global variable "<Emphasis>{text}</Emphasis>"."
            },
        );

        if let Some(message) = message {
            diag = diag.note(message);
        } else {
            diag = diag.note(markup! { "Use a local variable instead." });
        }

        Some(diag)
    }
}

fn is_restricted(
    name: &str,
    binding: &Option<Binding>,
    denied_globals: Option<&FxHashMap<Box<str>, Box<str>>>,
) -> bool {
    if binding.is_some() {
        return false;
    }

    if RESTRICTED_GLOBALS.contains(&name) {
        return true;
    }

    denied_globals.is_some_and(|denied_globals| denied_globals.contains_key(name))
}

fn custom_restricted_message<'a>(
    name: &str,
    denied_globals: Option<&'a FxHashMap<Box<str>, Box<str>>>,
) -> Option<&'a str> {
    denied_globals
        .and_then(|denied_globals| denied_globals.get(name))
        .map(|message| message.as_ref())
}

use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::Binding;
use biome_js_syntax::{AnyJsIdentifierReference, JsSyntaxToken};
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
    type Query = Semantic<AnyJsIdentifierReference>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let reference = ctx.query();
        let options = ctx.options();

        if !model.is_unresolved_reference(reference) && !model.is_global_reference(reference) {
            return None;
        }
        let token = reference.value_token().ok()?;
        let binding = model.binding(reference);
        is_restricted(
            token.text_trimmed(),
            &binding,
            options.denied_globals.as_ref(),
        )
        .then_some(token)
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

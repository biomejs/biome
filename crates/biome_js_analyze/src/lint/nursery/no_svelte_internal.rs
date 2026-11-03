use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsImportLike, inner_string_text};
use biome_rowan::TextRange;
use biome_rule_options::no_svelte_internal::NoSvelteInternalOptions;

declare_lint_rule! {
    /// Disallow importing from `svelte/internal`.
    ///
    /// The `svelte/internal` module and its submodules, such as `svelte/internal/client`, hold
    /// code that Svelte uses for itself. They aren't part of the public API, so they can change
    /// without warning in any release. In Svelte 5, importing from `svelte/internal` itself
    /// throws an error, because that module only existed for Svelte 4.
    ///
    /// Import from public modules such as `svelte` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { get_current_component } from "svelte/internal";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { inspect } from "svelte/internal/client";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export * from "svelte/internal/server";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await import("svelte/internal");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { mount } from "svelte";
    /// import { writable } from "svelte/store";
    /// ```
    ///
    pub NoSvelteInternal {
        version: "2.6.3",
        name: "noSvelteInternal",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-svelte-internal").same()],
        recommended: true,
    }
}

impl Rule for NoSvelteInternal {
    type Query = Ast<AnyJsImportLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoSvelteInternalOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }

        let module_name_token = node.module_name_token()?;
        let import_path = inner_string_text(&module_name_token);

        is_svelte_internal(import_path.text()).then(|| module_name_token.text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Don't import from "<Emphasis>"svelte/internal"</Emphasis>"."
                },
            )
            .note(markup! {
                "This module is private to Svelte and can change or disappear in any release."
            })
            .note(markup! {
                "Import from a public module, such as "<Emphasis>"svelte"</Emphasis>", instead."
            }),
        )
    }
}

const SVELTE_INTERNAL_MODULE: &str = "svelte/internal";

fn is_svelte_internal(path: &str) -> bool {
    path.strip_prefix(SVELTE_INTERNAL_MODULE)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
}

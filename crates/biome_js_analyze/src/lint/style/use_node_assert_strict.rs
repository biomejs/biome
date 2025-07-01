use crate::JsRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsImportLike, JsSyntaxKind, JsSyntaxToken, inner_string_text};
use biome_rowan::BatchMutationExt;
use biome_rule_options::use_node_assert_strict::UseNodeAssertStrictOptions;

declare_lint_rule! {
    /// Promotes the usage of `node:assert/strict` over `node:assert`.
    ///
    /// If you prefer stricter assertions when using the Node.js assertion module, the package `node:assert/strict` exposes a set of alias for stricter assertions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import * as assert from "node:assert"
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import * as assert from "node:assert/strict"
    /// ```
    ///
    pub UseNodeAssertStrict {
        version: "1.6.0",
        name: "useNodeAssertStrict",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNodeAssertStrict {
    type Query = Ast<AnyJsImportLike>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = UseNodeAssertStrictOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }
        let module_name = node.module_name_token()?;
        if inner_string_text(&module_name) == "node:assert" {
            return Some(module_name);
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().module_name_token()?.text_trimmed_range(),
                markup! {
                    "Use "<Emphasis>"node:assert/strict"</Emphasis>" instead."
                },
            )
            .note(markup! {
                "The use of stricter assertion is preferred."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, module_name: &Self::State) -> Option<JsRuleAction> {
        let new_module_name = JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_STRING_LITERAL,
            "\"node:assert/strict\"",
            [],
            [],
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_token(module_name.clone(), new_module_name);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>"node:assert/strict"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

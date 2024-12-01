use crate::JsRuleAction;
use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{inner_string_text, AnyJsImportLike, JsSyntaxKind, JsSyntaxToken};
use biome_rowan::BatchMutationExt;

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
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNodeAssertStrict {
    type Query = Ast<AnyJsImportLike>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

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

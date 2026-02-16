use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};
use biome_rule_options::no_playwright_element_handle::NoPlaywrightElementHandleOptions;

use crate::JsRuleAction;
use crate::frameworks::playwright::get_page_or_frame_name;

declare_lint_rule! {
    /// Disallow usage of element handles (`page.$()` and `page.$$()`).
    ///
    /// Element handles are discouraged in Playwright. Use locators instead, which auto-wait
    /// and are more reliable. Locators represent a way to find elements at any moment,
    /// while element handles are references to specific elements that may become stale.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const button = await page.$('button');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const buttons = await page.$$('.btn');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const element = await frame.$('#element');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const button = page.locator('button');
    /// await button.click();
    /// ```
    ///
    /// ```js
    /// const buttons = page.locator('.btn');
    /// await expect(buttons).toHaveCount(3);
    /// ```
    ///
    /// ```js
    /// await page.getByRole('button', { name: 'Submit' }).click();
    /// ```
    ///
    pub NoPlaywrightElementHandle {
        version: "2.4.2",
        name: "noPlaywrightElementHandle",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-element-handle").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoPlaywrightElementHandle {
    type Query = Ast<JsCallExpression>;
    type State = ElementHandleCall;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightElementHandleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let member_name = member_expr.member().ok()?;
        let member_str = member_name
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed();

        // Check if the method is $ or $$
        if member_str != "$" && member_str != "$$" {
            return None;
        }

        let object = member_expr.object().ok()?;
        let receiver = get_page_or_frame_name(&object)?;
        Some(ElementHandleCall {
            receiver,
            method: member_str,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let receiver = state.receiver.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of element handles."
                },
            )
            .note(markup! {
                "Locators auto-wait and are more reliable than element handles, which can become stale."
            })
            .note(markup! {
                "Use "<Emphasis>{receiver}".locator()"</Emphasis>" or "<Emphasis>"getByRole()"</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;
        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;
        let mut mutation = ctx.root().begin();

        let method = state.method.text();

        // Create the new member name "locator"
        let new_member = make::js_name(make::ident("locator"));

        // Replace the member ($ or $$) with "locator"
        let old_member = member_expr.member().ok()?;
        mutation.replace_element(old_member.into(), new_member.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{method}"()"</Emphasis>" with "<Emphasis>"locator()"</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}

pub struct ElementHandleCall {
    receiver: TokenText,
    method: TokenText,
}

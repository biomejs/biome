use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallExpression, JsStaticMemberExpression, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TokenText, TriviaPieceKind};
use biome_rule_options::no_playwright_wait_for_selector::NoPlaywrightWaitForSelectorOptions;

use crate::JsRuleAction;
use crate::frameworks::playwright::get_page_or_frame_name;

declare_lint_rule! {
    /// Disallow using `page.waitForSelector()`.
    ///
    /// Playwright's `page.waitForSelector()` is discouraged in favor of more reliable locator-based APIs.
    /// Using locators with assertions or actions automatically waits for elements to be ready.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForSelector('.submit-button');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForSelector('#dialog', { state: 'visible' });
    /// await page.click('#dialog .button');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.locator('.submit-button').click();
    /// ```
    ///
    /// ```js
    /// await expect(page.locator('#dialog')).toBeVisible();
    /// ```
    ///
    /// ```js
    /// const button = page.getByRole('button', { name: 'Submit' });
    /// await button.click();
    /// ```
    ///
    pub NoPlaywrightWaitForSelector {
        version: "2.4.2",
        name: "noPlaywrightWaitForSelector",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-wait-for-selector").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoPlaywrightWaitForSelector {
    type Query = Ast<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightWaitForSelectorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;

        if member_text.text_trimmed() != "waitForSelector" {
            return None;
        }

        let object = member_expr.object().ok()?;
        get_page_or_frame_name(&object)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let receiver = state.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>{receiver}".waitForSelector()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Locators automatically wait for elements to be ready, making explicit waits unnecessary."
            })
            .note(markup! {
                "Use locator-based "<Emphasis>{receiver}".locator()"</Emphasis>" or "<Emphasis>{receiver}".getByRole()"</Emphasis>" APIs instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;
        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;
        let args = call_expr.arguments().ok()?;
        let mut mutation = ctx.root().begin();

        // Get the arguments: first arg is the selector, optional second arg is options
        let arg_list: Vec<_> = args.args().iter().collect();
        if arg_list.is_empty() {
            return None;
        }

        let selector_arg = arg_list.first()?.clone().ok()?;
        let options_arg = arg_list.get(1).and_then(|a| a.clone().ok());

        // Build: page.locator(selector).waitFor() or page.locator(selector).waitFor(options)
        let object = member_expr.object().ok()?;

        // Create page.locator(selector)
        let locator_member = make::js_static_member_expression(
            object.clone(),
            make::token(T![.]),
            make::js_name(make::ident("locator")).into(),
        );

        let locator_args =
            make::js_call_arguments(make::token(T!['(']), make_arg_list([selector_arg]), make::token(T![')']));

        let locator_call = make::js_call_expression(
            AnyJsExpression::JsStaticMemberExpression(locator_member),
            locator_args,
        )
        .build();

        // Create .waitFor() or .waitFor(options)
        let wait_for_member = make::js_static_member_expression(
            AnyJsExpression::JsCallExpression(locator_call),
            make::token(T![.]),
            make::js_name(make::ident("waitFor")).into(),
        );

        let wait_for_args = if let Some(opts) = options_arg {
            make::js_call_arguments(make::token(T!['(']), make_arg_list([opts.clone()]), make::token(T![')']))
        } else {
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list([], []),
                make::token(T![')']),
            )
        };

        let wait_for_call = make::js_call_expression(
            AnyJsExpression::JsStaticMemberExpression(wait_for_member),
            wait_for_args,
        )
        .build();

        mutation.replace_node_transfer_trivia(call_expr.clone(), wait_for_call)?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>"locator(selector).waitFor()"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Helper to create an argument list from an iterator of call arguments
fn make_arg_list(
    args: impl IntoIterator<Item = AnyJsCallArgument>,
) -> biome_js_syntax::JsCallArgumentList {
    let args: Vec<_> = args.into_iter().collect();
    let len = args.len();
    if len == 0 {
        return make::js_call_argument_list([], []);
    }

    let mut separators = Vec::new();
    for _ in 0..len.saturating_sub(1) {
        separators.push(make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
    }

    make::js_call_argument_list(args, separators)
}

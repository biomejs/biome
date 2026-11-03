use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_playwright_nth_methods::NoPlaywrightNthMethodsOptions;

declare_lint_rule! {
    /// Disallow selecting Playwright elements by position with `first()`, `last()`, and `nth()`.
    ///
    /// In Playwright, a locator describes how to find elements on the page.
    /// When a locator matches several elements, `first()`, `last()`, and `nth()`
    /// pick one of them by its position.
    /// The position of an element can change when matching elements are added,
    /// removed, or reordered, so the test can end up acting on a different
    /// element without any error.
    /// A locator that matches only the element you want keeps working when the
    /// rest of the page changes.
    ///
    /// This rule reports every call to a method named `first`, `last`, or `nth`,
    /// whatever object the method is called on.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.locator("button").first().click();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.locator("button").last().click();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.locator("button").nth(3).click();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.getByRole("button", { name: "Submit" }).click();
    /// await page.getByTestId("submit-button").click();
    /// await page.getByRole("listitem").filter({ hasText: "Product 2" }).click();
    /// ```
    ///
    pub NoPlaywrightNthMethods {
        version: "2.6.3",
        name: "noPlaywrightNthMethods",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-nth-methods").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightNthMethods {
    type Query = Ast<JsCallExpression>;
    type State = NthMethod;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightNthMethodsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = called_member(ctx.query())?.member_name()?;
        NthMethod::from_name(name.text())
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        let start = match called_member(call)? {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.range().start()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.l_brack_token().ok()?.text_trimmed_range().start()
            }
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::new(start, call.range().end()),
                markup! {
                    <Emphasis>{method}</Emphasis>" selects an element by its position."
                },
            )
            .note(markup! {
                "When elements are added, removed, or reordered on the page, the test can act on a different element."
            })
            .note(markup! {
                "Use a locator that matches only the element you want, such as "<Emphasis>"getByRole()"</Emphasis>" with a name or "<Emphasis>"getByTestId()"</Emphasis>", or narrow the matches with "<Emphasis>"filter()"</Emphasis>"."
            }),
        )
    }
}

/// A Playwright locator method that picks an element by its position.
#[derive(Clone, Copy)]
pub enum NthMethod {
    First,
    Last,
    Nth,
}

impl NthMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "first" => Some(Self::First),
            "last" => Some(Self::Last),
            "nth" => Some(Self::Nth),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for NthMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::First => "first()",
            Self::Last => "last()",
            Self::Nth => "nth()",
        })
    }
}

/// Returns the member expression that `call` invokes, such as `locator.first`
/// in `locator.first()`.
fn called_member(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

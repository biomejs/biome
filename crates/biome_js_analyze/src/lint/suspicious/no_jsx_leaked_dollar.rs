use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxChild, JsSyntaxKind, JsSyntaxToken, JsxChildList, JsxText};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange, TextSize};
use biome_rule_options::no_jsx_leaked_dollar::NoJsxLeakedDollarOptions;

declare_lint_rule! {
    /// Flags text nodes with a trailing `$` before a JSX expression.
    ///
    /// This can happen when refactoring from a template literal to JSX and forgetting
    /// to remove the dollar sign. This results in an unintentional `$` being rendered
    /// as text in the output.
    ///
    /// ```jsx
    /// function MyComponent({ user }) {
    ///   return `Hello ${user.name}`;
    /// }
    /// ```
    ///
    /// When refactored to JSX, it might look like this:
    ///
    /// ```jsx,ignore
    /// function MyComponent({ user }) {
    ///   return <>Hello ${user.name}</>;
    /// }
    /// ```
    ///
    /// However, the `$` before `{user.name}` is unnecessary and will be rendered as text in the output.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function MyComponent({ user }) {
    ///   return <div>Hello ${user.name}</div>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// function MyComponent({ user }) {
    ///   return <div>${user.name} is your name</div>;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function MyComponent({ user }) {
    ///   return <div>Hello {user.name}</div>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// // A lone `$` before a single expression is treated as intentional (e.g. a price).
    /// function MyComponent({ price }) {
    ///   return <div>${price}</div>;
    /// }
    /// ```
    ///
    pub NoJsxLeakedDollar {
        version: "2.4.13",
        name: "noJsxLeakedDollar",
        language: "jsx",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        severity: Severity::Warning,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactJsx("no-leaked-dollar").same(), RuleSource::EslintReactXyz("jsx-no-leaked-dollar").same()],
    }
}

impl Rule for NoJsxLeakedDollar {
    type Query = Ast<JsxText>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoJsxLeakedDollarOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let value_token = node.value_token().ok()?;
        let text = value_token.text();

        // Check if the text ends with `$`
        if !text.ends_with('$') {
            return None;
        }

        // Check if the next sibling is a JsxExpressionChild
        let next_sibling = node.syntax().next_sibling()?;
        if next_sibling.kind() != JsSyntaxKind::JSX_EXPRESSION_CHILD {
            return None;
        }

        // Exception: if the text is exactly "$" and the parent has only 2 children,
        // it looks like an intentional dollar sign (e.g. `<div>${price}</div>`).
        if text == "$"
            && let Some(parent) = node.syntax().parent()
            && let Some(parent) = JsxChildList::cast(parent)
            && parent.len() == 2
        {
            return None;
        }

        // Return the range of the trailing `$` character
        let end = value_token.text_range().end();
        let start = end - TextSize::from(1u32);
        Some(TextRange::new(start, end))
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Possible unintentional "<Emphasis>"'$'"</Emphasis>" before a JSX expression."
                },
            )
            .note(markup! {
                "This "<Emphasis>"'$'"</Emphasis>" will be rendered as text. Remove the "<Emphasis>"'$'"</Emphasis>" from the text node or add a suppression if it is intentional."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let value_token = node.value_token().ok()?;
        let text = value_token.text();

        // Remove the trailing `$`
        let new_text = text[..text.len() - 1].to_string();

        let new_token = JsSyntaxToken::new_detached(JsSyntaxKind::JSX_TEXT, &new_text, [], []);
        let new_jsx_text = AnyJsxChild::JsxText(make::jsx_text(new_token));
        let mut mutation = ctx.root().begin();
        mutation.replace_node(AnyJsxChild::from(node.clone()), new_jsx_text);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove dollar sign." }.to_owned(),
            mutation,
        ))
    }
}

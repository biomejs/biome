use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxChild, JsSyntaxKind, JsSyntaxToken, JsxText};
use biome_rowan::{BatchMutationExt, TextRange, TextSize};
use biome_rule_options::no_jsx_leaked_semicolon::NoJsxLeakedSemicolonOptions;

declare_lint_rule! {
    /// Flags text nodes with a leading `;` after a JSX element.
    ///
    /// When refactoring JSX, trailing semicolons may be accidentally left immediately
    /// after JSX elements or fragments. This causes `;` to be unexpectedly rendered
    /// as text nodes.
    ///
    /// ```jsx
    /// function MyComponent() {
    ///   return <div />;
    /// }
    /// ```
    ///
    /// When refactored to add a wrapper, it might look like this:
    ///
    /// ```jsx,ignore
    /// function MyComponent() {
    ///   return (
    ///     <div>
    ///       <div />;
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    /// However, the `;` after the JSX element is unnecessary and will be rendered as text in the output.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function MyComponent() {
    ///   return (
    ///     <div>
    ///       <div />;
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// function MyComponent() {
    ///   return (
    ///     <div>
    ///       <Component>
    ///         <div />
    ///       </Component>;
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function MyComponent() {
    ///  return (
    ///     <div>
    ///       <div />
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    /// ```jsx
    /// function MyComponent() {
    ///   return (
    ///     <div>
    ///       <div />
    ///       ;
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    pub NoJsxLeakedSemicolon {
        version: "next",
        name: "noJsxLeakedSemicolon",
        language: "jsx",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        severity: Severity::Warning,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactJsx("no-leaked-semicolon").same(), RuleSource::EslintReactXyz("jsx-no-leaked-semicolon").same()],
    }
}

impl Rule for NoJsxLeakedSemicolon {
    type Query = Ast<JsxText>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoJsxLeakedSemicolonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let value_token = node.value_token().ok()?;
        let text = value_token.text();

        if !(text.starts_with(";\n") || text.starts_with(";\r")) {
            return None;
        }

        let start = value_token.text_range().start();
        let end = start + TextSize::from(1);
        Some(TextRange::new(start, end))
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Possible unintentional "<Emphasis>"';'"</Emphasis>" after a JSX element."
                },
            )
            .note(markup! {
                "This "<Emphasis>"';'"</Emphasis>" will be rendered as text. Remove the "<Emphasis>"';'"</Emphasis>" from the text node or add a suppression if it is intentional."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let value_token = node.value_token().ok()?;
        let text = value_token.text();

        // Remove the leading `;`
        let new_text = text[1..].to_string();

        let new_token = JsSyntaxToken::new_detached(JsSyntaxKind::JSX_TEXT, &new_text, [], []);
        let new_jsx_text = AnyJsxChild::JsxText(make::jsx_text(new_token));
        let mut mutation = ctx.root().begin();
        mutation.replace_node(AnyJsxChild::from(node.clone()), new_jsx_text);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove semicolon." }.to_owned(),
            mutation,
        ))
    }
}

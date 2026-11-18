use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsSyntaxKind, JsTemplateExpression};
use biome_rowan::{AstNode, SyntaxNodeText, TextRange, TextSize};
use biome_rule_options::no_nested_template_literals::NoNestedTemplateLiteralsOptions;

declare_lint_rule! {
    /// Disallow template literals inside other template literals.
    ///
    /// Template literals (strings written with backticks, like `` `Hello ${name}` ``) make it easy to
    /// build strings. When one template literal is written inside the `${...}` part of another,
    /// the code becomes hard to read, because it is no longer clear which backtick closes
    /// which string.
    ///
    /// Move the inner template literal into its own variable instead.
    ///
    /// A nested template literal is allowed when it starts on a different line than the outer
    /// template literal starts, and ends on a different line than the outer template literal ends.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const message = `I have ${color ? `${count} ${color}` : count} apples`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const message = html`<p>${`${count} apples`}</p>`;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const apples = color ? `${count} ${color}` : count;
    /// const message = `I have ${apples} apples`;
    ///
    /// const list = `
    ///   ${items.map((item) => `<li>${item}</li>`)}
    /// `;
    /// ```
    ///
    pub NoNestedTemplateLiterals {
        version: "2.6.0",
        name: "noNestedTemplateLiterals",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintSonarJs("no-nested-template-literals").same()],
    }
}

impl Rule for NoNestedTemplateLiterals {
    type Query = Ast<JsTemplateExpression>;
    /// The closest template literal that contains the queried one.
    type State = JsTemplateExpression;
    type Signals = Option<Self::State>;
    type Options = NoNestedTemplateLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let nested = ctx.query();
        // Only the `${...}` parts count as being inside a template literal. A template literal
        // used as the tag of another one (`` `a``b` ``) is not nested in it.
        let outer = nested
            .syntax()
            .ancestors()
            .skip(1)
            .find(|node| node.kind() == JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST)?
            .parent()
            .and_then(JsTemplateExpression::cast)?;

        let outer_start = outer.l_tick_token().ok()?.text_trimmed_range().start();
        let outer_end = outer.r_tick_token().ok()?.text_trimmed_range().end();
        let nested_start = nested.l_tick_token().ok()?.text_trimmed_range().start();
        let nested_end = nested.r_tick_token().ok()?.text_trimmed_range().end();

        let text = outer.syntax().text_with_trivia();
        let offset = outer.syntax().text_range_with_trivia().start();
        let starts_on_same_line =
            !has_line_break(&text, TextRange::new(outer_start, nested_start), offset);
        let ends_on_same_line =
            !has_line_break(&text, TextRange::new(nested_end, outer_end), offset);

        (starts_on_same_line || ends_on_same_line).then_some(outer)
    }

    fn diagnostic(ctx: &RuleContext<Self>, outer: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This template literal is nested inside another template literal."
                },
            )
            .detail(
                outer.range(),
                markup! {
                    "This is the outer template literal."
                },
            )
            .note(markup! {
                "Nested template literals are hard to read, because it is unclear which backtick ends which string."
            })
            .note(markup! {
                "Move the inner template literal into a separate variable."
            }),
        )
    }
}

/// Returns `true` if the source text in `range` contains a line break.
///
/// `text` is the text of a node starting at `offset`, and `range` uses positions in the whole
/// file, so it must lie inside that node.
fn has_line_break(text: &SyntaxNodeText, range: TextRange, offset: TextSize) -> bool {
    text.slice(range - offset)
        .try_for_each_chunk(|chunk| {
            if chunk.contains(['\n', '\r', '\u{2028}', '\u{2029}']) {
                Err(())
            } else {
                Ok(())
            }
        })
        .is_err()
}

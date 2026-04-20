use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsxText;
use biome_rowan::{TextRange, TextSize};
use biome_rule_options::no_jsx_leaked_comment::NoJsxLeakedCommentOptions;

declare_lint_rule! {
    /// Prevent comments from being inserted as JSX text nodes.
    ///
    /// Comments inside JSX children must be wrapped in braces, otherwise they are rendered as text.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div>// comment</div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>/* comment */</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div>{/* comment */}</div>
    /// ```
    ///
    pub NoJsxLeakedComment {
        version: "next",
        name: "noJsxLeakedComment",
        language: "jsx",
        domains: &[RuleDomain::Qwik, RuleDomain::React],
        sources: &[RuleSource::EslintReact("jsx-no-comment-textnodes").same(), RuleSource::EslintReactJsx("no-comment-textnodes").same(), RuleSource::EslintReactXyz("jsx-no-comment-textnodes").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoJsxLeakedComment {
    type Query = Ast<JsxText>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoJsxLeakedCommentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let jsx_value = node.value_token().ok()?;
        let node_range_start = jsx_value.text_range().start();
        let jsx_value = jsx_value.text();
        let bytes = jsx_value.as_bytes();
        let mut bytes_iter = jsx_value.bytes().enumerate();

        let is_comment_start = |index: usize| -> bool {
            if index == 0 {
                return true;
            }
            let prev_byte = bytes.get(index - 1);
            // Allow comment if preceded by whitespace, but not if it's "://" (URL)
            prev_byte.is_some_and(|&b| {
                b.is_ascii_whitespace() && !(b == b':' && bytes.get(index + 1) == Some(&b'/'))
            })
        };

        while let Some((index, byte)) = bytes_iter.next() {
            if byte != b'/' {
                continue;
            }

            match bytes_iter.next()? {
                (_, b'/') => {
                    if is_comment_start(index) {
                        let end = bytes_iter
                            .find(|(_, current)| current == &b'\n')
                            .map_or(bytes.len(), |(idx, _)| idx);
                        return Some(TextRange::new(
                            node_range_start + TextSize::from(index as u32),
                            node_range_start + TextSize::from(end as u32),
                        ));
                    }
                }
                (_, b'*') => {
                    if is_comment_start(index) {
                        let comment_start = index;
                        let mut end = bytes.len(); // Default to end of text if no closing found

                        while let Some((_, byte)) = bytes_iter.next() {
                            if byte != b'*' {
                                continue;
                            }

                            let Some((close_index, b'/')) = bytes_iter.next() else {
                                continue;
                            };

                            end = close_index + 1;
                            break;
                        }

                        return Some(TextRange::new(
                            node_range_start + TextSize::from(comment_start as u32),
                            node_range_start + TextSize::from(end as u32),
                        ));
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "Unexpected comment syntax in a text node."
            },
        ).note(
            markup! {
                "This comment syntax will be rendered as text. Wrap "<Emphasis>"comments"</Emphasis>" inside JSX children with "<Emphasis>"braces"</Emphasis>" or remove them entirely."
            }
        ))
    }
}

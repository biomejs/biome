use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::AnyJsStatement;
use biome_rowan::{AstNode, TextRange, TextSize};

const IRREGULAR_WHITESPACES: &[char; 22] = &[
    '\u{c}', '\u{b}', '\u{85}', '\u{feff}', '\u{a0}', '\u{1680}', '\u{180e}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200a}', '\u{200b}', '\u{202f}', '\u{205f}', '\u{3000}',
];

declare_rule! {
    /// Disallows the use of irregular whitespace characters.
    ///
    /// Invalid or irregular whitespace causes issues with ECMAScript 5 parsers and also makes code harder to debug.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// constcount=1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = 'thing';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const count = 1;
    /// ```
    ///
    /// ```js
    /// const foo = '';
    /// ```
    ///
    pub NoIrregularWhitespace {
        version: "next",
        name: "noIrregularWhitespace",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyJsStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        get_irregular_whitespace(node)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Irregular whitespaces found."
                },
            )
            .note(markup! {
                "Replace the irregular whitespaces with normal whitespaces or tabs."
            }),
        )
    }
}

fn get_irregular_whitespace(node: &AnyJsStatement) -> Option<()> {
    let syntax = node.syntax();
    let node_text = syntax.text_trimmed();
    let range_start: u32 = node.range().start().into();

    IRREGULAR_WHITESPACES
        .iter()
        .find_map(|whitespace_character| {
            let text_size = node_text
                .find_char(*whitespace_character)?
                .checked_add(TextSize::from(range_start))?;
            let text_range = TextRange::new(text_size, text_size);

            let element_at_index = node
                .range()
                .contains(text_size)
                .then(|| syntax.covering_element(text_range))?;

            let is_string_literal = matches!(
                element_at_index.kind(),
                biome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL
                    | biome_js_syntax::JsSyntaxKind::JSX_TEXT_LITERAL
                    | biome_js_syntax::JsSyntaxKind::TEMPLATE_CHUNK
            );

            if is_string_literal {
                return None;
            }

            node_text
                .chars()
                .find(|char| whitespace_character.eq(char))
                .and(Some(()))
        })
}

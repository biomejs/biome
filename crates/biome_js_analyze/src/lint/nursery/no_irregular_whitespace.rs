use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::AnyJsStatement;
use biome_rowan::{declare_node_union, AstNode, TextRange, TextSize};

const IRREGULAR_WHITESPACES: &[&str; 2] = &[
    "\u{c}",
    "\u{b}",
    // "\u{85}", "\u{feff}", "\u{a0}", "\u{1680}", "\u{180e}", "\u{2000}",
    // "\u{2001}", "\u{2002}", "\u{2003}", "\u{2004}", "\u{2005}", "\u{2006}", "\u{2007}", "\u{2008}",
    // "\u{2009}", "\u{200a}", "\u{200b}", "\u{202f}", "\u{205f}", "\u{3000}",
];

declare_lint_rule! {
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
    /// ### Valid
    ///
    /// ```js
    /// const count = 1;
    /// ```
    ///
    pub NoIrregularWhitespace {
        version: "next",
        name: "noIrregularWhitespace",
        language: "js",
        recommended: false,
    }
}

#[derive(Debug)]
pub struct NoIrregularWhitespaceState {
    pub range: TextRange,
    pub character: &'static str,
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<IrregularWhitespaceNode>;
    type State = NoIrregularWhitespaceState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        get_irregular_whitespace(node)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Irregular whitespace found."
                },
            )
            .note(markup! {
                "Replace the irregular whitespace character with a normal whitespace or tab."
            }),
        )
    }
}

declare_node_union! {
    pub IrregularWhitespaceNode = AnyJsStatement
}

fn get_irregular_whitespace(node: &IrregularWhitespaceNode) -> Option<NoIrregularWhitespaceState> {
    let node_text = node.syntax().text().to_string();

    IRREGULAR_WHITESPACES
        .iter()
        .find_map(|whitespace_character| {
            let character_index = node_text.find(&whitespace_character.to_string())?;
            let range_start = node
                .range()
                .add_start(TextSize::try_from(character_index).ok()?)
                .start();

            let whitespace_token = node
                .syntax()
                .token_at_offset(range_start)
                .flat_map(|token| {
                    token
                        .leading_trivia()
                        .pieces()
                        .chain(token.trailing_trivia().pieces())
                        .filter(|trivia_piece| trivia_piece.is_whitespace())
                        .collect::<Vec<_>>()
                })
                .filter(|whitespace_token| whitespace_token.text().contains(whitespace_character))
                .collect::<Vec<_>>();

            dbg!(whitespace_token);

            Some(NoIrregularWhitespaceState {
                // range: TextRange::new(range_start, range_end),
                range: node.range(),
                character: whitespace_character,
            })
        })
}

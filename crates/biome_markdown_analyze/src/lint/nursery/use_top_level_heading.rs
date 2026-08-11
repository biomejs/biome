use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::{AnyMdBlock, AnyMdLeafBlock, MdRoot};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_top_level_heading::UseTopLevelHeadingOptions;

declare_lint_rule! {
    /// Require Markdown documents to start with a top-level heading.
    ///
    /// Requires the first block in the document to be a level-1 heading,
    /// either ATX (`# Heading`) or setext (`Heading` followed by `===`).
    ///
    /// Leading HTML comments used as file preamble are ignored when determining
    /// the first block.
    ///
    /// The rule does not report when the first block is an HTML block or a thematic break.
    /// HTML blocks are permitted because some projects use HTML markup for their heading, especially in READMEs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```md,expect_diagnostic
    /// Some text
    ///
    /// # Heading
    /// ```
    ///
    /// ### Valid
    ///
    /// ```md
    /// # Heading
    ///
    /// Some text
    /// ```
    ///
    /// ```md
    /// ---
    /// title: Example
    /// ---
    ///
    /// ## Section
    /// ```
    ///
    /// ```md
    /// <div>Intro</div>
    ///
    /// ## Section
    /// ```
    ///
    pub UseTopLevelHeading {
        version: "2.5.8",
        name: "useTopLevelHeading",
        language: "md",
        recommended: false,
        sources: &[RuleSource::MarkdownLint("md041", "first-line-heading").same()],
    }
}

impl Rule for UseTopLevelHeading {
    type Query = Ast<MdRoot>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseTopLevelHeadingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let first_block = root
            .value()
            .iter()
            .find(|block| !is_ignorable_leading_block(block))?;

        match first_block {
            AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdHeader(header)) => {
                if header.level() == 1 {
                    None
                } else {
                    Some(header.range())
                }
            }
            AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdSetextHeader(header)) => {
                if header.is_level_1() {
                    None
                } else {
                    Some(header.range())
                }
            }
            AnyMdBlock::AnyMdLeafBlock(
                AnyMdLeafBlock::MdThematicBreakBlock(_) | AnyMdLeafBlock::MdHtmlBlock(_),
            ) => None,
            _ => Some(first_block.range()),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *state,
                markup! {
                    "Missing top-level heading."
                },
            )
            .note(markup! {
                "The document should start with a top-level heading (h1) so readers and tools can identify its title. Add a "<Emphasis>"# Heading"</Emphasis>" (or a level-1 setext heading) at the start of the document."
            }),
        )
    }
}

fn is_html_comment_block(block: &AnyMdBlock) -> bool {
    match block {
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdHtmlBlock(html_block)) => {
            html_block.is_html_comment()
        }
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
            let Some(first_token) = paragraph.syntax().first_token() else {
                return false;
            };
            let Some(last_token) = paragraph.syntax().last_token() else {
                return false;
            };

            first_token.token_text_trimmed().starts_with("<!--")
                && last_token.token_text_trimmed().ends_with("-->")
        }
        _ => false,
    }
}

fn is_ignorable_leading_block(block: &AnyMdBlock) -> bool {
    is_html_comment_block(block) || block.is_newline() || block.is_continuation_indent()
}

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
    /// Enforces the first meaningful block in the document to be an h1 heading,
    /// either ATX (`# Heading`) or setext level 1 (`Heading` followed by `===`).
    ///
    /// Leading HTML comments used as file preamble are ignored when determining
    /// the first meaningful block.
    ///
    /// The rule does not report when the first meaningful block is an HTML block
    /// or a thematic break block.
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
        version: "next",
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
                "The first meaningful block should be a top-level heading (h1) so readers and tools can identify the document title. Add a "<Emphasis>"# Heading"</Emphasis>" (or a level-1 setext heading) to the start of the document."
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
            let text = paragraph.syntax().text_trimmed().to_string();
            text.starts_with("<!--") && text.ends_with("-->")
        }
        _ => false,
    }
}

fn is_ignorable_leading_block(block: &AnyMdBlock) -> bool {
    is_html_comment_block(block) || block.is_newline() || block.is_continuation_indent()
}

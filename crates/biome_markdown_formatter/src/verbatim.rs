use crate::MarkdownFormatContext;
use biome_formatter::FormatContext;
use biome_formatter::{
    Format, FormatResult, LINE_TERMINATORS, normalize_newlines,
    prelude::{Formatter, tag::VerbatimKind, text},
};
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode};
use biome_rowan::{SyntaxNode, TextRange};

pub fn format_verbatim_node(node: &MarkdownSyntaxNode) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range_with_trivia().len(),
        },
        format_comments: true,
    }
}

pub fn format_suppressed_node(node: &MarkdownSyntaxNode) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: true,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatMarkdownVerbatimNode<'node> {
    node: &'node MarkdownSyntaxNode,
    kind: VerbatimKind,
    format_comments: bool,
}

impl Format<MarkdownFormatContext> for FormatMarkdownVerbatimNode<'_> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let trimmed_source_range = f.context().source_map().map_or_else(
            || self.node.text_trimmed_range(),
            |source_map| source_map.trimmed_source_range(self.node),
        );

        fn source_range<Context>(f: &Formatter<Context>, range: TextRange) -> TextRange
        where
            Context: biome_formatter::CstFormatContext,
        {
            f.context()
                .source_map()
                .map_or_else(|| range, |source_map| source_map.source_range(range))
        }

        let start_source = self
            .node
            .first_leading_trivia()
            .into_iter()
            .flat_map(|trivia| trivia.pieces())
            .filter(|trivia| trivia.is_skipped())
            .map(|trivia| source_range(f, trivia.text_range()).start())
            .take_while(|start| *start < trimmed_source_range.start())
            .next()
            .unwrap_or_else(|| trimmed_source_range.start());

        let original_source = f.context().source_map().map_or_else(
            || self.node.text_trimmed().to_string(),
            |source_map| {
                source_map
                    .source()
                    .text_slice(trimmed_source_range.cover_offset(start_source))
                    .to_string()
            },
        );
        text(
            &normalize_newlines(&original_source, LINE_TERMINATORS),
            self.node.text_trimmed_range().start(),
        )
        .fmt(f)
    }
}

pub fn format_bogus_node(node: &SyntaxNode<MarkdownLanguage>) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode {
        node,
        kind: VerbatimKind::Bogus,
        format_comments: true,
    }
}

use crate::prelude::*;
use crate::utils::comment_trivia::FormatCommentGap;
use biome_css_syntax::{AnyCssDeclarationOrRuleBlock, CssSyntaxNode};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::{format_dangling_comment, should_nestle_adjacent_doc_comments};
use biome_formatter::{Format, FormatResult, write};

use crate::CssFormatter;
use biome_css_syntax::stmt_ext::CssBlockLike;

/// Formats comments between a selector and its block.
///
/// ```scss
/// .a
/// // comment
/// {}
/// ```
#[derive(Debug, Copy, Clone)]
pub(crate) struct FormatSelectorBlockBoundary<'a> {
    rule: &'a CssSyntaxNode,
    block: &'a AnyCssDeclarationOrRuleBlock,
}

impl<'a> FormatSelectorBlockBoundary<'a> {
    /// Creates the boundary between a qualified rule and its block.
    pub(crate) fn new(rule: &'a CssSyntaxNode, block: &'a AnyCssDeclarationOrRuleBlock) -> Self {
        Self { rule, block }
    }
}

impl Format<CssFormatContext> for FormatSelectorBlockBoundary<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let comments = f.comments().clone();
        let boundary_comments = comments.dangling_comments(self.rule);
        let lines_before = boundary_comments
            .first()
            .or_else(|| comments.leading_comments(self.block.syntax()).first())
            .map_or(0, SourceComment::lines_before);

        FormatCommentGap::new(lines_before).fmt(f)?;

        let mut boundary_comments = boundary_comments.iter().peekable();
        while let Some(comment) = boundary_comments.next() {
            write!(f, [format_dangling_comment(comment)])?;

            let Some(next) = boundary_comments.peek().copied() else {
                return if comment.kind().is_line() {
                    write!(f, [hard_line_break()])
                } else {
                    write!(f, [space()])
                };
            };

            if !should_nestle_adjacent_doc_comments(comment, next) {
                let lines_after = comment
                    .lines_after()
                    .max(u32::from(comment.kind().is_line()));
                FormatCommentGap::new(lines_after).fmt(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FormatCssBlockLike<'a> {
    block: &'a CssBlockLike,
}

impl<'a> FormatCssBlockLike<'a> {
    pub(crate) fn new(block: &'a CssBlockLike) -> Self {
        Self { block }
    }

    fn write_items(&self, f: &mut CssFormatter) -> FormatResult<()> {
        match self.block {
            CssBlockLike::CssKeyframesBlock(block) => {
                write!(f, [block.items().format()])
            }
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => {
                write!(f, [block.items().format()])
            }
            CssBlockLike::CssDeclarationBlock(block) => {
                write!(f, [block.declarations().format()])
            }
            CssBlockLike::CssRuleBlock(block) => {
                write!(f, [block.rules().format()])
            }
            CssBlockLike::CssFontFeatureValuesBlock(block) => {
                write!(f, [block.items().format()])
            }
            CssBlockLike::CssPageAtRuleBlock(block) => {
                write!(f, [block.items().format()])
            }
            CssBlockLike::CssDeclarationOrRuleBlock(block) => {
                write!(f, [block.items().format()])
            }
        }
    }
}

impl Format<CssFormatContext> for FormatCssBlockLike<'_> {
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        write!(f, [self.block.l_curly_token().format()])?;

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if self.block.is_empty() || self.block.has_only_empty_declarations() {
            let comments = f.comments();

            let has_dangling_comments = comments.has_dangling_comments(self.block.syntax());

            if has_dangling_comments {
                write!(
                    f,
                    [format_dangling_comments(self.block.syntax()).with_block_indent()]
                )?;
            } else {
                // we still need to write items because the block may have empty declarations
                self.write_items(f)?;
                write!(f, [hard_line_break()])?;
            }
        } else {
            write!(f, [block_indent(&format_with(|f| self.write_items(f)))])?;
        }
        write!(f, [self.block.r_curly_token().format()])
    }
}

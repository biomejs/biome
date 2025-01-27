use crate::prelude::*;
use biome_formatter::{CstFormatContext, Format, FormatResult};

use crate::CssFormatter;
use biome_css_syntax::stmt_ext::CssBlockLike;
use biome_formatter::write;

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

        let r_curly_token = self.block.r_curly_token()?;

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if self.block.is_empty() {
            let comments = f.context().comments();

            let has_dangling_comments = comments.has_dangling_comments(self.block.syntax());

            if has_dangling_comments {
                write!(
                    f,
                    [format_dangling_comments(self.block.syntax()).with_block_indent()]
                )?;
            } else {
                write!(f, [soft_line_break()])?;
            }
        } else {
            write!(
                f,
                [soft_block_indent(&format_with(|f| self.write_items(f)))]
            )?;
        }
        write!(f, [r_curly_token.format()])
    }
}

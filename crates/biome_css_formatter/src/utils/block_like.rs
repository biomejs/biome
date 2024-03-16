use crate::prelude::*;
use biome_formatter::{CstFormatContext, Format, FormatResult};
use biome_rowan::{declare_node_union, SyntaxResult};

use crate::CssFormatter;
use biome_css_syntax::*;
use biome_formatter::write;

declare_node_union! {
    pub CssBlockLike = CssKeyframesBlock | CssDeclarationOrAtRuleBlock | CssDeclarationBlock | CssRuleBlock | CssFontFeatureValuesBlock | CssPageAtRuleBlock | CssDeclarationOrRuleBlock
}

impl CssBlockLike {
    fn l_curly_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            CssBlockLike::CssKeyframesBlock(block) => block.l_curly_token(),
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => block.l_curly_token(),
            CssBlockLike::CssDeclarationBlock(block) => block.l_curly_token(),
            CssBlockLike::CssRuleBlock(block) => block.l_curly_token(),
            CssBlockLike::CssFontFeatureValuesBlock(block) => block.l_curly_token(),
            CssBlockLike::CssPageAtRuleBlock(block) => block.l_curly_token(),
            CssBlockLike::CssDeclarationOrRuleBlock(block) => block.l_curly_token(),
        }
    }

    fn r_curly_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            CssBlockLike::CssKeyframesBlock(block) => block.r_curly_token(),
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => block.r_curly_token(),
            CssBlockLike::CssDeclarationBlock(block) => block.r_curly_token(),
            CssBlockLike::CssRuleBlock(block) => block.r_curly_token(),
            CssBlockLike::CssFontFeatureValuesBlock(block) => block.r_curly_token(),
            CssBlockLike::CssPageAtRuleBlock(block) => block.r_curly_token(),
            CssBlockLike::CssDeclarationOrRuleBlock(block) => block.r_curly_token(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            CssBlockLike::CssKeyframesBlock(block) => block.items().is_empty(),
            CssBlockLike::CssDeclarationOrAtRuleBlock(block) => block.items().is_empty(),
            CssBlockLike::CssDeclarationBlock(block) => block.declarations().is_empty(),
            CssBlockLike::CssRuleBlock(block) => block.rules().is_empty(),
            CssBlockLike::CssFontFeatureValuesBlock(block) => block.items().is_empty(),
            CssBlockLike::CssPageAtRuleBlock(block) => block.items().is_empty(),
            CssBlockLike::CssDeclarationOrRuleBlock(block) => block.items().is_empty(),
        }
    }

    fn write_items(&self, f: &mut CssFormatter) -> FormatResult<()> {
        match self {
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

impl Format<CssFormatContext> for CssBlockLike {
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        write!(f, [self.l_curly_token().format()])?;

        let r_curly_token = self.r_curly_token()?;

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if self.is_empty() {
            let comments = f.context().comments();

            let has_dangling_comments = comments.has_dangling_comments(self.syntax());

            if has_dangling_comments {
                write!(
                    f,
                    [format_dangling_comments(self.syntax()).with_block_indent()]
                )?;
            } else {
                write!(f, [hard_line_break()])?;
            }
        } else {
            write!(f, [block_indent(&format_with(|f| self.write_items(f)))])?;
        }
        write!(f, [r_curly_token.format()])
    }
}

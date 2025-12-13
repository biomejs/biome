use std::fmt::Debug;

use crate::{FormatCssSyntaxToken, prelude::*};
use biome_css_syntax::{AnyCssAttrName, CssAttrNameList, CssLanguage};
use biome_formatter::{trivia::FormatToken, write};
use biome_rowan::AstSeparatedElement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrNameList;

impl FormatRule<CssAttrNameList> for FormatCssAttrNameList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssAttrNameList, f: &mut CssFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        f.join_with(space())
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatAttrNameItem {
                        last: index == last_index,
                        element: item,
                    }),
            )
            .finish()
    }
}

struct FormatAttrNameItem {
    last: bool,
    element: AstSeparatedElement<CssLanguage, AnyCssAttrName>,
}

impl Format<CssFormatContext> for FormatAttrNameItem {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = self.element.trailing_separator()?;
        let node = self.element.node()?;

        write!(f, [node.format()])?;

        if let Some(token) = separator {
            if self.last {
                FormatCssSyntaxToken.format_removed(token, f)?;
            } else {
                write![f, [soft_line_break_or_space(), token.format()]]?;
            }
        }

        Ok(())
    }
}

use crate::{FormatCssSyntaxToken, prelude::*};
use biome_css_syntax::{AnyCssSyntaxComponent, CssLanguage, CssSyntaxComponentList};
use biome_formatter::{trivia::FormatToken, write};
use biome_rowan::AstSeparatedElement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxComponentList;

impl FormatRule<CssSyntaxComponentList> for FormatCssSyntaxComponentList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSyntaxComponentList, f: &mut CssFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        f.join_with(space())
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatSyntaxComponentItem {
                        last: index == last_index,
                        element: item,
                    }),
            )
            .finish()
    }
}

struct FormatSyntaxComponentItem {
    last: bool,
    element: AstSeparatedElement<CssLanguage, AnyCssSyntaxComponent>,
}

impl Format<CssFormatContext> for FormatSyntaxComponentItem {
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

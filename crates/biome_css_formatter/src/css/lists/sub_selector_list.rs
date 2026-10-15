use crate::prelude::*;
use biome_css_syntax::CssSubSelectorList;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSubSelectorList;
impl FormatRule<CssSubSelectorList> for FormatCssSubSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSubSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        let comments = f.comments().clone();
        let boundaries = comments.dangling_comments(node.syntax());
        if boundaries.is_empty() {
            return f.join().entries(node.iter().formatted()).finish();
        }

        let mut boundaries = boundaries.iter().peekable();
        for selector in node {
            let start = selector.syntax().text_trimmed_range().start();
            while let Some(comment) =
                boundaries.next_if(|comment| comment.piece().text_range().end() <= start)
            {
                write!(f, [format_dangling_comment(comment)])?;
            }
            write!(f, [selector.format()])?;
        }

        for comment in boundaries {
            write!(f, [format_dangling_comment(comment)])?;
        }

        Ok(())
    }
}

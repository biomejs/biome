use crate::prelude::*;
use biome_css_syntax::ScssEachValueList;
use biome_formatter::write;

/// Formats the values after `@each ... in`.
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachValueList;
impl FormatRule<ScssEachValueList> for FormatScssEachValueList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssEachValueList, f: &mut CssFormatter) -> FormatResult<()> {
        write!(f, [format_leading_comments(node.syntax())])?;

        let separator = soft_line_break_or_space();
        let mut fill = f.fill();

        for formatted in node.format_separated(",") {
            fill.entry(&separator, &formatted);
        }

        fill.finish()
    }
}

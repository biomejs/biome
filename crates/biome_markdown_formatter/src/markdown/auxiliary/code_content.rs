use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdCodeContent, MdCodeContentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdCodeContent;
impl FormatNodeRule<MdCodeContent> for FormatMdCodeContent {
    fn fmt_fields(&self, node: &MdCodeContent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdCodeContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        // The literal starts with the newline that ends the opening-fence
        // line. The fenced code block formatter already writes that line
        // break, so strip it here and print the rest verbatim.
        let token_text = value_token.text();
        let content = token_text
            .strip_prefix("\r\n")
            .or_else(|| token_text.strip_prefix('\n'))
            .or_else(|| token_text.strip_prefix('\r'))
            .unwrap_or(token_text);

        write!(
            f,
            [format_replaced(
                &value_token,
                &text(content, Some(value_token.text_trimmed_range().start()))
            )]
        )
    }
}

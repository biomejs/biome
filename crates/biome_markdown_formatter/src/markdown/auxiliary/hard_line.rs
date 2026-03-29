use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::MdHardLine;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHardLine {
    print_mode: TextPrintMode,
}
impl FormatNodeRule<MdHardLine> for FormatMdHardLine {
    fn fmt_fields(&self, node: &MdHardLine, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let token = node.value_token()?;

        if self.print_mode.is_pristine() {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let text_content = token.text();

        if text_content.trim_end().ends_with('\\') {
            // Preserve backslash form
            write!(
                f,
                [
                    format_removed(&token),
                    text("\\", token.text_range().start()),
                    hard_line_break()
                ]
            )
        } else {
            // Given two or more spaces in MdHardLine, only two spaces has semantic meaning
            // so we are adding back two spaces as required by the spec
            // https://spec.commonmark.org/0.31.2/#hard-line-break
            write!(
                f,
                [
                    format_removed(&token),
                    text("  ", token.text_range().start()),
                    hard_line_break(),
                ]
            )
        }
    }
}

pub(crate) struct FormatMdFormatHardLineOptions {
    pub(crate) print_mode: TextPrintMode,
}

impl FormatRuleWithOptions<MdHardLine> for FormatMdHardLine {
    type Options = FormatMdFormatHardLineOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.print_mode = options.print_mode;
        self
    }
}

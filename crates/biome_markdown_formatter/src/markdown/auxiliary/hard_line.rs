use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MarkdownSyntaxKind, MdHardLine};
use biome_rowan::Direction;

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
            // Detect if the hard line break is the last one of the paragraph.
            let is_last_hard_line = match node.syntax().siblings(Direction::Next).nth(1) {
                None => true,
                Some(s) => {
                    s.kind() == MarkdownSyntaxKind::MD_TEXTUAL && s.text_trimmed().is_empty()
                }
            };

            if is_last_hard_line {
                // Drop the two-space marker but keep a single newline so the
                // paragraph still terminates on its own line.
                return write!(f, [format_removed(&token), hard_line_break()]);
            }

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

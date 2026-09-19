use crate::markdown::lists::block_list::list_ends_with_line_break;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{AnyMdBlock, MdHeader, MdNewline, MdSetextHeader};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdNewline {
    print_mode: TextPrintMode,
}
impl FormatNodeRule<MdNewline> for FormatMdNewline {
    fn fmt_fields(&self, node: &MdNewline, f: &mut MarkdownFormatter) -> FormatResult<()> {
        if f.context().comments().has_comments(node.syntax()) {
            return write!(f, [format_removed(&node.value_token()?), hard_line_break()]);
        }
        if self.print_mode.is_remove() {
            return write!(f, [format_removed(&node.value_token()?)]);
        }

        // List printers emit their own final line break, even when the parsed
        // list ends at a fence and leaves its line terminator in the block list.
        if node
            .syntax()
            .prev_sibling()
            .and_then(AnyMdBlock::cast)
            .and_then(|block| block.as_any_list_item())
            .is_some_and(|list| !list_ends_with_line_break(&list))
        {
            return write!(f, [format_removed(&node.value_token()?)]);
        }

        let after_header = node
            .syntax()
            .prev_sibling()
            .is_some_and(|s| MdHeader::can_cast(s.kind()) || MdSetextHeader::can_cast(s.kind()));

        if after_header {
            let token = node.value_token()?;
            write!(f, [format_removed(&token), hard_line_break()])
        } else {
            node.value_token().format().fmt(f)
        }
    }
}

pub(crate) struct FormatMdNewlineOptions {
    pub(crate) print_mode: TextPrintMode,
}

impl FormatRuleWithOptions<MdNewline> for FormatMdNewline {
    type Options = FormatMdNewlineOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.print_mode = options.print_mode;
        self
    }
}

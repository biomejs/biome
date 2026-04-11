use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdHeader, MdNewline};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdNewline {
    should_remove: bool,
}
impl FormatNodeRule<MdNewline> for FormatMdNewline {
    fn fmt_fields(&self, node: &MdNewline, f: &mut MarkdownFormatter) -> FormatResult<()> {
        if self.should_remove {
            return write!(f, [format_removed(&node.value_token()?)]);
        }

        let after_header = node
            .syntax()
            .prev_sibling()
            .is_some_and(|s| MdHeader::can_cast(s.kind()));

        if after_header {
            let token = node.value_token()?;
            write!(f, [format_removed(&token), hard_line_break()])
        } else {
            node.value_token().format().fmt(f)
        }
    }
}

pub(crate) struct FormatMdNewlineOptions {
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdNewline> for FormatMdNewline {
    type Options = FormatMdNewlineOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_remove = options.should_remove;
        self
    }
}

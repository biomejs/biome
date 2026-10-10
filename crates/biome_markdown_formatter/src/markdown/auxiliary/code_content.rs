use crate::FormatEmbedded;
use crate::prelude::*;
use crate::shared::FormatLiteralLines;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdCodeContent, MdCodeContentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdCodeContent {
    opening_fence_indent: usize,
}
impl FormatNodeRule<MdCodeContent> for FormatMdCodeContent {
    fn fmt_fields(&self, node: &MdCodeContent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdCodeContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        // The literal starts with the newline that ends the opening-fence
        // line. The fenced code block formatter writes that line break and
        // normalizes the optional fence indentation separately.
        let lines = FormatLiteralLines {
            token: &value_token,
            max_indent: self.opening_fence_indent,
        };
        let content = format_replaced(&value_token, &lines);

        let range = value_token.text_range();
        if f.context().is_embedded_node_range(range) {
            let embedded = FormatEmbedded::new(range, &content, f)?;
            // The closing fence must start on its own line.
            write!(f, [embedded, hard_line_break()])
        } else {
            write!(f, [content])
        }
    }
}

pub(crate) struct FormatMdCodeContentOptions {
    pub(crate) opening_fence_indent: usize,
}

impl FormatRuleWithOptions<MdCodeContent> for FormatMdCodeContent {
    type Options = FormatMdCodeContentOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.opening_fence_indent = options.opening_fence_indent;
        self
    }
}

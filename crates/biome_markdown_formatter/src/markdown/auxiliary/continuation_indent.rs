use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdContinuationIndent, MdContinuationIndentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdContinuationIndent {
    should_remove: bool,
}
impl FormatNodeRule<MdContinuationIndent> for FormatMdContinuationIndent {
    fn fmt_fields(
        &self,
        node: &MdContinuationIndent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdContinuationIndentFields { indent } = node.as_fields();

        for token in indent.iter() {
            if self.should_remove {
                f.context()
                    .comments()
                    .mark_suppression_checked(token.syntax());

                write!(f, [format_removed(&token.md_indent_char_token()?)])?;
            } else {
                write!(f, [token.format()])?;
            }
        }

        Ok(())
    }
}

pub(crate) struct FormatMdContinuationIndentOptions {
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdContinuationIndent> for FormatMdContinuationIndent {
    type Options = FormatMdContinuationIndentOptions;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_remove = options.should_remove;

        self
    }
}

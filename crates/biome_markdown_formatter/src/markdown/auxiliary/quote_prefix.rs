use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdQuotePrefix, MdQuotePrefixFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuotePrefix {
    should_remove: bool,
}
impl FormatNodeRule<MdQuotePrefix> for FormatMdQuotePrefix {
    fn fmt_fields(&self, node: &MdQuotePrefix, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdQuotePrefixFields {
            marker_token,
            post_marker_space_token,
            pre_marker_indent,
        } = node.as_fields();

        if self.should_remove {
            for token in pre_marker_indent.iter() {
                f.context().comments().is_suppressed(token.syntax());
                write!(
                    f,
                    [format_removed(&token.md_quote_pre_marker_indent_token()?)]
                )?;
            }
            write!(f, [format_removed(&marker_token?)])?;
            if let Some(post_marker_space_token) = post_marker_space_token {
                write!(f, [format_removed(&post_marker_space_token)])?;
            }
            return Ok(());
        }

        write!(f, [pre_marker_indent.format(), marker_token.format(),])?;

        if let Some(post_marker_space_token) = post_marker_space_token {
            write!(f, [post_marker_space_token.format()])?;
        } else {
            let marker = marker_token?;
            let next_has_text = marker
                .next_token()
                .is_some_and(|t| t.text().starts_with(|c: char| !c.is_whitespace()));
            if next_has_text {
                write!(f, [space()])?;
            }
        }

        Ok(())
    }
}

pub(crate) struct FormatMdQuotePrefixOptions {
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdQuotePrefix> for FormatMdQuotePrefix {
    type Options = FormatMdQuotePrefixOptions;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_remove = options.should_remove;
        self
    }
}

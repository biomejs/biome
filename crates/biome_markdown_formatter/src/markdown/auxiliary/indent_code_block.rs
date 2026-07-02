use crate::markdown::auxiliary::indent_token::FormatMdIndentTokenOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{AnyMdInline, MdIndentCodeBlock, MdIndentCodeBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentCodeBlockOptions {
    pub(crate) in_list: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentCodeBlock {
    in_list: bool,
}

impl FormatRuleWithOptions<MdIndentCodeBlock> for FormatMdIndentCodeBlock {
    type Options = FormatMdIndentCodeBlockOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.in_list = options.in_list;
        self
    }
}

impl FormatNodeRule<MdIndentCodeBlock> for FormatMdIndentCodeBlock {
    fn fmt_fields(&self, node: &MdIndentCodeBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdIndentCodeBlockFields { content } = node.as_fields();

        if !self.in_list {
            let mut at_line_start = true;
            for item in content.iter() {
                match &item {
                    AnyMdInline::MdIndentToken(indent) => {
                        write!(
                            f,
                            [indent.format().with_options(FormatMdIndentTokenOptions {
                                replace_tabs_with_spaces: at_line_start,
                            })]
                        )?;
                        at_line_start = false;
                    }
                    AnyMdInline::MdTextual(text) => {
                        write!(f, [item.format()])?;
                        at_line_start = text.is_newline()?;
                    }
                    _ => {
                        write!(f, [item.format()])?;
                        at_line_start = false;
                    }
                }
            }

            return Ok(());
        }

        write!(
            f,
            [align(
                4,
                &format_with(|f: &mut MarkdownFormatter| {
                    write!(f, [token("    ")])?;
                    let mut past_leading_spaces = false;
                    for item in content.iter() {
                        if !past_leading_spaces {
                            match &item {
                                AnyMdInline::MdIndentToken(indent) => {
                                    f.context().comments().is_suppressed(indent.syntax());
                                    write!(f, [format_removed(&indent.md_indent_char_token()?)])?;
                                    continue;
                                }
                                AnyMdInline::MdTextual(text) => {
                                    let token = text.value_token()?;
                                    if token.text_trimmed().chars().all(|c| c == ' ' || c == '\t') {
                                        f.context().comments().is_suppressed(text.syntax());
                                        write!(f, [format_removed(&token)])?;
                                        continue;
                                    }
                                }
                                _ => {}
                            }
                            past_leading_spaces = true;
                        }
                        write!(f, [item.format()])?;
                    }
                    Ok(())
                })
            )]
        )
    }
}

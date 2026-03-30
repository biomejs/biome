use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdTextual, MdTextualFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdTextual {
    should_remove: bool,
    trim_start: bool,
    print_mode: TextPrintMode,
}
impl FormatNodeRule<MdTextual> for FormatMdTextual {
    fn fmt_fields(&self, node: &MdTextual, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdTextualFields { value_token } = node.as_fields();

        let value_token = value_token?;

        if self.should_remove {
            format_removed(&value_token).fmt(f)
        } else if self.print_mode.is_clean() {
            // Clean mode: strip spaces/tabs but preserve newlines.
            // Used for code block content where trailing whitespace on empty
            // lines should be removed but newlines must be kept.
            let cleaned = value_token
                .text()
                .trim_matches(|c: char| c == ' ' || c == '\t');
            if cleaned == value_token.text() {
                write!(f, [value_token.format()])
            } else {
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &text(cleaned, value_token.text_trimmed_range().start())
                    )]
                )
            }
        } else if self.print_mode.is_normalize_words() {
            // Collapse whitespace: words are joined with hard_space,
            // leading/trailing whitespace becomes a single hard_space.
            let raw = value_token.text();
            let words: Vec<&str> = raw.split_whitespace().collect();

            if words.is_empty() {
                // Whitespace-only — normalize to a single hard_space.
                write!(f, [format_replaced(&value_token, &hard_space())])
            } else {
                let has_leading_ws = raw.starts_with(char::is_whitespace);
                let has_trailing_ws = raw.ends_with(char::is_whitespace);
                let position = value_token.text_trimmed_range().start();
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &format_with(|f| {
                            if has_leading_ws {
                                write!(f, [hard_space()])?;
                            }
                            for (i, word) in words.iter().enumerate() {
                                if i > 0 {
                                    write!(f, [hard_space()])?;
                                }
                                write!(f, [text(word, position)])?;
                            }
                            if has_trailing_ws {
                                write!(f, [hard_space()])?;
                            }
                            Ok(())
                        })
                    )]
                )
            }
        } else if self.trim_start {
            let trimmed_text = value_token.text().trim_start();
            write!(
                f,
                [format_replaced(
                    &value_token,
                    &text(trimmed_text, value_token.text_trimmed_range().start())
                )]
            )
        } else {
            write!(f, [value_token.format()])
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatMdTextualOptions {
    pub(crate) should_remove: bool,
    pub(crate) trim_start: bool,
    pub(crate) print_mode: TextPrintMode,
}

impl FormatRuleWithOptions<MdTextual> for FormatMdTextual {
    type Options = FormatMdTextualOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_remove = options.should_remove;
        self.trim_start = options.trim_start;
        self.print_mode = options.print_mode;
        self
    }
}

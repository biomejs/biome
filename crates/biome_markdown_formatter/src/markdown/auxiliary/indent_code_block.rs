use crate::markdown::auxiliary::indent_token::FormatMdIndentTokenOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{
    AnyMdInline, MdIndentCodeBlock, MdIndentCodeBlockFields, MdInlineItemList,
};

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
                                should_remove: false,
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

        let leading_whitespace = minimum_leading_whitespace(&content)?;

        write!(
            f,
            [align(
                "    ",
                &format_with(|f: &mut MarkdownFormatter| {
                    write!(f, [token("    ")])?;
                    let mut stripper = LinePrefixStripper::new(leading_whitespace);

                    for item in content.iter() {
                        match &item {
                            AnyMdInline::MdIndentToken(indent) if stripper.strip_indent_token() => {
                                f.context().comments().is_suppressed(indent.syntax());
                                write!(f, [format_removed(&indent.md_indent_char_token()?)])?;
                            }
                            AnyMdInline::MdTextual(textual) => {
                                let token = textual.value_token()?;
                                if let Some(stripped) = stripper.strip_text(token.text()) {
                                    f.context().comments().is_suppressed(textual.syntax());
                                    write!(
                                        f,
                                        [format_replaced(
                                            &token,
                                            &text(&stripped, Some(token.text_range().start()))
                                        )]
                                    )?;
                                } else {
                                    write!(f, [item.format()])?;
                                }
                            }
                            _ => {
                                stripper.finish_line_prefix();
                                write!(f, [item.format()])?;
                            }
                        }
                    }

                    Ok(())
                })
            )]
        )
    }
}

fn minimum_leading_whitespace(content: &MdInlineItemList) -> FormatResult<usize> {
    let mut counter = LeadingWhitespaceCounter::new();

    for item in content.iter() {
        match &item {
            AnyMdInline::MdIndentToken(_) => counter.observe_indent_token(),
            AnyMdInline::MdTextual(text) => counter.observe_text(text.value_token()?.text()),
            _ => counter.observe_content(),
        }
    }

    Ok(counter.finish())
}

struct LeadingWhitespaceCounter {
    minimum: Option<usize>,
    current: usize,
    at_line_start: bool,
    line_has_content: bool,
}

impl LeadingWhitespaceCounter {
    fn new() -> Self {
        Self {
            minimum: None,
            current: 0,
            at_line_start: true,
            line_has_content: false,
        }
    }

    fn observe_indent_token(&mut self) {
        if self.at_line_start {
            self.current += 1;
        } else {
            self.line_has_content = true;
        }
    }

    fn observe_text(&mut self, text: &str) {
        for char in text.chars() {
            if char == '\n' {
                self.finish_line();
            } else if self.at_line_start && is_indent_char(char) {
                self.current += 1;
            } else {
                self.observe_content();
            }
        }
    }

    fn observe_content(&mut self) {
        self.at_line_start = false;
        self.line_has_content = true;
    }

    fn finish_line(&mut self) {
        if self.line_has_content {
            self.minimum = Some(
                self.minimum
                    .map_or(self.current, |minimum| minimum.min(self.current)),
            );
        }

        self.current = 0;
        self.at_line_start = true;
        self.line_has_content = false;
    }

    fn finish(mut self) -> usize {
        self.finish_line();
        self.minimum.unwrap_or(0)
    }
}

struct LinePrefixStripper {
    remaining: usize,
    at_line_start: bool,
}

impl LinePrefixStripper {
    fn new(width: usize) -> Self {
        Self {
            remaining: width,
            at_line_start: true,
        }
    }

    fn strip_indent_token(&mut self) -> bool {
        if self.at_line_start && self.remaining > 0 {
            self.remaining -= 1;
            true
        } else {
            self.finish_line_prefix();
            false
        }
    }

    fn strip_text(&mut self, text: &str) -> Option<String> {
        let mut stripped = String::with_capacity(text.len());
        let mut changed = false;

        for char in text.chars() {
            if char == '\n' {
                stripped.push(char);
                // Raw code-block newlines do not receive structural `align()` indentation.
                // Keep later source indentation so multiline code stays inside the list item.
                self.finish_line_prefix();
            } else if self.at_line_start && self.remaining > 0 && is_indent_char(char) {
                self.remaining -= 1;
                changed = true;
            } else {
                stripped.push(char);
                self.finish_line_prefix();
            }
        }

        changed.then_some(stripped)
    }

    fn finish_line_prefix(&mut self) {
        self.remaining = 0;
        self.at_line_start = false;
    }
}

fn is_indent_char(char: char) -> bool {
    matches!(char, ' ' | '\t')
}

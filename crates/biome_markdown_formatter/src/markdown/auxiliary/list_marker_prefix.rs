use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, format_args, write};
use biome_markdown_syntax::list_ext::{ListMarker, OrderedListDelimiter};
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdLeafBlock, MdBullet, MdListMarkerPrefix, MdListMarkerPrefixFields,
};
use biome_rowan::TextSize;
use std::ops::Add;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdListMarkerPrefix {
    /// Target marker to replace with. `None` keeps the original marker.
    target_marker: Option<TargetMarker>,
    keep_pre_marker: bool,
    min_post_marker_len: usize,
}
impl FormatNodeRule<MdListMarkerPrefix> for FormatMdListMarkerPrefix {
    fn fmt_fields(&self, node: &MdListMarkerPrefix, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdListMarkerPrefixFields {
            pre_marker_indent,
            marker,
            post_marker_space_token,
            content_indent,
        } = node.as_fields();

        let marker = marker?;
        let list_marker = node.list_marker()?;
        if self.keep_pre_marker {
            for indent_token in pre_marker_indent.iter() {
                write!(f, [indent_token.format()])?;
            }
        } else {
            write!(f, [pre_marker_indent.format()])?;
        }
        // Note that for `-   `, the parser treats the indent as part of the marker, not the content
        // This is a parser bug that causes a regression
        // in crates/biome_markdown_formatter/tests/specs/prettier/markdown/spec/example-242.md.snap
        match &self.target_marker {
            Some(target) => {
                let target = LocatedTargetMarker {
                    target: *target,
                    source_position: marker.text_trimmed_range().start(),
                };
                write!(f, [format_replaced(&marker, &target)])?
            }
            None => {
                if list_marker.is_ordered_with_paren()
                    && let Some(trimmed_text) = marker.text_trimmed().strip_suffix(")")
                {
                    write!(
                        f,
                        [format_replaced(
                            &marker,
                            &format_args![
                                text(trimmed_text, Some(marker.text_trimmed_range().start()),),
                                token(".")
                            ]
                        )]
                    )?
                } else {
                    write!(f, [marker.format()])?
                }
            }
        }

        let post_marker_len = if is_marker_only_bullet(node) {
            // marker-only bullets have no post-marker space to preserve
            0
        } else {
            // this returns the number of spaces to preserve after the marker
            post_marker_space_token
                .as_ref()
                .map_or(0, |token| token.text_trimmed().len())
                .max(self.min_post_marker_len)
        };

        if let Some(post_marker_space_token) = post_marker_space_token {
            write!(f, [format_removed(&post_marker_space_token)])?;

            for index in 0..post_marker_len {
                let pos = post_marker_space_token
                    .text_trimmed_range()
                    .start()
                    .add(TextSize::from(index as u32));
                write!(f, [text(" ", Some(pos)),])?;
            }
        } else {
            for _ in 0..post_marker_len {
                write!(f, [token(" ")])?;
            }
        }
        write!(f, [content_indent.format()])
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum TargetMarker {
    Unordered(ListMarker),
    Ordered(OrderedMarker),
}

impl TargetMarker {
    pub(crate) fn width(self) -> usize {
        match self {
            Self::Unordered(marker) => marker.unordered_marker_text().map_or(0, str::len),
            Self::Ordered(marker) => marker.width(),
        }
    }
}

impl Format<MarkdownFormatContext> for TargetMarker {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match self {
            Self::Unordered(marker) => {
                if let Some(marker) = marker.unordered_marker_text() {
                    write!(f, [token(marker)])?;
                }
            }
            Self::Ordered(marker) => marker.fmt(f)?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct OrderedMarker {
    number: usize,
    delimiter: OrderedListDelimiter,
}

impl OrderedMarker {
    pub(crate) const fn new(number: usize, delimiter: OrderedListDelimiter) -> Self {
        Self { number, delimiter }
    }

    fn width(self) -> usize {
        let mut number = self.number;
        let mut width = 1;

        while number >= 10 {
            number /= 10;
            width += 1;
        }

        width + self.delimiter.marker_text().len()
    }
}

impl Format<MarkdownFormatContext> for OrderedMarker {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        const MAX_DECIMAL_DIGITS: usize = (usize::BITS as usize * 1233 / 4096) + 1;

        let mut digits = [0usize; MAX_DECIMAL_DIGITS];
        let mut number = self.number;
        let mut len = 0;

        loop {
            digits[len] = number % 10;
            len += 1;
            number /= 10;

            if number == 0 {
                break;
            }
        }

        for digit in digits[..len].iter().rev() {
            write!(f, [token(DIGITS[*digit])])?;
        }

        write!(f, [token(self.delimiter.marker_text())])
    }
}

struct LocatedTargetMarker {
    target: TargetMarker,
    source_position: TextSize,
}

impl Format<MarkdownFormatContext> for LocatedTargetMarker {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        write!(f, [source_position(self.source_position), self.target])
    }
}

// A marker-only bullet (`-\n`, or `-   \n` before formatting) has no block
// content besides the newline. Prettier removes the separator and indent
// padding for those items, so the prefix formatter needs to detect them before
// deciding how many spaces to print after the marker.
fn is_marker_only_bullet(node: &MdListMarkerPrefix) -> bool {
    let Some(bullet) = node.syntax().parent().and_then(MdBullet::cast) else {
        return false;
    };

    let mut blocks = bullet.content().iter();
    matches!(
        blocks.next(),
        Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_)))
    ) && blocks.next().is_none()
}

pub(crate) struct FormatMdListMarkerPrefixOptions {
    /// Target marker to replace with. `None` keeps the original marker.
    pub(crate) target_marker: Option<TargetMarker>,
    /// When true, emit pre-marker indent tokens verbatim instead of removing them.
    pub(crate) keep_pre_marker: bool,
    /// Minimum number of spaces to emit after the list marker.
    /// Existing longer post-marker spacing is preserved.
    pub(crate) min_post_marker_len: usize,
}

impl FormatRuleWithOptions<MdListMarkerPrefix> for FormatMdListMarkerPrefix {
    type Options = FormatMdListMarkerPrefixOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.target_marker = options.target_marker;
        self.keep_pre_marker = options.keep_pre_marker;
        self.min_post_marker_len = options.min_post_marker_len;
        self
    }
}

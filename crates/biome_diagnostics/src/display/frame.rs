use std::{
    borrow::Cow,
    io,
    iter::FusedIterator,
    num::NonZeroUsize,
    ops::{Bound, RangeBounds},
};

use biome_console::{fmt, markup};
use biome_text_size::{TextLen, TextRange, TextSize};
use unicode_width::UnicodeWidthChar;

use crate::{
    location::{BorrowedSourceCode, LineIndex},
    LineIndexBuf, Location,
};

/// A const Option::unwrap without nightly features:
/// https://github.com/rust-lang/rust/issues/67441
const fn unwrap<T: Copy>(option: Option<T>) -> T {
    match option {
        Some(value) => value,
        None => panic!("unwrapping None"),
    }
}

const ONE: NonZeroUsize = unwrap(NonZeroUsize::new(1));
pub(super) const CODE_FRAME_CONTEXT_LINES: NonZeroUsize = unwrap(NonZeroUsize::new(2));

const MAX_CODE_FRAME_LINES: usize = 8;
const HALF_MAX_CODE_FRAME_LINES: usize = MAX_CODE_FRAME_LINES / 2;

/// Prints a code frame advice
pub(super) fn print_frame(fmt: &mut fmt::Formatter<'_>, location: Location<'_>) -> io::Result<()> {
    let source_span = location
        .source_code
        .and_then(|source_code| Some((source_code, location.span?)));

    let (source_code, span) = match source_span {
        Some(source_span) => source_span,
        None => return Ok(()),
    };

    let source_file = SourceFile::new(source_code);

    let start_index = span.start();
    let start_location = match source_file.location(start_index) {
        Ok(location) => location,
        Err(_) => return Ok(()),
    };

    let end_index = span.end();
    let end_location = match source_file.location(end_index) {
        Ok(location) => location,
        Err(_) => return Ok(()),
    };

    // Increase the amount of lines we should show for "context"
    let context_start = start_location
        .line_number
        .saturating_sub(CODE_FRAME_CONTEXT_LINES.get());

    let mut context_end = end_location
        .line_number
        .saturating_add(CODE_FRAME_CONTEXT_LINES.get())
        .min(OneIndexed::new(source_file.line_starts.len()).unwrap_or(OneIndexed::MIN));

    // Remove trailing blank lines
    for line_index in IntoIter::new(context_start..=context_end).rev() {
        if line_index == end_location.line_number {
            break;
        }

        let line_start = match source_file.line_start(line_index.to_zero_indexed()) {
            Ok(index) => index,
            Err(_) => continue,
        };
        let line_end = match source_file.line_start(line_index.to_zero_indexed() + 1) {
            Ok(index) => index,
            Err(_) => continue,
        };

        let line_range = TextRange::new(line_start, line_end);
        let line_text = source_file.source[line_range].trim();
        if !line_text.is_empty() {
            break;
        }

        context_end = line_index;
    }

    // If we have too many lines in our selection, then collapse them to an ellipsis
    let range_len = (context_end.get() + 1).saturating_sub(context_start.get());
    let ellipsis_range = if range_len > MAX_CODE_FRAME_LINES + 2 {
        let ellipsis_start = context_start.saturating_add(HALF_MAX_CODE_FRAME_LINES);
        let ellipsis_end = context_end.saturating_sub(HALF_MAX_CODE_FRAME_LINES);
        Some(ellipsis_start..=ellipsis_end)
    } else {
        None
    };

    // Calculate the maximum width of the line number
    let max_gutter_len = calculate_print_width(context_end);
    let mut printed_lines = false;

    for line_index in IntoIter::new(context_start..=context_end) {
        if let Some(ellipsis_range) = &ellipsis_range {
            if ellipsis_range.contains(&line_index) {
                if *ellipsis_range.start() == line_index {
                    for _ in 0..max_gutter_len.get() {
                        fmt.write_str(" ")?;
                    }

                    fmt.write_markup(markup! { <Emphasis>"    ...\n"</Emphasis> })?;
                    printed_lines = true;
                }
                continue;
            }
        }

        let line_start = match source_file.line_start(line_index.to_zero_indexed()) {
            Ok(index) => index,
            Err(_) => continue,
        };
        let line_end = match source_file.line_start(line_index.to_zero_indexed() + 1) {
            Ok(index) => index,
            Err(_) => continue,
        };

        let line_range = TextRange::new(line_start, line_end);
        let line_text = source_file.source[line_range].trim_end_matches(['\r', '\n']);

        // Ensure that the frame doesn't start with whitespace
        if !printed_lines && line_index != start_location.line_number && line_text.trim().is_empty()
        {
            continue;
        }

        printed_lines = true;

        // If this is within the highlighted line range
        let should_highlight =
            line_index >= start_location.line_number && line_index <= end_location.line_number;

        let padding_width = max_gutter_len
            .get()
            .saturating_sub(calculate_print_width(line_index).get());

        for _ in 0..padding_width {
            fmt.write_str(" ")?;
        }

        if should_highlight {
            fmt.write_markup(markup! {
                <Emphasis><Error>'>'</Error></Emphasis>' '
            })?;
        } else {
            fmt.write_str("  ")?;
        }

        fmt.write_markup(markup! {
            <Emphasis>{format_args!("{line_index} \u{2502} ")}</Emphasis>
        })?;

        // Show invisible characters
        print_invisibles(
            fmt,
            line_text,
            PrintInvisiblesOptions {
                ignore_trailing_carriage_return: true,
                ignore_leading_tabs: true,
                ignore_lone_spaces: true,
                at_line_start: true,
                at_line_end: true,
            },
        )?;

        fmt.write_str("\n")?;

        if should_highlight {
            let is_first_line = line_index == start_location.line_number;
            let is_last_line = line_index == end_location.line_number;

            let start_index_relative_to_line =
                start_index.max(line_range.start()) - line_range.start();
            let end_index_relative_to_line = end_index.min(line_range.end()) - line_range.start();

            let marker = if is_first_line && is_last_line {
                // Only line in the selection
                Some(TextRange::new(
                    start_index_relative_to_line,
                    end_index_relative_to_line,
                ))
            } else if is_first_line {
                // First line in selection
                Some(TextRange::new(
                    start_index_relative_to_line,
                    line_text.text_len(),
                ))
            } else if is_last_line {
                // Last line in selection
                let start_index = line_text
                    .text_len()
                    .checked_sub(line_text.trim_start().text_len())
                    // SAFETY: The length of `line_text.trim_start()` should
                    // never be larger than `line_text` itself
                    .expect("integer overflow");
                Some(TextRange::new(start_index, end_index_relative_to_line))
            } else {
                None
            };

            if let Some(marker) = marker {
                for _ in 0..max_gutter_len.get() {
                    fmt.write_str(" ")?;
                }

                fmt.write_markup(markup! {
                    <Emphasis>"   \u{2502} "</Emphasis>
                })?;

                // Align the start of the marker with the line above by a
                // number of space characters equal to the unicode print width
                // of the leading part of the line (before the start of the
                // marker), with a special exception for tab characters that
                // still get printed as tabs to respect the user-defined tab
                // display width
                let leading_range = TextRange::new(TextSize::from(0), marker.start());
                for c in line_text[leading_range].chars() {
                    match c {
                        '\t' => fmt.write_str("\t")?,
                        _ => {
                            for _ in 0..char_width(c) {
                                fmt.write_str(" ")?;
                            }
                        }
                    }
                }

                let marker_width = text_width(&line_text[marker]);
                for _ in 0..marker_width {
                    fmt.write_markup(markup! {
                        <Emphasis><Error>'^'</Error></Emphasis>
                    })?;
                }

                fmt.write_str("\n")?;
            }
        }
    }

    fmt.write_str("\n")
}

pub(super) fn print_highlighted_frame(
    fmt: &mut fmt::Formatter<'_>,
    location: Location<'_>,
) -> io::Result<()> {
    let Some(span) = location.span else {
        return Ok(());
    };
    let Some(source_code) = location.source_code else {
        return Ok(());
    };

    // TODO: instead of calculating lines for every match,
    // check if the Grit engine is able to pull them out
    let source = SourceFile::new(source_code);

    let start = source.location(span.start())?;
    let end = source.location(span.end())?;

    let match_line_start = start.line_number;
    let match_line_end = end.line_number.saturating_add(1);

    for line_index in IntoIter::new(match_line_start..match_line_end) {
        let current_range = source.line_range(line_index.to_zero_indexed());
        let current_range = match current_range {
            Ok(v) => v,
            Err(_) => continue,
        };

        let current_text = source_code.text[current_range].trim_end_matches(['\r', '\n']);

        let is_first_line = line_index == start.line_number;
        let is_last_line = line_index == end.line_number;

        let start_index_relative_to_line =
            span.start().max(current_range.start()) - current_range.start();
        let end_index_relative_to_line =
            span.end().min(current_range.end()) - current_range.start();

        let marker = if is_first_line && is_last_line {
            TextRange::new(start_index_relative_to_line, end_index_relative_to_line)
        } else if is_last_line {
            let start_index: u32 = current_text.text_len().into();

            let safe_start_index =
                start_index.saturating_sub(current_text.trim_start().text_len().into());

            TextRange::new(TextSize::from(safe_start_index), end_index_relative_to_line)
        } else {
            TextRange::new(start_index_relative_to_line, current_text.text_len())
        };

        fmt.write_markup(markup! {
            <Emphasis>{format_args!("{line_index} \u{2502} ")}</Emphasis>
        })?;

        let start_range = &current_text[0..marker.start().into()];
        let highlighted_range = &current_text[marker.start().into()..marker.end().into()];
        let end_range = &current_text[marker.end().into()..current_text.text_len().into()];

        write!(fmt, "{start_range}")?;
        fmt.write_markup(markup! { <Emphasis><Info>{highlighted_range}</Info></Emphasis> })?;
        write!(fmt, "{end_range}")?;

        writeln!(fmt)?;
    }

    Ok(())
}

/// Calculate the length of the string representation of `value`
pub(super) fn calculate_print_width(mut value: OneIndexed) -> NonZeroUsize {
    // SAFETY: Constant is being initialized with a non-zero value
    const TEN: OneIndexed = unwrap(OneIndexed::new(10));

    let mut width = ONE;

    while value >= TEN {
        value = OneIndexed::new(value.get() / 10).unwrap_or(OneIndexed::MIN);
        width = width.checked_add(1).unwrap();
    }

    width
}

/// Compute the unicode display width of a string, with the width of tab
/// characters set to [TAB_WIDTH] and the width of control characters set to 0
pub(super) fn text_width(text: &str) -> usize {
    text.chars().map(char_width).sum()
}

/// We need to set a value here since we have no way of knowing what the user's
/// preferred tab display width is, so this is set to `2` to match how tab
/// characters are printed by [print_invisibles]
const TAB_WIDTH: usize = 2;

/// Some esoteric space characters don't return a width using `char.width()`, so
/// we need to assume a fixed length for them
const ESOTERIC_SPACE_WIDTH: usize = 1;

/// Return the width of characters, treating whitespace characters in the way
/// we need to properly display it
pub(super) fn char_width(char: char) -> usize {
    match char {
        '\t' => TAB_WIDTH,
        '\u{c}' => ESOTERIC_SPACE_WIDTH,
        '\u{b}' => ESOTERIC_SPACE_WIDTH,
        '\u{85}' => ESOTERIC_SPACE_WIDTH,
        '\u{feff}' => ESOTERIC_SPACE_WIDTH,
        '\u{180e}' => ESOTERIC_SPACE_WIDTH,
        '\u{200b}' => ESOTERIC_SPACE_WIDTH,
        '\u{3000}' => ESOTERIC_SPACE_WIDTH,
        _ => char.width().unwrap_or(0),
    }
}

pub(super) struct PrintInvisiblesOptions {
    /// Do not print tab characters at the start of the string
    pub(super) ignore_leading_tabs: bool,
    /// If this is set to true, space characters will only be substituted when
    /// at least two of them are found in a row
    pub(super) ignore_lone_spaces: bool,
    /// Do not print `'\r'` characters if they're followed by `'\n'`
    pub(super) ignore_trailing_carriage_return: bool,
    // Set to `true` to show invisible characters at the start of the string
    pub(super) at_line_start: bool,
    // Set to `true` to show invisible characters at the end of the string
    pub(super) at_line_end: bool,
}

/// Print `input` to `fmt` with invisible characters replaced with an
/// appropriate visual representation. Return `true` if any non-whitespace
/// character was printed
pub(super) fn print_invisibles(
    fmt: &mut fmt::Formatter<'_>,
    input: &str,
    options: PrintInvisiblesOptions,
) -> io::Result<bool> {
    let mut had_non_whitespace = false;

    // Get the first trailing whitespace character in the string
    let trailing_whitespace_index = input
        .bytes()
        .enumerate()
        .rev()
        .find(|(_, byte)| !byte.is_ascii_whitespace())
        .map_or(input.len(), |(index, _)| index);

    let mut iter = input.char_indices().peekable();
    let mut prev_char_was_whitespace = false;

    while let Some((i, char)) = iter.next() {
        let mut show_invisible = true;

        // Only highlight spaces when surrounded by other spaces
        if char == ' ' && options.ignore_lone_spaces {
            show_invisible = false;

            let next_char_is_whitespace = iter
                .peek()
                .is_some_and(|(_, char)| char.is_ascii_whitespace());

            if prev_char_was_whitespace || next_char_is_whitespace {
                show_invisible = false;
            }
        }

        prev_char_was_whitespace = char.is_ascii_whitespace();

        // Don't show leading tabs
        if options.at_line_start
            && !had_non_whitespace
            && char == '\t'
            && options.ignore_leading_tabs
        {
            show_invisible = false;
        }

        // Always show if at the end of line
        if options.at_line_end && i >= trailing_whitespace_index {
            show_invisible = true;
        }

        // If we are a carriage return next to a \n then don't show the character as visible
        if options.ignore_trailing_carriage_return && char == '\r' {
            let next_char_is_line_feed = iter.peek().is_some_and(|(_, char)| *char == '\n');
            if next_char_is_line_feed {
                continue;
            }
        }

        if !show_invisible {
            if !char.is_ascii_whitespace() {
                had_non_whitespace = true;
            }

            write!(fmt, "{char}")?;
            continue;
        }

        if let Some(visible) = show_invisible_char(char) {
            fmt.write_markup(markup! { <Dim>{visible}</Dim> })?;
            continue;
        }

        if (char.is_whitespace() && !char.is_ascii_whitespace()) || char.is_control() {
            let code = u32::from(char);
            fmt.write_markup(markup! { <Inverse>"U+"{format_args!("{code:x}")}</Inverse> })?;
            continue;
        }

        write!(fmt, "{char}")?;
    }

    Ok(had_non_whitespace)
}

fn show_invisible_char(char: char) -> Option<&'static str> {
    match char {
        ' ' => Some("\u{b7}"),          // Middle Dot
        '\r' => Some("\u{240d}"),       // Carriage Return Symbol
        '\n' => Some("\u{23ce}"),       // Return Symbol
        '\t' => Some("\u{2192} "),      // Rightwards Arrow
        '\0' => Some("\u{2400}"),       // Null Symbol
        '\x0b' => Some("\u{240b}"),     // Vertical Tabulation Symbol
        '\x08' => Some("\u{232b}"),     // Backspace Symbol
        '\x0c' => Some("\u{21a1}"),     // Downwards Two Headed Arrow
        '\u{85}' => Some("\u{2420}"),   // Space Symbol
        '\u{a0}' => Some("\u{2420}"),   // Space Symbol
        '\u{1680}' => Some("\u{2420}"), // Space Symbol
        '\u{2000}' => Some("\u{2420}"), // Space Symbol
        '\u{2001}' => Some("\u{2420}"), // Space Symbol
        '\u{2002}' => Some("\u{2420}"), // Space Symbol
        '\u{2003}' => Some("\u{2420}"), // Space Symbol
        '\u{2004}' => Some("\u{2420}"), // Space Symbol
        '\u{2005}' => Some("\u{2420}"), // Space Symbol
        '\u{2006}' => Some("\u{2420}"), // Space Symbol
        '\u{2007}' => Some("\u{2420}"), // Space Symbol
        '\u{2008}' => Some("\u{2420}"), // Space Symbol
        '\u{2009}' => Some("\u{2420}"), // Space Symbol
        '\u{200a}' => Some("\u{2420}"), // Space Symbol
        '\u{202f}' => Some("\u{2420}"), // Space Symbol
        '\u{205f}' => Some("\u{2420}"), // Space Symbol
        '\u{3000}' => Some("\u{2420}"), // Space Symbol
        _ => None,
    }
}

/// A user-facing location in a source file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// The user-facing line number.
    pub line_number: OneIndexed,
    /// The user-facing column number.
    pub column_number: OneIndexed,
}

/// Representation of a single source file holding additional information for
/// efficiently rendering code frames
#[derive(Clone)]
pub struct SourceFile<'diagnostic> {
    /// The source code of the file.
    source: &'diagnostic str,
    /// The starting byte indices in the source code.
    line_starts: Cow<'diagnostic, LineIndex>,
}

impl<'diagnostic> SourceFile<'diagnostic> {
    /// Create a new [SourceFile] from a slice of text
    pub fn new(source_code: BorrowedSourceCode<'diagnostic>) -> Self {
        // Either re-use the existing line index provided by the diagnostic or create one
        Self {
            source: source_code.text,
            line_starts: source_code.line_starts.map_or_else(
                || Cow::Owned(LineIndexBuf::from_source_text(source_code.text)),
                Cow::Borrowed,
            ),
        }
    }

    /// Return the starting byte index of the line with the specified line index.
    /// Convenience method that already generates errors if necessary.
    fn line_start(&self, line_index: usize) -> io::Result<TextSize> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self
                .line_starts
                .get(line_index)
                .copied()
                .expect("failed despite previous check")),
            Ordering::Equal => Ok(self.source.text_len()),
            Ordering::Greater => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "overflow error",
            )),
        }
    }

    fn line_index(&self, byte_index: TextSize) -> usize {
        self.line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1)
    }

    fn line_range(&self, line_index: usize) -> io::Result<TextRange> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(TextRange::new(line_start, next_line_start))
    }

    fn line_number(&self, line_index: usize) -> OneIndexed {
        // SAFETY: Adding `1` to the value of `line_index` ensures it's non-zero
        OneIndexed::from_zero_indexed(line_index)
    }

    fn column_number(&self, line_index: usize, byte_index: TextSize) -> io::Result<OneIndexed> {
        let source = self.source;
        let line_range = self.line_range(line_index)?;
        let column_index = column_index(source, line_range, byte_index);

        // SAFETY: Adding `1` to the value of `column_index` ensures it's non-zero
        Ok(OneIndexed::from_zero_indexed(column_index))
    }

    /// Get a source location from a byte index into the text of this file
    pub fn location(&self, byte_index: TextSize) -> io::Result<SourceLocation> {
        let line_index = self.line_index(byte_index);

        Ok(SourceLocation {
            line_number: self.line_number(line_index),
            column_number: self.column_number(line_index, byte_index)?,
        })
    }
}

/// The column index at the given byte index in the source file.
/// This is the number of characters to the given byte index.
///
/// If the byte index is smaller than the start of the line, then `0` is returned.
/// If the byte index is past the end of the line, the column index of the last
/// character `+ 1` is returned.
fn column_index(source: &str, line_range: TextRange, byte_index: TextSize) -> usize {
    let end_index = std::cmp::min(
        byte_index,
        std::cmp::min(line_range.end(), source.text_len()),
    );

    (usize::from(line_range.start())..usize::from(end_index))
        .filter(|byte_index| source.is_char_boundary(byte_index + 1))
        .count()
}

/// Type-safe wrapper for a value whose logical range starts at `1`, for
/// instance the line or column numbers in a file
///
/// Internally this is represented as a [NonZeroUsize], this enables some
/// memory optimizations
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneIndexed(NonZeroUsize);

impl OneIndexed {
    // SAFETY: These constants are being initialized with non-zero values
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = unwrap(Self::new(1));
    /// The largest value that can be represented by this integer type
    pub const MAX: Self = unwrap(Self::new(usize::MAX));

    /// Creates a non-zero if the given value is not zero.
    pub const fn new(value: usize) -> Option<Self> {
        match NonZeroUsize::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Construct a new [OneIndexed] from a zero-indexed value
    pub const fn from_zero_indexed(value: usize) -> Self {
        Self(ONE.saturating_add(value))
    }

    /// Returns the value as a primitive type.
    pub const fn get(self) -> usize {
        self.0.get()
    }

    /// Return the zero-indexed primitive value for this [OneIndexed]
    pub const fn to_zero_indexed(self) -> usize {
        self.0.get() - 1
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at
    /// the numeric bounds instead of overflowing.
    pub const fn saturating_add(self, rhs: usize) -> Self {
        match NonZeroUsize::new(self.0.get().saturating_add(rhs)) {
            Some(value) => Self(value),
            None => Self::MAX,
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating
    /// at the numeric bounds instead of overflowing.
    pub const fn saturating_sub(self, rhs: usize) -> Self {
        match NonZeroUsize::new(self.0.get().saturating_sub(rhs)) {
            Some(value) => Self(value),
            None => Self::MIN,
        }
    }
}

impl fmt::Display for OneIndexed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> io::Result<()> {
        self.0.get().fmt(f)
    }
}

impl std::fmt::Display for OneIndexed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.get().fmt(f)
    }
}

/// Adapter type implementing [Iterator] for ranges of [OneIndexed],
/// since [std::iter::Step] is unstable
pub struct IntoIter(std::ops::Range<usize>);

impl IntoIter {
    /// Construct a new iterator over a range of [OneIndexed] of any kind
    /// (`..`, `a..`, `..b`, `..=c`, `d..e`, or `f..=g`)
    pub fn new<R: RangeBounds<OneIndexed>>(range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Included(value) => value.get(),
            Bound::Excluded(value) => value.get() + 1,
            Bound::Unbounded => 1,
        };

        let end = match range.end_bound() {
            Bound::Included(value) => value.get() + 1,
            Bound::Excluded(value) => value.get(),
            Bound::Unbounded => usize::MAX,
        };

        Self(start..end)
    }
}

impl Iterator for IntoIter {
    type Item = OneIndexed;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|index| OneIndexed::new(index).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|index| OneIndexed::new(index).unwrap())
    }
}

impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::{calculate_print_width, OneIndexed};

    #[test]
    fn print_width() {
        let one = NonZeroUsize::new(1).unwrap();
        let two = NonZeroUsize::new(2).unwrap();
        let three = NonZeroUsize::new(3).unwrap();
        let four = NonZeroUsize::new(4).unwrap();

        assert_eq!(calculate_print_width(OneIndexed::new(1).unwrap()), one);
        assert_eq!(calculate_print_width(OneIndexed::new(9).unwrap()), one);

        assert_eq!(calculate_print_width(OneIndexed::new(10).unwrap()), two);
        assert_eq!(calculate_print_width(OneIndexed::new(11).unwrap()), two);
        assert_eq!(calculate_print_width(OneIndexed::new(19).unwrap()), two);
        assert_eq!(calculate_print_width(OneIndexed::new(20).unwrap()), two);
        assert_eq!(calculate_print_width(OneIndexed::new(21).unwrap()), two);
        assert_eq!(calculate_print_width(OneIndexed::new(99).unwrap()), two);

        assert_eq!(calculate_print_width(OneIndexed::new(100).unwrap()), three);
        assert_eq!(calculate_print_width(OneIndexed::new(101).unwrap()), three);
        assert_eq!(calculate_print_width(OneIndexed::new(110).unwrap()), three);
        assert_eq!(calculate_print_width(OneIndexed::new(199).unwrap()), three);
        assert_eq!(calculate_print_width(OneIndexed::new(999).unwrap()), three);

        assert_eq!(calculate_print_width(OneIndexed::new(1000).unwrap()), four);
    }
}

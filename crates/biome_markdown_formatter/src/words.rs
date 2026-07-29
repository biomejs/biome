use biome_formatter::prelude::*;
use biome_formatter::{Format, FormatOptions, FormatResult, write};
use biome_markdown_syntax::{
    AnyMdInline, MdHardLine, MdInlineEmphasis, MdInlineItalic, MdInlineItemList,
    emphasis_ext::{MdEmphasisFence, MdItalicFence},
};
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, TextRange, TextSize, TokenText};

use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::{AsFormat, MarkdownFormatContext, MarkdownFormatter, format_removed, format_replaced};

/// A word slice from a syntax token — stores the text and source position for source maps.
#[derive(Debug, Clone)]
pub(crate) struct MdWord {
    text: TokenText,
    source_position: TextSize,
}

impl MdWord {
    fn new(text: TokenText, source_position: TextSize) -> Self {
        Self {
            text,
            source_position,
        }
    }

    fn is_single_underscore(&self) -> bool {
        self.text.text() == "_"
    }

    fn fmt_escaped(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        write!(f, [token("\\"), self])
    }
}

impl Format<MarkdownFormatContext> for MdWord {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        if f.source_map_generation().is_enabled() {
            f.write_element(FormatElement::MappedLocatedTokenText {
                slice: self.text.clone(),
                source_position: self.source_position,
            })
        } else {
            f.write_element(FormatElement::LocatedTokenText {
                slice: self.text.clone(),
                text_width: TextWidth::from_text(&self.text, f.options().indent_width()),
            })
        }
    }
}

/// A single atom within a word group.
#[derive(Debug, Clone)]
enum ProseAtom {
    /// A plain text word slice from an MdTextual token.
    Word(MdWord),
    /// An atomic inline element: emphasis, code, link, image, autolink, etc.
    InlineElement(AnyMdInline),
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
enum WordGroupEscape {
    /// Print the word group normally.
    #[default]
    None,
    /// Prefix each marker atom with a backslash.
    EscapeEachMarker,
    /// Prefix the first marker in the word group with a backslash.
    EscapeLeadingMarker,
    /// Print a five-marker sequence as strong emphasis around an escaped marker.
    EmptyStrongWithEscapedMarker,
}

/// Adjacent prose atoms that cannot be separated by whitespace or a line break.
#[derive(Debug, Clone, Default)]
pub(crate) struct WordGroup {
    atoms: Vec<ProseAtom>,
    escape: WordGroupEscape,
}

impl WordGroup {
    fn push(&mut self, atom: ProseAtom) {
        self.atoms.push(atom);
    }

    fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }

    pub(crate) fn starts_with_block_marker(&self) -> bool {
        let Some(ProseAtom::Word(word)) = self.atoms.first() else {
            return false;
        };
        let text = word.text.text();

        if text.starts_with('>')
            || matches!(text, "*" | "+" | "-")
            || matches!(text, "#" | "##" | "###" | "####" | "#####" | "######")
        {
            return true;
        }

        text.strip_suffix(['.', ')']).is_some_and(|number| {
            !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
        })
    }
}

/// An item in a flattened prose list.
#[derive(Debug, Clone)]
pub(crate) enum ProseItem {
    /// One or more adjacent atoms with no whitespace between them (a single "word").
    WordGroup(WordGroup),
    /// Whitespace between words — becomes the fill separator.
    Space,
    /// Source line break (\n) — behavior depends on proseWrap mode.
    SoftBreak,
    /// Hard line break (  \n or \\\n) — always breaks, segments the fill.
    /// Carries the original node so it can be formatted with proper token tracking.
    HardBreak(MdHardLine),
    /// Marks a source-line indent removed by structural prose formatting.
    OutdentedLineStart,
}

/// Prose items collected from an inline item list.
pub(crate) struct ProseItemList(Vec<ProseItem>);

impl ProseItemList {
    /// Collects prose as word groups separated by spaces and line breaks.
    pub(crate) fn from_inline_item_list(
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> SyntaxResult<Self> {
        let mut items = Self(Vec::new());
        let mut current_word_group = WordGroup::default();

        for item in node.iter() {
            match &item {
                AnyMdInline::MdTextual(text) => {
                    let token = text.value_token()?;
                    let token_text_str = token.text();

                    f.context().comments().is_suppressed(text.syntax());

                    if text.is_newline()? {
                        items.finish_word_group(&mut current_word_group);
                        items.0.push(ProseItem::SoftBreak);
                        f.state_mut().track_token(&token);
                        continue;
                    }

                    f.state_mut().track_token(&token);
                    let token_start = token.text_range().start();

                    let bytes = token_text_str.as_bytes();
                    let len = bytes.len();
                    let mut pos = 0usize;

                    while pos < len {
                        let white_space_start = pos;
                        while pos < len && bytes[pos].is_ascii_whitespace() {
                            pos += 1;
                        }
                        if pos > white_space_start {
                            items.finish_word_group(&mut current_word_group);
                            items.0.push(ProseItem::Space);
                        }

                        let word_start = pos;
                        while pos < len && !bytes[pos].is_ascii_whitespace() {
                            pos += 1;
                        }
                        if pos > word_start {
                            let start = TextSize::from(word_start as u32);
                            let end = TextSize::from(pos as u32);
                            let text_slice = token.token_text().slice(TextRange::new(start, end));
                            current_word_group.push(ProseAtom::Word(MdWord::new(
                                text_slice,
                                token_start + start,
                            )));
                        }
                    }
                }

                AnyMdInline::MdHardLine(hard_line) => {
                    items.finish_word_group(&mut current_word_group);
                    items.0.push(ProseItem::HardBreak(hard_line.clone()));
                }

                AnyMdInline::MdInlineItalic(_)
                | AnyMdInline::MdInlineEmphasis(_)
                | AnyMdInline::MdInlineCode(_)
                | AnyMdInline::MdInlineLink(_)
                | AnyMdInline::MdInlineImage(_)
                | AnyMdInline::MdAutolink(_)
                | AnyMdInline::MdReferenceLink(_)
                | AnyMdInline::MdReferenceImage(_)
                | AnyMdInline::MdInlineHtml(_)
                | AnyMdInline::MdEntityReference(_) => {
                    current_word_group.push(ProseAtom::InlineElement(item));
                }

                AnyMdInline::MdIndentToken(indent) => {
                    f.context().comments().is_suppressed(indent.syntax());
                    let token = indent.md_indent_char_token()?;
                    format_removed(&token).fmt(f).ok();
                    if current_word_group.is_empty()
                        && matches!(items.0.last(), Some(ProseItem::SoftBreak))
                    {
                        items.0.push(ProseItem::OutdentedLineStart);
                    }
                }

                AnyMdInline::MdHtmlBlock(html_block) => {
                    f.context().comments().is_suppressed(html_block.syntax());
                    items.finish_word_group(&mut current_word_group);
                    current_word_group.push(ProseAtom::InlineElement(item));
                    items.finish_word_group(&mut current_word_group);
                }

                AnyMdInline::MdCodeContent(code) => {
                    f.context().comments().is_suppressed(code.syntax());
                    items.finish_word_group(&mut current_word_group);
                    current_word_group.push(ProseAtom::InlineElement(item));
                    items.finish_word_group(&mut current_word_group);
                }
                AnyMdInline::MdQuotePrefix(prefix) => {
                    prefix
                        .format()
                        .with_options(FormatMdQuotePrefixOptions {
                            should_remove: true,
                        })
                        .fmt(f)
                        .ok();
                }
            }
        }

        items.finish_word_group(&mut current_word_group);
        items.mark_word_groups_for_escaping();
        items.mark_outdented_setext_markers();

        Ok(items)
    }

    fn finish_word_group(&mut self, current: &mut WordGroup) {
        if !current.is_empty() {
            self.0.push(ProseItem::WordGroup(std::mem::take(current)));
        }
    }

    pub(crate) fn as_slice(&self) -> &[ProseItem] {
        &self.0
    }

    pub(crate) fn remove_spaces_after_soft_breaks(&mut self) {
        let mut index = 0;
        while index < self.0.len() {
            if matches!(self.0[index], ProseItem::SoftBreak) {
                let mut start = index + 1;
                while start < self.0.len() && matches!(self.0[start], ProseItem::OutdentedLineStart)
                {
                    start += 1;
                }
                let mut end = start;
                while end < self.0.len() && matches!(self.0[end], ProseItem::Space) {
                    end += 1;
                }
                if end > start {
                    self.0.drain(start..end);
                }
            }
            index += 1;
        }
    }

    fn mark_word_groups_for_escaping(&mut self) {
        for index in 0..self.0.len() {
            let escape = WordGroupEscape::from(self, index);
            if let ProseItem::WordGroup(group) = &mut self.0[index] {
                group.escape = escape;
            }
        }
    }

    fn mark_outdented_setext_markers(&mut self) {
        let mut line_start = 0;

        for line_end in 0..=self.0.len() {
            let is_line_end = line_end == self.0.len()
                || matches!(
                    self.0[line_end],
                    ProseItem::SoftBreak | ProseItem::HardBreak(_)
                );
            if !is_line_end {
                continue;
            }

            if let Some(relative_index) =
                Self::outdented_setext_word_group(&self.0[line_start..line_end])
            {
                let ProseItem::WordGroup(group) = &mut self.0[line_start + relative_index] else {
                    unreachable!();
                };
                group.escape = WordGroupEscape::EscapeLeadingMarker;
            }

            line_start = line_end + 1;
        }
    }

    fn outdented_setext_word_group(line: &[ProseItem]) -> Option<usize> {
        let mut has_removed_indent = false;
        let mut first_word_group = None;
        let mut delimiter = None;

        for (index, item) in line.iter().enumerate() {
            match item {
                ProseItem::OutdentedLineStart if first_word_group.is_none() => {
                    has_removed_indent = true;
                }
                ProseItem::Space => {}
                ProseItem::WordGroup(group) if has_removed_indent => {
                    if first_word_group.is_some() && delimiter == Some(b'=') {
                        return None;
                    }
                    first_word_group.get_or_insert(index);
                    for atom in &group.atoms {
                        let ProseAtom::Word(word) = atom else {
                            return None;
                        };
                        for byte in word.text.text().bytes() {
                            if !matches!(byte, b'=' | b'-') {
                                return None;
                            }
                            if delimiter.is_some_and(|delimiter| delimiter != byte) {
                                return None;
                            }
                            delimiter = Some(byte);
                        }
                    }
                }
                _ => return None,
            }
        }

        delimiter.and(first_word_group)
    }

    fn has_matching_marker_boundary(
        &self,
        index: usize,
        marker_sequence: EmphasisMarkerSequence,
    ) -> bool {
        self.0[..index].iter().any(|item| match item {
            ProseItem::WordGroup(group) => marker_sequence.matches_start_of(group),
            _ => false,
        }) || self.0[index + 1..].iter().any(|item| match item {
            ProseItem::WordGroup(group) => marker_sequence.matches_end_of(group),
            _ => false,
        })
    }
}

enum WordGroupLayout<'a> {
    /// Prints every atom without additional escaping.
    ///
    /// ```text
    /// word -> word
    /// ```
    Default,
    /// Escapes every marker in a delimiter-only word group.
    ///
    /// ```text
    /// __ -> \_\_
    /// ```
    EscapeEachMarker,
    /// Escapes the first marker in an outdented Setext-like sequence.
    ///
    /// ```text
    /// --- -> \---
    /// ```
    EscapeLeadingMarker,
    /// Prints five markers as strong emphasis containing one escaped marker.
    ///
    /// ```text
    /// _____ -> **\_**
    /// ```
    EmptyStrongSequence {
        delimiter: u8,
        positions: [TextSize; 5],
    },
    /// Escapes an underscore that follows literal punctuation.
    ///
    /// ```text
    /// !_1_2 -> !\_1_2
    /// ```
    LiteralLeadingUnderscore {
        leading: &'a MdWord,
        trailing: &'a MdWord,
    },
    /// Escapes unmatched underscores adjacent to parsed italic or strong emphasis.
    ///
    /// ```text
    /// __foo_ -> \__foo_
    /// ```
    UnmatchedUnderscore(UnmatchedUnderscoreLayout<'a>),
}

enum UnmatchedUnderscoreLayout<'a> {
    /// Escapes a literal underscore before underscore-delimited italic text.
    ///
    /// ```text
    /// __foo_ -> \__foo_
    /// ```
    LeadingWithItalic {
        leading: &'a MdWord,
        italic: &'a MdInlineItalic,
    },
    /// Escapes an underscore-delimited italic span and its trailing literal underscore.
    ///
    /// ```text
    /// _foo__ -> \_foo\_\_
    /// ```
    ItalicWithTrailing {
        italic: &'a MdInlineItalic,
        trailing: &'a MdWord,
    },
    /// Escapes the unmatched markers around italic text in a four-underscore sequence.
    ///
    /// ```text
    /// ____foo_ -> \__\_\_foo_
    /// ```
    LeadingSequenceWithItalic {
        first: &'a MdWord,
        second: &'a MdWord,
        third: &'a MdWord,
        italic: &'a MdInlineItalic,
    },
    /// Moves a leading literal underscore inside normalized strong emphasis.
    ///
    /// ```text
    /// ___foo__ -> **\_foo**
    /// ```
    LeadingWithEmphasis {
        leading: &'a MdWord,
        emphasis: &'a MdInlineEmphasis,
    },
    /// Moves a trailing literal underscore inside normalized strong emphasis.
    ///
    /// ```text
    /// __foo___ -> **foo\_**
    /// ```
    EmphasisWithTrailing {
        emphasis: &'a MdInlineEmphasis,
        trailing: &'a MdWord,
    },
}

impl Format<MarkdownFormatContext> for UnmatchedUnderscoreLayout<'_> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        match self {
            Self::LeadingWithItalic { leading, italic } => {
                write!(
                    f,
                    [format_with(|f| leading.fmt_escaped(f)), italic.format()]
                )
            }
            Self::ItalicWithTrailing { italic, trailing } => {
                let l_fence = italic.l_fence()?;
                let r_fence = italic.r_fence()?;
                f.context().comments().is_suppressed(italic.syntax());

                write!(
                    f,
                    [
                        format_replaced(&l_fence, &text("\\_", Some(l_fence.text_range().start()))),
                        italic.content().format(),
                        format_replaced(&r_fence, &text("\\_", Some(r_fence.text_range().start()))),
                        format_with(|f| trailing.fmt_escaped(f)),
                    ]
                )
            }
            Self::LeadingSequenceWithItalic {
                first,
                second,
                third,
                italic,
            } => {
                let l_fence = italic.l_fence()?;
                let r_fence = italic.r_fence()?;
                f.context().comments().is_suppressed(italic.syntax());

                write!(
                    f,
                    [
                        format_with(|f| first.fmt_escaped(f)),
                        second,
                        format_with(|f| third.fmt_escaped(f)),
                        format_replaced(&l_fence, &text("\\_", Some(l_fence.text_range().start()))),
                        italic.content().format(),
                        r_fence.format(),
                    ]
                )
            }
            Self::LeadingWithEmphasis { leading, emphasis } => {
                let l_fence = emphasis.l_fence()?;
                let r_fence = emphasis.r_fence()?;
                f.context().comments().is_suppressed(emphasis.syntax());

                write!(
                    f,
                    [
                        format_replaced(&l_fence, &text("**", Some(l_fence.text_range().start()))),
                        format_with(|f| leading.fmt_escaped(f)),
                        emphasis.content().format(),
                        format_replaced(&r_fence, &text("**", Some(r_fence.text_range().start()))),
                    ]
                )
            }
            Self::EmphasisWithTrailing { emphasis, trailing } => {
                let l_fence = emphasis.l_fence()?;
                let r_fence = emphasis.r_fence()?;
                f.context().comments().is_suppressed(emphasis.syntax());

                write!(
                    f,
                    [
                        format_replaced(&l_fence, &text("**", Some(l_fence.text_range().start()))),
                        emphasis.content().format(),
                        format_with(|f| trailing.fmt_escaped(f)),
                        format_replaced(&r_fence, &text("**", Some(r_fence.text_range().start()))),
                    ]
                )
            }
        }
    }
}

impl WordGroup {
    fn layout(&self) -> SyntaxResult<WordGroupLayout<'_>> {
        Ok(match self.escape {
            WordGroupEscape::EscapeEachMarker => WordGroupLayout::EscapeEachMarker,
            WordGroupEscape::EscapeLeadingMarker => WordGroupLayout::EscapeLeadingMarker,
            WordGroupEscape::EmptyStrongWithEscapedMarker => self.empty_strong_sequence_layout(),
            WordGroupEscape::None => self
                .literal_leading_underscore_layout()
                .or(self
                    .unmatched_underscore_layout()?
                    .map(WordGroupLayout::UnmatchedUnderscore))
                .unwrap_or(WordGroupLayout::Default),
        })
    }

    fn empty_strong_sequence_layout(&self) -> WordGroupLayout<'_> {
        let Some(marker_sequence) = self.emphasis_marker_sequence() else {
            return WordGroupLayout::Default;
        };

        debug_assert_eq!(
            marker_sequence.len, 5,
            "empty strong formatting is selected only for five-marker sequences"
        );

        let mut positions = Vec::with_capacity(marker_sequence.len);
        for atom in &self.atoms {
            let ProseAtom::Word(word) = atom else {
                return WordGroupLayout::Default;
            };
            positions.extend(
                (0..word.text.text().len())
                    .map(|index| word.source_position + TextSize::from(index as u32)),
            );
        }

        let Ok(positions) = positions.try_into() else {
            return WordGroupLayout::Default;
        };

        WordGroupLayout::EmptyStrongSequence {
            delimiter: marker_sequence.delimiter,
            positions,
        }
    }

    fn literal_leading_underscore_layout(&self) -> Option<WordGroupLayout<'_>> {
        let [ProseAtom::Word(leading), ProseAtom::Word(trailing)] = self.atoms.as_slice() else {
            return None;
        };
        let trailing_text = trailing.text.text();
        let remainder = trailing_text.strip_prefix('_')?;
        if !remainder.contains('_')
            || !leading.text.text().chars().next_back().is_some_and(|char| {
                !char.is_alphanumeric() && !char.is_whitespace() && !matches!(char, '*' | '_')
            })
        {
            return None;
        }

        Some(WordGroupLayout::LiteralLeadingUnderscore { leading, trailing })
    }

    fn unmatched_underscore_layout(&self) -> SyntaxResult<Option<UnmatchedUnderscoreLayout<'_>>> {
        let layout = match self.atoms.as_slice() {
            [
                ProseAtom::Word(leading),
                ProseAtom::InlineElement(AnyMdInline::MdInlineItalic(italic)),
            ] if leading.is_single_underscore() && italic.fence()? == MdItalicFence::Underscore => {
                Some(UnmatchedUnderscoreLayout::LeadingWithItalic { leading, italic })
            }
            [
                ProseAtom::InlineElement(AnyMdInline::MdInlineItalic(italic)),
                ProseAtom::Word(trailing),
            ] if trailing.is_single_underscore()
                && italic.fence()? == MdItalicFence::Underscore =>
            {
                Some(UnmatchedUnderscoreLayout::ItalicWithTrailing { italic, trailing })
            }
            [
                ProseAtom::Word(first),
                ProseAtom::Word(second),
                ProseAtom::Word(third),
                ProseAtom::InlineElement(AnyMdInline::MdInlineItalic(italic)),
            ] if first.is_single_underscore()
                && second.is_single_underscore()
                && third.is_single_underscore()
                && italic.fence()? == MdItalicFence::Underscore =>
            {
                Some(UnmatchedUnderscoreLayout::LeadingSequenceWithItalic {
                    first,
                    second,
                    third,
                    italic,
                })
            }
            [
                ProseAtom::Word(leading),
                ProseAtom::InlineElement(AnyMdInline::MdInlineEmphasis(emphasis)),
            ] if leading.is_single_underscore()
                && emphasis.fence()? == MdEmphasisFence::DoubleUnderscore =>
            {
                Some(UnmatchedUnderscoreLayout::LeadingWithEmphasis { leading, emphasis })
            }
            [
                ProseAtom::InlineElement(AnyMdInline::MdInlineEmphasis(emphasis)),
                ProseAtom::Word(trailing),
            ] if trailing.is_single_underscore()
                && emphasis.fence()? == MdEmphasisFence::DoubleUnderscore =>
            {
                Some(UnmatchedUnderscoreLayout::EmphasisWithTrailing { emphasis, trailing })
            }
            _ => None,
        };

        Ok(layout)
    }

    fn emphasis_marker_sequence(&self) -> Option<EmphasisMarkerSequence> {
        let mut delimiter = None;
        let mut len = 0usize;

        for atom in &self.atoms {
            let ProseAtom::Word(word) = atom else {
                return None;
            };

            for byte in word.text.text().bytes() {
                if !matches!(byte, b'*' | b'_') {
                    return None;
                }

                if let Some(delimiter) = delimiter {
                    if delimiter != byte {
                        return None;
                    }
                } else {
                    delimiter = Some(byte);
                }

                len += 1;
            }
        }

        delimiter.map(|delimiter| EmphasisMarkerSequence { delimiter, len })
    }

    fn fmt_default(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        for atom in &self.atoms {
            match atom {
                ProseAtom::Word(word) => word.fmt(f)?,
                ProseAtom::InlineElement(elem) => elem.format().fmt(f)?,
            }
        }
        Ok(())
    }

    fn fmt_each_marker(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        for atom in &self.atoms {
            match atom {
                ProseAtom::Word(word) => write!(f, [token("\\"), word])?,
                ProseAtom::InlineElement(elem) => elem.format().fmt(f)?,
            }
        }
        Ok(())
    }

    fn fmt_empty_strong_sequence(
        &self,
        delimiter: u8,
        positions: [TextSize; 5],
        f: &mut Formatter<MarkdownFormatContext>,
    ) -> FormatResult<()> {
        let escaped_marker = if delimiter == b'*' { "\\*" } else { "\\_" };
        write!(
            f,
            [
                text("**", Some(positions[0])),
                text(escaped_marker, Some(positions[2])),
                text("**", Some(positions[3])),
            ]
        )
    }

    fn fmt_literal_leading_underscore(
        &self,
        leading: &MdWord,
        trailing: &MdWord,
        f: &mut Formatter<MarkdownFormatContext>,
    ) -> FormatResult<()> {
        write!(f, [leading, token("\\"), trailing])
    }
}

impl Format<MarkdownFormatContext> for WordGroup {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let layout = self.layout()?;

        match layout {
            WordGroupLayout::Default => self.fmt_default(f),
            WordGroupLayout::EscapeEachMarker => self.fmt_each_marker(f),
            WordGroupLayout::EscapeLeadingMarker => {
                write!(f, [token("\\")])?;
                self.fmt_default(f)
            }
            WordGroupLayout::EmptyStrongSequence {
                delimiter,
                positions,
            } => self.fmt_empty_strong_sequence(delimiter, positions, f),
            WordGroupLayout::LiteralLeadingUnderscore { leading, trailing } => {
                self.fmt_literal_leading_underscore(leading, trailing, f)
            }
            WordGroupLayout::UnmatchedUnderscore(layout) => layout.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct EmphasisMarkerSequence {
    delimiter: u8,
    len: usize,
}

impl EmphasisMarkerSequence {
    fn matches_start_of(self, group: &WordGroup) -> bool {
        self.matches_boundary(group.atoms.iter().map(|atom| match atom {
            ProseAtom::Word(word) => Some(word.text.text().bytes()),
            ProseAtom::InlineElement(_) => None,
        }))
    }

    fn matches_end_of(self, group: &WordGroup) -> bool {
        self.matches_boundary(group.atoms.iter().rev().map(|atom| match atom {
            ProseAtom::Word(word) => Some(word.text.text().bytes().rev()),
            ProseAtom::InlineElement(_) => None,
        }))
    }

    fn matches_boundary<I, B>(self, words: I) -> bool
    where
        I: IntoIterator<Item = Option<B>>,
        B: IntoIterator<Item = u8>,
    {
        let mut remaining = self.len;

        for word in words {
            let Some(bytes) = word else {
                return false;
            };

            for byte in bytes {
                if byte != self.delimiter {
                    return false;
                }

                remaining -= 1;
                if remaining == 0 {
                    return true;
                }
            }
        }

        false
    }
}

impl WordGroupEscape {
    /// Returns how a word group should print marker-only text.
    ///
    /// Marker sequences containing two to five identical emphasis markers need
    /// escaping unless another group forms a matching boundary in the paragraph.
    fn from(items: &ProseItemList, index: usize) -> Self {
        let Some(ProseItem::WordGroup(group)) = items.0.get(index) else {
            return Self::None;
        };

        let Some(marker_sequence) = group.emphasis_marker_sequence() else {
            return Self::None;
        };

        if items.has_matching_marker_boundary(index, marker_sequence) {
            return Self::None;
        }

        match marker_sequence.len {
            2..=4 => Self::EscapeEachMarker,
            5 => Self::EmptyStrongWithEscapedMarker,
            _ => Self::None,
        }
    }
}

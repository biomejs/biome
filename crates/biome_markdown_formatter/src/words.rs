use biome_formatter::rowan::prelude::*;
use biome_formatter::{Format, FormatOptions, FormatResult, write};
use biome_markdown_syntax::{
    AnyMdInline, GfmStrikethrough, MarkdownSyntaxNode, MdHardLine, MdInlineEmphasis,
    MdInlineItalic, MdInlineItemList, MdQuote,
    emphasis_ext::{MdEmphasisFence, MdItalicFence},
};
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, TextRange, TextSize, TokenText};
use biome_unicode_table::{
    is_cjk_punctuation, is_cjk_segment_break_character, is_default_ignorable_code_point,
};
use smallvec::SmallVec;

use crate::context::ProseWrap;
use crate::markdown::auxiliary::inline_emphasis::format_emphasis_fence;
use crate::markdown::auxiliary::inline_italic::{
    is_inside_reference_image, normalized_fence_kind, swapped_strong_emphasis, write_fence,
};
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
    /// The opening delimiter of an inline element whose content is collected as prose.
    OpeningDelimiter(DelimitedInline),
    /// The closing delimiter of an inline element whose content is collected as prose.
    ClosingDelimiter(DelimitedInline),
}

impl Format<MarkdownFormatContext> for ProseAtom {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        match self {
            Self::Word(word) => word.fmt(f),
            Self::InlineElement(element) => element.format().fmt(f),
            Self::OpeningDelimiter(delimited) => delimited.fmt_opening_delimiter(f),
            Self::ClosingDelimiter(delimited) => delimited.fmt_closing_delimiter(f),
        }
    }
}

/// An inline element that wraps with the surrounding prose.
///
/// Its delimiters attach to the first and last words of its content, and the words in between
/// become word groups of the enclosing paragraph. This lets a line break fall inside the element
/// instead of only before or after it.
#[derive(Debug, Clone)]
enum DelimitedInline {
    Emphasis(MdInlineEmphasis),
    Italic(MdInlineItalic),
    /// An italic containing only strong emphasis, printed with swapped fences as `**_..._**`.
    StrongInsideItalic {
        italic: MdInlineItalic,
        emphasis: MdInlineEmphasis,
    },
    Strikethrough(GfmStrikethrough),
}

impl DelimitedInline {
    /// Returns the element if its content can wrap with the surrounding prose.
    ///
    /// The content qualifies only if one of its text nodes contains whitespace, or it contains
    /// another element that qualifies. Other content has nothing to wrap, and keeps the element
    /// formatter's layouts for escaped delimiters. Italics in reference images keep their source
    /// delimiters, and suppressed or commented elements keep their own formatting.
    ///
    /// Elements inside block quotes don't qualify either. When an element spans lines there, the
    /// parser keeps the quote markers of its continuation lines as text of the content, which would
    /// otherwise be printed as words. Elements on a single line are excluded too, so that wrapping
    /// one across lines doesn't change its layout when the output is formatted again.
    fn from_inline(item: &AnyMdInline, f: &MarkdownFormatter) -> Option<Self> {
        let delimited = match item {
            AnyMdInline::MdInlineEmphasis(emphasis) => Self::Emphasis(emphasis.clone()),
            AnyMdInline::MdInlineItalic(italic) if !is_inside_reference_image(italic) => {
                match swapped_strong_emphasis(italic).ok()? {
                    Some(emphasis) => Self::StrongInsideItalic {
                        italic: italic.clone(),
                        emphasis,
                    },
                    None => Self::Italic(italic.clone()),
                }
            }
            AnyMdInline::GfmStrikethrough(strikethrough) => {
                Self::Strikethrough(strikethrough.clone())
            }
            _ => return None,
        };

        let syntax = delimited.syntax();
        if syntax
            .ancestors()
            .any(|ancestor| MdQuote::can_cast(ancestor.kind()))
        {
            return None;
        }

        let comments = f.context().comments();
        let is_formatted_by_node = |node: &MarkdownSyntaxNode| {
            comments.has_comments(node)
                || comments.is_suppressed(node)
                || comments.is_global_suppressed(node)
        };
        if is_formatted_by_node(syntax) {
            return None;
        }
        if let Self::StrongInsideItalic { emphasis, .. } = &delimited
            && is_formatted_by_node(emphasis.syntax())
        {
            return None;
        }

        let has_wrappable_content = delimited.content().iter().any(|item| match &item {
            AnyMdInline::MdTextual(textual) => textual
                .value_token()
                .is_ok_and(|token| token.text().bytes().any(|byte| byte.is_ascii_whitespace())),
            item => Self::from_inline(item, f).is_some(),
        });

        has_wrappable_content.then_some(delimited)
    }

    fn syntax(&self) -> &MarkdownSyntaxNode {
        match self {
            Self::Emphasis(emphasis) => emphasis.syntax(),
            Self::Italic(italic) | Self::StrongInsideItalic { italic, .. } => italic.syntax(),
            Self::Strikethrough(strikethrough) => strikethrough.syntax(),
        }
    }

    fn content(&self) -> MdInlineItemList {
        match self {
            Self::Emphasis(emphasis) | Self::StrongInsideItalic { emphasis, .. } => {
                emphasis.content()
            }
            Self::Italic(italic) => italic.content(),
            Self::Strikethrough(strikethrough) => strikethrough.content(),
        }
    }

    fn fmt_opening_delimiter(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        match self {
            Self::Emphasis(emphasis) => {
                format_emphasis_fence(&emphasis.l_fence()?, emphasis.fence()?).fmt(f)
            }
            Self::Italic(italic) => {
                write_fence(&italic.l_fence()?, normalized_fence_kind(italic)?, f)
            }
            Self::StrongInsideItalic { italic, emphasis } => {
                let italic_fence = italic.l_fence()?;
                let emphasis_fence = emphasis.l_fence()?;
                write!(
                    f,
                    [
                        format_replaced(
                            &italic_fence,
                            &text("**", Some(italic_fence.text_range().start()))
                        ),
                        format_replaced(
                            &emphasis_fence,
                            &text("_", Some(emphasis_fence.text_range().start()))
                        ),
                    ]
                )
            }
            Self::Strikethrough(strikethrough) => strikethrough.l_fence_token()?.format().fmt(f),
        }
    }

    fn fmt_closing_delimiter(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        match self {
            Self::Emphasis(emphasis) => {
                format_emphasis_fence(&emphasis.r_fence()?, emphasis.fence()?).fmt(f)
            }
            Self::Italic(italic) => {
                write_fence(&italic.r_fence()?, normalized_fence_kind(italic)?, f)
            }
            Self::StrongInsideItalic { italic, emphasis } => {
                let emphasis_fence = emphasis.r_fence()?;
                let italic_fence = italic.r_fence()?;
                write!(
                    f,
                    [
                        format_replaced(
                            &emphasis_fence,
                            &text("_", Some(emphasis_fence.text_range().start()))
                        ),
                        format_replaced(
                            &italic_fence,
                            &text("**", Some(italic_fence.text_range().start()))
                        ),
                    ]
                )
            }
            Self::Strikethrough(strikethrough) => strikethrough.r_fence_token()?.format().fmt(f),
        }
    }
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
    atoms: SmallVec<[ProseAtom; 1]>,
    escape: WordGroupEscape,
    /// Whether the group is collected from the content of a [DelimitedInline] element. The
    /// element's formatter prints its content without escaping, so such groups use the default
    /// layout and don't count as marker boundaries for groups outside the element.
    is_delimited_content: bool,
}

/// Result of inspecting prose content for a character that controls soft-break handling.
enum BoundaryCharacter {
    /// The nearest rendered character at the inspected boundary.
    Significant(char),
    /// The content contributes no relevant character, so inspection can continue past it.
    Transparent,
    /// The rendered boundary cannot be determined reliably, so inspection must stop.
    Unknown,
}

fn first_significant_character_in_text(text: &str) -> BoundaryCharacter {
    text.chars()
        .find(|character| !is_default_ignorable_code_point(*character))
        .map_or(
            BoundaryCharacter::Transparent,
            BoundaryCharacter::Significant,
        )
}

fn last_significant_character_in_text(text: &str) -> BoundaryCharacter {
    text.chars()
        .rev()
        .find(|character| !is_default_ignorable_code_point(*character))
        .map_or(
            BoundaryCharacter::Transparent,
            BoundaryCharacter::Significant,
        )
}

fn first_significant_character_in_inline_list(list: &MdInlineItemList) -> BoundaryCharacter {
    for item in list.iter() {
        let character = match &item {
            AnyMdInline::MdTextual(textual) => match textual.value_token() {
                Ok(token) => first_significant_character_in_text(token.text()),
                Err(_) => BoundaryCharacter::Unknown,
            },
            AnyMdInline::MdIndentToken(_) | AnyMdInline::MdQuotePrefix(_) => {
                BoundaryCharacter::Transparent
            }
            _ => first_significant_character_in_inline(&item),
        };
        match character {
            BoundaryCharacter::Transparent => {}
            _ => return character,
        }
    }
    BoundaryCharacter::Transparent
}

fn last_significant_character_in_inline_list(list: &MdInlineItemList) -> BoundaryCharacter {
    for item in list.iter().rev() {
        let character = match &item {
            AnyMdInline::MdTextual(textual) => match textual.value_token() {
                Ok(token) => last_significant_character_in_text(token.text()),
                Err(_) => BoundaryCharacter::Unknown,
            },
            AnyMdInline::MdIndentToken(_) | AnyMdInline::MdQuotePrefix(_) => {
                BoundaryCharacter::Transparent
            }
            _ => last_significant_character_in_inline(&item),
        };
        match character {
            BoundaryCharacter::Transparent => {}
            _ => return character,
        }
    }
    BoundaryCharacter::Transparent
}

fn numeric_entity_character(text: &str) -> Option<char> {
    let digits = text.strip_prefix("&#")?.strip_suffix(';')?;
    let (digits, radix) = digits
        .strip_prefix(['x', 'X'])
        .map_or((digits, 10), |digits| (digits, 16));
    u32::from_str_radix(digits, radix)
        .ok()
        .and_then(char::from_u32)
}

fn first_significant_character_in_inline(item: &AnyMdInline) -> BoundaryCharacter {
    match item {
        AnyMdInline::MdInlineItalic(italic) => {
            first_significant_character_in_inline_list(&italic.content())
        }
        AnyMdInline::MdInlineEmphasis(emphasis) => {
            first_significant_character_in_inline_list(&emphasis.content())
        }
        AnyMdInline::MdInlineCode(code) => {
            first_significant_character_in_inline_list(&code.content())
        }
        AnyMdInline::MdInlineLink(link) => first_significant_character_in_inline_list(&link.text()),
        AnyMdInline::MdAutolink(autolink) => {
            first_significant_character_in_inline_list(&autolink.value())
        }
        AnyMdInline::MdReferenceLink(link) => {
            first_significant_character_in_inline_list(&link.text())
        }
        AnyMdInline::MdEntityReference(entity) => match entity.value_token() {
            Ok(token) => numeric_entity_character(token.text()).map_or_else(
                || first_significant_character_in_text(token.text()),
                |character| {
                    if is_default_ignorable_code_point(character) {
                        BoundaryCharacter::Transparent
                    } else {
                        BoundaryCharacter::Significant(character)
                    }
                },
            ),
            Err(_) => BoundaryCharacter::Unknown,
        },
        AnyMdInline::GfmStrikethrough(strikethrough) => {
            first_significant_character_in_inline_list(&strikethrough.content())
        }
        _ => BoundaryCharacter::Unknown,
    }
}

fn last_significant_character_in_inline(item: &AnyMdInline) -> BoundaryCharacter {
    match item {
        AnyMdInline::MdInlineItalic(italic) => {
            last_significant_character_in_inline_list(&italic.content())
        }
        AnyMdInline::MdInlineEmphasis(emphasis) => {
            last_significant_character_in_inline_list(&emphasis.content())
        }
        AnyMdInline::MdInlineCode(code) => {
            last_significant_character_in_inline_list(&code.content())
        }
        AnyMdInline::MdInlineLink(link) => last_significant_character_in_inline_list(&link.text()),
        AnyMdInline::MdAutolink(autolink) => {
            last_significant_character_in_inline_list(&autolink.value())
        }
        AnyMdInline::MdReferenceLink(link) => {
            last_significant_character_in_inline_list(&link.text())
        }
        AnyMdInline::MdEntityReference(entity) => match entity.value_token() {
            Ok(token) => numeric_entity_character(token.text()).map_or_else(
                || last_significant_character_in_text(token.text()),
                |character| {
                    if is_default_ignorable_code_point(character) {
                        BoundaryCharacter::Transparent
                    } else {
                        BoundaryCharacter::Significant(character)
                    }
                },
            ),
            Err(_) => BoundaryCharacter::Unknown,
        },
        AnyMdInline::GfmStrikethrough(strikethrough) => {
            last_significant_character_in_inline_list(&strikethrough.content())
        }
        _ => BoundaryCharacter::Unknown,
    }
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

    fn first_significant_character(&self) -> Option<char> {
        for atom in &self.atoms {
            let character = match atom {
                ProseAtom::Word(word) => first_significant_character_in_text(word.text.text()),
                ProseAtom::InlineElement(item) => first_significant_character_in_inline(item),
                ProseAtom::OpeningDelimiter(_) | ProseAtom::ClosingDelimiter(_) => {
                    BoundaryCharacter::Transparent
                }
            };
            match character {
                BoundaryCharacter::Significant(character) => return Some(character),
                BoundaryCharacter::Transparent => {}
                BoundaryCharacter::Unknown => return None,
            }
        }
        None
    }

    fn last_significant_character(&self) -> Option<char> {
        for atom in self.atoms.iter().rev() {
            let character = match atom {
                ProseAtom::Word(word) => last_significant_character_in_text(word.text.text()),
                ProseAtom::InlineElement(item) => last_significant_character_in_inline(item),
                ProseAtom::OpeningDelimiter(_) | ProseAtom::ClosingDelimiter(_) => {
                    BoundaryCharacter::Transparent
                }
            };
            match character {
                BoundaryCharacter::Significant(character) => return Some(character),
                BoundaryCharacter::Transparent => {}
                BoundaryCharacter::Unknown => return None,
            }
        }
        None
    }

    /// Returns whether a soft break between this group and `next` must remain a line break.
    ///
    /// The boundary uses the nearest rendered characters and skips default-ignorable code points.
    /// A soft break is preserved when both characters use Chinese or Japanese no-space line
    /// joining, or when either character is CJK punctuation. This returns `false` when either
    /// boundary character cannot be determined.
    pub(crate) fn should_preserve_soft_break_before(&self, next: &Self) -> bool {
        let (Some(before), Some(after)) = (
            self.last_significant_character(),
            next.first_significant_character(),
        ) else {
            return false;
        };

        (is_cjk_segment_break_character(before) && is_cjk_segment_break_character(after))
            || is_cjk_punctuation(before)
            || is_cjk_punctuation(after)
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

impl ProseItem {
    pub(crate) const fn as_word_group(&self) -> Option<&WordGroup> {
        match self {
            Self::WordGroup(group) => Some(group),
            _ => None,
        }
    }

    pub(crate) const fn is_prose_separator(&self) -> bool {
        matches!(
            self,
            Self::Space | Self::SoftBreak | Self::OutdentedLineStart
        )
    }
}

/// Prose items collected from an inline item list.
pub(crate) struct ProseItemList(Vec<ProseItem>);

impl ProseItemList {
    /// Collects prose as word groups separated by spaces and line breaks.
    ///
    /// Unless prose wrap is `preserve`, the content of emphasis, italic, and strikethrough
    /// elements is collected as prose of the enclosing list, so it wraps with the surrounding
    /// words. See [DelimitedInline].
    pub(crate) fn from_inline_item_list(
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> SyntaxResult<Self> {
        let mut collector = ProseItemCollector {
            items: Vec::new(),
            current_word_group: WordGroup::default(),
            expand_delimited_inlines: f.options().prose_wrap() != ProseWrap::Preserve,
            delimited_depth: 0,
        };
        collector.collect(node, f)?;
        collector.finish_word_group(f)?;

        let mut items = Self(collector.items);
        items.mark_word_groups_for_escaping();
        items.mark_outdented_setext_markers();

        Ok(items)
    }

    pub(crate) fn as_slice(&self) -> &[ProseItem] {
        &self.0
    }

    /// Returns the indices of the lines that follow a soft break and start with
    /// a block marker after indentation, such as `- item` in `text\n    - item`.
    ///
    /// Lines are counted from 0, and every soft or hard break starts a new line.
    /// The indentation keeps such a line in the paragraph. The formatter removes
    /// it, so the marker would start a block at the start of a line.
    ///
    /// Inside a list item, the formatter prints the item's content indentation
    /// again, so only the indentation past it counts.
    pub(crate) fn indented_block_marker_lines(&self, inside_list: bool) -> Vec<usize> {
        let mut lines = Vec::new();
        let mut line_index = 0;

        for (index, item) in self.0.iter().enumerate() {
            match item {
                ProseItem::SoftBreak => {
                    line_index += 1;
                    let mut line = self.0[index + 1..].iter().peekable();
                    let mut has_indent = false;
                    while let Some(item) = line.next_if(|item| {
                        matches!(item, ProseItem::OutdentedLineStart | ProseItem::Space)
                    }) {
                        has_indent |= !inside_list || matches!(item, ProseItem::Space);
                    }
                    // An escaped marker, such as `\-` for a setext-like line,
                    // no longer starts a block.
                    if has_indent
                        && line
                            .next()
                            .and_then(ProseItem::as_word_group)
                            .is_some_and(|group| {
                                group.starts_with_block_marker()
                                    && !matches!(group.escape, WordGroupEscape::EscapeLeadingMarker)
                            })
                    {
                        lines.push(line_index);
                    }
                }
                ProseItem::HardBreak(_) => line_index += 1,
                _ => {}
            }
        }

        lines
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
            ProseItem::WordGroup(group) => {
                !group.is_delimited_content && marker_sequence.matches_start_of(group)
            }
            _ => false,
        }) || self.0[index + 1..].iter().any(|item| match item {
            ProseItem::WordGroup(group) => {
                !group.is_delimited_content && marker_sequence.matches_end_of(group)
            }
            _ => false,
        })
    }
}

/// Builds the items of a [ProseItemList].
struct ProseItemCollector {
    items: Vec<ProseItem>,
    /// The word group receiving atoms until the next separator.
    current_word_group: WordGroup,
    /// Whether [DelimitedInline] elements are expanded into the prose of their content.
    expand_delimited_inlines: bool,
    /// The number of [DelimitedInline] elements whose content is being collected.
    delimited_depth: usize,
}

impl ProseItemCollector {
    fn collect(&mut self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> SyntaxResult<()> {
        for item in node.iter() {
            match &item {
                AnyMdInline::MdTextual(text) => {
                    let token = text.value_token()?;
                    let token_text_str = token.text();

                    f.context().comments().is_suppressed(text.syntax());

                    if text.is_newline()? {
                        self.finish_word_group(f)?;
                        self.items.push(ProseItem::SoftBreak);
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
                            self.finish_word_group(f)?;
                            self.items.push(ProseItem::Space);
                        }

                        let word_start = pos;
                        while pos < len && !bytes[pos].is_ascii_whitespace() {
                            pos += 1;
                        }
                        if pos > word_start {
                            let start = TextSize::from(word_start as u32);
                            let end = TextSize::from(pos as u32);
                            let text_slice = token.token_text().slice(TextRange::new(start, end));
                            self.current_word_group.push(ProseAtom::Word(MdWord::new(
                                text_slice,
                                token_start + start,
                            )));
                        }
                    }
                }

                AnyMdInline::MdHardLine(hard_line) => {
                    self.finish_word_group(f)?;
                    self.items.push(ProseItem::HardBreak(hard_line.clone()));
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
                | AnyMdInline::MdEntityReference(_)
                | AnyMdInline::GfmStrikethrough(_)
                | AnyMdInline::GfmTaskListItem(_) => {
                    self.current_word_group.push(ProseAtom::InlineElement(item));
                }

                AnyMdInline::MdIndentToken(indent) => {
                    f.context().comments().is_suppressed(indent.syntax());
                    let token = indent.md_indent_char_token()?;
                    format_removed(&token).fmt(f).ok();
                    if self.current_word_group.is_empty()
                        && matches!(self.items.last(), Some(ProseItem::SoftBreak))
                    {
                        self.items.push(ProseItem::OutdentedLineStart);
                    }
                }

                AnyMdInline::MdHtmlBlock(html_block) => {
                    f.context().comments().is_suppressed(html_block.syntax());
                    self.finish_word_group(f)?;
                    self.current_word_group.push(ProseAtom::InlineElement(item));
                    self.finish_word_group(f)?;
                }

                AnyMdInline::MdCodeContent(code) => {
                    f.context().comments().is_suppressed(code.syntax());
                    self.finish_word_group(f)?;
                    self.current_word_group.push(ProseAtom::InlineElement(item));
                    self.finish_word_group(f)?;
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

        Ok(())
    }

    /// Appends the current word group to the items.
    ///
    /// Expands each [DelimitedInline] element of the group into its delimiters and the prose of its
    /// content. The group stays unchanged when it needs an escaping layout for underscores next to
    /// an element, because that layout formats the element as a whole.
    fn finish_word_group(&mut self, f: &mut MarkdownFormatter) -> SyntaxResult<()> {
        let group = std::mem::take(&mut self.current_word_group);
        if group.is_empty() {
            return Ok(());
        }

        if !self.expand_delimited_inlines || group.unmatched_underscore_layout()?.is_some() {
            self.push_word_group(group);
            return Ok(());
        }

        let mut has_expanded = false;
        for atom in group.atoms {
            let delimited = match &atom {
                ProseAtom::InlineElement(item) => DelimitedInline::from_inline(item, f),
                _ => None,
            };

            if let Some(delimited) = delimited {
                has_expanded = true;
                self.current_word_group
                    .push(ProseAtom::OpeningDelimiter(delimited.clone()));
                self.delimited_depth += 1;
                self.collect(&delimited.content(), f)?;
                self.delimited_depth -= 1;
                self.current_word_group
                    .push(ProseAtom::ClosingDelimiter(delimited));
            } else {
                self.current_word_group.push(atom);
            }
        }

        // The last group can still hold elements from expanded content, such as the italic in
        // `**_a b_**`, because no separator after them finished their group.
        if has_expanded {
            self.finish_word_group(f)
        } else {
            let group = std::mem::take(&mut self.current_word_group);
            self.push_word_group(group);
            Ok(())
        }
    }

    fn push_word_group(&mut self, mut group: WordGroup) {
        group.is_delimited_content = self.delimited_depth > 0;
        self.items.push(ProseItem::WordGroup(group));
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
            WordGroupEscape::None if self.is_delimited_content => WordGroupLayout::Default,
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
            atom.fmt(f)?;
        }
        Ok(())
    }

    fn fmt_each_marker(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        for atom in &self.atoms {
            match atom {
                ProseAtom::Word(word) => write!(f, [token("\\"), word])?,
                atom => atom.fmt(f)?,
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
            _ => None,
        }))
    }

    fn matches_end_of(self, group: &WordGroup) -> bool {
        self.matches_boundary(group.atoms.iter().rev().map(|atom| match atom {
            ProseAtom::Word(word) => Some(word.text.text().bytes().rev()),
            _ => None,
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

        if group.is_delimited_content {
            return Self::None;
        }

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

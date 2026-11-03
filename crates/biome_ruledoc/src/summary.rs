//! Lowers the first rendered paragraph into summary fragments.
//!
//! Configuration schemas and website indexes need a short semantic summary,
//! not the paragraph's original Markdown delimiters. The lowering resolves text
//! escapes and entities, removes emphasis containers, keeps inline code distinct
//! for output-specific rendering, and rejects constructs whose meaning cannot be
//! represented by the summary HIR.

use crate::{RuleSummary, RuleSummaryFragment};
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdInline, AnyMdLeafBlock, MdInlineItemList, MdParagraph, MdRoot,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use std::fmt::{Display, Formatter};

/// A failure to lower the first content block into a rule summary.
///
/// This error reports either a non-paragraph first block, malformed recovered
/// syntax, or an inline Markdown construct that generated summaries do not
/// support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleSummaryError {
    range: TextRange,
    message: String,
}

impl RuleSummaryError {
    fn new(range: TextRange, message: impl Into<String>) -> Self {
        Self {
            range,
            message: message.into(),
        }
    }

    /// Returns the range that prevented summary lowering in the original
    /// Markdown source.
    pub fn range(&self) -> TextRange {
        self.range
    }
}

impl Display for RuleSummaryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RuleSummaryError {}

/// Lowers the first block that contributes rendered content.
///
/// Blank lines and link-reference definitions are transparent because they do
/// not emit paragraph content. The first remaining block must be a paragraph;
/// silently searching past another block would let headings, lists, or HTML
/// change which text becomes a rule's public summary.
pub(crate) fn lower_summary(root: &MdRoot) -> Result<Option<RuleSummary>, RuleSummaryError> {
    let Some(block) = root
        .value()
        .iter()
        .find(|block| !is_transparent_before_summary(block))
    else {
        return Ok(None);
    };

    let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) = block else {
        return Err(RuleSummaryError::new(
            block.syntax().text_trimmed_range(),
            format!(
                "rule documentation must start with a paragraph, but starts with {:?}",
                block.syntax().kind()
            ),
        ));
    };

    let mut fragments = Vec::new();
    lower_inline_list(&paragraph, &paragraph.list(), &mut fragments)?;
    trim_structural_indent(&mut fragments);
    Ok(Some(RuleSummary::new(fragments)))
}

/// Returns whether a leading block contributes no rendered summary content.
///
/// Link-reference definitions affect links elsewhere in the document but do not
/// render at their definition site, so their labels and destinations must not
/// appear in the generated summary.
fn is_transparent_before_summary(block: &AnyMdBlock) -> bool {
    matches!(
        block,
        AnyMdBlock::AnyMdLeafBlock(
            AnyMdLeafBlock::MdNewline(_) | AnyMdLeafBlock::MdLinkReferenceDefinition(_)
        )
    )
}

/// Appends the summary representation of an inline list.
///
/// Emphasis containers recurse into their content without emitting delimiters.
/// Textual newlines become soft breaks, while indentation and quote-prefix nodes
/// are structural. Links, HTML, images, hard breaks, and other unsupported
/// inline constructs return an error instead of producing ambiguous output.
/// `paragraph` identifies the outermost list so its final source newline can be
/// removed without deleting a soft break nested in another inline container.
fn lower_inline_list(
    paragraph: &MdParagraph,
    list: &MdInlineItemList,
    fragments: &mut Vec<RuleSummaryFragment>,
) -> Result<(), RuleSummaryError> {
    for item in list.iter() {
        match item {
            AnyMdInline::MdTextual(text) => {
                if text.is_newline().unwrap_or(false) {
                    fragments.push(RuleSummaryFragment::SoftBreak);
                } else {
                    let value = text.unescaped_text().map_err(|_| {
                        RuleSummaryError::new(
                            text.syntax().text_trimmed_range(),
                            "rule summary contains malformed text",
                        )
                    })?;
                    if !value.is_empty() {
                        fragments.push(RuleSummaryFragment::Text(value));
                    }
                }
            }
            AnyMdInline::MdEntityReference(entity) => {
                let text = entity.decoded_text().map_err(|_| {
                    RuleSummaryError::new(
                        entity.syntax().text_trimmed_range(),
                        "rule summary contains a malformed entity reference",
                    )
                })?;
                fragments.push(RuleSummaryFragment::Text(text));
            }
            AnyMdInline::MdInlineItalic(emphasis) => {
                lower_inline_list(paragraph, &emphasis.content(), fragments)?;
            }
            AnyMdInline::MdInlineEmphasis(emphasis) => {
                lower_inline_list(paragraph, &emphasis.content(), fragments)?;
            }
            AnyMdInline::MdInlineCode(code) => {
                fragments.push(RuleSummaryFragment::InlineCode(code.normalized_text()));
            }
            AnyMdInline::MdIndentToken(_) | AnyMdInline::MdQuotePrefix(_) => {}
            unsupported => {
                return Err(RuleSummaryError::new(
                    unsupported.syntax().text_trimmed_range(),
                    format!(
                        "unsupported {:?} in the first rule documentation paragraph",
                        unsupported.syntax().kind()
                    ),
                ));
            }
        }
    }

    // A paragraph-ending newline is structural rather than a soft break.
    if paragraph.list().syntax() == list.syntax()
        && matches!(fragments.last(), Some(RuleSummaryFragment::SoftBreak))
    {
        fragments.pop();
    }

    Ok(())
}

/// Removes source indentation before the first visible summary fragment.
///
/// Rust doc attributes normally contribute one leading ASCII space, and
/// CommonMark permits up to three structural spaces before a paragraph. Only
/// ASCII space and tab characters from the first text fragment are removed;
/// whitespace produced by an entity remains semantic content.
fn trim_structural_indent(fragments: &mut Vec<RuleSummaryFragment>) {
    if let Some(RuleSummaryFragment::Text(text)) = fragments.first_mut() {
        let trimmed = text.trim_start_matches([' ', '\t']);
        if trimmed.len() != text.len() {
            *text = trimmed.to_string();
        }
    }
    if matches!(fragments.first(), Some(RuleSummaryFragment::Text(text)) if text.is_empty()) {
        fragments.remove(0);
    }
}

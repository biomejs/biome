use crate::markdown::auxiliary::{
    indent_token::FormatMdIndentTokenOptions, thematic_break_char::FormatMdThematicBreakCharOptions,
};
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdThematicBreakPart, MdDocument, MdParagraph, MdThematicBreakBlock,
};

/// CommonMark thematic breaks require at least three marker characters.
/// Prettier prints normalized thematic breaks with the minimum-width `---` form.
const CANONICAL_THEMATIC_BREAK_MARKER_COUNT: usize = 3;
const NORMALIZED_THEMATIC_BREAK_MARKER: &str = "-";
const UNDERSCORE_THEMATIC_BREAK_MARKER: &str = "_";

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakBlock;
impl FormatNodeRule<MdThematicBreakBlock> for FormatMdThematicBreakBlock {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let mut marker_count = 0;
        let mut has_indent = false;
        let mut has_only_hyphen_markers = true;
        let mut has_only_underscore_markers = true;

        for part in node.parts().iter() {
            match &part {
                AnyMdThematicBreakPart::MdIndentToken(_) => {
                    has_indent = true;
                }
                AnyMdThematicBreakPart::MdThematicBreakChar(part) => {
                    let marker = part.value()?;
                    let marker_text = marker.text_trimmed();

                    has_only_hyphen_markers &= marker_text == NORMALIZED_THEMATIC_BREAK_MARKER;
                    has_only_underscore_markers &= marker_text == UNDERSCORE_THEMATIC_BREAK_MARKER;
                    marker_count += 1;
                }
            }
        }

        let is_top_level_block = node
            .syntax()
            .grand_parent()
            .is_some_and(|parent| MdDocument::can_cast(parent.kind()));
        let is_after_paragraph = node
            .syntax()
            .prev_sibling()
            .is_some_and(|sibling| MdParagraph::can_cast(sibling.kind()));

        let should_normalize_spaced_hyphen_break = is_top_level_block
            && !is_after_paragraph
            && has_indent
            && has_only_hyphen_markers
            && marker_count >= CANONICAL_THEMATIC_BREAK_MARKER_COUNT;
        let should_normalize_long_underscore_break = !has_indent
            && has_only_underscore_markers
            && marker_count > CANONICAL_THEMATIC_BREAK_MARKER_COUNT;

        if !should_normalize_spaced_hyphen_break && !should_normalize_long_underscore_break {
            return node.parts().format().fmt(f);
        }

        let mut emitted_markers = 0;
        for part in node.parts().iter() {
            match part {
                AnyMdThematicBreakPart::MdIndentToken(indent) => write!(
                    f,
                    [indent.format().with_options(FormatMdIndentTokenOptions {
                        replace_tabs_with_spaces: false,
                        should_remove: true,
                    })]
                )?,
                AnyMdThematicBreakPart::MdThematicBreakChar(part) => {
                    let should_remove = emitted_markers >= CANONICAL_THEMATIC_BREAK_MARKER_COUNT;
                    emitted_markers += 1;

                    write!(
                        f,
                        [part
                            .format()
                            .with_options(FormatMdThematicBreakCharOptions {
                                replacement: if should_remove {
                                    None
                                } else {
                                    Some(NORMALIZED_THEMATIC_BREAK_MARKER)
                                },
                                should_remove,
                            })]
                    )?;
                }
            }
        }

        Ok(())
    }
}

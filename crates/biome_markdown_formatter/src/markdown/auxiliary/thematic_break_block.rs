use crate::markdown::auxiliary::{
    indent_token::FormatMdIndentTokenOptions, thematic_break_char::FormatMdThematicBreakCharOptions,
};
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::thematic_break_ext::MdThematicBreakMarker;
use biome_markdown_syntax::{AnyMdThematicBreakPart, MdBlockList, MdBullet, MdThematicBreakBlock};
use biome_rowan::AstNode;

/// CommonMark thematic breaks require at least three marker characters.
/// Prettier prints normalized thematic breaks with the minimum-width `---` form.
const CANONICAL_THEMATIC_BREAK_MARKER_COUNT: usize = 3;

/// Whether a thematic break can be normalized by the formatter.
enum ThematicBreakNormalization {
    /// The parts contain at least three markers of a single thematic break kind.
    /// This can be safely rewritten to the canonical three-marker form.
    Normalize,
    /// The parts don't describe one thematic break marker kind. Preserve the
    /// source shape to avoid changing recovered or otherwise unexpected syntax.
    Preserve,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakBlock;
impl FormatNodeRule<MdThematicBreakBlock> for FormatMdThematicBreakBlock {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        if matches!(
            thematic_break_normalization(node)?,
            ThematicBreakNormalization::Preserve
        ) {
            return node.parts().format().fmt(f);
        }
        let replacement_marker = if is_first_block_in_dash_bullet(node)? {
            MdThematicBreakMarker::Star
        } else {
            MdThematicBreakMarker::Hyphen
        };

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
                                    Some(replacement_marker)
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

fn thematic_break_normalization(
    node: &MdThematicBreakBlock,
) -> FormatResult<ThematicBreakNormalization> {
    let mut marker = None;
    let mut marker_count = 0;

    for part in node.parts().iter() {
        let AnyMdThematicBreakPart::MdThematicBreakChar(part) = part else {
            continue;
        };

        let current_marker = part.marker()?;
        if marker.is_some_and(|marker| marker != current_marker) {
            return Ok(ThematicBreakNormalization::Preserve);
        }

        marker = Some(current_marker);
        marker_count += 1;
    }

    if marker_count < CANONICAL_THEMATIC_BREAK_MARKER_COUNT {
        return Ok(ThematicBreakNormalization::Preserve);
    }

    Ok(ThematicBreakNormalization::Normalize)
}

fn is_first_block_in_dash_bullet(node: &MdThematicBreakBlock) -> FormatResult<bool> {
    let Some(block_list) = node.syntax().parent().and_then(MdBlockList::cast) else {
        return Ok(false);
    };

    let is_first_block = block_list
        .iter()
        .next()
        .is_some_and(|block| block.syntax() == node.syntax());
    if !is_first_block {
        return Ok(false);
    }

    let Some(bullet) = block_list.syntax().parent().and_then(MdBullet::cast) else {
        return Ok(false);
    };

    Ok(bullet.prefix()?.list_marker()?.is_minus())
}

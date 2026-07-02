use crate::markdown::auxiliary::thematic_break_char::FormatMdThematicBreakCharOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdThematicBreakPart, MdThematicBreakBlock, MdThematicBreakChar};

/// Number of marker characters emitted for normalized underscore-only thematic breaks.
const NORMALIZED_THEMATIC_BREAK_MARKER_COUNT: usize = 3;
const NORMALIZED_THEMATIC_BREAK_MARKER: &str = "-";

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakBlock;
impl FormatNodeRule<MdThematicBreakBlock> for FormatMdThematicBreakBlock {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let mut parts: Vec<MdThematicBreakChar> = Vec::new();
        for part in node.parts().iter() {
            let AnyMdThematicBreakPart::MdThematicBreakChar(part) = part else {
                return node.parts().format().fmt(f);
            };

            if part.value()?.text_trimmed() != "_" {
                return node.parts().format().fmt(f);
            }

            parts.push(part);
        }

        if parts.len() <= NORMALIZED_THEMATIC_BREAK_MARKER_COUNT {
            return node.parts().format().fmt(f);
        }

        let (replaced_parts, removed_parts) =
            parts.split_at(NORMALIZED_THEMATIC_BREAK_MARKER_COUNT);

        for part in replaced_parts {
            write!(
                f,
                [part
                    .format()
                    .with_options(FormatMdThematicBreakCharOptions {
                        replacement: Some(NORMALIZED_THEMATIC_BREAK_MARKER),
                        should_remove: false,
                    })]
            )?;
        }

        for part in removed_parts {
            write!(
                f,
                [part
                    .format()
                    .with_options(FormatMdThematicBreakCharOptions {
                        replacement: None,
                        should_remove: true,
                    })]
            )?;
        }

        Ok(())
    }
}

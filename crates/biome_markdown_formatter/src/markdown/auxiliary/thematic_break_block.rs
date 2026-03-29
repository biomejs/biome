use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdThematicBreakPart, MarkdownSyntaxKind, MdThematicBreakBlock};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakBlock;
impl FormatNodeRule<MdThematicBreakBlock> for FormatMdThematicBreakBlock {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        // If directly preceded by a paragraph (no blank line), `---` would be
        // re-parsed as a setext heading underline. Keep the original character.
        let preceded_by_paragraph = node
            .syntax()
            .prev_sibling()
            .is_some_and(|s| {
                matches!(
                    s.kind(),
                    MarkdownSyntaxKind::MD_PARAGRAPH | MarkdownSyntaxKind::MD_SETEXT_HEADER
                )
            });

        if preceded_by_paragraph {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let parts = node.parts();
        let mut emitted_separator = false;

        for part in parts.iter() {
            match part {
                AnyMdThematicBreakPart::MdThematicBreakChar(char_node) => {
                    f.context()
                        .comments()
                        .mark_suppression_checked(char_node.syntax());
                    let value = char_node.value()?;
                    if !emitted_separator {
                        write!(f, [format_replaced(&value, &token("---"))])?;
                        emitted_separator = true;
                    } else {
                        write!(f, [format_removed(&value)])?;
                    }
                }
                AnyMdThematicBreakPart::MdIndentToken(indent) => {
                    f.context()
                        .comments()
                        .mark_suppression_checked(indent.syntax());
                    write!(f, [format_removed(&indent.md_indent_char_token()?)])?;
                }
            }
        }
        Ok(())
    }
}

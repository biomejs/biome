use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use crate::verbatim::format_verbatim_node;
use biome_formatter::write;
use biome_markdown_syntax::{MdHeader, MdHeaderFields};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHeader;
impl FormatNodeRule<MdHeader> for FormatMdHeader {
    fn fmt_fields(&self, node: &MdHeader, f: &mut MarkdownFormatter) -> FormatResult<()> {
        // Headers inside list items can have a non-standard slot layout
        // (e.g. missing MdIndentTokenList in slot 0). Fall back to verbatim.
        if node
            .syntax()
            .slots()
            .next()
            .is_none_or(|s| s.into_node().is_none())
        {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let MdHeaderFields {
            indent,
            before,
            content,
            after,
        } = node.as_fields();

        write!(f, [format_verbatim_node(indent.syntax())])?;

        write!(f, [before.format()])?;

        if let Some(content) = content {
            write!(
                f,
                [
                    space(),
                    content.format().with_options(FormatMdParagraphOptions {
                        trim_mode: TextPrintMode::Trim(TrimMode::Start)
                    })
                ]
            )?;
        }

        for hash in after.iter() {
            // TODO: remove this once we remove the skipped trivia from the hash
            f.context()
                .comments()
                .mark_suppression_checked(hash.syntax());
            write!(f, [format_removed(&hash.hash_token()?)])?;
        }
        Ok(())
    }
}

use crate::markdown::auxiliary::list_marker_prefix::FormatMdListMarkerPrefixOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdLeafBlock, MarkdownSyntaxKind, MdBullet, MdBulletFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBullet;
impl FormatNodeRule<MdBullet> for FormatMdBullet {
    fn fmt_fields(&self, node: &MdBullet, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdBulletFields { prefix, content } = node.as_fields();

        let prefix = prefix?;
        let marker = prefix.marker()?;

        // `* - - -` is a bullet containing a `-` thematic break. Normalizing `*`
        // to `-` produces `- - - -` which CommonMark 4.1 parses as a thematic
        // break, not a list item. Same for `+ - - -`. Skip normalization for marker
        // but still format content through child formatters.
        let target_marker = if marker.kind() == MarkdownSyntaxKind::MINUS
            || first_block_is_dash_thematic_break(&content)
        {
            None
        } else {
            Some("-")
        };

        write!(
            f,
            [prefix
                .format()
                .with_options(FormatMdListMarkerPrefixOptions { target_marker })]
        )?;
        content.format().fmt(f)?;
        Ok(())
    }
}

/// Returns true if the first block in `content` is a thematic break using `-`.
fn first_block_is_dash_thematic_break(content: &biome_markdown_syntax::MdBlockList) -> bool {
    let Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdThematicBreakBlock(block))) =
        content.iter().next()
    else {
        return false;
    };
    block
        .parts()
        .into_iter()
        .find_map(|p| p.as_md_thematic_break_char().cloned())
        .and_then(|c| c.value().ok())
        .is_some_and(|t| t.text_trimmed() == "-")
}

use crate::markdown::lists::block_list::{FormatMdBlockListOptions, QuoteBoundaryTrim};
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdBlock, AnyMdInline, AnyMdLeafBlock, MdRoot, MdRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdRoot;
impl FormatNodeRule<MdRoot> for FormatMdRoot {
    fn fmt_fields(&self, node: &MdRoot, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdRootFields {
            bom_token,
            frontmatter,
            value,
            eof_token,
        } = node.as_fields();

        if let Some(bom) = bom_token {
            write!(f, [bom.format()])?;
        }
        if let Some(frontmatter) = frontmatter {
            write!(f, [frontmatter.format()])?;
            if value.iter().any(|block| {
                !matches!(
                    block,
                    AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))
                )
            }) {
                write!(f, [empty_line()])?;
            }
        }
        let already_ends_with_newline = content_ends_with_newline(&value);

        write!(
            f,
            [
                value.format().with_options(FormatMdBlockListOptions {
                    quote_boundary_trim: QuoteBoundaryTrim::None,
                }),
                format_removed(&eof_token?)
            ]
        )?;

        if !already_ends_with_newline {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}

fn content_ends_with_newline(value: &biome_markdown_syntax::MdBlockList) -> bool {
    let mut iter = value.iter();
    // Walk backwards past trailing MdNewline blocks to find the last content block.
    let last_content = loop {
        match iter.next_back() {
            Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))) => {}
            other => break other,
        }
    };
    matches!(
        last_content,
        Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(ref p)))
            if p.list().iter().last().is_some_and(|item| matches!(
                item,
                AnyMdInline::MdTextual(ref t) if t.is_newline().unwrap_or(false)
            ))
    )
}

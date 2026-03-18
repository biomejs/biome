//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdBulletListMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdBulletListMember;
impl FormatRule<AnyMdBulletListMember> for FormatAnyMdBulletListMember {
    type Context = MdFormatContext;
    fn fmt(&self, node: &AnyMdBulletListMember, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdBulletListMember::MdBullet(node) => node.format().fmt(f),
            AnyMdBulletListMember::MdNewline(node) => node.format().fmt(f),
        }
    }
}

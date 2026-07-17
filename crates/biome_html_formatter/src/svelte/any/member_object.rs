//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteMemberObject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteMemberObject;
impl FormatRule<AnySvelteMemberObject> for FormatAnySvelteMemberObject {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteMemberObject, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteMemberObject::SvelteMemberProperty(node) => node.format().fmt(f),
            AnySvelteMemberObject::SvelteName(node) => node.format().fmt(f),
        }
    }
}

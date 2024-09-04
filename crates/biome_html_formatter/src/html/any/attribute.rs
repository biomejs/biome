//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlAttribute;
impl FormatRule<AnyHtmlAttribute> for FormatAnyHtmlAttribute {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlAttribute::HtmlAttribute(node) => node.format().fmt(f),
            AnyHtmlAttribute::HtmlBogusAttribute(node) => node.format().fmt(f),
        }
    }
}

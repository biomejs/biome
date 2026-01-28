//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlTagName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlTagName;
impl FormatRule<AnyHtmlTagName> for FormatAnyHtmlTagName {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlTagName, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlTagName::HtmlComponentName(node) => node.format().fmt(f),
            AnyHtmlTagName::HtmlMemberName(node) => node.format().fmt(f),
            AnyHtmlTagName::HtmlTagName(node) => node.format().fmt(f),
        }
    }
}

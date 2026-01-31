//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlComponentObjectName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlComponentObjectName;
impl FormatRule<AnyHtmlComponentObjectName> for FormatAnyHtmlComponentObjectName {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlComponentObjectName, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlComponentObjectName::HtmlComponentName(node) => node.format().fmt(f),
            AnyHtmlComponentObjectName::HtmlMemberName(node) => node.format().fmt(f),
            AnyHtmlComponentObjectName::HtmlTagName(node) => node.format().fmt(f),
        }
    }
}

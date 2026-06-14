//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlRootDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlRootDirective;
impl FormatRule<AnyHtmlRootDirective> for FormatAnyHtmlRootDirective {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlRootDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlRootDirective::HtmlDirective(node) => node.format().fmt(f),
            AnyHtmlRootDirective::HtmlProcessingInstructionDirective(node) => node.format().fmt(f),
        }
    }
}

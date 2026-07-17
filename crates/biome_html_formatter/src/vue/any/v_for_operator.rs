//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyVueVForOperator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyVueVForOperator;
impl FormatRule<AnyVueVForOperator> for FormatAnyVueVForOperator {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyVueVForOperator, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyVueVForOperator::VueVForInOperator(node) => node.format().fmt(f),
            AnyVueVForOperator::VueVForOfOperator(node) => node.format().fmt(f),
        }
    }
}

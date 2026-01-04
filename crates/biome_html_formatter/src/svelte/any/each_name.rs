//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteEachName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteEachName;
impl FormatRule<AnySvelteEachName> for FormatAnySvelteEachName {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteEachName, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteEachName::AnySvelteDestructuredName(node) => node.format().fmt(f),
            AnySvelteEachName::HtmlTextExpression(node) => node.format().fmt(f),
            AnySvelteEachName::SvelteName(node) => node.format().fmt(f),
        }
    }
}

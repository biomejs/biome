//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::GritCodeSnippetSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCodeSnippetSource;
impl FormatRule<GritCodeSnippetSource> for FormatGritCodeSnippetSource {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritCodeSnippetSource, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            GritCodeSnippetSource::GritBacktickSnippetLiteral(node) => node.format().fmt(f),
            GritCodeSnippetSource::GritLanguageSpecificSnippet(node) => node.format().fmt(f),
            GritCodeSnippetSource::GritRawBacktickSnippetLiteral(node) => node.format().fmt(f),
        }
    }
}

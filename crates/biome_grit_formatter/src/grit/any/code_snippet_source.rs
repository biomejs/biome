//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritCodeSnippetSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritCodeSnippetSource;
impl FormatRule<AnyGritCodeSnippetSource> for FormatAnyGritCodeSnippetSource {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritCodeSnippetSource, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritCodeSnippetSource::GritBacktickSnippetLiteral(node) => node.format().fmt(f),
            AnyGritCodeSnippetSource::GritLanguageSpecificSnippet(node) => node.format().fmt(f),
            AnyGritCodeSnippetSource::GritRawBacktickSnippetLiteral(node) => node.format().fmt(f),
        }
    }
}

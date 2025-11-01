//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwSource;
impl FormatRule<AnyTwSource> for FormatAnyTwSource {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwSource, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwSource::CssString(node) => node.format().fmt(f),
            AnyTwSource::TwSourceInline(node) => node.format().fmt(f),
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritLanguageName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritLanguageName;
impl FormatRule<AnyGritLanguageName> for FormatAnyGritLanguageName {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritLanguageName, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritLanguageName::GritBogusLanguageName(node) => node.format().fmt(f),
            AnyGritLanguageName::GritLanguageName(node) => node.format().fmt(f),
        }
    }
}

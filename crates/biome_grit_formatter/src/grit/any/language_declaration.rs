//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritLanguageDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritLanguageDeclaration;
impl FormatRule<AnyGritLanguageDeclaration> for FormatAnyGritLanguageDeclaration {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritLanguageDeclaration, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritLanguageDeclaration::GritBogusLanguageDeclaration(node) => node.format().fmt(f),
            AnyGritLanguageDeclaration::GritLanguageDeclaration(node) => node.format().fmt(f),
        }
    }
}

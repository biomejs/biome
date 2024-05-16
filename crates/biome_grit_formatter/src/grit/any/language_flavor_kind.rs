//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritLanguageFlavorKind;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritLanguageFlavorKind;
impl FormatRule<AnyGritLanguageFlavorKind> for FormatAnyGritLanguageFlavorKind {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritLanguageFlavorKind, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritLanguageFlavorKind::GritBogusLanguageFlavorKind(node) => node.format().fmt(f),
            AnyGritLanguageFlavorKind::GritLanguageFlavorKind(node) => node.format().fmt(f),
        }
    }
}

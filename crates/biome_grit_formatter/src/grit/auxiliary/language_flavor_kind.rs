use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritLanguageFlavorKind, GritLanguageFlavorKindFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageFlavorKind;
impl FormatNodeRule<GritLanguageFlavorKind> for FormatGritLanguageFlavorKind {
    fn fmt_fields(&self, node: &GritLanguageFlavorKind, f: &mut GritFormatter) -> FormatResult<()> {
        let GritLanguageFlavorKindFields { flavor_kind } = node.as_fields();

        write!(f, [flavor_kind.format()])
    }
}

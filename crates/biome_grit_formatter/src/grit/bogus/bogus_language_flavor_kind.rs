use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusLanguageFlavorKind;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLanguageFlavorKind;
impl FormatBogusNodeRule<GritBogusLanguageFlavorKind> for FormatGritBogusLanguageFlavorKind {}

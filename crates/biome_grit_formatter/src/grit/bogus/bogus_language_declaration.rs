use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusLanguageDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLanguageDeclaration;
impl FormatBogusNodeRule<GritBogusLanguageDeclaration> for FormatGritBogusLanguageDeclaration {}

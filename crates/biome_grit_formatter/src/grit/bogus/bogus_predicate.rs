use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusPredicate;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusPredicate;
impl FormatBogusNodeRule<GritBogusPredicate> for FormatGritBogusPredicate {}

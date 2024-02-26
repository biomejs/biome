use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusNamedArg;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusNamedArg;
impl FormatBogusNodeRule<GritBogusNamedArg> for FormatGritBogusNamedArg {}

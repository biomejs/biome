use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::GritDotdotdot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDotdotdot;
impl FormatNodeRule<GritDotdotdot> for FormatGritDotdotdot {
    fn fmt_fields(&self, node: &GritDotdotdot, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.dotdotdot_token().format()])
    }
}

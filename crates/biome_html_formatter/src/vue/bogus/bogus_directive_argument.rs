use crate::FormatBogusNodeRule;
use biome_html_syntax::VueBogusDirectiveArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueBogusDirectiveArgument;
impl FormatBogusNodeRule<VueBogusDirectiveArgument> for FormatVueBogusDirectiveArgument {}

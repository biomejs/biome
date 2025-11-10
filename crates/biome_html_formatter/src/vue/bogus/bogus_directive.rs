use crate::FormatBogusNodeRule;
use biome_html_syntax::VueBogusDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueBogusDirective;
impl FormatBogusNodeRule<VueBogusDirective> for FormatVueBogusDirective {}

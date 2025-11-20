use crate::FormatBogusNodeRule;
use biome_html_syntax::GlimmerBogusExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBogusExpression;
impl FormatBogusNodeRule<GlimmerBogusExpression> for FormatGlimmerBogusExpression {}

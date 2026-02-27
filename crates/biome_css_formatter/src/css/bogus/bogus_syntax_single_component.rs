use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusSyntaxSingleComponent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSyntaxSingleComponent;
impl FormatBogusNodeRule<CssBogusSyntaxSingleComponent> for FormatCssBogusSyntaxSingleComponent {}

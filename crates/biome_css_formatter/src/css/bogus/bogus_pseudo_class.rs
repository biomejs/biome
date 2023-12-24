use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusPseudoClass;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPseudoClass;
impl FormatBogusNodeRule<CssBogusPseudoClass> for FormatCssBogusPseudoClass {}

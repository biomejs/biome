use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusFunctionParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusFunctionParameter;
impl FormatBogusNodeRule<CssBogusFunctionParameter> for FormatCssBogusFunctionParameter {}

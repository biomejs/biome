use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusDashFunctionArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusDashFunctionArgument;
impl FormatBogusNodeRule<CssBogusDashFunctionArgument> for FormatCssBogusDashFunctionArgument {}

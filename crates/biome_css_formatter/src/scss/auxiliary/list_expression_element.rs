use crate::prelude::*;
use biome_css_syntax::{ScssListExpressionElement, ScssListExpressionElementFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElement;
impl FormatNodeRule<ScssListExpressionElement> for FormatScssListExpressionElement {
    fn fmt_fields(
        &self,
        node: &ScssListExpressionElement,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssListExpressionElementFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}

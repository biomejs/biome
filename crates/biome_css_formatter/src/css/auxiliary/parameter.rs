use crate::prelude::*;
use biome_css_syntax::{CssParameter, CssParameterFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameter;
impl FormatNodeRule<CssParameter> for FormatCssParameter {
    fn fmt_fields(&self, node: &CssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        let CssParameterFields {
            css_component_value_list,
        } = node.as_fields();

        write!(f, [css_component_value_list.format()])
    }
}

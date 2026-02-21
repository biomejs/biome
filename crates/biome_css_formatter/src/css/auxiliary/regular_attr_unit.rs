use crate::{prelude::*, utils::string_utils::FormatDimensionUnit};
use biome_css_syntax::{CssRegularAttrUnit, CssRegularAttrUnitFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularAttrUnit;

impl FormatNodeRule<CssRegularAttrUnit> for FormatCssRegularAttrUnit {
    fn fmt_fields(&self, node: &CssRegularAttrUnit, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularAttrUnitFields { unit_token } = node.as_fields();
        write!(f, [FormatDimensionUnit::from(unit_token?)])
    }
}

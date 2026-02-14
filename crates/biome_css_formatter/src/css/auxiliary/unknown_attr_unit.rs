use crate::{prelude::*, utils::string_utils::FormatDimensionUnit};
use biome_css_syntax::{CssUnknownAttrUnit, CssUnknownAttrUnitFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownAttrUnit;

impl FormatNodeRule<CssUnknownAttrUnit> for FormatCssUnknownAttrUnit {
    fn fmt_fields(&self, node: &CssUnknownAttrUnit, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownAttrUnitFields { unit_token } = node.as_fields();
        write!(f, [FormatDimensionUnit::from(unit_token?)])
    }
}

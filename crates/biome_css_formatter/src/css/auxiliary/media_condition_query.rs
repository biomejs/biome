use crate::prelude::*;
use biome_css_syntax::{CssMediaConditionQuery, CssMediaConditionQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaConditionQuery;
impl FormatNodeRule<CssMediaConditionQuery> for FormatCssMediaConditionQuery {
    fn fmt_fields(&self, node: &CssMediaConditionQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaConditionQueryFields { condition } = node.as_fields();

        write!(f, [condition.format()])
    }
}

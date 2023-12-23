use crate::prelude::*;
use biome_css_syntax::{CssMediaNotCondition, CssMediaNotConditionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaNotCondition;
impl FormatNodeRule<CssMediaNotCondition> for FormatCssMediaNotCondition {
    fn fmt_fields(&self, node: &CssMediaNotCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaNotConditionFields {
            not_token,
            condition,
        } = node.as_fields();

        write!(f, [not_token.format(), space(), condition.format()])
    }
}

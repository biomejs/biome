use crate::prelude::*;
use biome_css_syntax::{
    ScssSupportsInterpolatedCondition, ScssSupportsInterpolatedConditionFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssSupportsInterpolatedCondition;

impl FormatNodeRule<ScssSupportsInterpolatedCondition> for FormatScssSupportsInterpolatedCondition {
    fn fmt_fields(
        &self,
        node: &ScssSupportsInterpolatedCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssSupportsInterpolatedConditionFields { condition } = node.as_fields();

        write!(f, [condition.format()])
    }
}

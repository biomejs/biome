use crate::prelude::*;
use biome_css_syntax::{ScssPartialCombinatorSelector, ScssPartialCombinatorSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssPartialCombinatorSelector;

impl FormatNodeRule<ScssPartialCombinatorSelector> for FormatScssPartialCombinatorSelector {
    fn fmt_fields(
        &self,
        node: &ScssPartialCombinatorSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssPartialCombinatorSelectorFields { left, combinator } = node.as_fields();

        if let Some(left) = left {
            write!(f, [left.format(), soft_line_break_or_space()])?;
        }

        write!(f, [combinator?.format()])
    }
}

use crate::prelude::*;
use biome_css_syntax::{ScssContainerInterpolatedQuery, ScssContainerInterpolatedQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssContainerInterpolatedQuery;

impl FormatNodeRule<ScssContainerInterpolatedQuery> for FormatScssContainerInterpolatedQuery {
    fn fmt_fields(
        &self,
        node: &ScssContainerInterpolatedQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssContainerInterpolatedQueryFields { query } = node.as_fields();

        write!(f, [query.format()])
    }
}

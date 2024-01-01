use crate::prelude::*;
use biome_css_syntax::{CssQueryFeatureBoolean, CssQueryFeatureBooleanFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureBoolean;
impl FormatNodeRule<CssQueryFeatureBoolean> for FormatCssQueryFeatureBoolean {
    fn fmt_fields(&self, node: &CssQueryFeatureBoolean, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeatureBooleanFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}

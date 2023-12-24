use crate::prelude::*;
use biome_css_syntax::{CssAttributeMatcher, CssAttributeMatcherFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeMatcher;
impl FormatNodeRule<CssAttributeMatcher> for FormatCssAttributeMatcher {
    fn fmt_fields(&self, node: &CssAttributeMatcher, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttributeMatcherFields {
            operator,
            value,
            modifier,
        } = node.as_fields();

        write!(f, [operator.format(), value.format()])?;

        if modifier.is_some() {
            write!(f, [space(), modifier.format()])?;
        }

        Ok(())
    }
}

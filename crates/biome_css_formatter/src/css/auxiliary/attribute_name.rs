use crate::prelude::*;
use biome_css_syntax::{CssAttributeName, CssAttributeNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeName;
impl FormatNodeRule<CssAttributeName> for FormatCssAttributeName {
    fn fmt_fields(&self, node: &CssAttributeName, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttributeNameFields { namespace, name } = node.as_fields();

        write!(f, [namespace.format(), name.format()])
    }
}

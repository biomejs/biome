use crate::prelude::*;
use biome_css_syntax::{CssFontFamilyName, CssFontFamilyNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFamilyName;
impl FormatNodeRule<CssFontFamilyName> for FormatCssFontFamilyName {
    fn fmt_fields(&self, node: &CssFontFamilyName, f: &mut CssFormatter) -> FormatResult<()> {
        let CssFontFamilyNameFields { names } = node.as_fields();

        write!(f, [names.format()])
    }
}

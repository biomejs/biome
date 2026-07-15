use crate::prelude::*;
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdentifier;

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();
        let case = f.context().identifier_case();

        #[cfg(debug_assertions)]
        if case == CssCase::Auto {
            crate::utils::case::record_auto_identifier(node, f);
        }

        write!(f, [value_token.format()?.with_text_case(case)])
    }
}

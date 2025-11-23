use biome_css_syntax::{CssWildcard, CssWildcardFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssWildcard;

impl FormatNodeRule<CssWildcard> for FormatCssWildcard {
    fn fmt_fields(&self, node: &CssWildcard, f: &mut CssFormatter) -> FormatResult<()> {
        let CssWildcardFields { star_token } = node.as_fields();
        write!(f, [star_token.format()])
    }
}

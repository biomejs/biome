use crate::prelude::*;
use biome_css_syntax::{ScssUseAllNamespace, ScssUseAllNamespaceFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssUseAllNamespace;

impl FormatNodeRule<ScssUseAllNamespace> for FormatScssUseAllNamespace {
    fn fmt_fields(&self, node: &ScssUseAllNamespace, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssUseAllNamespaceFields { star_token } = node.as_fields();

        write!(f, [star_token.format()])
    }
}

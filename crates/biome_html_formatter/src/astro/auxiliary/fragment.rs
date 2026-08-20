use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AstroFragment, AstroFragmentFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroFragment;
impl FormatNodeRule<AstroFragment> for FormatAstroFragment {
    fn fmt_fields(&self, node: &AstroFragment, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AstroFragmentFields {
            opening_fragment,
            children,
            closing_fragment,
        } = node.as_fields();

        write!(
            f,
            [
                opening_fragment.format(),
                children.format(),
                closing_fragment.format()
            ]
        )
    }
}

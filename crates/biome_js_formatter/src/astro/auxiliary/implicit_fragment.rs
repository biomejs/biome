use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{AstroImplicitFragment, AstroImplicitFragmentFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroImplicitFragment;
impl FormatNodeRule<AstroImplicitFragment> for FormatAstroImplicitFragment {
    fn fmt_fields(&self, node: &AstroImplicitFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let AstroImplicitFragmentFields { children } = node.as_fields();

        write!(f, [children.format()])
    }
}

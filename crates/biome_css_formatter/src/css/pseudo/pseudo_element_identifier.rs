use crate::prelude::*;
use crate::utils::case::pseudo_name_case;
use biome_css_syntax::{CssPseudoElementIdentifier, CssPseudoElementIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementIdentifier;
impl FormatNodeRule<CssPseudoElementIdentifier> for FormatCssPseudoElementIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementIdentifierFields { name } = node.as_fields();
        let name = name?;
        let case = pseudo_name_case(&name);

        write!(f, [name.format().with_text_case(case)])
    }
}

use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AngularTemplateRefVariable, AngularTemplateRefVariableFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularTemplateRefVariable;
impl FormatNodeRule<AngularTemplateRefVariable> for FormatAngularTemplateRefVariable {
    fn fmt_fields(
        &self,
        node: &AngularTemplateRefVariable,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let AngularTemplateRefVariableFields { hash_token, name } = node.as_fields();
        write!(f, [hash_token.format(), name.format()])
    }
}

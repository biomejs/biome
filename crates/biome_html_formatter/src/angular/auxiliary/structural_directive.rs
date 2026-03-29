use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AngularStructuralDirective, AngularStructuralDirectiveFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularStructuralDirective;
impl FormatNodeRule<AngularStructuralDirective> for FormatAngularStructuralDirective {
    fn fmt_fields(
        &self,
        node: &AngularStructuralDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let AngularStructuralDirectiveFields {
            star_token,
            name,
            initializer,
        } = node.as_fields();
        write!(
            f,
            [star_token.format(), name.format(), initializer.format()]
        )
    }
}

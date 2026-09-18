use crate::prelude::*;
use biome_css_syntax::{
    CssValueAtRuleNamedImportSpecifier, CssValueAtRuleNamedImportSpecifierFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleNamedImportSpecifier;
impl FormatNodeRule<CssValueAtRuleNamedImportSpecifier>
    for FormatCssValueAtRuleNamedImportSpecifier
{
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleNamedImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleNamedImportSpecifierFields {
            name,
            as_token,
            local_name,
        } = node.as_fields();

        write!(
            f,
            [
                name?.format().with_text_case(CssCase::Preserve),
                space(),
                as_token.format()?.with_text_case(CssCase::Preserve),
                space(),
                local_name?.format().with_text_case(CssCase::Preserve)
            ]
        )
    }
}

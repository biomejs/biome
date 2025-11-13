use crate::prelude::*;
use biome_css_syntax::{CssScopeAtRuleDeclarator, CssScopeAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeAtRuleDeclarator;

impl FormatNodeRule<CssScopeAtRuleDeclarator> for FormatCssScopeAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssScopeAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssScopeAtRuleDeclaratorFields { scope_token, range } = node.as_fields();

        write!(f, [scope_token.format()])?;

        if range.is_some() {
            write!(f, [space(), range.format()])?;
        }

        Ok(())
    }
}

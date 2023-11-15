use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use biome_js_syntax::JsExportFromClause;
use biome_js_syntax::JsExportFromClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportFromClause;

impl FormatNodeRule<JsExportFromClause> for FormatJsExportFromClause {
    fn fmt_fields(&self, node: &JsExportFromClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportFromClauseFields {
            type_token,
            star_token,
            export_as,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write!(f, [star_token.format(), space(),])?;

        if let Some(export_as) = export_as {
            write!(f, [export_as.format(), space()])?;
        }

        write!(f, [from_token.format(), space(), source.format()])?;

        if let Some(assertion) = assertion {
            write!(f, [assertion.format()])?;
        }

        FormatStatementSemicolon::new(semicolon_token.as_ref()).fmt(f)
    }
}

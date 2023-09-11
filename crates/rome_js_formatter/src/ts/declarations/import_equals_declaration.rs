use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;

use biome_formatter::write;
use biome_js_syntax::TsImportEqualsDeclaration;
use biome_js_syntax::TsImportEqualsDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportEqualsDeclaration;

impl FormatNodeRule<TsImportEqualsDeclaration> for FormatTsImportEqualsDeclaration {
    fn fmt_fields(
        &self,
        node: &TsImportEqualsDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsImportEqualsDeclarationFields {
            import_token,
            type_token,
            id,
            eq_token,
            module_reference,
            semicolon_token,
        } = node.as_fields();

        write!(f, [import_token.format(), space(),])?;

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write!(
            f,
            [
                id.format(),
                space(),
                eq_token.format(),
                space(),
                module_reference.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}

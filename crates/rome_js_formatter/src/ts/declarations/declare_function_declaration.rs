use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;

use crate::js::declarations::function_declaration::FormatFunction;
use biome_formatter::write;
use biome_js_syntax::TsDeclareFunctionDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareFunctionDeclaration;

impl FormatNodeRule<TsDeclareFunctionDeclaration> for FormatTsDeclareFunctionDeclaration {
    fn fmt_fields(
        &self,
        node: &TsDeclareFunctionDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [
                FormatFunction::from(node.clone()),
                FormatStatementSemicolon::new(node.semicolon_token().as_ref())
            ]
        )
    }
}

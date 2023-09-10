use crate::js::declarations::function_declaration::FormatFunction;
use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;
use biome_js_syntax::TsDeclareFunctionExportDefaultDeclaration;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareFunctionExportDefaultDeclaration;
impl FormatNodeRule<TsDeclareFunctionExportDefaultDeclaration>
    for FormatTsDeclareFunctionExportDefaultDeclaration
{
    fn fmt_fields(
        &self,
        node: &TsDeclareFunctionExportDefaultDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![
            f,
            [
                FormatFunction::from(node.clone()),
                FormatStatementSemicolon::new(node.semicolon_token().as_ref())
            ]
        ]
    }
}

use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::TsGlobalDeclaration;
use biome_js_syntax::TsGlobalDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsGlobalDeclaration;

impl FormatNodeRule<TsGlobalDeclaration> for FormatTsGlobalDeclaration {
    fn fmt_fields(&self, node: &TsGlobalDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsGlobalDeclarationFields { global_token, body } = node.as_fields();

        write![f, [global_token.format(), space(), body.format()]]
    }
}

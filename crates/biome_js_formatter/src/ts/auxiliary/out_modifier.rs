use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{TsOutModifier, TsOutModifierFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsOutModifier;

impl FormatNodeRule<TsOutModifier> for FormatTsOutModifier {
    fn fmt_fields(&self, node: &TsOutModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsOutModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}

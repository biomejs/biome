use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsDeclareModifier;
use biome_js_syntax::TsDeclareModifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareModifier;

impl FormatNodeRule<TsDeclareModifier> for FormatTsDeclareModifier {
    fn fmt_fields(&self, node: &TsDeclareModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDeclareModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}

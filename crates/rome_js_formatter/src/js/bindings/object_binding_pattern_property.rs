use crate::prelude::*;

use biome_js_syntax::JsObjectBindingPatternProperty;
use biome_js_syntax::JsObjectBindingPatternPropertyFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternProperty;

impl FormatNodeRule<JsObjectBindingPatternProperty> for FormatJsObjectBindingPatternProperty {
    fn fmt_fields(
        &self,
        node: &JsObjectBindingPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        write![
            f,
            [
                member.format(),
                colon_token.format(),
                space(),
                pattern.format(),
            ]
        ]?;

        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }

        Ok(())
    }
}

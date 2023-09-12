use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsObjectBindingPatternShorthandProperty;
use biome_js_syntax::JsObjectBindingPatternShorthandPropertyFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternShorthandProperty;

impl FormatNodeRule<JsObjectBindingPatternShorthandProperty>
    for FormatJsObjectBindingPatternShorthandProperty
{
    fn fmt_fields(
        &self,
        node: &JsObjectBindingPatternShorthandProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternShorthandPropertyFields { identifier, init } = node.as_fields();

        write![f, [identifier.format()]]?;

        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }

        Ok(())
    }
}

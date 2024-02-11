use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsArrayAssignmentPatternElement;
use biome_js_syntax::JsArrayAssignmentPatternElementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayAssignmentPatternElement;

impl FormatNodeRule<JsArrayAssignmentPatternElement> for FormatJsArrayAssignmentPatternElement {
    fn fmt_fields(
        &self,
        node: &JsArrayAssignmentPatternElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayAssignmentPatternElementFields { pattern, init } = node.as_fields();

        write!(f, [pattern.format()?,])?;
        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }

        Ok(())
    }
}

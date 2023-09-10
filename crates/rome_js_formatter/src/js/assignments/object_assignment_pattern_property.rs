use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;
use biome_formatter::write;
use biome_js_syntax::JsObjectAssignmentPatternProperty;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectAssignmentPatternProperty;

impl FormatNodeRule<JsObjectAssignmentPatternProperty> for FormatJsObjectAssignmentPatternProperty {
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(f, [AnyJsAssignmentLike::from(node.clone())])
    }
}

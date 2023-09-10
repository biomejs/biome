use crate::prelude::*;

use crate::js::classes::method_class_member::FormatAnyJsMethodMember;
use biome_js_syntax::JsMethodObjectMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMethodObjectMember;

impl FormatNodeRule<JsMethodObjectMember> for FormatJsMethodObjectMember {
    fn fmt_fields(&self, node: &JsMethodObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        FormatAnyJsMethodMember::from(node.clone()).fmt(f)
    }
}

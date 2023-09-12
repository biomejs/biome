use crate::prelude::*;

use crate::js::classes::method_class_member::FormatAnyJsMethodMember;
use biome_formatter::write;
use biome_js_syntax::JsConstructorClassMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorClassMember;

impl FormatNodeRule<JsConstructorClassMember> for FormatJsConstructorClassMember {
    fn fmt_fields(&self, node: &JsConstructorClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![
            f,
            [
                node.modifiers().format(),
                space(),
                FormatAnyJsMethodMember::from(node.clone())
            ]
        ]
    }
}

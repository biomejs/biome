use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use crate::js::classes::method_class_member::FormatAnyJsMethodMember;
use biome_formatter::write;
use biome_js_syntax::TsMethodSignatureTypeMember;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureTypeMember;

impl FormatNodeRule<TsMethodSignatureTypeMember> for FormatTsMethodSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsMethodSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![
            f,
            [
                FormatAnyJsMethodMember::from(node.clone()),
                FormatTypeMemberSeparator::new(node.separator_token().as_ref())
            ]
        ]
    }
}

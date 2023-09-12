use crate::js::classes::property_class_member::{
    AnyJsPropertyClassMember, FormatClassPropertySemicolon,
};
use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;
use biome_formatter::write;
use biome_js_syntax::TsInitializedPropertySignatureClassMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsInitializedPropertySignatureClassMember;
impl FormatNodeRule<TsInitializedPropertySignatureClassMember>
    for FormatTsInitializedPropertySignatureClassMember
{
    fn fmt_fields(
        &self,
        node: &TsInitializedPropertySignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let semicolon_token = node.semicolon_token();

        write!(
            f,
            [
                AnyJsAssignmentLike::from(node.clone()),
                FormatClassPropertySemicolon::new(
                    &AnyJsPropertyClassMember::from(node.clone()),
                    semicolon_token.as_ref()
                )
            ]
        )
    }
}

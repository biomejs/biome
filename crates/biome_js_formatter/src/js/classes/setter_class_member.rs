use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsSetterClassMember;
use biome_js_syntax::JsSetterClassMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSetterClassMember;

impl FormatNodeRule<JsSetterClassMember> for FormatJsSetterClassMember {
    fn fmt_fields(&self, node: &JsSetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSetterClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            comma_token,
            r_paren_token,
            body,
        } = node.as_fields();

        let l_paren = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [l_paren_token.format(), space()])
            } else {
                write!(f, [l_paren_token.format()])
            }
        });

        let r_paren = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space(), r_paren_token.format()])
            } else {
                write!(f, [r_paren_token.format()])
            }
        });

        write![
            f,
            [
                modifiers.format(),
                space(),
                set_token.format(),
                space(),
                name.format(),
                l_paren,
                parameter.format(),
                comma_token.format(),
                r_paren,
                space(),
                body.format(),
            ]
        ]
    }
}

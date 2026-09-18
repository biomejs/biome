use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsComputedMemberName;
use biome_js_syntax::JsComputedMemberNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsComputedMemberName;

impl FormatNodeRule<JsComputedMemberName> for FormatJsComputedMemberName {
    fn fmt_fields(&self, node: &JsComputedMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsComputedMemberNameFields {
            l_brack_token,
            expression,
            r_brack_token,
        } = node.as_fields();

        let l_brack = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [l_brack_token.format(), space()])
            } else {
                write!(f, [l_brack_token.format()])
            }
        });

        let r_brack = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space(), r_brack_token.format()])
            } else {
                write!(f, [r_brack_token.format()])
            }
        });

        write![f, [l_brack, expression.format(), r_brack]]
    }
}

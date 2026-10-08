use crate::js::bindings::parameters::ParameterLayout;
use crate::js::lists::parameter_list::FormatJsAnyParameterList;
use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::parameter_ext::AnyJsParameterList;
use biome_js_syntax::{JsVueSlotPropsRoot, JsVueSlotPropsRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVueSlotPropsRoot;
impl FormatNodeRule<JsVueSlotPropsRoot> for FormatJsVueSlotPropsRoot {
    fn fmt_fields(&self, node: &JsVueSlotPropsRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVueSlotPropsRootFields {
            parameters,
            eof_token,
        } = node.as_fields();

        // Vue writes the parameters between parentheses, so a trailing comma
        // after the last one would be a syntax error.
        let parameters = AnyJsParameterList::from(parameters);
        write!(
            f,
            [
                group(&FormatJsAnyParameterList::with_layout(
                    &parameters,
                    ParameterLayout::Hug
                )),
                eof_token.format()
            ]
        )
    }
}

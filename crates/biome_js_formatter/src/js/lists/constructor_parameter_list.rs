use crate::js::bindings::parameters::ParameterLayout;
use crate::js::lists::parameter_list::FormatJsAnyParameterList;
use crate::prelude::*;
use biome_js_syntax::JsConstructorParameterList;
use biome_js_syntax::parameter_ext::AnyJsParameterList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorParameterList;

impl FormatRule<JsConstructorParameterList> for FormatJsConstructorParameterList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsConstructorParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        FormatJsAnyParameterList::with_layout(
            &AnyJsParameterList::from(node.clone()),
            ParameterLayout::Default,
        )
        .fmt(f)
    }
}

use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusMember;

impl FormatBogusNodeRule<JsBogusMember> for FormatJsBogusMember {}

use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusRoot;
impl FormatBogusNodeRule<JsBogusRoot> for FormatJsBogusRoot {}

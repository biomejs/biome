use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusVariableDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusVariableDeclaration;
impl FormatBogusNodeRule<JsBogusVariableDeclaration> for FormatJsBogusVariableDeclaration {}

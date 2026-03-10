use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusIfTestBooleanExpr;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusIfTestBooleanExpr;
impl FormatBogusNodeRule<CssBogusIfTestBooleanExpr> for FormatCssBogusIfTestBooleanExpr {}

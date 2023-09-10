use crate::prelude::*;

use biome_js_syntax::TsAssertsCondition;
use biome_js_syntax::TsAssertsConditionFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAssertsCondition;

impl FormatNodeRule<TsAssertsCondition> for FormatTsAssertsCondition {
    fn fmt_fields(&self, node: &TsAssertsCondition, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAssertsConditionFields { is_token, ty } = node.as_fields();
        write![f, [is_token.format(), space(), ty.format()]]
    }
}

use crate::prelude::*;
use biome_js_syntax::JsSwitchCaseList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSwitchCaseList;

impl FormatRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsSwitchCaseList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for case in node {
            join.entry(case.syntax(), &format_or_verbatim(case.format()));
        }

        join.finish()
    }
}

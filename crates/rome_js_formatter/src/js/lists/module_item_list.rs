use crate::prelude::*;
use biome_js_syntax::{AnyJsModuleItem, AnyJsStatement, JsModuleItemList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsModuleItemList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for module_item in node {
            match module_item {
                AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsEmptyStatement(empty)) => {
                    join.entry_no_separator(&empty.format());
                }
                _ => {
                    join.entry(
                        module_item.syntax(),
                        &format_or_verbatim(module_item.format()),
                    );
                }
            }
        }

        join.finish()
    }
}

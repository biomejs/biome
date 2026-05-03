use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_markdown_syntax::MdCodeNameList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdCodeNameList;
impl FormatRule<MdCodeNameList> for FormatMdCodeNameList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdCodeNameList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        for entry in node.iter() {
            joiner.entry(&entry.format().with_options(FormatMdTextualOptions {
                print_mode: TextPrintMode::trim_all(),
                should_remove: false,
            }));
        }

        joiner.finish()
    }
}

use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_markdown_syntax::MdCodeNameList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdCodeNameList;
impl FormatRule<MdCodeNameList> for FormatMdCodeNameList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdCodeNameList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        for entry in node.iter() {
            joiner.entry(&entry.format().with_options(FormatMdTextualOptions {
                print_mode: TextPrintMode::Trim(TrimMode::All),
                should_remove: false,
                trim_start: true,
            }));
        }

        joiner.finish()
    }
}

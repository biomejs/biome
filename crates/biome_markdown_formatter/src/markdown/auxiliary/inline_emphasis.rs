use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineEmphasis, MdInlineEmphasisFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineEmphasis;
impl FormatNodeRule<MdInlineEmphasis> for FormatMdInlineEmphasis {
    fn fmt_fields(&self, node: &MdInlineEmphasis, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineEmphasisFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        write!(f, [l_fence.format(), content.format(), r_fence.format()])
    }
}

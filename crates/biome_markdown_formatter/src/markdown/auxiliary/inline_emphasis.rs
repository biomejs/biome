use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    MdInlineEmphasis, MdInlineEmphasisFields, emphasis_ext::MdEmphasisFence,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineEmphasis;
impl FormatNodeRule<MdInlineEmphasis> for FormatMdInlineEmphasis {
    fn fmt_fields(&self, node: &MdInlineEmphasis, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineEmphasisFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        if node.fence().ok() == Some(MdEmphasisFence::DoubleStar) {
            write!(f, [l_fence.format(), content.format(), r_fence.format()])
        } else {
            write!(
                f,
                [
                    format_replaced(&l_fence?, &token(MdEmphasisFence::DoubleStar.as_str())),
                    content.format(),
                    format_replaced(&r_fence?, &token(MdEmphasisFence::DoubleStar.as_str())),
                ]
            )
        }
    }
}

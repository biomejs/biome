use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritRewrite, GritRewriteFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRewrite;
impl FormatNodeRule<GritRewrite> for FormatGritRewrite {
    fn fmt_fields(&self, node: &GritRewrite, f: &mut GritFormatter) -> FormatResult<()> {
        let GritRewriteFields {
            left,
            right,
            annotation,
            fat_arrow_token,
        } = node.as_fields();

        write!(f, [left.format(), space()])?;
        if let Some(annotation) = annotation {
            write!(f, [annotation.format(), space()])?;
        };

        write!(f, [fat_arrow_token.format(), space(), right.format()])
    }
}

use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_grit_syntax::{GritList, GritListFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritList;
impl FormatNodeRule<GritList> for FormatGritList {
    fn fmt_fields(&self, node: &GritList, f: &mut GritFormatter) -> FormatResult<()> {
        let GritListFields {
            l_brack_token,
            name,
            patterns,
            r_brack_token,
        } = node.as_fields();

        let should_expand = f.comments().has_dangling_comments(node.syntax());

        write!(
            f,
            [
                l_brack_token.format(),
                name.format(),
                group(&soft_block_indent(&format_args![
                    patterns.format(),
                    format_dangling_comments(node.syntax())
                ]))
                .should_expand(should_expand),
                line_suffix_boundary(),
                r_brack_token.format()
            ]
        )
    }
}

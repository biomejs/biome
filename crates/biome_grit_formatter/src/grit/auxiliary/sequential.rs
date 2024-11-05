use crate::prelude::*;
use biome_grit_syntax::GritSequential;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSequential;
impl FormatNodeRule<GritSequential> for FormatGritSequential {
    fn fmt_fields(&self, node: &GritSequential, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)

        // TODO: investigate the verbatim panic when this code runs
        // let GritSequentialFields {
        //     l_curly_token,
        //     sequential_token,
        //     sequential,
        //     r_curly_token,
        // } = node.as_fields();
        //
        // write!(
        //     f,
        //     [
        //         sequential_token.format(),
        //         space(),
        //         l_curly_token.format(),
        //         sequential.format(),
        //         r_curly_token.format()
        //     ]
        // )
    }
}

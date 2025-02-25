use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritNamedArg, GritNamedArgFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNamedArg;
impl FormatNodeRule<GritNamedArg> for FormatGritNamedArg {
    fn fmt_fields(&self, node: &GritNamedArg, f: &mut GritFormatter) -> FormatResult<()> {
        let GritNamedArgFields {
            name,
            pattern,
            eq_token,
        } = node.as_fields();

        write!(f, [name.format(), eq_token.format(), pattern.format()])
    }
}

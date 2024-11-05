use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateCall, GritPredicateCallFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateCall;
impl FormatNodeRule<GritPredicateCall> for FormatGritPredicateCall {
    fn fmt_fields(&self, node: &GritPredicateCall, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateCallFields {
            name,
            l_paren_token,
            named_args,
            r_paren_token,
        } = node.as_fields();
        write!(
            f,
            [
                name.format(),
                l_paren_token.format(),
                named_args.format(),
                r_paren_token.format()
            ]
        )
    }
}

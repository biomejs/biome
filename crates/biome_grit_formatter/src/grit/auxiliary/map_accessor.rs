use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritMapAccessor, GritMapAccessorFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapAccessor;
impl FormatNodeRule<GritMapAccessor> for FormatGritMapAccessor {
    fn fmt_fields(&self, node: &GritMapAccessor, f: &mut GritFormatter) -> FormatResult<()> {
        let GritMapAccessorFields {
            map,
            dot_token,
            key,
        } = node.as_fields();

        write!(f, [map.format(), dot_token.format(), key.format()])
    }
}

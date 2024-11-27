use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritRegexPattern, GritRegexPatternFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRegexPattern;
impl FormatNodeRule<GritRegexPattern> for FormatGritRegexPattern {
    fn fmt_fields(&self, node: &GritRegexPattern, f: &mut GritFormatter) -> FormatResult<()> {
        let GritRegexPatternFields { variables, regex } = node.as_fields();

        write!(f, [regex.format(), variables.format()])
    }
}

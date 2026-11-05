use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_toml_syntax::TomlKeyPartList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlKeyPartList;

impl FormatRule<TomlKeyPartList> for FormatTomlKeyPartList {
    type Context = TomlFormatContext;

    /// Prints the parts of a dotted key without the whitespace around the dots.
    fn fmt(&self, node: &TomlKeyPartList, f: &mut TomlFormatter) -> FormatResult<()> {
        f.join()
            .entries(node.format_separated(".", TrailingSeparator::Disallowed))
            .finish()
    }
}

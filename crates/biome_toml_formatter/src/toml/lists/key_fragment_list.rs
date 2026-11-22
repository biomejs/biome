use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_toml_syntax::TomlKeyFragmentList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlKeyFragmentList;

impl FormatRule<TomlKeyFragmentList> for FormatTomlKeyFragmentList {
    type Context = TomlFormatContext;

    /// Prints the fragments of a dotted key without the whitespace around the dots.
    fn fmt(&self, node: &TomlKeyFragmentList, f: &mut TomlFormatter) -> FormatResult<()> {
        f.join()
            .entries(node.format_separated(".", TrailingSeparator::Disallowed))
            .finish()
    }
}

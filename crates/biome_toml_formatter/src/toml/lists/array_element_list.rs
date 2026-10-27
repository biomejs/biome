use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_toml_syntax::TomlArrayElementList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlArrayElementList;

impl FormatRule<TomlArrayElementList> for FormatTomlArrayElementList {
    type Context = TomlFormatContext;

    /// Prints one element per line when the array breaks, with a trailing
    /// comma, and all elements on one line otherwise.
    fn fmt(&self, node: &TomlArrayElementList, f: &mut TomlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();
        for (element, formatted) in node
            .elements()
            .zip(node.format_separated(",", TrailingSeparator::Allowed))
        {
            join.entry(element.node()?.syntax(), &formatted);
        }
        join.finish()
    }
}

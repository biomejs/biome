use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::YamlFlowMapEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapEntryList;
impl FormatRule<YamlFlowMapEntryList> for FormatYamlFlowMapEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowMapEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut first = true;

        for (element, formatted) in node.elements().zip(
            node.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Allowed),
        ) {
            let element = element.node()?;

            if first {
                first = false;
            } else {
                write!(f, [soft_line_break_or_space()])?;
                // A blank line between entries survives only when the
                // collection is already expanded; as literal content it
                // doesn't itself expand the group. The literal break resets
                // the indentation, so a soft line break follows to restore
                // it without printing anything
                if get_lines_before(element.syntax()) > 1 {
                    write!(
                        f,
                        [if_group_breaks(&format_args![
                            literal_line_break_without_parent(),
                            soft_line_break()
                        ])]
                    )?;
                }
            }

            write!(f, [formatted])?;
        }

        Ok(())
    }
}

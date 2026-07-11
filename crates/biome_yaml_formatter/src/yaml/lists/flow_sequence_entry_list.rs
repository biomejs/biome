use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::write;
use biome_yaml_syntax::YamlFlowSequenceEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequenceEntryList;
impl FormatRule<YamlFlowSequenceEntryList> for FormatYamlFlowSequenceEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowSequenceEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut first = true;

        for (element, formatted) in node.elements().zip(
            node.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Allowed),
        ) {
            let element = element.node()?;

            if first {
                first = false;
            } else if get_lines_before(element.syntax()) > 1 {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [soft_line_break_or_space()])?;
            }

            write!(f, [formatted])?;
        }

        Ok(())
    }
}

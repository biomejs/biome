use crate::comments::preserved_lines_before;
use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::YamlBlockMapEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapEntryList;
impl FormatRule<YamlBlockMapEntryList> for FormatYamlBlockMapEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlBlockMapEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let comments = f.comments().clone();
        let mut joined = false;
        for entry in node {
            if joined {
                if preserved_lines_before(&comments, entry.syntax()) > 1 {
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
            }
            joined = true;
            write!(f, [entry.format()])?;
        }
        Ok(())
    }
}

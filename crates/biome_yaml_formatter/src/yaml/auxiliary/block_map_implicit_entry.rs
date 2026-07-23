use crate::comments::subtree_has_comments;
use crate::prelude::*;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockMapImplicitEntry, YamlBlockMapImplicitEntryFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapImplicitEntry;
impl FormatNodeRule<YamlBlockMapImplicitEntry> for FormatYamlBlockMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [key.format(), colon_token.format()])?;

        if let Some(value) = value {
            let has_leading_comments = f.comments().has_leading_comments(value.syntax());
            if has_leading_comments {
                return write!(
                    f,
                    [indent(&format_args![hard_line_break(), value.format()])]
                );
            }

            if value.is_nested_block_collection() {
                return write!(
                    f,
                    [indent(&format_args![hard_line_break(), value.format()])]
                );
            }

            if value.is_flow_collection() {
                // A collection that a comment forces to break
                // stays on the key's line, while one that only breaks because
                // it doesn't fit moves as a whole to its own indented line.
                // Both keep the collection's entries and closing bracket
                // indented deeper than the mapping key, as the spec requires
                if subtree_has_comments(f.comments(), value.syntax()) {
                    return write!(f, [space(), align("  ", &value.format())]);
                }
                return write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break_or_space(),
                        value.format()
                    ]))]
                );
            }

            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}

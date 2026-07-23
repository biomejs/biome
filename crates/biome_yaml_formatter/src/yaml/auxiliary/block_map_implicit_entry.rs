use crate::comments::{FormatEntryDanglingComments, subtree_has_comments};
use crate::prelude::*;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_yaml_syntax::{
    AnyYamlBlockNode, YamlBlockMapImplicitEntry, YamlBlockMapImplicitEntryFields,
};

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
            // A comment on the key's line, between the colon and the value,
            // ends that line, so the value moves to its own indented line:
            //
            // ```yaml
            // key: # comment
            //   value
            // ```
            let comment_ends_key_line = key
                .as_ref()
                .is_some_and(|key| f.comments().has_trailing_comments(key.syntax()));
            let has_leading_comments = f.comments().has_leading_comments(value.syntax());

            if has_leading_comments || comment_ends_key_line || value.is_nested_block_collection() {
                write!(
                    f,
                    [indent(&format_args![hard_line_break(), value.format()])]
                )?;
            } else if value.is_flow_collection() {
                // A collection that a comment forces to break
                // stays on the key's line, while one that only breaks because
                // it doesn't fit moves as a whole to its own indented line.
                // Both keep the collection's entries and closing bracket
                // indented deeper than the mapping key, as the spec requires
                if subtree_has_comments(f.comments(), value.syntax()) {
                    write!(f, [space(), align("  ", &value.format())])?;
                } else {
                    write!(
                        f,
                        [group(&indent(&format_args![
                            soft_line_break_or_space(),
                            value.format()
                        ]))]
                    )?;
                }
            } else if matches!(&value, AnyYamlBlockNode::YamlFlowInBlockNode(_)) {
                // The continuation lines of a multiline flow scalar are
                // indented past the key:
                //
                // ```yaml
                // key: word
                //   word
                // ```
                write!(f, [space(), indent(&value.format())])?;
            } else {
                write!(f, [space(), value.format()])?;
            }
        }

        write!(f, [FormatEntryDanglingComments::new(node.syntax())])
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlBlockMapImplicitEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments sit in the entry's value slot, indented
        // deeper than the entry:
        //
        // ```yaml
        // key:
        //   # comment
        // ```
        //
        // They are printed by `FormatEntryDanglingComments`, which
        // `fmt_fields` writes with that indentation; the default
        // implementation would print them without it
        Ok(())
    }
}

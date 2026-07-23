use crate::comments::FormatEntryDanglingComments;
use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockSequenceEntry, YamlBlockSequenceEntryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequenceEntry;
impl FormatNodeRule<YamlBlockSequenceEntry> for FormatYamlBlockSequenceEntry {
    fn fmt_fields(&self, node: &YamlBlockSequenceEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockSequenceEntryFields { minus_token, value } = node.as_fields();

        let minus_token = minus_token?;
        write!(f, [minus_token.format()])?;

        // An entry without a value but with a comment on its line gets the
        // space that would separate the value from the `-` even though
        // there is none, matching Prettier: `-  # comment`
        if value.is_none()
            && f.comments()
                .dangling_comments(node.syntax())
                .first()
                .is_some_and(|comment| comment.lines_before() == 0)
        {
            write!(f, [text(" ", None)])?;
        }

        if let Some(value) = value {
            if value.is_block_scalar() {
                // A block scalar indents its own content one level past the
                // `-`, so the alignment for continuation lines is not needed:
                //
                // ```yaml
                // - |
                //   content
                // ```
                write!(f, [space(), value.format()])?;
            } else {
                write!(f, [space(), align("  ", &value.format())])?;
            }
        }

        write!(f, [FormatEntryDanglingComments::new(node.syntax())])
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlBlockSequenceEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments sit in the entry's value slot, indented
        // deeper than the entry:
        //
        // ```yaml
        // - value
        //     # comment
        // ```
        //
        // They are printed by `FormatEntryDanglingComments`, which
        // `fmt_fields` writes with that indentation; the default
        // implementation would print them without it
        Ok(())
    }
}

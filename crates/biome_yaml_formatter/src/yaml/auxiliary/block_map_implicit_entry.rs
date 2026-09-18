use super::flow_map_implicit_entry::FormatCollectionKeyEntry;
use crate::comments::{FormatEntryDanglingComments, subtree_has_comments};
use crate::prelude::*;
use crate::utils::{flow_scalar_has_line_break, needs_space_before_colon};
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

        // A flow collection key that breaks across multiple lines is only a
        // valid mapping key in the explicit `? key : value` form, which
        // `FormatCollectionKeyEntry` synthesizes when the key breaks:
        //
        // ```yaml
        // ? [key, that, does, not, fit, on, a, single, line]
        // : value
        // ```
        if let (Some(entry_key), Ok(colon_token), Some(AnyYamlBlockNode::YamlFlowInBlockNode(_))) =
            (&key, &colon_token, &value)
            && entry_key.is_flow_collection()
            && let Some(entry_value) = &value
        {
            return write!(
                f,
                [FormatCollectionKeyEntry {
                    key: entry_key,
                    colon_token,
                    value: entry_value.clone().into(),
                }]
            );
        }

        write!(f, [key.format()])?;

        // A `:` directly following an alias, anchor, or tag would be lexed as
        // part of that token, so the separating space is required
        if key.as_ref().is_some_and(needs_space_before_colon) {
            write!(f, [space()])?;
        }

        write!(f, [colon_token.format()])?;

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

            write!(
                f,
                [FormatEntryValue {
                    value: &value,
                    on_next_line: comment_ends_key_line
                }]
            )?;
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

/// Formats the value of a mapping entry, placing it relative to the `:` that
/// precedes it
pub(crate) struct FormatEntryValue<'a> {
    pub(crate) value: &'a AnyYamlBlockNode,
    /// Forces the value onto its own indented line below the entry, for
    /// when a comment already ended the line the value would have gone on
    pub(crate) on_next_line: bool,
}

impl Format<YamlFormatContext> for FormatEntryValue<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let value = self.value;
        let has_leading_comments = f.comments().has_leading_comments(value.syntax());

        if self.on_next_line || has_leading_comments {
            write!(
                f,
                [indent(&format_args![hard_line_break(), value.format()])]
            )
        } else if value.is_nested_block_collection() {
            // The properties of the collection stay on the key's line,
            // with the collection itself below:
            //
            // ```yaml
            // key: &anchor
            //   a: 1
            // ```
            if value.has_properties() {
                write!(f, [space(), value.format()])
            } else {
                write!(
                    f,
                    [indent(&format_args![hard_line_break(), value.format()])]
                )
            }
        } else if value.is_flow_collection() {
            // A collection that a comment forces to break
            // stays on the key's line, while one that only breaks because
            // it doesn't fit moves as a whole to its own indented line.
            // Both keep the collection's entries and closing bracket
            // indented deeper than the mapping key, as the spec requires
            if subtree_has_comments(f.comments(), value.syntax()) {
                write!(f, [space(), align("  ", &value.format())])
            } else {
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break_or_space(),
                        value.format()
                    ]))]
                )
            }
        } else if let AnyYamlBlockNode::YamlFlowInBlockNode(flow_in_block) = value {
            // The continuation lines of a multiline flow scalar are
            // indented past the key:
            //
            // ```yaml
            // key: word
            //   word
            // ```
            //
            // When the key and the scalar's first line together don't fit
            // within the line width, the whole scalar moves below the key
            // instead:
            //
            // ```yaml
            // key:
            //   word that would have overflowed the line
            //   word
            // ```
            //
            // Only a scalar that is already multiline moves; a single-line
            // scalar stays on the key's line no matter how long. The break
            // is inside the scalar's own token: the properties and the
            // content of the value are joined onto one line, so the breaks
            // in the trivia between them don't reach the output
            let is_multiline = flow_scalar_has_line_break(flow_in_block);
            if is_multiline {
                let value = value.format().memoized();
                write!(
                    f,
                    [best_fitting![
                        format_args![space(), indent(&value)],
                        format_args![indent(&format_args![hard_line_break(), value])],
                    ]]
                )
            } else {
                write!(f, [space(), indent(&value.format())])
            }
        } else {
            write!(f, [space(), value.format()])
        }
    }
}

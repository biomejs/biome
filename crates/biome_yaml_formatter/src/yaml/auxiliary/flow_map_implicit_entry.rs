use super::flow_map_explicit_entry::FormatExplicitPair;
use crate::comments::{YamlComments, subtree_has_comments};
use crate::prelude::*;
use crate::utils::{FormatMultilineKeyEntry, multiline_plain_key, needs_space_before_colon};
use biome_formatter::{format_args, write};
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    AnyYamlEntryValue, AnyYamlFlowNode, AnyYamlMappingImplicitKey, YamlFlowMapImplicitEntry,
    YamlFlowMapImplicitEntryFields, YamlSyntaxKind, YamlSyntaxNode, YamlSyntaxToken,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapImplicitEntry;
impl FormatNodeRule<YamlFlowMapImplicitEntry> for FormatYamlFlowMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlFlowMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        let in_flow_mapping = node
            .syntax()
            .parent()
            .is_some_and(|parent| parent.kind() == YamlSyntaxKind::YAML_FLOW_MAP_ENTRY_LIST);

        // The comments on lines of their own between the key and the `:` are
        // only kept there in the explicit form:
        //
        // ```yaml
        // {
        //   ? key
        //   # comment
        //   : value,
        // }
        // ```
        if let (Some(colon_token), Some(_)) = (&colon_token, &value)
            && f.comments().has_dangling_comments(node.syntax())
        {
            return write!(
                f,
                [FormatExplicitPair {
                    question_mark_token: None,
                    key: &key,
                    colon_token,
                    value: &value,
                    dangling_comments_of: Some(node.syntax()),
                }]
            );
        }

        // A key spanning multiple lines can only be held by the explicit
        // `? key : value` form, so the entry converts to it
        if key.as_ref().and_then(multiline_plain_key).is_some()
            && let (Some(entry_key), Some(colon_token)) = (&key, &colon_token)
        {
            return write!(
                f,
                [FormatMultilineKeyEntry {
                    question_mark_token: None,
                    key: entry_key,
                    colon_token,
                    value: &value,
                }]
            );
        }

        if let (Some(entry_key), Some(colon_token), Some(entry_value)) =
            (&key, &colon_token, &value)
            && (entry_key.is_flow_collection()
                || f.comments().has_trailing_comments(entry_key.syntax()))
        {
            return write!(
                f,
                [FormatBreakableKeyEntry {
                    key: entry_key,
                    colon_token,
                    value: entry_value.clone().into(),
                }]
            );
        }

        let body = FormatImplicitEntryBody {
            key: &key,
            colon_token: &colon_token,
            value: &value,
            in_flow_mapping,
        };
        write!(f, [body.group(&body, f)])
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowMapImplicitEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments sit between the key and the `:`. They are
        // printed inside `FormatExplicitPair` on their own lines before the
        // `:`; the default implementation would print them after the value
        Ok(())
    }
}

/// Formats a `key: value` entry whose key may break: a flow collection, or
/// a key followed by a comment.
///
/// A key that breaks across multiple lines is only a valid mapping key in
/// the explicit `? key : value` form, so a `?` is synthesized for such a key:
///
/// ```yaml
/// {
///   ? [key, that, does, not, fit, on, a, single, line]
///   : value,
/// }
/// ```
///
/// Whether the key breaks is the printer's decision: the key gets a group
/// of its own, and conditional content on that group picks between the
/// implicit and the explicit form. A comment inside the key or after it
/// expands the group, so it picks the explicit form as well:
///
/// ```yaml
/// { ? key # comment
///   : value }
/// ```
///
/// The entry is a boundary group, where Prettier uses a single-state
/// `conditionalGroup`: its line breaks don't expand the enclosing collection,
/// which stays on one line if the content up to the first line break fits.
pub(crate) struct FormatBreakableKeyEntry<'a> {
    pub(crate) key: &'a AnyYamlMappingImplicitKey,
    pub(crate) colon_token: &'a YamlSyntaxToken,
    pub(crate) value: AnyYamlEntryValue,
}

impl Format<YamlFormatContext> for FormatBreakableKeyEntry<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let group_id = f.group_id("collection_key");
        let key_format = self.key.format().memoized();
        let colon_format = self.colon_token.format().memoized();
        let value_format = format_with(|f| match &self.value {
            AnyYamlEntryValue::AnyYamlFlowNode(value) => write!(f, [value.format()]),
            AnyYamlEntryValue::AnyYamlBlockNode(value) => write!(f, [value.format()]),
        })
        .memoized();
        // A comment inside the value forces it to break
        let value_has_comments = subtree_has_comments(f.comments(), self.value.syntax());
        let value_on_own_line =
            value_starts_on_own_line(f.comments(), self.colon_token, self.value.syntax());

        write!(
            f,
            [boundary_group(&format_args![
                group(&format_args![
                    if_group_breaks(&format_args![text("?", None), space()]),
                    group(&align("  ", &key_format)).with_group_id(Some(group_id)),
                ]),
                if_group_breaks(&format_args![
                    hard_line_break(),
                    colon_format,
                    space(),
                    align("  ", &value_format)
                ])
                .with_group_id(Some(group_id)),
                if_group_fits_on_line(&format_with(|f| {
                    if value_on_own_line {
                        return write!(
                            f,
                            [align(
                                "  ",
                                &format_args![colon_format, hard_line_break(), value_format]
                            )]
                        );
                    }
                    write!(f, [colon_format])?;
                    if value_has_comments {
                        // A value that a comment forces to break stays on the
                        // key's line instead of moving to its own line:
                        //
                        // ```yaml
                        // { [key]: [a, # comment
                        //     b] }
                        // ```
                        //
                        // becomes
                        //
                        // ```yaml
                        // {
                        //   [key]: [
                        //       a, # comment
                        //       b,
                        //     ],
                        // }
                        // ```
                        write!(f, [space(), align("  ", &value_format)])
                    } else {
                        // A value that fits stays on the key's line; one that
                        // doesn't moves as a whole to its own indented line
                        write!(
                            f,
                            [group(&indent(&format_args![
                                soft_line_break_or_space(),
                                value_format
                            ]))]
                        )
                    }
                }))
                .with_group_id(Some(group_id)),
            ])]
        )
    }
}

/// Formats a `key: value` entry of a flow mapping, where each of the key, the
/// `:`, and the value can individually be absent.
///
/// `in_flow_mapping` is `false` for the compact single-pair form inside a
/// flow sequence (`[a: b]`), where the `:` cannot be dropped: `[a:]` is a
/// mapping while `[a]` is a scalar.
pub(crate) struct FormatImplicitEntryBody<'a> {
    pub(crate) key: &'a Option<AnyYamlMappingImplicitKey>,
    pub(crate) colon_token: &'a Option<YamlSyntaxToken>,
    pub(crate) value: &'a Option<AnyYamlFlowNode>,
    pub(crate) in_flow_mapping: bool,
}

/// Whether comments sit on lines between the `:` and the value, which then
/// starts on a line of its own, as in Prettier:
///
/// ```yaml
/// { key:
///     # comment
///     value }
/// ```
///
/// A comment leading the value on the line of the key, a suppression
/// comment, stays on that line.
fn value_starts_on_own_line(
    comments: &YamlComments,
    colon_token: &YamlSyntaxToken,
    value: &YamlSyntaxNode,
) -> bool {
    comments
        .leading_comments(value)
        .first()
        .is_some_and(|comment| comment.lines_before() > 0 || colon_token.has_leading_newline())
}

impl FormatImplicitEntryBody<'_> {
    /// Whether the value starts on a line of its own, see
    /// [value_starts_on_own_line]. Without a key, the `:` starts the entry
    /// and the value stays on its line, after any comment
    pub(crate) fn value_on_own_line(&self, f: &YamlFormatter) -> bool {
        match (self.key, self.colon_token, self.value) {
            (Some(_), Some(colon_token), Some(value)) => {
                value_starts_on_own_line(f.comments(), colon_token, value.syntax())
            }
            _ => false,
        }
    }

    /// Groups `content`, the entry holding this body. The group is a
    /// boundary group, where Prettier uses a single-state `conditionalGroup`,
    /// when the value starts on a line of its own, so that the line break
    /// doesn't expand the enclosing collection
    pub(crate) fn group<'b>(
        &self,
        content: &'b impl Format<YamlFormatContext>,
        f: &YamlFormatter,
    ) -> Group<'b, YamlFormatContext> {
        if self.value_on_own_line(f) {
            boundary_group(content)
        } else {
            group(content)
        }
    }
}

impl Format<YamlFormatContext> for FormatImplicitEntryBody<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        if let Some(key) = self.key {
            write!(f, [key.format()])?;
        }

        let Some(colon_token) = self.colon_token else {
            return Ok(());
        };

        // `key:` without a value is equivalent to just `key`. A lone `:`
        // denotes an entry with an empty key and an empty value, so its `:`
        // has to stay:
        //
        // ```yaml
        // { key: , : }
        // ```
        //
        // becomes
        //
        // ```yaml
        // { key, : }
        // ```
        if self.in_flow_mapping && self.key.is_some() && self.value.is_none() {
            return write!(f, [format_removed(colon_token)]);
        }

        // A `:` directly following an alias, anchor, or tag would be lexed as
        // part of that token, so the separating space is required
        let colon = format_with(|f| {
            if self.key.as_ref().is_some_and(needs_space_before_colon) {
                write!(f, [space()])?;
            }
            write!(f, [colon_token.format()])
        });

        if let Some(value) = self.value
            && self.value_on_own_line(f)
        {
            return write!(
                f,
                [align(
                    "  ",
                    &format_args![colon, hard_line_break(), value.format()]
                )]
            );
        }

        write!(f, [colon])?;

        // In the single-pair form inside a flow sequence, the space that
        // would separate the value is printed even when there is none:
        // `[ : ]` becomes `[: ]`
        if self.value.is_none() && !self.in_flow_mapping {
            write!(f, [space()])?;
        }

        if let Some(value) = self.value {
            if !value.is_flow_collection() {
                write!(f, [space(), align("  ", &value.format())])?;
            } else if self.key.is_none() || subtree_has_comments(f.comments(), value.syntax()) {
                // With no key before it, the collection stays on the `:` line
                // even when it breaks. The same goes for a collection that a
                // comment forces to break
                write!(f, [space(), align("  ", &value.format())])?;
            } else {
                // A collection value that fits stays on the key's line; one
                // that doesn't moves as a whole to its own indented line
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break_or_space(),
                        value.format()
                    ]))]
                )?;
            }
        }

        Ok(())
    }
}

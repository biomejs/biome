use crate::comments::{FormatCommentsSlice, source_column};
use crate::prelude::*;
use crate::yaml::auxiliary::block_map_implicit_entry::FormatEntryValue;
use biome_formatter::comments::SourceComment;
use biome_formatter::{format_args, write};
use biome_rowan::{AstNode, TextSize};
use biome_yaml_syntax::{
    AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlProperty, YamlBlockInBlockNode,
    YamlBlockMapExplicitEntry, YamlBlockMapExplicitEntryFields, YamlLanguage,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapExplicitEntry;
impl FormatNodeRule<YamlBlockMapExplicitEntry> for FormatYamlBlockMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockMapExplicitEntryFields {
            question_mark_token,
            key,
            colon_token,
            value,
        } = node.as_fields();
        let question_mark_token = question_mark_token?;

        let comments = f.comments().clone();
        let dangling = comments.dangling_comments(node.syntax());

        // The dangling comments split into those before the `:`, those on
        // the line of the `:` before the value, and those following the
        // whole entry, in its value slot
        let colon_start = colon_token
            .as_ref()
            .map(|colon| colon.text_trimmed_range().start());
        let content_end = value
            .as_ref()
            .map(|value| value.range().end())
            .or_else(|| {
                colon_token
                    .as_ref()
                    .map(|colon| colon.text_trimmed_range().end())
            })
            .or_else(|| key.as_ref().map(|key| key.range().end()))
            .unwrap_or_else(|| question_mark_token.text_trimmed_range().end());
        let (before_colon, rest) = split_comments(dangling, colon_start.unwrap_or(content_end));
        let (after_colon, value_slot) = split_comments(rest, content_end);

        // An explicit entry converts to the `key: value` form whenever the
        // key can be an implicit key: it fits on one line and no comment
        // sits in the entry's head. Block nodes and multiline flow scalars
        // can't; a flow collection reflows onto one line, so it can. The
        // keys of a `!!set` mapping have no values by definition and keep
        // the `? key` form that makes them recognizable as set members
        let keep_explicit = match &key {
            None => false,
            Some(key_node @ AnyYamlBlockNode::YamlFlowInBlockNode(_)) => {
                let text = key_node.syntax().text_trimmed();
                !key_node.is_flow_collection()
                    && (text.contains_char('\n') || text.contains_char('\r'))
            }
            Some(_) => true,
        };
        // A lone comment on the key's line doesn't stand in the way of the
        // implicit form when a value follows: it moves after the `:` and
        // the value goes below it
        let key_trailing_inline = value.is_some()
            && before_colon.len() == 1
            && before_colon[0].lines_before() == 0
            && key.as_ref().is_some_and(|key| {
                before_colon[0].piece().text_range().start() >= key.range().end()
            });

        // A comment on the key's own line has no place in the implicit
        // form; but one at the end of the key's line moves after the `:`,
        // with the value below it
        let key_comments_force_explicit = key.as_ref().is_some_and(|key| {
            key.syntax().descendants().any(|descendant| {
                f.comments().has_leading_comments(&descendant)
                    || f.comments().has_dangling_comments(&descendant)
                    || f.comments()
                        .trailing_comments(&descendant)
                        .iter()
                        .any(|comment| comment.lines_before() > 0)
            })
        });
        let key_has_inline_trailing = key.as_ref().is_some_and(|key| {
            key.syntax()
                .descendants()
                .any(|descendant| f.comments().has_trailing_comments(&descendant))
        });

        let keep_explicit = keep_explicit
            || !(before_colon.is_empty() || key_trailing_inline)
            || (value.is_none() && (in_set_mapping(node) || key_has_inline_trailing))
            || key_comments_force_explicit;

        if !keep_explicit && (key.is_some() || value.is_some()) {
            write!(f, [format_removed(&question_mark_token), key.format()])?;

            // A `:` directly following an alias, anchor, or tag would be
            // lexed as part of that token, so the separating space is
            // required
            if key.as_ref().is_some_and(needs_space_before_colon) {
                write!(f, [space()])?;
            }

            match &colon_token {
                Some(colon) => write!(f, [colon.format()])?,
                // A key without a value has no `:` of its own, but the
                // implicit form requires one
                None => write!(f, [text(":", None)])?,
            }

            // The comment that trailed the key stays on its line, now after
            // the `:`
            write!(
                f,
                [FormatCommentsSlice {
                    comments: before_colon,
                    inline_first: true
                }]
            )?;

            // A comment on the line of the `:` moves below the key, in
            // front of the value:
            //
            // ```yaml
            // key:
            //   # comment
            //   value
            // ```
            write!(
                f,
                [indent(&FormatCommentsSlice {
                    comments: after_colon,
                    inline_first: false
                })]
            )?;

            if let Some(value) = value {
                write!(
                    f,
                    [FormatEntryValue {
                        value: &value,
                        on_next_line: key_has_inline_trailing
                            || !before_colon.is_empty()
                            || !after_colon.is_empty()
                    }]
                )?;
            }

            return write!(
                f,
                [indent(&FormatCommentsSlice {
                    comments: value_slot,
                    inline_first: false
                })]
            );
        }

        write!(f, [question_mark_token.format()])?;

        let key_start = key.as_ref().map_or(content_end, |key| key.range().start());
        let (before_key, after_key) = split_comments(before_colon, key_start);

        write!(
            f,
            [align(
                "  ",
                &FormatCommentsSlice {
                    comments: before_key,
                    inline_first: true
                }
            )]
        )?;
        if let Some(key) = &key {
            // A block scalar key indents its own content past the `?`, so
            // the alignment for the other kinds of keys is not needed:
            //
            // ```yaml
            // ? |
            //   content
            // ```
            let key_content = format_with(|f| {
                if key.is_block_scalar() {
                    write!(f, [key.format()])
                } else {
                    write!(f, [align("  ", &key.format())])
                }
            });
            if before_key.is_empty() {
                write!(f, [space(), key_content])?;
            } else {
                write!(
                    f,
                    [align("  ", &format_args![hard_line_break(), key_content])]
                )?;
            }
        }
        // The comments between the key and the `:` keep their own
        // placement: one indented like the key's content stays there, while
        // one at the start of its line leads the `:` line:
        //
        // ```yaml
        // ? key
        //   # part of the key
        // # before the colon
        // : value
        // ```
        for comment in after_key {
            let indented = source_column(
                &comment.piece().as_piece().token(),
                comment.piece().text_range().start(),
            ) >= 2;
            let slice = std::slice::from_ref(comment);
            let format_comment = FormatCommentsSlice {
                comments: slice,
                inline_first: true,
            };
            if indented {
                write!(f, [align("  ", &format_comment)])?;
            } else {
                write!(f, [format_comment])?;
            }
        }

        if let Some(colon_token) = &colon_token {
            write!(f, [hard_line_break(), colon_token.format()])?;
            write!(
                f,
                [align(
                    "  ",
                    &FormatCommentsSlice {
                        comments: after_colon,
                        inline_first: true
                    }
                )]
            )?;

            if let Some(value) = &value {
                if after_colon.is_empty() {
                    write!(f, [space(), align("  ", &value.format())])?;
                } else {
                    write!(
                        f,
                        [align(
                            "  ",
                            &format_args![hard_line_break(), value.format()]
                        )]
                    )?;
                }
            }
        }

        write!(
            f,
            [indent(&FormatCommentsSlice {
                comments: value_slot,
                inline_first: false
            })]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlBlockMapExplicitEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments sit in the entry's head — next to the `?`,
        // the key, and the `:` — or follow the entry in its value slot:
        //
        // ```yaml
        // ? key
        //   # comment
        // : value
        // ```
        //
        // They are printed inside `fmt_fields` at the position they came
        // from; the default implementation would print them all at the end
        Ok(())
    }
}

/// Splits ordered comments into those starting before `boundary` and the
/// rest
fn split_comments(
    comments: &[SourceComment<YamlLanguage>],
    boundary: TextSize,
) -> (
    &[SourceComment<YamlLanguage>],
    &[SourceComment<YamlLanguage>],
) {
    let index = comments
        .iter()
        .position(|comment| comment.piece().text_range().start() >= boundary)
        .unwrap_or(comments.len());
    comments.split_at(index)
}

/// Whether the mapping this entry belongs to is tagged `!!set`
fn in_set_mapping(node: &YamlBlockMapExplicitEntry) -> bool {
    node.syntax()
        .ancestors()
        .find_map(YamlBlockInBlockNode::cast)
        .is_some_and(|block| {
            block.properties().iter().any(|property| {
                matches!(
                    &property,
                    AnyYamlProperty::YamlTagProperty(tag)
                        if tag.value_token().is_ok_and(|token| {
                            matches!(
                                token.text_trimmed(),
                                "!!set" | "!<tag:yaml.org,2002:set>"
                            )
                        })
                )
            })
        })
}

/// Whether a `:` placed directly after this key would be lexed as part of
/// the key's last token. Alias, anchor, and tag tokens may all contain `:`
fn needs_space_before_colon(key: &AnyYamlBlockNode) -> bool {
    match key {
        AnyYamlBlockNode::YamlFlowInBlockNode(node) => match node.flow() {
            Ok(AnyYamlFlowNode::YamlAliasNode(_)) => true,
            // A node without content ends with its last property
            Ok(AnyYamlFlowNode::YamlFlowYamlNode(node)) => node.content().is_none(),
            Ok(AnyYamlFlowNode::YamlFlowJsonNode(node)) => node.content().is_err(),
            Ok(AnyYamlFlowNode::YamlBogusFlowNode(_)) | Err(_) => false,
        },
        _ => false,
    }
}

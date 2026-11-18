use super::flow_map_implicit_entry::{FormatBreakableKeyEntry, FormatImplicitEntryBody};
use crate::comments::FormatCommentsSlice;
use crate::prelude::*;
use crate::utils::{FormatMultilineKeyEntry, multiline_plain_key};
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    AnyYamlFlowNode, AnyYamlMappingImplicitKey, YamlFlowMapExplicitEntry,
    YamlFlowMapExplicitEntryFields, YamlSyntaxKind, YamlSyntaxNode, YamlSyntaxToken,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapExplicitEntry;
impl FormatNodeRule<YamlFlowMapExplicitEntry> for FormatYamlFlowMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlFlowMapExplicitEntryFields {
            question_mark_token,
            key,
            colon_token,
            value,
        } = node.as_fields();

        let question_mark_token = question_mark_token?;
        let has_dangling_comments = f.comments().has_dangling_comments(node.syntax());

        // An entry whose key may break is formatted like an implicit one,
        // with a `?` that `FormatBreakableKeyEntry` synthesizes when the key
        // breaks
        if !has_dangling_comments
            && let (Some(entry_key), Some(colon_token), Some(entry_value)) =
                (&key, &colon_token, &value)
            && (entry_key.is_flow_collection()
                || f.comments().has_trailing_comments(entry_key.syntax()))
        {
            return write!(
                f,
                [
                    format_removed(&question_mark_token),
                    FormatBreakableKeyEntry {
                        key: entry_key,
                        colon_token,
                        value: entry_value.clone().into(),
                    }
                ]
            );
        }

        // A key spanning multiple lines can only be held by the explicit
        // form, which is kept
        if !has_dangling_comments
            && key.as_ref().and_then(multiline_plain_key).is_some()
            && let (Some(entry_key), Some(colon_token)) = (&key, &colon_token)
        {
            return write!(
                f,
                [FormatMultilineKeyEntry {
                    question_mark_token: Some(&question_mark_token),
                    key: entry_key,
                    colon_token,
                    value: &value,
                }]
            );
        }
        let in_flow_mapping = node
            .syntax()
            .parent()
            .is_some_and(|parent| parent.kind() == YamlSyntaxKind::YAML_FLOW_MAP_ENTRY_LIST);

        // The explicit form is required when the key is a flow collection
        // that may break across multiple lines, or when a comment sits
        // between the key and the `:`:
        //
        // ```yaml
        // {
        //   ? [a, b]
        //   : v,
        //   ? key
        //   # comment
        //   : v,
        // }
        // ```
        //
        // Otherwise print the entry in the implicit `key: value` form:
        //
        // ```yaml
        // { ? key : v }
        // ```
        //
        // becomes
        //
        // ```yaml
        // { key: v }
        // ```
        let keep_explicit = colon_token.is_some()
            && (has_dangling_comments
                || (value.is_some() && key.as_ref().is_some_and(|key| key.is_flow_collection())));

        if let (Some(colon_token), true) = (&colon_token, keep_explicit) {
            write!(
                f,
                [FormatExplicitPair {
                    question_mark_token: Some(&question_mark_token),
                    key: &key,
                    colon_token,
                    value: &value,
                    dangling_comments_of: has_dangling_comments.then(|| node.syntax()),
                }]
            )
        } else if !in_flow_mapping && value.is_none() {
            // Inside a flow sequence, `? key` is a compact single-pair
            // mapping while a bare `key` is a plain entry, so the `?` is
            // semantically significant and must be kept. With a value, the
            // pair is `key: value` either way
            write!(
                f,
                [group(&format_with(|f| {
                    write!(f, [question_mark_token.format()])?;
                    if key.is_some() || colon_token.is_some() {
                        write!(f, [space()])?;
                    }
                    write!(
                        f,
                        [align(
                            "  ",
                            &FormatImplicitEntryBody {
                                key: &key,
                                colon_token: &colon_token,
                                value: &value,
                                in_flow_mapping: false
                            }
                        )]
                    )
                }))]
            )
        } else if key.is_none() && colon_token.is_none() && value.is_none() {
            // A lone `?` denotes an entry with an empty key and an empty
            // value, which we normalize to `:`.
            write!(f, [format_replaced(&question_mark_token, &text(":", None))])
        } else {
            let body = FormatImplicitEntryBody {
                key: &key,
                colon_token: &colon_token,
                value: &value,
                in_flow_mapping,
            };
            let entry = format_with(|f| write!(f, [format_removed(&question_mark_token), body]));
            write!(f, [body.group(&entry, f)])
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowMapExplicitEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments sit between the explicit key and the `:`:
        //
        // ```yaml
        // {? key # comment
        //   : value}
        // ```
        //
        // They are printed inside `FormatExplicitPair` on their own lines before the
        // `:`; the default implementation would print them after the value
        Ok(())
    }
}

/// Formats a mapping entry in the multi-line explicit form:
///
/// ```yaml
/// ? [key, that, breaks]
/// : value
/// ```
///
pub(crate) struct FormatExplicitPair<'a> {
    /// The `?` of an entry already in the explicit form; a synthesized `?`
    /// is printed without one
    pub(crate) question_mark_token: Option<&'a YamlSyntaxToken>,
    pub(crate) key: &'a Option<AnyYamlMappingImplicitKey>,
    pub(crate) colon_token: &'a YamlSyntaxToken,
    pub(crate) value: &'a Option<AnyYamlFlowNode>,
    pub(crate) dangling_comments_of: Option<&'a YamlSyntaxNode>,
}

impl Format<YamlFormatContext> for FormatExplicitPair<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        match self.question_mark_token {
            Some(token) => write!(f, [token.format()])?,
            None => write!(f, [text("?", None)])?,
        }
        if let Some(key) = self.key {
            write!(f, [space(), align("  ", &key.format())])?;
        }
        if let Some(node) = self.dangling_comments_of {
            // Every comment is printed on a line of its own, keeping the
            // blank lines between them
            let comments = f.comments().clone();
            write!(
                f,
                [FormatCommentsSlice {
                    comments: comments.dangling_comments(node),
                    inline_first: false
                }]
            )?;
        }
        write!(f, [hard_line_break(), self.colon_token.format()])?;
        if let Some(value) = self.value {
            write!(f, [space(), align("  ", &value.format())])?;
        }
        Ok(())
    }
}

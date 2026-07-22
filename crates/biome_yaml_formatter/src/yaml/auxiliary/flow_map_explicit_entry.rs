use super::flow_map_implicit_entry::FormatImplicitEntryBody;
use crate::prelude::*;
use biome_formatter::trivia::format_dangling_comments;
use biome_formatter::{format_args, write};
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
                    question_mark_token: &question_mark_token,
                    key: &key,
                    colon_token,
                    value: &value,
                    dangling_comments_of: has_dangling_comments.then(|| node.syntax()),
                }]
            )
        } else if !in_flow_mapping {
            // Inside a flow sequence, `? key` is a compact single-pair
            // mapping while a bare `key` is a plain entry, so the `?` is
            // semantically significant and must be kept
            write!(
                f,
                [group(&format_with(|f| {
                    write!(f, [question_mark_token.format()])?;
                    if key.is_some() || colon_token.is_some() || value.is_some() {
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
            write!(
                f,
                [group(&format_args![
                    format_removed(&question_mark_token),
                    FormatImplicitEntryBody {
                        key: &key,
                        colon_token: &colon_token,
                        value: &value,
                        in_flow_mapping: true
                    }
                ])]
            )
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowMapExplicitEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // Printed in `FormatExplicitPair` between the key and the `:`
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
struct FormatExplicitPair<'a> {
    question_mark_token: &'a YamlSyntaxToken,
    key: &'a Option<AnyYamlMappingImplicitKey>,
    colon_token: &'a YamlSyntaxToken,
    value: &'a Option<AnyYamlFlowNode>,
    dangling_comments_of: Option<&'a YamlSyntaxNode>,
}

impl Format<YamlFormatContext> for FormatExplicitPair<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        write!(f, [self.question_mark_token.format()])?;
        if let Some(key) = self.key {
            write!(f, [space(), align("  ", &key.format())])?;
        }
        write!(f, [hard_line_break()])?;
        if let Some(node) = self.dangling_comments_of {
            write!(f, [format_dangling_comments(node), hard_line_break()])?;
        }
        write!(f, [self.colon_token.format()])?;
        if let Some(value) = self.value {
            write!(f, [space(), align("  ", &value.format())])?;
        }
        Ok(())
    }
}

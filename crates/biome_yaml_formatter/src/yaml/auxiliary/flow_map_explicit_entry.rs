use super::flow_map_implicit_entry::format_implicit_entry_body;
use crate::prelude::*;
use biome_formatter::trivia::format_dangling_comments;
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
        let in_flow_mapping = node
            .syntax()
            .parent()
            .is_some_and(|parent| parent.kind() == YamlSyntaxKind::YAML_FLOW_MAP_ENTRY_LIST);

        // The explicit form is required when the key is a flow collection
        // that may break across multiple lines, or when a comment sits
        // between the key and the `:`. Everywhere else Prettier prints the
        // entry in the implicit `key: value` form
        let keep_explicit = colon_token.is_some()
            && (has_dangling_comments
                || (value.is_some() && key.as_ref().is_some_and(|key| key.is_flow_collection())));

        if let (Some(colon_token), true) = (&colon_token, keep_explicit) {
            format_explicit_pair(
                Some(&question_mark_token),
                &key,
                colon_token,
                &value,
                has_dangling_comments.then(|| node.syntax()),
                f,
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
                            &format_with(|f| {
                                format_implicit_entry_body(&key, &colon_token, &value, false, f)
                            })
                        )]
                    )
                }))]
            )
        } else if key.is_none() && colon_token.is_none() && value.is_none() {
            // A lone `?` denotes an entry with an empty key and an empty
            // value, which Prettier prints as a lone `:`
            write!(f, [format_replaced(&question_mark_token, &text(":", None))])
        } else {
            write!(
                f,
                [group(&format_with(|f| {
                    write!(f, [format_removed(&question_mark_token)])?;
                    format_implicit_entry_body(&key, &colon_token, &value, true, f)
                }))]
            )
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowMapExplicitEntry,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // Printed in `fmt_fields` between the key and the `:`
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
/// When `question_mark_token` is absent (an implicit entry whose collection
/// key breaks across multiple lines, where only the explicit form is valid),
/// a `?` is synthesized, exactly like Prettier does.
pub(crate) fn format_explicit_pair(
    question_mark_token: Option<&YamlSyntaxToken>,
    key: &Option<AnyYamlMappingImplicitKey>,
    colon_token: &YamlSyntaxToken,
    value: &Option<AnyYamlFlowNode>,
    dangling_comments_of: Option<&YamlSyntaxNode>,
    f: &mut YamlFormatter,
) -> FormatResult<()> {
    match question_mark_token {
        Some(question_mark_token) => write!(f, [question_mark_token.format()])?,
        None => write!(f, [text("?", None)])?,
    }
    if let Some(key) = key {
        write!(f, [space(), align("  ", &key.format())])?;
    }
    write!(f, [hard_line_break()])?;
    if let Some(node) = dangling_comments_of {
        write!(f, [format_dangling_comments(node), hard_line_break()])?;
    }
    write!(f, [colon_token.format()])?;
    if let Some(value) = value {
        write!(f, [space(), align("  ", &value.format())])?;
    }
    Ok(())
}

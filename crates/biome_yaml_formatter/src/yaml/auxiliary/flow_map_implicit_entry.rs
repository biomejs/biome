use super::flow_map_explicit_entry::format_explicit_pair;
use crate::comments::subtree_has_comments;
use crate::prelude::*;
use biome_formatter::{FormatOptions, format_args, write};
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    AnyYamlFlowNode, AnyYamlMappingImplicitKey, YamlFlowMapImplicitEntry,
    YamlFlowMapImplicitEntryFields, YamlSyntaxKind, YamlSyntaxToken,
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

        // A collection key that breaks across multiple lines is only a valid
        // mapping key in the explicit `? key : value` form, so a `?` is
        // synthesized for it, exactly like Prettier does. Whether the key
        // breaks is approximated at build time: a comment inside it forces a
        // break, and a key longer than the line width can't possibly fit
        if let (Some(entry_key), Some(colon_token), Some(_)) = (&key, &colon_token, &value)
            && entry_key.is_flow_collection()
            && (u32::from(entry_key.syntax().text_trimmed_range().len())
                > u32::from(f.options().line_width().value())
                || subtree_has_comments(f.comments(), entry_key.syntax()))
        {
            return format_explicit_pair(None, &key, colon_token, &value, None, f);
        }

        write!(
            f,
            [group(&format_with(|f| {
                format_implicit_entry_body(&key, &colon_token, &value, in_flow_mapping, f)
            }))]
        )
    }
}

/// Formats a `key: value` entry of a flow mapping, where each of the key, the
/// `:`, and the value can individually be absent.
///
/// `in_flow_mapping` is `false` for the compact single-pair form inside a
/// flow sequence (`[a: b]`), where the `:` cannot be dropped: `[a:]` is a
/// mapping while `[a]` is a scalar.
pub(crate) fn format_implicit_entry_body(
    key: &Option<AnyYamlMappingImplicitKey>,
    colon_token: &Option<YamlSyntaxToken>,
    value: &Option<AnyYamlFlowNode>,
    in_flow_mapping: bool,
    f: &mut YamlFormatter,
) -> FormatResult<()> {
    if let Some(key) = key {
        write!(f, [key.format()])?;
    }

    let Some(colon_token) = colon_token else {
        return Ok(());
    };

    // `key:` without a value is equivalent to just `key`, which is how
    // Prettier prints it. A lone `:` stays as is
    if in_flow_mapping && key.is_some() && value.is_none() {
        return write!(f, [format_removed(colon_token)]);
    }

    // A `:` directly following an alias, anchor, or tag would be lexed as
    // part of that token, so the separating space is required
    if key.as_ref().is_some_and(needs_space_before_colon) {
        write!(f, [space()])?;
    }

    write!(f, [colon_token.format()])?;

    if let Some(value) = value {
        if !value.is_flow_collection() {
            write!(f, [space(), value.format()])?;
        } else if key.is_none() || subtree_has_comments(f.comments(), value.syntax()) {
            // With no key before it, the collection stays on the `:` line
            // even when it breaks. The same goes for a collection that a
            // comment forces to break, like Prettier does
            write!(f, [space(), align("  ", &value.format())])?;
        } else {
            // A collection value that fits stays on the key's line; one that
            // doesn't moves as a whole to its own indented line
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

/// Whether a `:` placed directly after this key would be lexed as part of
/// the key's last token. Alias, anchor, and tag tokens may all contain `:`
fn needs_space_before_colon(key: &AnyYamlMappingImplicitKey) -> bool {
    match key {
        AnyYamlMappingImplicitKey::YamlAliasNode(_) => true,
        // A node without content ends with its last property
        AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => node.content().is_none(),
        AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => node.content().is_err(),
    }
}

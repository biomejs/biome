use crate::comments::subtree_has_comments;
use crate::prelude::*;
use crate::utils::needs_space_before_colon;
use biome_formatter::{format_args, write};
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

        if let (Some(entry_key), Some(colon_token), Some(entry_value)) =
            (&key, &colon_token, &value)
            && entry_key.is_flow_collection()
        {
            return write!(
                f,
                [FormatCollectionKeyEntry {
                    key: entry_key,
                    colon_token,
                    value: entry_value
                }]
            );
        }

        write!(
            f,
            [group(&FormatImplicitEntryBody {
                key: &key,
                colon_token: &colon_token,
                value: &value,
                in_flow_mapping
            })]
        )
    }
}

/// Formats a `key: value` entry whose key is a flow collection.
///
/// A collection key that breaks across multiple lines is only a valid
/// mapping key in the explicit `? key : value` form, so a `?` is
/// synthesized for such a key:
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
/// implicit and the explicit form. A comment inside the key expands the
/// group, so it picks the explicit form as well.
struct FormatCollectionKeyEntry<'a> {
    key: &'a AnyYamlMappingImplicitKey,
    colon_token: &'a YamlSyntaxToken,
    value: &'a AnyYamlFlowNode,
}

impl Format<YamlFormatContext> for FormatCollectionKeyEntry<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let group_id = f.group_id("collection_key");
        let key_format = self.key.format().memoized();
        let colon_format = self.colon_token.format().memoized();
        let value_format = self.value.format().memoized();

        write!(
            f,
            [
                group(&format_args![
                    if_group_breaks(&format_args![text("?", None), space()]),
                    group(&align("  ", &key_format)).with_group_id(Some(group_id)),
                ]),
                if_group_breaks(&format_args![
                    // A hard line break would mark the enclosing groups as
                    // expanded even though this content is only printed when
                    // the key group breaks. A soft line break prints the same
                    // line break here, since a broken key implies that the
                    // enclosing groups are already expanded
                    soft_line_break(),
                    colon_format,
                    space(),
                    align("  ", &value_format)
                ])
                .with_group_id(Some(group_id)),
                if_group_fits_on_line(&format_with(|f| {
                    write!(f, [colon_format])?;
                    if subtree_has_comments(f.comments(), self.value.syntax()) {
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
            ]
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
        if self.key.as_ref().is_some_and(needs_space_before_colon) {
            write!(f, [space()])?;
        }

        write!(f, [colon_token.format()])?;

        // In the single-pair form inside a flow sequence, Prettier prints
        // the space that would separate the value even when there is none:
        // `[ : ]` becomes `[: ]`
        if self.value.is_none() && !self.in_flow_mapping {
            write!(f, [space()])?;
        }

        if let Some(value) = self.value {
            if !value.is_flow_collection() {
                write!(f, [space(), value.format()])?;
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

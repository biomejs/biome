use crate::prelude::*;
use biome_rowan::AstNodeList;
use biome_yaml_syntax::{
    AnyYamlMappingImplicitKey, AnyYamlProperty, YamlPropertyList, YamlSyntaxNode,
};

/// Whether a `:` placed directly after this key would be lexed as part of
/// the key's last token. Alias, anchor, and tag tokens may all contain `:`
pub(crate) fn needs_space_before_colon(key: &AnyYamlMappingImplicitKey) -> bool {
    match key {
        AnyYamlMappingImplicitKey::YamlAliasNode(_) => true,
        // A node without content ends with its last property
        AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => node.content().is_none(),
        AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => node.content().is_err(),
    }
}

/// Formats a list of node properties joined by spaces, with the tag before
/// the anchor whatever order they come in
pub(crate) struct FormatProperties<'a> {
    properties: &'a YamlPropertyList,
    /// The leading properties of the list to leave out, which belong to the
    /// enclosing block mapping
    skip: usize,
    /// The properties of a block mapping that the parser attached to the
    /// mapping's first key, with the number of them that belong here
    from_first_key: Option<(YamlPropertyList, usize)>,
}

impl<'a> FormatProperties<'a> {
    /// The properties of a flow node, minus the `skip` leading ones the
    /// enclosing block mapping prints instead
    pub(crate) fn own(properties: &'a YamlPropertyList, skip: usize) -> Self {
        Self {
            properties,
            skip,
            from_first_key: None,
        }
    }

    /// The properties of a block node, followed by the ones the parser
    /// attached to the first key of its mapping
    pub(crate) fn with_first_key(
        properties: &'a YamlPropertyList,
        from_first_key: Option<(YamlPropertyList, usize)>,
    ) -> Self {
        Self {
            properties,
            skip: 0,
            from_first_key,
        }
    }

    /// The properties to print, in source order
    fn iter(&self) -> impl Iterator<Item = AnyYamlProperty> + '_ {
        self.properties.iter().skip(self.skip).chain(
            self.from_first_key
                .iter()
                .flat_map(|(properties, count)| properties.iter().take(*count)),
        )
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}

impl Format<YamlFormatContext> for FormatProperties<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let tags = self
            .iter()
            .filter(|property| property.as_yaml_tag_property().is_some());
        let anchors = self
            .iter()
            .filter(|property| property.as_yaml_anchor_property().is_some());
        f.join_with(space())
            .entries(tags.chain(anchors).map(|property| property.into_format()))
            .finish()
    }
}

/// The number of line breaks in front of `node`.
///
/// The breaks that separate `node` from the content above it can end up in
/// the leading trivia of the zero-width end tokens (`MAPPING_END`,
/// `FLOW_END`, ...) that precede it, so the count walks through those tokens
/// as well. It stops at a comment, whose own leading breaks belong to it.
pub(crate) fn lines_before_through_end_tokens(node: &YamlSyntaxNode) -> usize {
    let mut count = 0;
    let Some(mut token) = node.first_token() else {
        return 0;
    };
    loop {
        for piece in token.leading_trivia().pieces().rev() {
            if piece.is_comments() {
                return count;
            }
            if piece.is_newline() {
                count += 1;
            }
        }
        let Some(prev) = token.prev_token() else {
            return count;
        };
        if !prev.text_trimmed().is_empty() {
            return count;
        }
        token = prev;
    }
}

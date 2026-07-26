use crate::prelude::*;
use biome_rowan::AstNodeList;
use biome_yaml_syntax::{AnyYamlMappingImplicitKey, AnyYamlProperty, YamlPropertyList};

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

    /// The property the content follows
    pub(crate) fn last(&self) -> Option<AnyYamlProperty> {
        self.iter().last()
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

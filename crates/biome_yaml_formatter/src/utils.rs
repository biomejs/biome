use crate::prelude::*;
use biome_yaml_syntax::{AnyYamlMappingImplicitKey, AnyYamlProperty, YamlSyntaxNode};

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

/// Formats a run of node properties joined by spaces, with the tag before
/// the anchor, the order Prettier normalizes properties to.
///
/// Takes the properties as a clonable iterator so call sites can pass lazy
/// `skip`/`chain` adapters over the property lists without collecting them
pub(crate) struct FormatProperties<I>(pub(crate) I);

impl<I> Format<YamlFormatContext> for FormatProperties<I>
where
    I: Iterator<Item = AnyYamlProperty> + Clone,
{
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let tags = self
            .0
            .clone()
            .filter(|property| matches!(property, AnyYamlProperty::YamlTagProperty(_)));
        let anchors = self
            .0
            .clone()
            .filter(|property| matches!(property, AnyYamlProperty::YamlAnchorProperty(_)));
        f.join_with(space())
            .entries(tags.chain(anchors).map(|property| property.into_format()))
            .finish()
    }
}

/// The number of line breaks in front of `node`, counting through the
/// zero-width end tokens (`MAPPING_END`, `FLOW_END`, ...) before it, whose
/// leading trivia carries the line breaks that separate `node` from the
/// content above. Stops at a comment, since the breaks above one belong to
/// it, not to `node`
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

use crate::content_lines::ContentLines;
use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNodeList;
use biome_yaml_syntax::{
    AnyYamlBlockHeader, AnyYamlBlockScalar, AnyYamlFlowNode, AnyYamlMappingImplicitKey,
    AnyYamlProperty, YamlPropertyList, YamlSyntaxNode, YamlSyntaxToken,
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

/// Whether the last node of `root` is a literal or folded block scalar with
/// keep chomping (`|+`). Such a scalar owns every line break that follows
/// it, so the enclosing structures print none of their own.
pub(crate) fn ends_in_keep_chomped_scalar(root: &YamlSyntaxNode) -> bool {
    let mut current = root.clone();
    while let Some(last) = current.last_child() {
        current = last;
    }
    current.ancestors().any(|ancestor| {
        AnyYamlBlockScalar::cast(ancestor).is_some_and(|scalar| {
            scalar
                .headers()
                .iter()
                .any(|header| matches!(header, AnyYamlBlockHeader::YamlBlockKeepIndicator(_)))
        })
    })
}

/// Returns the scalar token when `key` is an unqualified multiline plain
/// scalar, which only the explicit `? key : value` entry form can represent.
pub(crate) fn multiline_plain_key(key: &AnyYamlMappingImplicitKey) -> Option<YamlSyntaxToken> {
    let AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) = key else {
        return None;
    };
    if !node.properties().is_empty() {
        return None;
    }
    let token = node.content()?.value_token().ok()?;
    token.text_trimmed().contains(['\n', '\r']).then_some(token)
}

/// Formats a flow mapping entry whose key is a multiline plain scalar in the
/// explicit `? key : value` form, the only one that can hold such a key:
///
/// ```yaml
/// { ? matches
///     %
///   : 20 }
/// ```
///
/// The line breaks are literal so that they don't expand the enclosing flow
/// collection, which stays on one line; a literal break resets the printer to
/// the document root, so the continuation and `:` lines carry their own
/// indentation.
pub(crate) struct FormatMultilineKeyEntry<'a> {
    /// The `?` of an entry already in the explicit form; a synthesized `?`
    /// is printed without one
    pub(crate) question_mark_token: Option<&'a YamlSyntaxToken>,
    pub(crate) key: &'a AnyYamlMappingImplicitKey,
    pub(crate) colon_token: &'a YamlSyntaxToken,
    pub(crate) value: &'a Option<AnyYamlFlowNode>,
}

impl Format<YamlFormatContext> for FormatMultilineKeyEntry<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let Some(scalar_token) = multiline_plain_key(self.key) else {
            return Err(FormatError::SyntaxError);
        };

        // The key is written as text rather than through its formatting
        // rule, so its nodes must be marked as checked for suppression
        // comments by hand
        for node in self.key.syntax().descendants() {
            f.comments().mark_suppression_checked(&node);
        }

        match self.question_mark_token {
            Some(token) => write!(f, [token.format(), space()])?,
            None => write!(f, [text("?", None), space()])?,
        }

        let key = format_with(|f| {
            let value_text = scalar_token.text_trimmed().trim_end();
            for (index, line) in ContentLines::new(value_text).enumerate() {
                if index == 0 {
                    write!(f, [text(line.trim_end(), None)])?;
                } else {
                    write!(
                        f,
                        [
                            literal_line_break_without_parent(),
                            // Literal breaks reset indentation to the document
                            // root. Four spaces return to the scalar's column
                            // after the `{ ? ` prefix.
                            text("    ", None),
                            text(line.trim(), None)
                        ]
                    )?;
                }
            }
            Ok(())
        });
        write!(f, [format_replaced(&scalar_token, &key)])?;

        write!(
            f,
            [
                literal_line_break_without_parent(),
                // Two spaces align `:` with `?` after the `{ ` prefix.
                text("  ", None),
                self.colon_token.format()
            ]
        )?;

        if let Some(value) = self.value {
            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}

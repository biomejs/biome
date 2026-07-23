use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNodeList;
use biome_yaml_syntax::{
    AnyYamlBlockHeader, AnyYamlFlowNode, AnyYamlMappingImplicitKey, AnyYamlProperty,
    YamlFoldedScalar, YamlLiteralScalar, YamlSyntaxNode, YamlSyntaxToken,
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

/// Whether the last node of `root` is a literal or folded block scalar with
/// keep chomping (`|+`). Such a scalar owns every line break that follows
/// it, so the enclosing structures print none of their own, as Prettier
/// does
pub(crate) fn ends_in_keep_chomped_scalar(root: &YamlSyntaxNode) -> bool {
    let mut current = root.clone();
    while let Some(last) = current.last_child() {
        current = last;
    }
    current.ancestors().any(|ancestor| {
        let headers = match (
            YamlLiteralScalar::cast_ref(&ancestor),
            YamlFoldedScalar::cast_ref(&ancestor),
        ) {
            (Some(scalar), _) => scalar.headers(),
            (_, Some(scalar)) => scalar.headers(),
            _ => return false,
        };
        headers
            .iter()
            .any(|header| matches!(header, AnyYamlBlockHeader::YamlBlockKeepIndicator(_)))
    })
}

/// The value token of a key that is a plain scalar spanning multiple lines,
/// which only the explicit `? key : value` entry form can represent
pub(crate) fn multiline_plain_key_token(
    key: Option<&AnyYamlMappingImplicitKey>,
) -> Option<YamlSyntaxToken> {
    let AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) = key? else {
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
/// The line breaks are literal so the enclosing flow collection stays flat,
/// as Prettier keeps it; a literal break resets the printer to the document
/// root, so the continuation and `:` lines carry their own indentation.
pub(crate) struct FormatMultilineKeyEntry<'a> {
    /// The `?` of an entry already in the explicit form; a synthesized `?`
    /// is printed without one
    pub(crate) question_mark_token: Option<&'a YamlSyntaxToken>,
    pub(crate) key: &'a AnyYamlMappingImplicitKey,
    pub(crate) key_token: &'a YamlSyntaxToken,
    pub(crate) colon_token: &'a YamlSyntaxToken,
    pub(crate) value: &'a Option<AnyYamlFlowNode>,
}

impl Format<YamlFormatContext> for FormatMultilineKeyEntry<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
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
            let value_text = self.key_token.text_trimmed().trim_end();
            for (index, line) in value_text.lines().enumerate() {
                if index == 0 {
                    write!(f, [text(line.trim_end(), None)])?;
                } else {
                    let line = std::format!("    {}", line.trim());
                    write!(f, [literal_line_break_without_parent(), text(&line, None)])?;
                }
            }
            Ok(())
        });
        write!(f, [format_replaced(self.key_token, &key)])?;

        write!(
            f,
            [
                literal_line_break_without_parent(),
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

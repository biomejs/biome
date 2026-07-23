use crate::prelude::*;
use crate::utils::{ends_in_keep_chomped_scalar, lines_before_through_end_tokens};
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlDocument, YamlDocumentList, YamlSyntaxKind};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocumentList;
impl FormatRule<YamlDocumentList> for FormatYamlDocumentList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlDocumentList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut previous: Option<YamlDocument> = None;

        for document in node {
            if let Some(previous) = &previous {
                let lines = lines_before_through_end_tokens(document.syntax());

                if previous.dotdotdot_token().is_none()
                    && ends_in_keep_chomped_scalar(previous.syntax())
                {
                    // A keep-chomped scalar owns every line break between it
                    // and the next document's marker, so each one is kept.
                    // The count includes the break that closes the scalar's
                    // token, which its content doesn't print
                    let trailing = usize::from(scalar_ends_with_break(previous));
                    for _ in 1..(lines + trailing) {
                        write!(f, [text("\n", None)])?;
                    }
                    write!(f, [hard_line_break()])?;
                } else if lines > 1 && ends_in_multi_entry_collection(previous) {
                    // A blank line separating a document from the next `---`
                    // marker survives when the entries above it read as a
                    // list: Prettier keeps it only when the collection
                    // holding the previous document's last entry has at
                    // least two entries
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
            }

            write!(f, [document.format()])?;
            previous = document.as_yaml_document().cloned();
        }

        Ok(())
    }
}

/// Whether the block collection holding the last entry of the document has
/// two or more entries
fn ends_in_multi_entry_collection(document: &YamlDocument) -> bool {
    let Some(node) = document.node() else {
        return false;
    };
    let mut current = node.syntax().clone();
    while let Some(last) = current.last_child() {
        current = last;
    }
    current
        .ancestors()
        .find_map(|ancestor| {
            matches!(
                ancestor.kind(),
                YamlSyntaxKind::YAML_BLOCK_MAP_ENTRY_LIST
                    | YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY_LIST
            )
            .then(|| ancestor.children().count())
        })
        .is_some_and(|entries| entries >= 2)
}

/// Whether the last content token of the document ends with a line break of
/// its own, as the token of a block scalar can
fn scalar_ends_with_break(document: &YamlDocument) -> bool {
    let mut token = document.syntax().last_token();
    while let Some(current) = &token {
        if !current.text_trimmed().is_empty() {
            return current.text_trimmed().ends_with(['\n', '\r']);
        }
        token = current.prev_token();
    }
    false
}

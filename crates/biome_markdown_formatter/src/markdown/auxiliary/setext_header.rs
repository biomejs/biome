use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdInline, MdSetextHeader, MdSetextHeaderFields};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdSetextHeader;
impl FormatNodeRule<MdSetextHeader> for FormatMdSetextHeader {
    fn fmt_fields(&self, node: &MdSetextHeader, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdSetextHeaderFields {
            content,
            underline_token,
        } = node.as_fields();

        let underline_token = underline_token?;

        // YAML front matter: `---\nkey: value\n---` is parsed as a thematic break
        // followed by a setext header. Don't normalize the setext header in that case.
        if underline_token.text().trim_start().starts_with('-') && is_yaml_front_matter(node) {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let hashes = if underline_token.text().trim_start().starts_with('=') {
            "#"
        } else {
            "##"
        };

        write!(f, [token(hashes), space()])?;

        // Format content items manually. The trailing newline (before the underline) is
        // removed because the MdNewline block node after the underline provides the line ending.
        // Leading whitespace-only MdTextual nodes are skipped (the lexer emits one
        // MD_TEXTUAL_LITERAL per space at line start, so "   Foo" = [" ", " ", " ", "Foo"]).
        let items: Vec<_> = content.iter().collect();
        let last_idx = items.len().saturating_sub(1);
        let mut skip_leading_spaces = true;
        for (i, item) in items.iter().enumerate() {
            let is_last = i == last_idx;
            if skip_leading_spaces
                && let AnyMdInline::MdTextual(text) = item
                && text
                    .value_token()?
                    .text()
                    .chars()
                    .all(|c| c == ' ' || c == '\t')
            {
                write!(
                    f,
                    [text.format().with_options(FormatMdTextualOptions {
                        should_remove: true,
                        trim_start: false,
                        ..Default::default()
                    })]
                )?;
                continue;
            }
            if skip_leading_spaces {
                skip_leading_spaces = false;
            }
            match item {
                AnyMdInline::MdTextual(text) if is_last && text.is_newline()? => {
                    write!(
                        f,
                        [text.format().with_options(FormatMdTextualOptions {
                            should_remove: true,
                            trim_start: false,
                            ..Default::default()
                        })]
                    )?;
                }
                _ => write!(f, [item.format()])?,
            }
        }

        write!(f, [format_removed(&underline_token)])
    }
}

/// Check if this setext header is part of YAML front matter. YAML front matter
/// is `---\n...\n---` at the document start. The parser sees this as a thematic
/// break (`---`) followed by a setext header (content + `---` underline).
fn is_yaml_front_matter(node: &MdSetextHeader) -> bool {
    use biome_markdown_syntax::MarkdownSyntaxKind;
    // Walk backwards over MdNewline siblings to find a thematic break.
    let mut sibling = node.syntax().prev_sibling();
    while let Some(s) = &sibling {
        if s.kind() == MarkdownSyntaxKind::MD_NEWLINE {
            sibling = s.prev_sibling();
            continue;
        }
        return s.kind() == MarkdownSyntaxKind::MD_THEMATIC_BREAK_BLOCK;
    }
    false
}

use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdFencedCodeBlock, MdFencedCodeBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFencedCodeBlock;
impl FormatNodeRule<MdFencedCodeBlock> for FormatMdFencedCodeBlock {
    fn fmt_fields(&self, node: &MdFencedCodeBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdFencedCodeBlockFields {
            indent: _,
            l_fence,
            code_list: _,
            content: _,
            r_fence_indent: _,
            r_fence,
        } = node.as_fields();

        let (l_fence, r_fence) = match (l_fence, r_fence) {
            (Ok(l), Ok(r)) => (l, r),
            _ => return format_verbatim_node(node.syntax()).fmt(f),
        };

        let l_fence_text = l_fence.text_trimmed();
        let fence_char = l_fence_text.chars().next().unwrap_or('`');
        let l_len = l_fence_text.len();
        let r_len = r_fence.text_trimmed().len();

        // Prettier always normalizes fences to backticks.
        let target_char = '`';

        // Scan tokens between l_fence and r_fence for the longest run of the target_char.
        let r_start = r_fence.text_range().start();
        let mut max_run = 0usize;
        let mut tok = l_fence.next_token();
        while let Some(t) = tok {
            if t.text_range().start() >= r_start {
                break;
            }
            let run = max_fence_char_run(t.text(), target_char);
            if run > max_run {
                max_run = run;
            }
            tok = t.next_token();
        }

        // Minimum fence length: at least 3, and strictly greater than any run in content.
        let target_len = std::cmp::max(3, max_run + 1);

        if fence_char == target_char && l_len == target_len && r_len == target_len {
            // Already normalized — emit the entire block verbatim.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        // Need to replace fences. Emit the inner content as raw text so that
        // code indentation is preserved exactly.
        let new_fence: String = target_char.to_string().repeat(target_len);
        let l_pos = l_fence.text_trimmed_range().start();
        let r_pos = r_fence.text_trimmed_range().start();

        // Collect the raw source text between l_fence and r_fence.
        let mut inner = String::new();
        let inner_pos = l_fence.text_range().end();
        let mut tok = l_fence.next_token();
        while let Some(t) = tok {
            if t.text_range().start() >= r_start {
                break;
            }
            inner.push_str(t.text());
            f.state_mut().track_token(&t);
            tok = t.next_token();
        }

        // Mark all non-token nodes in the content as suppression-checked.
        mark_suppression_checked_all(node, f);

        write!(
            f,
            [
                format_replaced(&l_fence, &biome_formatter::prelude::text(&new_fence, l_pos)),
                biome_formatter::prelude::text(&inner, inner_pos),
                format_replaced(&r_fence, &biome_formatter::prelude::text(&new_fence, r_pos)),
            ]
        )
    }
}

/// Walk all descendant nodes and mark them as suppression-checked.
fn mark_suppression_checked_all(node: &MdFencedCodeBlock, f: &mut MarkdownFormatter) {
    use biome_rowan::{Direction, SyntaxElement};
    let comments = f.context().comments();
    for element in node.syntax().descendants_with_tokens(Direction::Next) {
        if let SyntaxElement::Node(n) = element {
            comments.mark_suppression_checked(&n);
        }
    }
}

/// Maximum consecutive run of `fence_char` in `s`.
fn max_fence_char_run(s: &str, fence_char: char) -> usize {
    let mut max_run = 0usize;
    let mut current = 0usize;
    for c in s.chars() {
        if c == fence_char {
            current += 1;
            if current > max_run {
                max_run = current;
            }
        } else {
            current = 0;
        }
    }
    max_run
}

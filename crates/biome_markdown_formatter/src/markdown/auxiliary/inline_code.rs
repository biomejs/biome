use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineCode, MdInlineCodeFields};
use biome_rowan::{AstNode, Direction, SyntaxElement};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineCode;
impl FormatNodeRule<MdInlineCode> for FormatMdInlineCode {
    fn fmt_fields(&self, node: &MdInlineCode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineCodeFields {
            l_tick_token,
            content: _,
            r_tick_token,
        } = node.as_fields();

        let (l_tick, r_tick) = match (l_tick_token, r_tick_token) {
            (Ok(l), Ok(r)) => (l, r),
            _ => return format_verbatim_node(node.syntax()).fmt(f),
        };

        let current_len = l_tick.text_trimmed().len();

        // Collect all exact backtick-run lengths in the content.
        let r_start = r_tick.text_range().start();
        let mut runs: BTreeSet<usize> = BTreeSet::new();
        let mut tok = l_tick.next_token();
        while let Some(t) = tok {
            if t.text_range().start() >= r_start {
                break;
            }
            collect_backtick_runs(t.text(), &mut runs);
            tok = t.next_token();
        }

        // Smallest N >= 1 not present as a run of exactly N backticks.
        let min_len = min_backtick_fence(&runs);

        if min_len == current_len {
            // No change needed — emit verbatim to preserve all whitespace exactly.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let new_fence: String = "`".repeat(min_len);
        let l_pos = l_tick.text_trimmed_range().start();
        let r_pos = r_tick.text_trimmed_range().start();

        // Collect the raw source text between the ticks to preserve trailing spaces.
        let inner_pos = l_tick.text_range().end();
        let mut inner = String::new();
        let mut tok = l_tick.next_token();
        while let Some(t) = tok {
            if t.text_range().start() >= r_start {
                break;
            }
            inner.push_str(t.text());
            f.state_mut().track_token(&t);
            tok = t.next_token();
        }

        // Mark all descendant nodes as suppression-checked.
        let comments = f.context().comments();
        for element in node.syntax().descendants_with_tokens(Direction::Next) {
            if let SyntaxElement::Node(n) = element {
                comments.mark_suppression_checked(&n);
            }
        }

        write!(
            f,
            [
                format_replaced(&l_tick, &biome_formatter::prelude::text(&new_fence, l_pos)),
                biome_formatter::prelude::text(&inner, inner_pos),
                format_replaced(&r_tick, &biome_formatter::prelude::text(&new_fence, r_pos)),
            ]
        )
    }
}

/// Scan `s` and insert the length of every maximal run of `` ` `` characters.
fn collect_backtick_runs(s: &str, runs: &mut BTreeSet<usize>) {
    let mut current = 0usize;
    for c in s.chars() {
        if c == '`' {
            current += 1;
        } else if current > 0 {
            runs.insert(current);
            current = 0;
        }
    }
    if current > 0 {
        runs.insert(current);
    }
}

/// Smallest N >= 1 not in `runs`.
fn min_backtick_fence(runs: &BTreeSet<usize>) -> usize {
    let mut n = 1;
    while runs.contains(&n) {
        n += 1;
    }
    n
}

//! Tailwind v4 class sorter built on top of `biome_tailwind_parser`.
//!
//! Phase 1 stub: walks the parsed candidates and re-emits them in source
//! order so the surrounding pipeline (rule wiring, fixture infra,
//! snapshot review) can be exercised end-to-end before the real sort
//! algorithm lands.

use biome_rowan::AstNode;
use biome_tailwind_syntax::{AnyTwFullCandidate, TwRoot};

/// Sort the candidates of a parsed Tailwind class list and return the joined,
/// space-separated result.
///
/// Stub: returns each candidate's source text in input order. The real
/// implementation will replace the body with the v4 sort algorithm while
/// keeping the same signature.
pub fn sort_class_list(root: &TwRoot) -> String {
    let mut result = String::new();
    for candidate in root.candidates() {
        let text = match candidate {
            AnyTwFullCandidate::TwBogusCandidate(node) => node.syntax().text_trimmed(),
            AnyTwFullCandidate::TwFullCandidate(node) => node.syntax().text_trimmed(),
        };
        if !result.is_empty() {
            result.push(' ');
        }
        text.for_each_chunk(|chunk| result.push_str(chunk));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::sort_class_list;
    use biome_tailwind_parser::parse_tailwind;
    use biome_test_utils::scripts_from_json;

    #[test]
    fn fixtures() {
        insta::glob!("sort_v4_fixtures/*.jsonc", |path| {
            let raw = std::fs::read_to_string(path).expect("read fixture");
            let cases = scripts_from_json("jsonc", &raw).expect("parse jsonc array");
            let rendered = cases
                .iter()
                .map(|input| {
                    let parsed = parse_tailwind(input);
                    let sorted = sort_class_list(&parsed.tree());
                    format!("input:  {input}\nsorted: {sorted}")
                })
                .collect::<Vec<_>>()
                .join("\n---\n");
            insta::with_settings!(
                {
                    snapshot_path => "sort_v4_fixtures/snapshots",
                    prepend_module_to_snapshot => false,
                },
                { insta::assert_snapshot!(rendered) }
            );
        });
    }
}

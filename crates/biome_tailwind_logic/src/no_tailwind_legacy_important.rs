//! Finds classes that use Tailwind CSS v3's leading important modifier, as in
//! `!flex`. Tailwind CSS v4 writes the modifier at the end of the class
//! (`flex!`) and only accepts the leading form for compatibility.

use biome_rowan::{AstNode, AstSeparatedList, Direction};
use biome_tailwind_syntax::{AnyTwFullCandidate, TwCandidateList, TwFullCandidate, TwRoot};
use smallvec::SmallVec;

/// Returns the candidates in `candidates` that start their utility with `!`.
pub fn legacy_important_candidates(candidates: &TwCandidateList) -> SmallVec<[TwFullCandidate; 2]> {
    candidates
        .iter()
        .flatten()
        .filter_map(|candidate| match candidate {
            AnyTwFullCandidate::TwFullCandidate(candidate)
                if candidate.legacy_important_token().is_some() =>
            {
                Some(candidate)
            }
            _ => None,
        })
        .collect()
}

/// Returns the class string of `root` with the `!` of `candidate` moved to
/// the end of the class.
///
/// Returns `None` if `candidate` doesn't have a leading `!`.
pub fn fixed_class_string(root: &TwRoot, candidate: &TwFullCandidate) -> Option<String> {
    let text = root.syntax().text_with_trivia().to_string();
    let important = candidate.legacy_important_token()?.text_trimmed_range();
    let end = usize::from(candidate.range().end());
    let mut fixed = String::with_capacity(text.len());
    fixed.push_str(text.get(..usize::from(important.start()))?);
    fixed.push_str(text.get(usize::from(important.end())..end)?);
    fixed.push('!');
    fixed.push_str(text.get(end..)?);
    Some(fixed)
}

/// Writes a class with its leading `!` moved to the end, as in `hover:flex!`
/// for `hover:!flex`.
pub struct TrailingImportantClass<'a>(pub &'a TwFullCandidate);

impl biome_console::fmt::Display for TrailingImportantClass<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        let important = self.0.legacy_important_token();
        for token in self.0.syntax().descendants_tokens(Direction::Next) {
            if Some(&token) != important.as_ref() {
                fmt.write_str(token.text_trimmed())?;
            }
        }
        fmt.write_str("!")
    }
}

#[cfg(test)]
mod tests {
    use super::{fixed_class_string, legacy_important_candidates};
    use biome_tailwind_parser::parse_tailwind;

    #[test]
    fn moves_important_to_the_end() {
        for (source, expected) in [
            ("!flex", "flex!"),
            ("hover:!flex", "hover:flex!"),
            ("md:hover:!bg-red-500/50", "md:hover:bg-red-500/50!"),
            ("!-m-4", "-m-4!"),
            ("![color:red]", "[color:red]!"),
            ("!bg-(--brand)", "bg-(--brand)!"),
            ("p-4 !flex text-red-500", "p-4 flex! text-red-500"),
        ] {
            let parse = parse_tailwind(source);
            assert!(!parse.has_errors(), "{source}: {:?}", parse.diagnostics());
            let root = parse.tree();
            let found = legacy_important_candidates(&root.candidates());
            assert_eq!(found.len(), 1, "{source}");
            assert_eq!(
                fixed_class_string(&root, &found[0]).as_deref(),
                Some(expected),
                "{source}"
            );
        }
    }

    #[test]
    fn ignores_trailing_important() {
        for source in ["flex!", "hover:flex!", "-m-4!", "[color:red]!", "flex"] {
            let parse = parse_tailwind(source);
            assert!(!parse.has_errors(), "{source}: {:?}", parse.diagnostics());
            assert!(
                legacy_important_candidates(&parse.tree().candidates()).is_empty(),
                "{source}"
            );
        }
    }
}

use std::hash::Hash;

use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, Direction, TokenText, WalkEvent,
};
use biome_tailwind_factory::make;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwModifier, AnyTwValue, TailwindLanguage, TailwindSyntaxKind,
    TailwindSyntaxNode, TailwindSyntaxToken, TwCandidateList, TwFullCandidate, TwRoot,
    TwVariantList, is_node_equal,
};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct TailwindShorthandViolation {
    pub uncompressed_nodes: Vec<TwFullCandidate>,
    pub replacement_bases: &'static [&'static str],
    /// When true, the action replaces the entire `TwFullCandidate` node rather
    /// than just its base token. This is needed when the input candidates are
    /// functional (base + value) but the replacement is a static class name.
    pub replace_whole_node: bool,
}

#[derive(Debug, Clone, Eq)]
struct GroupKey {
    variants: TwVariantList,
    negative: bool,
    important: bool,
    value: Option<AnyTwValue>,
    modifier: Option<AnyTwModifier>,
}

#[derive(Debug, Default)]
struct GroupCandidates {
    candidates: Vec<TwFullCandidate>,
    by_base: FxHashMap<TokenText, Vec<usize>>,
}

impl PartialEq for GroupKey {
    fn eq(&self, other: &Self) -> bool {
        let value_equal = match (&self.value, &other.value) {
            (Some(a), Some(b)) => is_node_equal(a.syntax(), b.syntax()),
            (None, None) => true,
            _ => return false,
        };
        let modifier_equal = match (&self.modifier, &other.modifier) {
            (Some(a), Some(b)) => is_node_equal(a.syntax(), b.syntax()),
            (None, None) => true,
            _ => return false,
        };

        self.negative == other.negative
            && self.important == other.important
            && value_equal
            && modifier_equal
            && is_node_equal(self.variants.syntax(), other.variants.syntax())
    }
}

impl std::hash::Hash for GroupKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_node(self.variants.syntax(), state);
        self.negative.hash(state);
        self.important.hash(state);
        if let Some(ref value) = self.value {
            hash_node(value.syntax(), state);
        }
        if let Some(ref modifier) = self.modifier {
            hash_node(modifier.syntax(), state);
        }
    }
}

fn hash_node<H: std::hash::Hasher>(node: &TailwindSyntaxNode, state: &mut H) {
    for event in node.preorder_with_tokens(Direction::Next) {
        match event {
            WalkEvent::Enter(element) => {
                element.kind().hash(state);
                if let Some(token) = element.as_token() {
                    token.text_trimmed().hash(state);
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }
}

/// Returns `true` if `full` is a functional candidate whose base token matches
/// `base` and whose value text matches `value`, without allocating any strings.
fn candidate_matches(full: &TwFullCandidate, base: &str, value: &str) -> bool {
    match full.candidate().ok() {
        Some(AnyTwCandidate::TwFunctionalCandidate(func)) => {
            func.base_token()
                .ok()
                .is_some_and(|t| t.text_trimmed() == base)
                && func
                    .value()
                    .ok()
                    .is_some_and(|v| v.syntax().text_trimmed() == value)
        }
        Some(AnyTwCandidate::TwStaticCandidate(st)) => {
            // Static candidates have no value part; match when `value` is empty.
            value.is_empty()
                && st
                    .base_token()
                    .ok()
                    .is_some_and(|t| t.text_trimmed() == base)
        }
        _ => false,
    }
}

/// Special-case detection for `overflow-hidden text-ellipsis whitespace-nowrap` → `truncate`.
///
/// These three classes cannot be handled by the general value-grouping logic
/// because each has a different value (`hidden`, `ellipsis`, `nowrap`), so
/// they never end up in the same group.
fn check_truncate_shorthand(candidates: &TwCandidateList) -> Option<TailwindShorthandViolation> {
    /// Each tuple is `(base, value)` for a functional candidate that forms part
    /// of the `truncate` shorthand.  Using `(base, value)` pairs avoids any
    /// string allocation during matching.
    const TRUNCATE_PARTS: &[(&str, &str)] = &[
        ("overflow", "hidden"),
        ("text", "ellipsis"),
        ("whitespace", "nowrap"),
    ];

    // Find the first part to establish the variants/negative/important context.
    let (first_base, first_value) = TRUNCATE_PARTS[0];
    let first = candidates
        .iter()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
        .find(|candidate| {
            candidate_matches(candidate, first_base, first_value)
                && candidate.negative_token().is_none()
                && candidate.excl_token().is_none()
        })?;

    let first_variants = first.variants();

    let mut matched: Vec<TwFullCandidate> = Vec::with_capacity(TRUNCATE_PARTS.len());
    for &(base, value) in TRUNCATE_PARTS {
        let candidate = candidates
            .iter()
            .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
            .find(|candidate| {
                candidate_matches(candidate, base, value)
                    && candidate.negative_token().is_none()
                    && candidate.excl_token().is_none()
                    && is_node_equal(candidate.variants().syntax(), first_variants.syntax())
            })?;
        matched.push(candidate);
    }

    Some(TailwindShorthandViolation {
        uncompressed_nodes: matched,
        replacement_bases: &["truncate"],
        replace_whole_node: true,
    })
}

pub fn analyze_tailwind_shorthand(candidates: &TwCandidateList) -> Vec<TailwindShorthandViolation> {
    fn extract_key_and_base(full: &TwFullCandidate) -> Option<(GroupKey, TokenText)> {
        let variants = full.variants();
        let negative = full.negative_token().is_some();
        let important = full.excl_token().is_some();
        let candidate = full.candidate().ok()?;
        match candidate {
            AnyTwCandidate::TwFunctionalCandidate(func) => {
                let base = func.base_token().ok()?.token_text_trimmed();
                let value = func.value().ok()?;
                let modifier = func.modifier();
                Some((
                    GroupKey {
                        variants,
                        negative,
                        important,
                        value: Some(value),
                        modifier,
                    },
                    base,
                ))
            }
            AnyTwCandidate::TwStaticCandidate(st) => {
                let base = st.base_token().ok()?.token_text_trimmed();
                Some((
                    GroupKey {
                        variants,
                        negative,
                        important,
                        value: None,
                        modifier: None,
                    },
                    base,
                ))
            }
            _ => None,
        }
    }

    if candidates.len() < 2 {
        // can't possibly compress anything if there isn't 2 candidates
        return vec![];
    }

    // build a map to make candidate lookups faster
    let mut groups: FxHashMap<GroupKey, GroupCandidates> = FxHashMap::default();
    for candidate in candidates
        .iter()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
    {
        let Some((key, base)) = extract_key_and_base(&candidate) else {
            continue;
        };
        let group = groups.entry(key).or_default();
        let candidate_index = group.candidates.len();
        group.candidates.push(candidate);
        group.by_base.entry(base).or_default().push(candidate_index);
    }

    let mut violations: Vec<TailwindShorthandViolation> = Vec::new();

    for pattern_group in TW_COMPRESSABLES {
        for (key, group) in &groups {
            let candidates = &group.candidates;
            let candidates_by_base = &group.by_base;
            let mut used_candidates = FxHashSet::default();

            for (required_bases, replacement_bases) in *pattern_group {
                // Special case: `w`/`h` → `size` must skip values that `size` doesn't support
                // (e.g. `screen`, container sizes like `xs`/`sm`/`md`/...).
                let is_size_pattern = *required_bases == ["w", "h"];

                if candidates.len() < required_bases.len() {
                    // Not enough candidates to match the required bases
                    continue;
                }

                // Skip if the value is a keyword that `size` doesn't support
                if is_size_pattern
                    && let Some(value) = &key.value
                    && let Some(value) = value.as_tw_named_value()
                    && let Some(value) = value.value_token().ok()
                    && SIZE_BLOCKED_VALUES.contains(value.text_trimmed())
                {
                    continue;
                }

                // Check if all required bases are present
                let mut found_all = true;
                let mut flagged_candidates = Vec::with_capacity(required_bases.len());
                for &rb in *required_bases {
                    let Some(candidate) =
                        candidates_by_base.get(rb).and_then(|candidate_indexes| {
                            candidate_indexes.iter().find_map(|&index| {
                                let candidate = &candidates[index];
                                (!used_candidates.contains(&candidate.range())).then_some(candidate)
                            })
                        })
                    else {
                        found_all = false;
                        break;
                    };
                    flagged_candidates.push((*candidate).clone());
                }
                if !found_all {
                    continue;
                }
                debug_assert!(
                    flagged_candidates.len() == required_bases.len(),
                    "should have found a candidate for each required base -- \n Group Key: {:#?}\n{} candidates: {:?}, required_bases: {:?}, replacement_bases: {:?}\noriginal strings: {:?}",
                    key,
                    flagged_candidates.len(),
                    flagged_candidates
                        .iter()
                        .map(|c| c.candidate().ok().and_then(|candidate| {
                            if let Some(func) = candidate.as_tw_functional_candidate() {
                                func.base_token().ok().map(|t| t.text().to_string())
                            } else if let Some(st) = candidate.as_tw_static_candidate() {
                                st.base_token().ok().map(|t| t.text().to_string())
                            } else {
                                None
                            }
                        }))
                        .collect::<Vec<_>>(),
                    required_bases,
                    replacement_bases,
                    flagged_candidates
                        .iter()
                        .map(|c| c.syntax().to_string())
                        .collect::<Vec<_>>()
                );
                // If the shorthand is already present in the class list, just
                // remove the longhands rather than adding a duplicate shorthand.
                let effective_replacement_bases: &[&str] = if replacement_bases
                    .iter()
                    .any(|&rb| candidates_by_base.contains_key(rb))
                {
                    &[]
                } else {
                    replacement_bases
                };

                used_candidates.extend(flagged_candidates.iter().map(AstNode::range));
                violations.push(TailwindShorthandViolation {
                    uncompressed_nodes: flagged_candidates,
                    replacement_bases: effective_replacement_bases,
                    replace_whole_node: false,
                });
            }
        }
    }

    // Special case: `overflow-hidden text-ellipsis whitespace-nowrap` → `truncate`.
    // These classes cannot be handled by the general value-grouping logic because
    // each has a different value, so they never end up in the same group.
    if let Some(violation) = check_truncate_shorthand(candidates) {
        violations.push(violation);
    }

    violations
}

/// Create a batch mutation that fixes a single shorthand violation.
pub fn auto_fix(
    root: &TwRoot,
    state: &TailwindShorthandViolation,
) -> Option<BatchMutation<TailwindLanguage>> {
    let mut mutation = root.clone().begin();
    apply_auto_fix(&mut mutation, state)?;

    Some(mutation)
}

pub fn auto_fix_all(
    root: &TwRoot,
    states: &[TailwindShorthandViolation],
) -> Option<BatchMutation<TailwindLanguage>> {
    let mut mutation = root.clone().begin();
    for state in states {
        apply_auto_fix(&mut mutation, state)?;
    }

    Some(mutation)
}

fn apply_auto_fix(
    mutation: &mut BatchMutation<TailwindLanguage>,
    state: &TailwindShorthandViolation,
) -> Option<()> {
    let mut old_candidates = state.uncompressed_nodes.iter().rev();
    for replacement_base in state.replacement_bases {
        let Some(to_modify) = old_candidates.next() else {
            break;
        };
        if state.replace_whole_node {
            // The replacement is a static class (e.g. `truncate`), but the candidate
            // being replaced is a functional candidate (e.g. `whitespace-nowrap`).
            // We must replace the entire TwFullCandidate node, not just the base token,
            // because changing the node kind from functional to static requires
            // rebuilding the whole subtree.
            let new_base_token = TailwindSyntaxToken::new_detached(
                TailwindSyntaxKind::TW_BASE,
                replacement_base,
                [],
                [],
            );
            let new_static = make::tw_static_candidate(new_base_token);
            let new_full = make::tw_full_candidate(
                to_modify.variants(),
                AnyTwCandidate::TwStaticCandidate(new_static),
            )
            .build();
            mutation.replace_node(to_modify.clone(), new_full);
        } else {
            match to_modify.candidate().ok()? {
                AnyTwCandidate::TwFunctionalCandidate(func) => {
                    // Functional candidate: replace just the base token.
                    // The `-value` part (e.g. `-4` in `border-x-4`) stays intact.
                    let base_token = func.base_token().ok()?;
                    mutation.replace_token(
                        base_token,
                        TailwindSyntaxToken::new_detached(
                            TailwindSyntaxKind::TW_BASE,
                            replacement_base,
                            [],
                            [],
                        ),
                    );
                }
                AnyTwCandidate::TwStaticCandidate(_) => {
                    // Static candidate (e.g. bare `border-x` with no value):
                    // replace the whole node since there is no value slot to keep.
                    let new_base = TailwindSyntaxToken::new_detached(
                        TailwindSyntaxKind::TW_BASE,
                        replacement_base,
                        [],
                        [],
                    );
                    let new_static = make::tw_static_candidate(new_base);
                    let new_full = make::tw_full_candidate(
                        to_modify.variants(),
                        AnyTwCandidate::TwStaticCandidate(new_static),
                    )
                    .build();
                    mutation.replace_node(to_modify.clone(), new_full);
                }
                _ => return None,
            }
        }
    }

    for to_remove in old_candidates {
        mutation.remove_node(to_remove.clone());
    }

    Some(())
}

pub static TW_COMPRESSABLES: &[&[(&[&str], &[&str])]] = &[
    &[(&["w", "h"], &["size"])],
    &[
        (&["ml", "mr", "mt", "mb"], &["m"]),
        (&["mx", "my"], &["m"]),
        (&["ms", "me"], &["mx"]),
        (&["ml", "mr"], &["mx"]),
        (&["mt", "mb"], &["my"]),
    ],
    &[
        (&["pl", "pr", "pt", "pb"], &["p"]),
        (&["px", "py"], &["p"]),
        (&["ps", "pe"], &["px"]),
        (&["pl", "pr"], &["px"]),
        (&["pt", "pb"], &["py"]),
    ],
    &[
        (
            &["border-t", "border-b", "border-l", "border-r"],
            &["border"],
        ),
        (&["border-x", "border-y"], &["border"]),
        (&["border-s", "border-e"], &["border-x"]),
        (&["border-l", "border-r"], &["border-x"]),
        (&["border-t", "border-b"], &["border-y"]),
    ],
    &[(
        &["border-spacing-x", "border-spacing-y"],
        &["border-spacing"],
    )],
    &[
        (
            &["rounded-tl", "rounded-tr", "rounded-bl", "rounded-br"],
            &["rounded"],
        ),
        (&["rounded-t", "rounded-b"], &["rounded"]),
        (&["rounded-l", "rounded-r"], &["rounded"]),
        (&["rounded-tl", "rounded-tr"], &["rounded-t"]),
        (&["rounded-bl", "rounded-br"], &["rounded-b"]),
        (&["rounded-tl", "rounded-bl"], &["rounded-l"]),
        (&["rounded-tr", "rounded-br"], &["rounded-r"]),
    ],
    &[
        (
            &["scroll-mt", "scroll-mb", "scroll-ml", "scroll-mr"],
            &["scroll-m"],
        ),
        (&["scroll-mx", "scroll-my"], &["scroll-m"]),
        (&["scroll-ms", "scroll-me"], &["scroll-mx"]),
        (&["scroll-ml", "scroll-mr"], &["scroll-mx"]),
        (&["scroll-mt", "scroll-mb"], &["scroll-my"]),
    ],
    &[
        (
            &["scroll-pt", "scroll-pb", "scroll-pl", "scroll-pr"],
            &["scroll-p"],
        ),
        (&["scroll-px", "scroll-py"], &["scroll-p"]),
        (&["scroll-pl", "scroll-pr"], &["scroll-px"]),
        (&["scroll-ps", "scroll-pe"], &["scroll-px"]),
        (&["scroll-pt", "scroll-pb"], &["scroll-py"]),
    ],
    &[
        (&["top", "right", "bottom", "left"], &["inset"]),
        (&["right", "left"], &["inset-x"]),
        (&["bottom", "top"], &["inset-y"]),
        (&["inset-x", "inset-y"], &["inset"]),
    ],
    &[(&["divide-x", "divide-y"], &["divide"])],
    &[(&["space-x", "space-y"], &["space"])],
    &[(&["gap-x", "gap-y"], &["gap"])],
    &[(&["translate-x", "translate-y"], &["translate"])],
    &[(&["rotate-x", "rotate-y"], &["rotate"])],
    &[(&["skew-x", "skew-y"], &["skew"])],
    &[
        (&["scale-x", "scale-y", "scale-z"], &["scale", "scale-3d"]),
        (&["scale-x", "scale-y"], &["scale"]),
    ],
    &[
        (&["content", "justify-content"], &["place-content"]),
        (&["items", "justify-items"], &["place-items"]),
        (&["self", "justify-self"], &["place-self"]),
    ],
];

/// Values supported by `w`/`h` but NOT by `size`.
/// These keyword values exist as width/height utilities but have no `size-*` equivalent,
/// so `w-screen h-screen` should NOT be compressed to `size-screen`.
static SIZE_BLOCKED_VALUES: phf::Set<&'static str> = phf::phf_set! {
    "screen", "3xs", "2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl",
};

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use biome_rowan::AstNode;
    use biome_tailwind_parser::parse_tailwind;
    use insta::assert_snapshot;

    use super::{analyze_tailwind_shorthand, auto_fix_all};

    const INVALID_CASES: &[&str] = &[
        "ml-2 mr-2",
        "pl-2 pr-2 pt-2 pb-2",
        "hover:w-4 hover:h-4",
        "border-x border-y",
        "overflow-hidden text-ellipsis whitespace-nowrap",
        "ml-2 mr-2 text-blue-500 font-bold",
        "mr-3 ml-3",
        "pt-1 pr-1 pb-1 pl-1",
        "border-x border-y overflow-hidden text-ellipsis whitespace-nowrap w-8 h-8",
        "hover:w-10 hover:h-10 bg-gray-100 p-4",
        "-mt-2 -mb-2",
        "pl-6 pr-6 py-6",
        "w-4 h-4",
        "pt-4 pr-2 pb-4 pl-4",
        "w-[10px] h-[10px]",
        "sm:hover:w-4 sm:hover:h-4",
        "size-4 w-4 h-4",
    ];

    const VALID_CASES: &[&str] = &[
        "mx-2 -my-2",
        "p-2 pl-4",
        "hover:size-4",
        "border",
        "truncate",
        "ml-2 mr-3",
        "hover:ml-2 focus:mr-2",
        "mx-2 mr-2",
        "pt-2 mr-3",
        "focus:w-4 hover:h-4",
        "w-[10px] h-[20px]",
        "w-screen h-screen",
    ];

    #[test]
    fn valid_cases() {
        let snapshot = render_cases(VALID_CASES, true);
        assert_snapshot!("valid_cases", snapshot);
    }

    #[test]
    fn invalid_cases() {
        let snapshot = render_cases(INVALID_CASES, false);
        assert_snapshot!("invalid_cases", snapshot);
    }

    fn render_cases(cases: &[&str], expect_no_violations: bool) -> String {
        let mut snapshot = String::new();

        for input in cases {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "failed to parse test input: {input}");

            let root = parse.tree();
            let violations = analyze_tailwind_shorthand(&root.candidates());
            if expect_no_violations {
                assert!(violations.is_empty(), "expected no violations for: {input}");
            } else {
                assert!(!violations.is_empty(), "expected violations for: {input}");
            }

            writeln!(snapshot, "## Input\n\n```text\n{input}\n```\n").unwrap();
            writeln!(snapshot, "Violations: {}", violations.len()).unwrap();

            for (index, violation) in violations.iter().enumerate() {
                writeln!(snapshot, "- Violation {}", index + 1).unwrap();
                writeln!(
                    snapshot,
                    "  nodes: {:?}",
                    violation
                        .uncompressed_nodes
                        .iter()
                        .map(|node| node.syntax().text_trimmed().to_string())
                        .collect::<Vec<_>>()
                )
                .unwrap();
                writeln!(
                    snapshot,
                    "  replacements: {:?}",
                    violation.replacement_bases
                )
                .unwrap();
                writeln!(
                    snapshot,
                    "  replace_whole_node: {}",
                    violation.replace_whole_node
                )
                .unwrap();
            }

            let fixed = auto_fix_all(&root, &violations).map_or_else(
                || input.to_string(),
                |mutation| mutation.commit().to_string(),
            );
            writeln!(snapshot, "Fixed: `{fixed}`\n").unwrap();
        }

        snapshot
    }
}
